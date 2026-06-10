//! nexus-ledger — Módulo de contabilidad y pagos electrónicos
//!
//! Incluye:
//! - SPEI/STP: Transferencias electrónicas interbancarias en México
//! - CLABE: Validación del número de cuenta normalizado
//! - Conciliación bancaria
//! - Tipos de movimiento contable

pub mod spei;
pub mod clabe;
pub mod conciliacion;
pub mod error;

pub use spei::{OrdenSpei, ResultadoSpei, ClienteStpConfig};
pub use clabe::{validar_clabe, info_banco_clabe};
pub use error::LedgerError;
