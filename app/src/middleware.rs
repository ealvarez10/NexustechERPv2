//! Middleware de autenticación JWT para NexusTech ERP v2

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
    Json,
};
use nexus_core::auth::{validar_access_token, Claims};
use crate::state::AppState;
use crate::api::ApiError;

/// Extractor de Claims JWT desde el request actual
#[derive(Clone)]
pub struct JwtClaims(pub Claims);

/// Middleware que verifica el Bearer token JWT en cada request
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<ApiError>)> {
    let token = extraer_bearer(&request).ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ApiError {
                success: false,
                error: "Se requiere token de autenticación".into(),
                codigo: Some("AUTH_REQUIRED".into()),
            }),
        )
    })?;

    let claims = validar_access_token(token, &state.config.jwt_secret).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ApiError {
                success: false,
                error: "Token inválido o expirado".into(),
                codigo: Some("TOKEN_INVALID".into()),
            }),
        )
    })?;

    request.extensions_mut().insert(JwtClaims(claims));
    Ok(next.run(request).await)
}

fn extraer_bearer(request: &Request) -> Option<&str> {
    request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
}
