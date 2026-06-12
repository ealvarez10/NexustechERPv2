//! FASE 3a (esqueleto): generación de código desde el IR.
//!
//! - [`rust_gen`]: `ModelIr` → módulo Rust con un `ModelFragment` de
//!   nexus-orm (campos completos; métodos como stubs `TODO` a traducir).
//! - [`js_gen`]: `ViewIr` → descriptor JSON + página JS que alimenta los
//!   componentes existentes del frontend (`form_view.js`, tablas).

pub mod js_gen;
pub mod rust_gen;
