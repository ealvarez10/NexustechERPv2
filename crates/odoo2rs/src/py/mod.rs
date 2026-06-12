//! FASE 1 (frontend Python): rustpython-parser → AST → OdooIR.

pub mod extract;
pub mod lit;
pub mod manifest;

pub use extract::{extract_models, Extraction};
pub use manifest::parse_manifest;
