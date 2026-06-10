//! nexus-core::db — Acceso a datos del schema PostgreSQL de NexusTech ERP
//!
//! Cada submódulo implementa CRUD para una tabla core.
//! Compatible DROP-IN con bases de datos NexusTech ERP.

pub mod partner;
pub mod product;
pub mod sale_order;
pub mod account_move;
pub mod user;
pub mod company;
