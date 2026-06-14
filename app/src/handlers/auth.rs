//! Handler de autenticación — Login / Refresh / Logout

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use nexus_core::auth::{generar_tokens, renovar_access_token};
use nexus_core::db::user as db_user;
use crate::state::AppState;
use crate::api::{self, ApiOk};

// ─── Requests / Responses ────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub login: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub user_id: i32,
    pub company_id: i32,
    pub email: String,
}

// ─── Handlers ────────────────────────────────────────────────────────────────

/// POST /api/v1/auth/login
///
/// Autentica un usuario y retorna un par de tokens JWT.
pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<(axum::http::StatusCode, Json<ApiOk<LoginResponse>>), (axum::http::StatusCode, Json<crate::api::ApiError>)> {
    // Validar que no lleguen vacíos
    if body.login.trim().is_empty() || body.password.trim().is_empty() {
        return Err(api::bad_request("Login y contraseña son requeridos"));
    }

    // Autenticar contra la DB
    let datos = db_user::autenticar(&state.db, &body.login, &body.password)
        .await
        .map_err(|e| {
            tracing::warn!("Fallo de autenticación para '{}': {:?}", body.login, e);
            api::unauthorized("Credenciales incorrectas")
        })?;

    // Generar tokens
    let par = generar_tokens(
        &datos,
        &state.config.jwt_secret,
        state.config.jwt_access_expires_secs,
        state.config.jwt_refresh_expires_secs,
    )
    .map_err(|_| api::internal_error())?;

    tracing::info!("Sesión iniciada: user_id={}", datos.user_id);

    Ok(api::ok(LoginResponse {
        access_token: par.access_token,
        refresh_token: par.refresh_token,
        token_type: par.token_type,
        expires_in: par.expires_in,
        user_id: datos.user_id,
        company_id: datos.company_id,
        email: datos.email,
    }))
}

/// POST /api/v1/auth/refresh
///
/// Renueva el access token usando un refresh token válido.
pub async fn refresh(
    State(state): State<AppState>,
    Json(body): Json<RefreshRequest>,
) -> Result<(axum::http::StatusCode, Json<ApiOk<serde_json::Value>>), (axum::http::StatusCode, Json<crate::api::ApiError>)> {
    let nuevo_token = renovar_access_token(
        &body.refresh_token,
        &state.config.jwt_secret,
        state.config.jwt_access_expires_secs,
    )
    .map_err(|_| api::unauthorized("Refresh token inválido o expirado"))?;

    Ok(api::ok(serde_json::json!({
        "access_token": nuevo_token,
        "token_type": "Bearer",
        "expires_in": state.config.jwt_access_expires_secs,
    })))
}

/// POST /api/v1/auth/logout
///
/// Invalida la sesión del usuario (client-side — el token expira naturalmente).
pub async fn logout() -> (axum::http::StatusCode, Json<ApiOk<&'static str>>) {
    // En una implementación completa: añadir token a blocklist en Redis
    // Por ahora: el cliente elimina el token localmente
    api::ok("Sesión cerrada correctamente")
}
