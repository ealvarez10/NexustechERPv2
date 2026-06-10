use thiserror::Error;

#[derive(Debug, Error)]
pub enum InventoryError {
    #[error("Base de datos: {0}")]
    Db(#[from] sqlx::Error),
    #[error("Movimiento inválido: {0}")]
    Invalido(String),
}
