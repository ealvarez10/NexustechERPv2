//! nexus-core::db — Queries SQLx sobre el schema Odoo 19
//!
//! Cada submódulo implementa CRUD para una tabla core.
//! Compatible DROP-IN con bases de datos NexusTech.

pub mod partner;
pub mod product;
pub mod sale_order;
pub mod account_move;
pub mod user;
pub mod company;
