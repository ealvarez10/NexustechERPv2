//! Complementos fiscales CFDI 4.0
//! Todos implementados en Rust — ninguno existía antes en el ecosistema.

pub mod pago20;
pub mod carta_porte31;
pub mod nomina12;

pub use pago20::ComplementoPago;
