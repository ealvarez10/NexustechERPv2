//! nexus-pos — Punto de Venta
//! Órdenes en mostrador, sesiones de caja, carrito en memoria

pub mod db;
pub mod sesion;
pub mod error;

pub use error::PosError;
pub use sesion::{Carrito, ItemCarrito};
