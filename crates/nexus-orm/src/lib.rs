//! # nexus-orm — el kernel ORM compatible con Odoo (la «rueda reinventada»)
//!
//! Componente A del plan `odoo2rs` (ver `docs/transpilador-odoo-a-rust.md`):
//! en lugar de traducir Odoo al paradigma de NexusTech, este crate trae el
//! paradigma de Odoo a Rust. Con este kernel, el transpilador deja de
//! traducir paradigmas y pasa a traducir *sintaxis* Python a llamadas
//! equivalentes — y eso sí es mecanizable al 100 %.
//!
//! ## Mapa del crate ↔ reporte técnico
//!
//! | Módulo        | Pieza del reporte                                          |
//! |---------------|------------------------------------------------------------|
//! | [`value`]     | §3.1 — `OVal`, tipado dinámico controlado                  |
//! | [`recordset`] | §3.1 — `Recordset` = `(Env, ModelId, Vec<RecordId>)`        |
//! | [`fields`]    | §3.3 — metadatos `fields.*`, `compute=` + `@api.depends`   |
//! | [`model`]     | §3.2 — `ModelDef` acumulado + trait `ModelFragment`        |
//! | [`registry`]  | §3.2 — registro en arranque, vtable `_inherit`, `super()`  |
//! | [`domain`]    | §3.3 — dominios `[('state','=','sale')]` → SQL sqlx        |
//! | [`ir`]        | FASE 2 — parseo de IR declarativo y registro dinámico      |
//! | [`env`]       | `Environment`: contexto, caché por transacción, CRUD       |
//! | [`sql`]       | puente `OVal` ↔ Postgres sobre el esquema Odoo existente   |
//!
//! ## Ejemplo (modo prototipo, sin BD)
//!
//! ```rust
//! use std::sync::Arc;
//! use nexus_orm::prelude::*;
//!
//! # fn main() -> OResult<()> {
//! // Registro dinámico desde IR declarativo (lo que emitirá odoo2rs FASE 2)
//! let registry = Arc::new(
//!     RegistryBuilder::new()
//!         .module("demo", &[])
//!         .register_ir_json(r#"{
//!             "model": "demo.task",
//!             "module": "demo",
//!             "fields": [
//!                 {"name": "name", "type": "char", "required": true},
//!                 {"name": "state", "type": "selection",
//!                  "selection": [["open","Abierta"],["done","Hecha"]]}
//!             ]
//!         }"#)?
//!         .build()?,
//! );
//!
//! let env = Env::mock(registry);
//! env.seed("demo.task", 1, vec![("name", "Migrar ventas".into()),
//!                               ("state", "open".into())])?;
//! let task = env.browse("demo.task", vec![1])?;
//! assert_eq!(task.get_str("state")?, "open");
//! # Ok(())
//! # }
//! ```
//!
//! ## Qué es v0 y qué falta (fronteras explícitas)
//!
//! - Escritura *write-through* (UPDATE inmediato); el flush diferido con
//!   cola de invalidación llega con el grafo de computes cross-model.
//! - Dominios sin rutas con punto (`partner_id.country_id`) todavía.
//! - `ir.model.access` / record rules: el `EnvCtx` ya transporta
//!   uid/company_id; la aplicación de reglas se integra en `search/read/
//!   write` en la siguiente iteración (§3.4).
//! - Lectura de one2many/many2many vía búsqueda inversa: pendiente.

pub mod domain;
pub mod env;
pub mod error;
pub mod fields;
pub mod ir;
pub mod model;
pub mod recordset;
pub mod registry;
pub mod sql;
pub mod value;

/// Lo que un fragmento transpilado (o escrito a mano) necesita importar.
pub mod prelude {
    pub use crate::domain::Domain;
    pub use crate::env::{Env, EnvCtx};
    pub use crate::error::{OError, OResult};
    pub use crate::fields::{ComputeDef, FieldDef, FieldType};
    pub use crate::ir::{parse_ir, IrFragment, ModelIr};
    pub use crate::model::{ModelDef, ModelFragment};
    pub use crate::recordset::{Mapped, Recordset};
    pub use crate::registry::{CallCtx, Registry, RegistryBuilder};
    pub use crate::value::{ModelId, OVal, RecordId};
    pub use async_trait::async_trait;
}
