//! nexus-cfdi — CFDI 4.0 en Rust
//!
//! El primer crate CFDI 4.0 completo en Rust.
//! Sella, genera XML y timbra CFDIs con PACs mexicanos (Facturama, SW Sapien).

pub mod builder;
pub mod cadena_original;
pub mod sellado;
pub mod pac;
pub mod error;

pub use builder::{CfdiBuilder, CfdiData, Emisor, Receptor, Concepto, Impuestos, Traslado, Retencion};
pub use error::CfdiError;
pub use pac::{Pac, TimbreResponse, CancelacionResponse};
pub use pac::facturama::FacturamaPac;
pub use pac::sw_sapien::SwSapienPac;

/// Versión CFDI soportada
pub const CFDI_VERSION: &str = "4.0";
/// Namespace CFDI 4.0
pub const CFDI_NS: &str = "http://www.sat.gob.mx/cfd/4";
