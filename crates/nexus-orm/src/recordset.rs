//! `Recordset` — la abstracción central de Odoo, reimplementada (§3.1).
//!
//! Estructuralmente es `(Env, ModelId, Vec<RecordId>)`, exactamente como
//! `odoo.models.BaseModel`: el Env aporta registro + caché + conexión, y
//! las operaciones (`mapped`, `filtered`, `sorted`, `browse`, `|`, `&`,
//! `-`, iteración por singletons) se implementan una sola vez aquí para
//! todos los modelos — nativos, transpilados o interpretados.

use std::collections::HashSet;
use std::ops::{BitAnd, BitOr, Sub};

use rust_decimal::Decimal;
use smol_str::SmolStr;

use crate::env::Env;
use crate::error::{OError, OResult};
use crate::model::ModelDef;
use crate::registry::CallCtx;
use crate::value::{ModelId, OVal, RecordId};

#[derive(Clone)]
pub struct Recordset {
    env: Env,
    model: ModelId,
    ids: Vec<RecordId>,
}

/// Resultado de `mapped()`: valores escalares o un recordset del comodelo.
pub enum Mapped {
    Values(Vec<OVal>),
    Records(Recordset),
}

impl Mapped {
    pub fn values(self) -> OResult<Vec<OVal>> {
        match self {
            Mapped::Values(v) => Ok(v),
            Mapped::Records(_) => Err(OError::Type {
                expected: "valores escalares",
                got: "Recordset",
            }),
        }
    }

    pub fn records(self) -> OResult<Recordset> {
        match self {
            Mapped::Records(r) => Ok(r),
            Mapped::Values(_) => Err(OError::Type {
                expected: "Recordset",
                got: "valores escalares",
            }),
        }
    }
}

impl Recordset {
    pub(crate) fn new(env: Env, model: ModelId, ids: Vec<RecordId>) -> Self {
        Recordset { env, model, ids }
    }

    // ─── Identidad ──────────────────────────────────────────────────────

    pub fn env(&self) -> &Env {
        &self.env
    }

    pub fn model_id(&self) -> ModelId {
        self.model
    }

    pub fn def(&self) -> &ModelDef {
        self.env.registry().def(self.model)
    }

    pub fn model_name(&self) -> &str {
        &self.def().name
    }

    pub fn ids(&self) -> &[RecordId] {
        &self.ids
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    /// `bool(recordset)` de Python.
    pub fn is_truthy(&self) -> bool {
        !self.is_empty()
    }

    /// `self.ensure_one()` de Odoo.
    pub fn ensure_one(&self) -> OResult<&Self> {
        if self.ids.len() == 1 {
            Ok(self)
        } else {
            Err(OError::EnsureOne(self.ids.len()))
        }
    }

    /// `record.id` (exige singleton).
    pub fn id(&self) -> OResult<RecordId> {
        self.ensure_one()?;
        Ok(self.ids[0])
    }

    // ─── Construcción derivada ──────────────────────────────────────────

    /// `self.browse(ids)` — mismo modelo, otros ids.
    pub fn browse(&self, ids: Vec<RecordId>) -> Recordset {
        Recordset::new(self.env.clone(), self.model, ids)
    }

    /// Iteración por singletons: `for order in orders`.
    pub fn iter(&self) -> impl Iterator<Item = Recordset> + '_ {
        self.ids.iter().map(move |&id| self.browse(vec![id]))
    }

    /// `recordset[i]` (singleton).
    pub fn at(&self, i: usize) -> OResult<Recordset> {
        self.ids
            .get(i)
            .map(|&id| self.browse(vec![id]))
            .ok_or_else(|| OError::key(format!("índice {i} fuera de rango (len {})", self.len())))
    }

    /// `recordset[:1]` — vacío si no hay registros (nunca falla).
    pub fn first(&self) -> Recordset {
        self.browse(self.ids.first().map(|&i| vec![i]).unwrap_or_default())
    }

    // ─── Lectura ────────────────────────────────────────────────────────

