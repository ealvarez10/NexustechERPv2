//! Error types para nexus-ledger

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LedgerError {
    #[error("CLABE inválida: {0}")]
    ClabeInvalida(String),

    #[error("Monto inválido: {0}")]
    MontoInvalido(String),

    #[error("Error SPEI/STP: {0}")]
    Spei(String),

    #[error("Error de red: {0}")]
    Red(#[from] reqwest::Error),

    #[error("Error de serialización: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Error contable: {0}")]
    Contable(String),
}
