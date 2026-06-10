use axum::{
    extract::{Path, Query, State, Extension},
    response::IntoResponse,
    Json,
};
use nexus_core::db::sale_order as db;
use crate::state::AppState;
use crate::api::{self, PaginaParams, from_core_error};
use crate::middleware::JwtClaims;

pub async fn listar(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Query(params): Query<PaginaParams>,
) -> impl IntoResponse {
    let p = params.pagina(); let pp = params.por_pagina(); let cid = claims.0.company_id;
    match db::listar(&state.db, cid, p, pp).await {
        Ok(data) => { let total = db::contar(&state.db, cid).await.unwrap_or(0); api::paginado(data, total, p, pp).into_response() }
        Err(e) => from_core_error(e).into_response(),
    }
}

pub async fn obtener(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match db::obtener_por_id(&state.db, id).await {
        Ok(orden) => {
            api::ok(orden).into_response()
        }
        Err(e) => from_core_error(e).into_response(),
    }
}

pub async fn kpis(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
) -> impl IntoResponse {
    match db::kpis(&state.db, claims.0.company_id).await {
        Ok(data) => api::ok(data).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// GET /ventas/{id}/lineas
pub async fn lineas(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match db::obtener_lineas(&state.db, id).await {
        Ok(data) => api::ok(data).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// PUT /ventas/{id}/confirmar — Cambia estado draft/sent → sale
pub async fn confirmar(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match db::confirmar(&state.db, id).await {
        Ok(Some(row)) => api::ok(serde_json::json!({ "ok": true, "state": row.state })).into_response(),
        Ok(None) => api::error(axum::http::StatusCode::CONFLICT, "No se puede confirmar: estado inválido").into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// PUT /ventas/{id}/cancelar — Cancela la orden de venta
pub async fn cancelar(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match db::cancelar(&state.db, id).await {
        Ok(Some(_)) => api::ok(serde_json::json!({ "ok": true })).into_response(),
        Ok(None) => api::error(axum::http::StatusCode::CONFLICT, "No se puede cancelar: estado inválido").into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// POST /ventas — Crear nueva orden de venta
pub async fn crear(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    let partner_id = match body["partner_id"].as_i64() {
        Some(v) if v > 0 => v as i32,
        _ => return api::bad_request("partner_id es requerido").into_response(),
    };
    let nota = body["note"].as_str().unwrap_or("");
    let company_id = claims.0.company_id;
    match db::crear(&state.db, company_id, partner_id, nota).await {
        Ok(row) => api::creado(serde_json::json!({ "id": row.id, "name": row.name })).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}
