use axum::{
    extract::{Path, Query, State, Extension},
    response::IntoResponse,
};
use nexus_core::db::stock as db;
use crate::state::AppState;
use crate::api::{self, PaginaParams, from_core_error};
use crate::middleware::JwtClaims;

/// GET /api/v1/stock — Lista stock paginado de la empresa
pub async fn listar(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Query(params): Query<PaginaParams>,
) -> impl IntoResponse {
    let p = params.pagina();
    let pp = params.por_pagina();
    let cid = claims.0.company_id;

    match db::listar_stock(&state.db, cid, p, pp).await {
        Ok(data) => {
            let total = db::contar_stock(&state.db, cid).await.unwrap_or(0);
            api::paginado(data, total, p, pp).into_response()
        }
        Err(e) => from_core_error(e).into_response(),
    }
}

/// GET /api/v1/stock/kpis — KPIs de inventario
pub async fn kpis(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
) -> impl IntoResponse {
    match db::kpis(&state.db, claims.0.company_id).await {
        Ok(data) => api::ok(data).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// GET /api/v1/stock/producto/{id} — Stock de un producto específico
pub async fn por_producto(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match db::stock_por_producto(&state.db, id).await {
        Ok(data) => api::ok(data).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// GET /api/v1/stock/bajo — Productos con stock bajo
pub async fn bajo(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
) -> impl IntoResponse {
    match db::productos_stock_bajo(&state.db, claims.0.company_id, 50).await {
        Ok(data) => api::ok(data).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

pub async fn ajustar(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    axum::extract::Json(payload): axum::extract::Json<nexus_core::db::stock::AjusteStock>,
) -> impl IntoResponse {
    match db::ajustar(&state.db, claims.0.company_id, payload).await {
        Ok(_) => api::ok("Stock ajustado").into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

// ─── Pickings (Órdenes de Entrega) ─────────────────────────────────────────

use serde::Deserialize;

#[derive(Deserialize)]
pub struct PickingParams {
    pub sale_id: Option<i32>,
    pub state:   Option<String>,
    pub pagina:  Option<i64>,
}

/// GET /picking — Lista pickings (entregas pendientes)
pub async fn listar_pickings(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Query(params): Query<PickingParams>,
) -> impl IntoResponse {
    match nexus_core::db::sale_order::listar_pickings(
        &state.db,
        claims.0.company_id,
        params.sale_id,
        params.state.as_deref(),
    ).await {
        Ok(data) => api::ok(data).into_response(),
        Err(e)   => from_core_error(e).into_response(),
    }
}

/// GET /picking/{id} — Detalle de un picking específico con sus moves
pub async fn obtener_picking(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match nexus_core::db::sale_order::obtener_picking(&state.db, id).await {
        Ok(data) => api::ok(data).into_response(),
        Err(e)   => from_core_error(e).into_response(),
    }
}

#[derive(Deserialize)]
pub struct ValidarPickingBody {
    pub moves: Vec<(i32, f64)>,   // (move_id, quantity_done)
}

/// PUT /picking/{id}/validar — Valida la entrega: descuenta stock y cierra picking
pub async fn validar_picking(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
    axum::extract::Json(body): axum::extract::Json<ValidarPickingBody>,
) -> impl IntoResponse {
    match nexus_core::db::sale_order::validar_picking(&state.db, id, claims.0.company_id, body.moves).await {
        Ok(()) => api::ok(serde_json::json!({ "ok": true })).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}
