//! Handlers de compras — órdenes de compra y KPIs
//!
//! GET /api/v1/compras       — Lista órdenes de compra (paginado)
//! GET /api/v1/compras/kpis  — KPIs de compras

use axum::{
    extract::{Query, State, Extension},
    response::IntoResponse,
};
use nexus_core::db::compras as db;
use crate::state::AppState;
use crate::api::{self, PaginaParams, from_core_error};
use crate::middleware::JwtClaims;

/// GET /api/v1/compras
pub async fn listar(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Query(params): Query<PaginaParams>,
) -> impl IntoResponse {
    let p = params.pagina();
    let pp = params.por_pagina();
    let cid = claims.0.company_id;
    match db::listar(&state.db, cid, p, pp).await {
        Ok(data) => {
            let total = db::contar(&state.db, cid).await.unwrap_or(0);
            api::paginado(data, total, p, pp).into_response()
        }
        Err(e) => from_core_error(e).into_response(),
    }
}

/// GET /api/v1/compras/kpis
pub async fn kpis(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
) -> impl IntoResponse {
    match db::kpis(&state.db, claims.0.company_id).await {
        Ok(data) => api::ok(data).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}
