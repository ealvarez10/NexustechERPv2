//! nexus-sale — Lógica de ventas: órdenes, líneas, precios y descuentos
//! Accede directamente al schema existente en PostgreSQL

pub mod db;
pub mod pricing;
pub mod error;

pub use error::SaleError;
pub use pricing::{aplicar_descuento, subtotal_linea, calcular_iva, calcular_totales, TotalesOrden};
