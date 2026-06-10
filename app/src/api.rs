//! Respuestas JSON estándar de la API REST de NexusTech ERP

use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

/// Respuesta exitosa estándar
#[derive(Debug, Serialize)]
pub struct ApiOk<T: Serialize> {
    pub success: bool,
    pub data: T,
}

/// Respuesta paginada estándar
#[derive(Debug, Serialize)]
pub struct ApiPaginado<T: Serialize> {
    pub success: bool,
    pub data: Vec<T>,
    pub total: i64,
    pub pagina: i64,
    pub por_pagina: i64,
    pub total_paginas: i64,
}

/// Respuesta de error estándar
#[derive(Debug, Serialize)]
pub struct ApiError {
    pub success: bool,
    pub error: String,
    pub codigo: Option<String>,
}

/// Parámetros de paginación
#[derive(Debug, Deserialize)]
pub struct PaginaParams {
    #[serde(default = "default_pagina")]
    pub pagina: i64,
    #[serde(default = "default_por_pagina")]
    pub por_pagina: i64,
}

fn default_pagina() -> i64 { 1 }
fn default_por_pagina() -> i64 { 25 }

impl PaginaParams {
    pub fn pagina(&self) -> i64 { self.pagina.max(1) }
    pub fn por_pagina(&self) -> i64 { self.por_pagina.clamp(1, 100) }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

pub fn ok<T: Serialize>(data: T) -> (StatusCode, Json<ApiOk<T>>) {
    (StatusCode::OK, Json(ApiOk { success: true, data }))
}

pub fn creado<T: Serialize>(data: T) -> (StatusCode, Json<ApiOk<T>>) {
    (StatusCode::CREATED, Json(ApiOk { success: true, data }))
}

pub fn paginado<T: Serialize>(
    data: Vec<T>,
    total: i64,
    pagina: i64,
    por_pagina: i64,
) -> (StatusCode, Json<ApiPaginado<T>>) {
    let total_paginas = (total + por_pagina - 1) / por_pagina;
    (
        StatusCode::OK,
        Json(ApiPaginado {
            success: true,
            data,
            total,
            pagina,
            por_pagina,
            total_paginas,
        }),
    )
}

pub fn error(status: StatusCode, msg: &str) -> (StatusCode, Json<ApiError>) {
    (
        status,
        Json(ApiError {
            success: false,
            error: msg.to_string(),
            codigo: None,
        }),
    )
}

pub fn not_found(msg: &str) -> (StatusCode, Json<ApiError>) {
    error(StatusCode::NOT_FOUND, msg)
}

pub fn unauthorized(msg: &str) -> (StatusCode, Json<ApiError>) {
    error(StatusCode::UNAUTHORIZED, msg)
}

pub fn bad_request(msg: &str) -> (StatusCode, Json<ApiError>) {
    error(StatusCode::BAD_REQUEST, msg)
}

pub fn internal_error() -> (StatusCode, Json<ApiError>) {
    error(StatusCode::INTERNAL_SERVER_ERROR, "Error interno del servidor")
}

/// Convierte CoreError en una respuesta HTTP apropiada
pub fn from_core_error(e: nexus_core::error::CoreError) -> (StatusCode, Json<ApiError>) {
    use nexus_core::error::CoreError;
    match e {
        CoreError::NotFound(msg) => not_found(&msg),
        CoreError::TokenInvalido => unauthorized("Token inválido o expirado"),
        CoreError::Auth(msg) => unauthorized(&msg),
        CoreError::Forbidden(msg) => error(StatusCode::FORBIDDEN, &msg),
        CoreError::Validation(msg) => bad_request(&msg),
        CoreError::Config(msg) => error(StatusCode::INTERNAL_SERVER_ERROR, &msg),
        CoreError::Db(e) => {
            tracing::error!("Error de base de datos: {}", e);
            internal_error()
        }
        CoreError::Internal(msg) => {
            tracing::error!("Error interno: {}", msg);
            internal_error()
        }
    }
}
