//! AppState — Estado compartido de NexusTech ERP v2
//!
//! Contiene todos los recursos compartidos entre handlers:
//! - Pool de conexiones PostgreSQL
//! - Cliente Redis para caché
//! - Configuración del servidor

use nexus_core::config::Config;
use sqlx::PgPool;

/// Estado global de la aplicación — se clona por cada request (Arc interno)
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Config,
    // Redis: opcional en desarrollo
    pub redis: Option<redis::aio::MultiplexedConnection>,
}

impl AppState {
    pub fn nueva(db: PgPool, config: Config, redis: Option<redis::aio::MultiplexedConnection>) -> Self {
        Self { db, config, redis }
    }
}
