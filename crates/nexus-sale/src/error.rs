use thiserror::Error;

#[derive(Debug, Error)]
pub enum SaleError {
    #[error("Base de datos: {0}")]
    Db(#[from] sqlx::Error),
    #[error("Precio inválido: {0}")]
    PrecioInvalido(String),
    #[error("Descuento inválido: debe estar entre 0 y 100")]
    DescuentoInvalido,
    #[error("Orden no encontrada: {0}")]
    NoEncontrada(i32),
}
