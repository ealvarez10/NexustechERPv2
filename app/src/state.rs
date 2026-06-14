//! AppState — Estado compartido de NexusTech ERP v2
//!
//! Contiene todos los recursos compartidos entre handlers:
//! - Pool de conexiones PostgreSQL
//! - Cliente Redis para caché
//! - Configuración del servidor
//! - PAC para timbrado CFDI
//! - Cliente del motor de búsqueda

use nexus_core::config::Config;
use sqlx::PgPool;
use std::sync::Arc;

/// Estado global de la aplicación — se clona por cada request (Arc interno en cada campo)
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Config,
    /// Redis: opcional en desarrollo
    #[allow(dead_code)]
    pub redis: Option<redis::aio::MultiplexedConnection>,
    /// PAC para timbrado y cancelación de CFDIs
    pub pac: Arc<dyn nexus_cfdi::Pac>,
    /// Cliente del motor de búsqueda integrado
    pub search_client: Arc<nexus_search::NexusSearchClient>,
    /// ORM Registry (kernel de la migración)
    #[allow(dead_code)]
    pub registry: Option<Arc<nexus_orm::registry::Registry>>,
}

impl AppState {
    pub fn nueva(
        db: PgPool,
        config: Config,
        redis: Option<redis::aio::MultiplexedConnection>,
        pac: Arc<dyn nexus_cfdi::Pac>,
        search_client: Arc<nexus_search::NexusSearchClient>,
        registry: Option<Arc<nexus_orm::registry::Registry>>,
    ) -> Self {
        Self { db, config, redis, pac, search_client, registry }
    }
}
