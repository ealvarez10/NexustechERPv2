//! Handlers para CFDIs timbrados — listado, consulta y KPIs
//!
//! GET /api/v1/cfdi/timbrados         — Lista CFDIs paginada
//! GET /api/v1/cfdi/timbrados/{uuid}  — Detalle por UUID
//! GET /api/v1/cfdi/kpis              — KPIs de timbrado

use axum::{
    extract::{Path, Query, State, Extension},
    response::IntoResponse,
};
use nexus_core::db::cfdi as db;
use crate::state::AppState;
use crate::api::{self, PaginaParams, from_core_error};
use crate::middleware::JwtClaims;

/// GET /api/v1/cfdi/timbrados
pub async fn listar_timbrados(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Query(params): Query<PaginaParams>,
) -> impl IntoResponse {
    let p = params.pagina();
    let pp = params.por_pagina();
    match db::listar_todos(&state.db, p, pp).await {
        Ok(data) => {
            let total = db::contar_todos(&state.db).await.unwrap_or(0);
            api::paginado(data, total, p, pp).into_response()
        }
        Err(e) => from_core_error(e).into_response(),
    }
}

/// GET /api/v1/cfdi/timbrados/{uuid}
pub async fn obtener_timbrado(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(uuid): Path<String>,
) -> impl IntoResponse {
    match db::por_uuid(&state.db, &uuid).await {
        Ok(Some(cfdi)) => api::ok(cfdi).into_response(),
        Ok(None) => api::not_found(&format!("CFDI {} no encontrado", uuid)).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// GET /api/v1/cfdi/kpis
pub async fn kpis_cfdi(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
) -> impl IntoResponse {
    match db::kpis_globales(&state.db).await {
        Ok(data) => api::ok(data).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}
