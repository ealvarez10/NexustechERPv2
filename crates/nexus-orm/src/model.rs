//! Definición de modelos y el trait `ModelFragment` (§3.2 del reporte).
//!
//! En Odoo un modelo final («sale.order») es la combinación de N clases
//! Python repartidas en módulos, unidas por `_name`/`_inherit` y el MRO.
//! Aquí cada una de esas clases se vuelve un **fragmento**: declara campos
//! sobre el `ModelDef` compartido y aporta implementaciones de métodos.
//! El `Registry` encadena los fragmentos en una vtable (más derivado
//! primero) y `CallCtx::call_super` reproduce el `super()` de Python.

use std::collections::BTreeMap;

use async_trait::async_trait;

use crate::env::Env;
use crate::error::{OError, OResult};
use crate::fields::FieldDef;
use crate::recordset::Recordset;
use crate::registry::CallCtx;
use crate::value::OVal;

/// Definición acumulada de un modelo: el resultado de aplicar todos sus
/// fragmentos en orden de carga de módulos.
#[derive(Debug, Clone)]
pub struct ModelDef {
    /// `_name` de Odoo: "sale.order".
    pub name: String,
    /// Tabla física: "sale_order" (el esquema ya existe — es el de Odoo).
    pub table: String,
    /// `_description`.
    pub description: String,
    /// `_order`.
    pub order: String,
    /// `_rec_name`.
    pub rec_name: String,
    pub fields: BTreeMap<String, FieldDef>,
}

impl ModelDef {
    pub fn new(name: &str) -> Self {
        let mut def = ModelDef {
            name: name.to_string(),
            table: name.replace('.', "_"),
            description: name.to_string(),
            order: "id".to_string(),
            rec_name: "name".to_string(),
            fields: BTreeMap::new(),
        };
        // Todo modelo Odoo tiene `id` implícito.
        def.add_field(FieldDef::integer("id").readonly());
        def
    }

    /// Agrega o **sobreescribe** un campo (así es como un módulo heredado
    /// redefine un campo del módulo base).
    pub fn add_field(&mut self, f: FieldDef) {
        self.fields.insert(f.name.clone(), f);
    }

    pub fn field(&self, name: &str) -> OResult<&FieldDef> {
        self.fields
            .get(name)
            .ok_or_else(|| OError::unknown_field(&self.name, name))
    }

    pub fn has_field(&self, name: &str) -> bool {
        self.fields.contains_key(name)
    }

    /// Campos que son columnas físicas (para SELECT/INSERT/UPDATE).
    pub fn stored_columns(&self) -> impl Iterator<Item = &FieldDef> {
        self.fields.values().filter(|f| f.is_column())
    }
}

/// Un fragmento de modelo: la unidad que el transpilador genera por cada
/// clase Python, y que el código manual también puede implementar.
///
/// El registro es dinámico (en arranque), como en Odoo: el orden de carga
/// de módulos define qué fragmento «gana» en campos y quién es el más
/// derivado en la cadena de métodos.
#[async_trait]
pub trait ModelFragment: Send + Sync {
    /// `_name` del modelo al que pertenece este fragmento.
    fn model_name(&self) -> &str;

    /// Módulo Odoo de origen (define el orden de carga vía `depends`).
    fn module(&self) -> &str {
        "base"
    }

    /// `true` si es un fragmento `_inherit` puro (extiende, no define).
    /// El Registry valida que exista un fragmento base para el modelo.
    fn is_extension(&self) -> bool {
        false
    }

    /// Declara/sobreescribe campos y atributos (`_order`, `_description`…)
    /// sobre la definición acumulada.
    fn build(&self, _def: &mut ModelDef) {}

    /// Métodos que este fragmento implementa (entradas de la vtable).
    fn methods(&self) -> Vec<&str> {
        Vec::new()
    }

    /// Despacho dinámico: ejecuta `ctx.method` sobre `rs`.
    /// `ctx.call_super(rs, args)` invoca el siguiente eslabón de la cadena
    /// `_inherit` — el `super()` de Python.
    async fn call(
        &self,
        _env: &Env,
        ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(format!(
            "fragmento de '{}' declara el método '{}' pero no lo implementa",
            self.model_name(),
            ctx.method()
        )))
    }
}
