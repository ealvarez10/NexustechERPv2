//! Tipos de error de nexus-core

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Error de base de datos: {0}")]
    Db(#[from] sqlx::Error),

    #[error("Registro no encontrado: {0}")]
    NotFound(String),

    #[error("Configuración inválida: {0}")]
    Config(String),

    #[error("Error de autenticación: {0}")]
    Auth(String),

    #[error("Token inválido o expirado")]
    TokenInvalido,

    #[error("Acceso denegado: {0}")]
    Forbidden(String),

    #[error("Datos inválidos: {0}")]
    Validation(String),

    #[error("Error interno: {0}")]
    Internal(String),
}

impl CoreError {
    pub fn not_found(entidad: &str, id: impl std::fmt::Display) -> Self {
        CoreError::NotFound(format!("{} con id={} no encontrado", entidad, id))
    }
}
