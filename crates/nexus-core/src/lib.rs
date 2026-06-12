//! nexus-core — Modelos y acceso a datos del schema PostgreSQL de NexusTech ERP
//!
//! Compatible con bases de datos NexusTech ERP: DROP-IN sin migración de schema.

pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod models;
pub mod payroll;

pub use config::Config;
pub use error::CoreError;
pub use auth::{Claims, DatosUsuario, TokenPair};
