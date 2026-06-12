//! Errores del kernel ORM.
//!
//! Reproducen la taxonomía de excepciones de Odoo (`UserError`,
//! `ValidationError`, `AccessError`, `KeyError`) para que el código
//! transpilado tenga un destino 1:1 para cada `raise`.

use thiserror::Error;

pub type OResult<T> = Result<T, OError>;

#[derive(Debug, Error)]
pub enum OError {
    /// `odoo.exceptions.UserError`
    #[error("UserError: {0}")]
    User(String),

    /// `odoo.exceptions.ValidationError` (@api.constrains)
    #[error("ValidationError: {0}")]
    Validation(String),

    /// `odoo.exceptions.AccessError` (ir.model.access / record rules)
    #[error("AccessError: {0}")]
    Access(String),

    /// `KeyError` de Python — campo/método inexistente, caché fría, etc.
    #[error("KeyError: {0}")]
    Key(String),

    #[error("Modelo desconocido en el Registry: {0}")]
    UnknownModel(String),

    #[error("Campo desconocido: {model}.{field}")]
    UnknownField { model: String, field: String },

    #[error("Tipo inválido: se esperaba {expected}, se encontró {got}")]
    Type {
        expected: &'static str,
        got: &'static str,
    },

    #[error("Dominio inválido: {0}")]
    Domain(String),

    #[error("Error de construcción del Registry: {0}")]
    Registry(String),

    #[error("ensure_one(): se esperaba un único registro, el recordset tiene {0}")]
    EnsureOne(usize),

    /// El Env es de prototipo (en memoria) y la operación exige Postgres.
    #[error("Sin conexión a base de datos (Env en modo prototipo)")]
    NoPool,

    #[error("Error de base de datos: {0}")]
    Db(#[from] sqlx::Error),

    #[error("Error interno del ORM: {0}")]
    Internal(String),
}

impl OError {
    pub fn user(msg: impl Into<String>) -> Self {
        OError::User(msg.into())
    }

    pub fn validation(msg: impl Into<String>) -> Self {
        OError::Validation(msg.into())
    }

    pub fn key(msg: impl Into<String>) -> Self {
        OError::Key(msg.into())
    }

    pub fn unknown_field(model: impl Into<String>, field: impl Into<String>) -> Self {
        OError::UnknownField {
            model: model.into(),
            field: field.into(),
        }
    }
}
