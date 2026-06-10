use thiserror::Error;

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("NexusSearch no disponible: {0}")]
    Unavailable(String),
    #[error("Error de indexación: {0}")]
    Indexing(String),
    #[error("Error HTTP: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Error de serialización: {0}")]
    Serde(#[from] serde_json::Error),
}
