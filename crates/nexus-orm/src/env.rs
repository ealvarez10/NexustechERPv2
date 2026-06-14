//! `Env` — el `odoo.api.Environment`: registro + conexión + contexto de
//! usuario + caché de campos por transacción.
//!
//! Es clonable barato (Arc interno); cada Recordset lleva el suyo, igual
//! que `self.env` en Odoo. Tiene dos modos:
//!  - **conectado** (`Env::new` con un `PgPool`): search/read/write/create
//!    ejecutan SQL real sobre el esquema Odoo existente;
//!  - **prototipo** (`Env::mock`): todo opera contra la caché en memoria —
//!    suficiente para tests del kernel, demos y el harness diferencial
//!    antes de tocar la BD.
//!
//! Simplificación v0 deliberada: las escrituras son *write-through*
//! (UPDATE inmediato + caché). El flush diferido con cola de invalidación
//! estilo Odoo llega cuando se cablee el grafo de computes cross-model.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use smol_str::SmolStr;
use sqlx::PgPool;

use crate::domain::Domain;
use crate::error::{OError, OResult};
use crate::recordset::Recordset;
use crate::registry::Registry;
use crate::sql;
use crate::value::{ModelId, OVal, RecordId};

/// Contexto de usuario — el `env.context`/`env.uid` de Odoo, alimentado
/// por el `JwtClaims` del middleware actual.
#[derive(Debug, Clone)]
pub struct EnvCtx {
    pub uid: i64,
    pub company_id: Option<i64>,
    pub lang: String,
    /// Superusuario (`sudo()`).
    pub su: bool,
}

impl Default for EnvCtx {
    fn default() -> Self {
        EnvCtx {
            uid: 1,
            company_id: None,
            lang: "es_MX".to_string(),
            su: false,
        }
    }
}

#[derive(Default)]
struct Cache {
    /// (modelo, id) → campo → valor. La caché de recordsets de Odoo.
    recs: HashMap<(ModelId, RecordId), HashMap<SmolStr, OVal>>,
    /// Secuencias para `create()` en modo prototipo.
    next_mock_id: HashMap<ModelId, RecordId>,
}

struct EnvInner {
    registry: Arc<Registry>,
    pool: Option<PgPool>,
    ctx: EnvCtx,
    cache: Mutex<Cache>,
}

#[derive(Clone)]
pub struct Env {
    inner: Arc<EnvInner>,
}

impl Env {
    pub fn new(registry: Arc<Registry>, pool: PgPool, ctx: EnvCtx) -> Env {
        Env {
            inner: Arc::new(EnvInner {
                registry,
                pool: Some(pool),
                ctx,
                cache: Mutex::new(Cache::default()),
            }),
        }
    }

    /// Env de prototipo: sin Postgres, todo en memoria.
    pub fn mock(registry: Arc<Registry>) -> Env {
        Env {
            inner: Arc::new(EnvInner {
                registry,
                pool: None,
                ctx: EnvCtx::default(),
                cache: Mutex::new(Cache::default()),
            }),
        }
    }

    pub fn registry(&self) -> &Registry {
        &self.inner.registry
    }

    pub fn ctx(&self) -> &EnvCtx {
        &self.inner.ctx
    }

    pub fn has_pool(&self) -> bool {
        self.inner.pool.is_some()
    }

    fn pool(&self) -> OResult<&PgPool> {
        self.inner.pool.as_ref().ok_or(OError::NoPool)
    }