    /// Lee un campo del singleton, con fetch a BD si no está en caché.
    pub async fn get(&self, field: &str) -> OResult<OVal> {
        self.ensure_one()?;
        self.def().field(field)?;
        if let Some(v) = self.env.cache_get(self.model, self.ids[0], field) {
            return Ok(v);
        }
        self.env
            .fetch_into_cache(self.model, &self.ids, &[field.to_string()])
            .await?;
        self.env
            .cache_get(self.model, self.ids[0], field)
            .ok_or_else(|| {
                OError::key(format!(
                    "registro {}({}) inexistente o campo '{field}' ilegible",
                    self.model_name(),
                    self.ids[0]
                ))
            })
    }

    /// Lectura síncrona desde caché (el camino del código transpilado tras
    /// un prefetch). Error si el campo no fue cargado.
    pub fn cached(&self, field: &str) -> OResult<OVal> {
        self.ensure_one()?;
        self.def().field(field)?;
        self.env
            .cache_get(self.model, self.ids[0], field)
            .ok_or_else(|| {
                OError::key(format!(
                    "'{}.{field}' no está en caché; llama a read()/get() primero",
                    self.model_name()
                ))
            })
    }

    /// Azúcar tipada sobre `cached()` con la coerción falsy de Odoo
    /// (`Null` → `""`/`0`/`0.0`).
    pub fn get_str(&self, field: &str) -> OResult<SmolStr> {
        self.cached(field)?.as_str()
    }

    pub fn get_int(&self, field: &str) -> OResult<i64> {
        self.cached(field)?.as_int()
    }

    pub fn get_bool(&self, field: &str) -> OResult<bool> {
        Ok(self.cached(field)?.is_truthy())
    }

    pub fn get_decimal(&self, field: &str) -> OResult<Decimal> {
        self.cached(field)?.as_decimal()
    }

    /// Prefetch explícito de columnas para todo el recordset.
    pub async fn read(&self, fields: &[&str]) -> OResult<()> {
        let fields: Vec<String> = fields.iter().map(|f| f.to_string()).collect();
        self.env
            .fetch_into_cache(self.model, &self.ids, &fields)
            .await
    }

    // ─── Escritura ──────────────────────────────────────────────────────

    /// `record.field = value` — aplica a TODOS los registros del set,
    /// como la asignación de Odoo. Dispara computes dependientes.
    pub async fn set(&self, field: &str, value: impl Into<OVal>) -> OResult<()> {
        self.env
            .write_values(self, &[(field.to_string(), value.into())])
            .await
    }

    /// `records.write({...})`.
    pub async fn write(&self, vals: Vec<(String, OVal)>) -> OResult<()> {
        self.env.write_values(self, &vals).await
    }

    // ─── Despacho dinámico ──────────────────────────────────────────────

    /// Invoca un método por la cadena `_inherit` (más derivado primero).
    /// Equivale al despacho de Odoo; `super()` es `ctx.call_super(...)`
    /// dentro del fragmento.
    pub async fn call(&self, method: &str, args: &[OVal]) -> OResult<OVal> {
        let chain = self.env.registry().method_chain(self.model, method)?;
        let frag = chain
            .first()
            .cloned()
            .ok_or_else(|| OError::key(format!("cadena vacía para '{method}'")))?;
        let ctx = CallCtx::new(method, chain);
        frag.call(&self.env, &ctx, self, args).await
    }

    // ─── Operaciones de recordset ───────────────────────────────────────

    /// `records.mapped('field')` — escalares o recordset del comodelo.
    /// Opera sobre caché (prefetch previo con `read()`).
    pub fn mapped(&self, field: &str) -> OResult<Mapped> {
        let fdef = self.def().field(field)?;
        if let Some(comodel) = fdef.ftype.comodel() {
            let comodel_id = self.env.registry().model_id(comodel)?;
            let mut seen = HashSet::new();
            let mut ids = Vec::new();
            for rec in self.iter() {
                match rec.cached(field)? {
                    OVal::Null => {}
                    OVal::Ref(_, id) => {
                        if seen.insert(id) {
                            ids.push(id);
                        }
                    }
                    OVal::RefSet(_, set_ids) => {
                        for id in set_ids {
                            if seen.insert(id) {
                                ids.push(id);
                            }
                        }
                    }
                    other => {
                        return Err(OError::Type {
                            expected: "Ref/RefSet",
                            got: other.type_name(),
                        })
                    }
                }
            }
            Ok(Mapped::Records(Recordset::new(
                self.env.clone(),
                comodel_id,
                ids,
            )))
        } else {
            let vals: Vec<OVal> = self
                .iter()
                .map(|rec| rec.cached(field))
                .collect::<OResult<_>>()?;
            Ok(Mapped::Values(vals))
        }
    }

