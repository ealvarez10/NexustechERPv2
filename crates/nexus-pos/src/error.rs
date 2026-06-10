use thiserror::Error;

#[derive(Debug, Error)]
pub enum PosError {
    #[error("Base de datos: {0}")]
    Db(#[from] sqlx::Error),
    #[error("Sesión inválida: {0}")]
    SesionInvalida(String),
    #[error("Pago insuficiente: recibido {recibido}, total {total}")]
    PagoInsuficiente { recibido: String, total: String },
}
