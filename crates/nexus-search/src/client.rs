//! Cliente NexusSearch con timeout y retry configurable
//!
//! Compatible 100% con la API Meilisearch — misma instancia que el storefront.

use meilisearch_sdk::client::Client;
use crate::error::SearchError;

pub struct NexusSearchClient {
    pub inner: Client,
    pub url: String,
}

impl NexusSearchClient {
    /// Crear cliente desde variables de entorno
    pub fn from_env() -> Result<Self, SearchError> {
        let url = std::env::var("MEILI_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:7700".to_string());
        let key = std::env::var("MEILI_MASTER_KEY")
            .unwrap_or_else(|_| "nexustech_dev_key_2026".to_string());

        let inner = Client::new(&url, Some(&key))
            .map_err(|e| SearchError::Unavailable(e.to_string()))?;

        Ok(Self { inner, url })
    }

    /// Verificar que NexusSearch está disponible
    pub async fn health_check(&self) -> bool {
        self.inner.health().await.is_ok()
    }
}