    fn cache(&self) -> MutexGuard<'_, Cache> {
        // El guard nunca cruza un .await (invariante de este módulo).
        self.inner.cache.lock().expect("nexus-orm: caché envenenada")
    }

    // ─── Construcción de recordsets ─────────────────────────────────────

    /// `env['sale.order']` — recordset vacío del modelo.
    pub fn model(&self, name: &str) -> OResult<Recordset> {
        let mid = self.registry().model_id(name)?;
        Ok(Recordset::new(self.clone(), mid, Vec::new()))
    }

    /// `env['sale.order'].browse(ids)`.
    pub fn browse(&self, name: &str, ids: Vec<RecordId>) -> OResult<Recordset> {
        let mid = self.registry().model_id(name)?;
        Ok(Recordset::new(self.clone(), mid, ids))
    }

    // ─── Caché ──────────────────────────────────────────────────────────

    pub fn cache_get(&self, model: ModelId, id: RecordId, field: &str) -> Option<OVal> {
        self.cache()
            .recs
            .get(&(model, id))
            .and_then(|m| m.get(field))
            .cloned()
    }

    pub fn cache_put(&self, model: ModelId, id: RecordId, field: &str, v: OVal) {
        self.cache()
            .recs
            .entry((model, id))
            .or_default()
            .insert(field.into(), v);
    }

    /// Siembra la caché de un registro — para tests, fixtures y el harness
    /// diferencial. En modo prototipo equivale a "cargar datos".
    pub fn seed(&self, model: &str, id: RecordId, vals: Vec<(&str, OVal)>) -> OResult<()> {
        let mid = self.registry().model_id(model)?;
        let def = self.registry().def(mid);
        let mut cache = self.cache();
        let rec = cache.recs.entry((mid, id)).or_default();
        rec.insert("id".into(), OVal::Int(id));
        for (f, v) in vals {
            if !def.has_field(f) {
                return Err(OError::unknown_field(&def.name, f));
            }
            rec.insert(f.into(), v);
        }
        Ok(())
    }

    /// Vacía la caché (fin de transacción).
    pub fn invalidate_cache(&self) {
        self.cache().recs.clear();
    }

    // ─── ORM: search / read / write / create ────────────────────────────

    /// `search()`: compila el dominio a SQL y devuelve el recordset.
    pub async fn search(
        &self,
        model: &str,
        domain: &Domain,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> OResult<Recordset> {
        let mid = self.registry().model_id(model)?;
        let def = self.registry().def(mid);
        let w = domain.to_sql(def, 1)?;

        let mut sql_text = format!(
            "SELECT id FROM {} WHERE {} ORDER BY {}",
            def.table, w.sql, def.order
        );
        if let Some(l) = limit {
            sql_text.push_str(&format!(" LIMIT {l}"));
        }
        if let Some(o) = offset {
            sql_text.push_str(&format!(" OFFSET {o}"));
        }

        let mut q = sqlx::query(&sql_text);
        for (ft, v) in &w.params {
            q = sql::bind_typed(q, ft, v)?;
        }
        let rows = q.fetch_all(self.pool()?).await?;

        let mut ids = Vec::with_capacity(rows.len());
        for row in &rows {
            ids.push(match sql::decode_field(
                row,
                0,
                def.field("id")?,
                self.registry(),
            )? {
                OVal::Int(i) => i,
                other => {
                    return Err(OError::Internal(format!(
                        "id no entero en search(): {other:?}"
                    )))
                }
            });
        }
        Ok(Recordset::new(self.clone(), mid, ids))
    }

    /// Lee columnas de la BD y las deja en caché (el `read()`/prefetch).
    pub async fn fetch_into_cache(
        &self,
        model: ModelId,
        ids: &[RecordId],
        fields: &[String],
    ) -> OResult<()> {
        if ids.is_empty() || fields.is_empty() {
            return Ok(());
        }
        let def = self.registry().def(model);

        let mut cols: Vec<&crate::fields::FieldDef> = Vec::with_capacity(fields.len());
        for f in fields {
            let fdef = def.field(f)?;
            if !fdef.is_column() {
                return Err(OError::key(format!(
                    "'{}.{}' no es columna almacenada; lectura x2many/computado-no-almacenado \
                     pendiente en v0",
                    def.name, f
                )));
            }
            cols.push(fdef);
        }

        let col_list: Vec<&str> = cols.iter().map(|c| c.name.as_str()).collect();
        let sql_text = format!(
            "SELECT id, {} FROM {} WHERE id = ANY($1)",
            col_list.join(", "),
            def.table
        );
        let pg_ids: Vec<i32> = ids
            .iter()
            .map(|&i| i32::try_from(i).map_err(|_| OError::Internal(format!("id {i} fuera de int4"))))
            .collect::<OResult<_>>()?;

        let rows = sqlx::query(&sql_text)
            .bind(&pg_ids)
            .fetch_all(self.pool()?)
            .await?;

        for row in &rows {
            let id = match sql::decode_field(row, 0, def.field("id")?, self.registry())? {
                OVal::Int(i) => i,
                other => return Err(OError::Internal(format!("id inválido: {other:?}"))),
            };
            let mut cache = self.cache();
            let rec = cache.recs.entry((model, id)).or_default();
            rec.insert("id".into(), OVal::Int(id));
            drop(cache);
            for (i, fdef) in cols.iter().enumerate() {
                let v = sql::decode_field(row, i + 1, fdef, self.registry())?;
                self.cache_put(model, id, &fdef.name, v);
            }
        }
        Ok(())
    }

    /// Escritura de valores: UPDATE (si hay pool y columnas) + caché +
    /// disparo de recálculo de computados dependientes.
    pub async fn write_values(
        &self,
        rs: &Recordset,
        vals: &[(String, OVal)],
    ) -> OResult<()> {
        if rs.is_empty() || vals.is_empty() {
            return Ok(());
        }
        let model = rs.model_id();
        let entry = self.registry().entry(model);
        let def = &entry.def;

        for (f, _) in vals {
            def.field(f)?; // valida existencia
        }

        // SQL write-through sobre columnas físicas.
        let sql_cols: Vec<(&crate::fields::FieldDef, &OVal)> = vals
            .iter()
            .filter_map(|(f, v)| {
                let fd = def.fields.get(f)?;
                fd.is_column().then_some((fd, v))
            })
            .collect();

        if self.has_pool() && !sql_cols.is_empty() {
            let mut sets = Vec::with_capacity(sql_cols.len());
            let mut n = 1usize;
            for (fd, v) in &sql_cols {
                if matches!(v, OVal::Null) {
                    sets.push(format!("{} = NULL", fd.name));
                } else {
                    sets.push(format!("{} = ${n}", fd.name));
                    n += 1;
                }
            }
            let sql_text = format!(
                "UPDATE {} SET {} WHERE id = ANY(${n})",
                def.table,
                sets.join(", ")
            );
            let mut q = sqlx::query(&sql_text);
            for (fd, v) in &sql_cols {
                if !matches!(v, OVal::Null) {
                    q = sql::bind_typed(q, &fd.ftype, v)?;
                }
            }
            let pg_ids: Vec<i32> = rs
                .ids()
                .iter()
                .map(|&i| {
                    i32::try_from(i)
                        .map_err(|_| OError::Internal(format!("id {i} fuera de int4")))
                })
                .collect::<OResult<_>>()?;
            q = q.bind(pg_ids);
            q.execute(self.pool()?).await?;
        }

        // Caché.
        {
            let mut cache = self.cache();
            for &id in rs.ids() {
                let rec = cache.recs.entry((model, id)).or_default();
                rec.insert("id".into(), OVal::Int(id));
                for (f, v) in vals {
                    rec.insert(f.as_str().into(), v.clone());
                }
            }
        }

        // Recalcular computados dependientes (@api.depends intra-modelo).
        let methods = Self::triggers_for(entry, vals.iter().map(|(f, _)| f.as_str()));
        for m in methods {
            rs.call(&m, &[]).await?;
        }
        Ok(())
    }

    /// `create()`: INSERT (o alta en memoria en modo prototipo) + computes.
    pub async fn create(&self, model: &str, vals: Vec<(String, OVal)>) -> OResult<Recordset> {
        let mid = self.registry().model_id(model)?;
        let entry = self.registry().entry(mid);
        let def = &entry.def;

        // Defaults declarados que no vienen en vals.
        let mut all_vals = vals;
        for fdef in def.fields.values() {
            if let Some(d) = &fdef.default {
                if !all_vals.iter().any(|(f, _)| f == &fdef.name) {
                    all_vals.push((fdef.name.clone(), d.clone()));
                }
            }
        }
        for (f, _) in &all_vals {
            def.field(f)?;
        }

        let id: RecordId = if self.has_pool() {
            let cols: Vec<(&crate::fields::FieldDef, &OVal)> = all_vals
                .iter()
                .filter_map(|(f, v)| {
                    let fd = def.fields.get(f)?;
                    (fd.is_column() && fd.name != "id").then_some((fd, v))
                })
                .collect();

            let mut names = Vec::with_capacity(cols.len());
            let mut holes = Vec::with_capacity(cols.len());
            let mut n = 1usize;
            for (fd, v) in &cols {
                names.push(fd.name.as_str());
                if matches!(v, OVal::Null) {
                    holes.push("NULL".to_string());
                } else {
                    holes.push(format!("${n}"));
                    n += 1;
                }
            }
            let sql_text = format!(
                "INSERT INTO {} ({}) VALUES ({}) RETURNING id",
                def.table,
                names.join(", "),
                holes.join(", ")
            );
            let mut q = sqlx::query(&sql_text);
            for (fd, v) in &cols {
                if !matches!(v, OVal::Null) {
                    q = sql::bind_typed(q, &fd.ftype, v)?;
                }
            }
            let row = q.fetch_one(self.pool()?).await?;
            match sql::decode_field(&row, 0, def.field("id")?, self.registry())? {
                OVal::Int(i) => i,
                other => return Err(OError::Internal(format!("id inválido: {other:?}"))),
            }
        } else {
            // Modo prototipo: secuencia en memoria.
            let mut cache = self.cache();
            let next = cache.next_mock_id.entry(mid).or_insert(1);
            let id = *next;
            *next += 1;
            id
        };

        // Sembrar caché con los valores creados.
        {
            let mut cache = self.cache();
            let rec = cache.recs.entry((mid, id)).or_default();
            rec.insert("id".into(), OVal::Int(id));
            for (f, v) in &all_vals {
                rec.insert(f.as_str().into(), v.clone());
            }
        }

        let rs = Recordset::new(self.clone(), mid, vec![id]);

        // Computes sobre el registro recién creado.
        let methods = Self::triggers_for(entry, all_vals.iter().map(|(f, _)| f.as_str()));
        for m in methods {
            rs.call(&m, &[]).await?;
        }
        Ok(rs)
    }

    /// Métodos compute disparados por la escritura de `fields` (dedup,
    /// orden de primera aparición).
    fn triggers_for<'a>(
        entry: &crate::registry::ModelEntry,
        fields: impl Iterator<Item = &'a str>,
    ) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for f in fields {
            if let Some(methods) = entry.recompute_triggers.get(f) {
                for m in methods {
                    if !out.contains(m) {
                        out.push(m.clone());
                    }
                }
            }
        }
        out
    }
}
