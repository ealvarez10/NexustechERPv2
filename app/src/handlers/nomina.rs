//! Handlers de nómina — empleados y KPIs
//!
//! GET /api/v1/nomina       — Lista empleados (paginado)
//! GET /api/v1/nomina/kpis  — KPIs de nómina

use axum::{
    extract::{Query, State, Extension},
    response::IntoResponse,
};
use nexus_core::db::nomina as db;
use crate::state::AppState;
use crate::api::{self, PaginaParams, from_core_error};
use crate::middleware::JwtClaims;

/// GET /api/v1/nomina
pub async fn listar(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Query(params): Query<PaginaParams>,
) -> impl IntoResponse {
    let p = params.pagina();
    let pp = params.por_pagina();
    let cid = claims.0.company_id;
    match db::listar_empleados(&state.db, cid, p, pp).await {
        Ok(data) => {
            let total = db::contar_empleados(&state.db, cid).await.unwrap_or(0);
            api::paginado(data, total, p, pp).into_response()
        }
        Err(e) => from_core_error(e).into_response(),
    }
}

/// GET /api/v1/nomina/kpis
pub async fn kpis(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
) -> impl IntoResponse {
    match db::kpis(&state.db, claims.0.company_id).await {
        Ok(data) => api::ok(data).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}
