//! nexus-cfdi — CFDI 4.0 en Rust
//!
//! El primer crate CFDI 4.0 completo en Rust.
//! Sella, genera XML y timbra CFDIs con PACs mexicanos.

pub mod builder;
pub mod cadena_original;
pub mod catalogs;
pub mod complementos;
pub mod curp;
pub mod error;
pub mod pac;
pub mod pdf;
pub mod qr;
pub mod rfc;
pub mod sellado;
pub mod validacion;
pub mod xml;

pub use builder::{CfdiBuilder, CfdiData, Concepto, Emisor, Impuestos, Receptor, Retencion, Traslado};
pub use error::CfdiError;
pub use pac::{CancelacionResponse, Pac, TimbreResponse};
pub use pac::facturama::FacturamaPac;
pub use pac::sw_sapien::SwSapienPac;
pub use rfc::{Rfc, RFC_PUBLICO_GENERAL, RFC_EXTRANJERO};
pub use validacion::validar;
pub use pdf::{generar_pdf, OpcionesPdf, PdfCfdi};
pub use qr::generar_url_verificacion;

/// Versión CFDI soportada
pub const CFDI_VERSION: &str = "4.0";
/// Namespace CFDI 4.0
pub const CFDI_NS: &str = "http://www.sat.gob.mx/cfd/4";
