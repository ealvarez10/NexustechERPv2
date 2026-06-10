use axum::{
    extract::{Path, Query, State, Extension},
    response::IntoResponse,
    Json,
};
use nexus_core::db::account_move as db;
use crate::state::AppState;
use crate::api::{self, PaginaParams, from_core_error};
use crate::middleware::JwtClaims;

pub async fn listar(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Query(params): Query<PaginaParams>,
) -> impl IntoResponse {
    let p = params.pagina(); let pp = params.por_pagina(); let cid = claims.0.company_id;
    match db::listar_facturas(&state.db, cid, p, pp).await {
        Ok(data) => { let total = db::contar(&state.db, cid, "out_invoice").await.unwrap_or(0); api::paginado(data, total, p, pp).into_response() }
        Err(e) => from_core_error(e).into_response(),
    }
}

pub async fn obtener(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match db::obtener_por_id(&state.db, id).await {
        Ok(data) => api::ok(data).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

pub async fn kpis(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
) -> impl IntoResponse {
    match db::kpis_facturacion(&state.db, claims.0.company_id).await {
        Ok(data) => api::ok(data).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

pub async fn por_cobrar(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Query(params): Query<PaginaParams>,
) -> impl IntoResponse {
    let p = params.pagina(); let pp = params.por_pagina();
    match db::listar_por_cobrar(&state.db, claims.0.company_id, p, pp).await {
        Ok(data) => api::paginado(data, 0, p, pp).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// GET /facturas/{id}/lineas — Líneas de la factura
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

/// PUT /facturas/{id}/confirmar — draft → posted
pub async fn confirmar(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match db::confirmar(&state.db, id).await {
        Ok(Some(_)) => api::ok(serde_json::json!({ "ok": true, "state": "posted" })).into_response(),
        Ok(None) => api::error(axum::http::StatusCode::CONFLICT, "No se puede publicar: la factura no está en borrador").into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// POST /facturas/{id}/pago — Registrar pago
pub async fn registrar_pago(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match db::registrar_pago(&state.db, id).await {
        Ok(Some(_)) => api::ok(serde_json::json!({ "ok": true, "payment_state": "paid" })).into_response(),
        Ok(None) => api::error(axum::http::StatusCode::CONFLICT, "La factura no está publicada o ya fue pagada").into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// PUT /facturas/{id}/cancelar — Cancelar factura
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

/// POST /facturas — Crear nueva factura de cliente
pub async fn crear(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    let partner_id = match body["partner_id"].as_i64() {
        Some(v) if v > 0 => v as i32,
        _ => return api::bad_request("partner_id es requerido").into_response(),
    };
    // journal_id y currency_id con defaults razonables
    let journal_id = body["journal_id"].as_i64().unwrap_or(1) as i32;
    let currency_id = body["currency_id"].as_i64().unwrap_or(1) as i32;
    let company_id = claims.0.company_id;
    match db::crear(&state.db, company_id, partner_id, journal_id, currency_id).await {
        Ok(row) => api::creado(serde_json::json!({ "id": row.id, "name": row.name })).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}
