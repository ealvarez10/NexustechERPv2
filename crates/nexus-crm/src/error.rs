use thiserror::Error;

#[derive(Debug, Error)]
pub enum CrmError {
    #[error("Base de datos: {0}")]
    Db(#[from] sqlx::Error),
    #[error("No encontrado: {0}")]
    NoEncontrado(String),
}