    /// `records.filtered(lambda r: ...)`.
    pub fn filtered(&self, pred: impl Fn(&Recordset) -> bool) -> Recordset {
        let ids = self
            .iter()
            .filter(|r| pred(r))
            .map(|r| r.ids[0])
            .collect();
        self.browse(ids)
    }

    /// `records.sorted(key='field')` sobre caché.
    pub fn sorted(&self, field: &str, reverse: bool) -> OResult<Recordset> {
        let mut keyed: Vec<(OVal, RecordId)> = self
            .iter()
            .map(|r| Ok((r.cached(field)?, r.ids[0])))
            .collect::<OResult<_>>()?;
        keyed.sort_by(|a, b| a.0.cmp_loose(&b.0));
        if reverse {
            keyed.reverse();
        }
        Ok(self.browse(keyed.into_iter().map(|(_, id)| id).collect()))
    }

    /// Unión preservando orden (el `|` de Odoo). Error si los modelos
    /// difieren (TypeError en Odoo).
    pub fn union(&self, other: &Recordset) -> OResult<Recordset> {
        self.check_same_model(other)?;
        let mut seen: HashSet<RecordId> = HashSet::new();
        let mut ids = Vec::new();
        for &id in self.ids.iter().chain(other.ids.iter()) {
            if seen.insert(id) {
                ids.push(id);
            }
        }
        Ok(self.browse(ids))
    }

    /// Diferencia (el `-` de Odoo).
    pub fn minus(&self, other: &Recordset) -> OResult<Recordset> {
        self.check_same_model(other)?;
        let exclude: HashSet<RecordId> = other.ids.iter().copied().collect();
        Ok(self.browse(
            self.ids
                .iter()
                .copied()
                .filter(|id| !exclude.contains(id))
                .collect(),
        ))
    }

    /// Intersección (el `&` de Odoo).
    pub fn intersect(&self, other: &Recordset) -> OResult<Recordset> {
        self.check_same_model(other)?;
        let keep: HashSet<RecordId> = other.ids.iter().copied().collect();
        Ok(self.browse(
            self.ids
                .iter()
                .copied()
                .filter(|id| keep.contains(id))
                .collect(),
        ))
    }

    /// `record in records`.
    pub fn contains(&self, other: &Recordset) -> bool {
        self.model == other.model && other.ids.iter().all(|id| self.ids.contains(id))
    }

    fn check_same_model(&self, other: &Recordset) -> OResult<()> {
        if self.model != other.model {
            return Err(OError::Type {
                expected: "recordsets del mismo modelo",
                got: "modelos distintos",
            });
        }
        Ok(())
    }
}

impl std::fmt::Debug for Recordset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:?}", self.model_name(), self.ids)
    }
}

// Operadores estilo Odoo. Entre modelos distintos hacen panic con mensaje
// claro (el TypeError de Python); para manejo de error usar
// union()/minus()/intersect().

impl BitOr for &Recordset {
    type Output = Recordset;
    fn bitor(self, rhs: &Recordset) -> Recordset {
        self.union(rhs).expect("`|` entre recordsets de modelos distintos")
    }
}

impl Sub for &Recordset {
    type Output = Recordset;
    fn sub(self, rhs: &Recordset) -> Recordset {
        self.minus(rhs).expect("`-` entre recordsets de modelos distintos")
    }
}

impl BitAnd for &Recordset {
    type Output = Recordset;
    fn bitand(self, rhs: &Recordset) -> Recordset {
        self.intersect(rhs)
            .expect("`&` entre recordsets de modelos distintos")
    }
}
