//! # odoo2rs — el transpilador Odoo → NexusTech ERP v2
//!
//! Implementa las FASES 1 y 2 del plan (`docs/transpilador-odoo-a-rust.md`)
//! y el esqueleto de la 3a:
//!
//! | Módulo      | Fase | Qué hace                                            |
//! |-------------|------|-----------------------------------------------------|
//! | [`py`]      | 1    | rustpython-parser → AST → extracción de modelos     |
//! | [`xml`]     | 1    | roxmltree → vistas/acciones/menús                   |
//! | [`ir`]      | 2    | OdooIR serializable — contrato con `nexus_orm::ir`  |
//! | [`codegen`] | 3a   | IR → fragmentos Rust (nexus-orm) y páginas JS       |
//!
//! El JSON de modelos que emite este crate se registra en caliente con
//! `nexus_orm::RegistryBuilder::register_ir_json` — sin recompilar el
//! kernel. El codegen Rust es la vía «nativizada»: mismos campos, más
//! stubs de métodos listos para la traducción FASE 3a.

pub mod codegen;
pub mod ir;
pub mod py;
pub mod xml;
