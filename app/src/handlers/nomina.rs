//! Handler: POST /api/v1/nomina/calcular — Simula el cálculo de recibo de nómina

use axum::{
    extract::{State, Extension},
    response::IntoResponse,
    Json,
};
use nexus_core::{
    payroll::{calcular_nomina, EntradaNomina},
    db::nomina as db,
};
use crate::state::AppState;
use crate::api::{self, from_core_error, PaginaParams};
use crate::middleware::JwtClaims;
use axum::extract::{Path, Query};

/// POST /api/v1/nomina/calcular — Simula el cálculo de un recibo de nómina
/// Body: { sdi: 250.00, dias_periodo: 30, tipo: "mensual" }
pub async fn calcular(
    Extension(_claims): Extension<JwtClaims>,
    Json(entrada): Json<EntradaNomina>,
) -> impl IntoResponse {
    let resultado = calcular_nomina(&entrada);
    api::ok(resultado).into_response()
}

/// GET /api/v1/nomina
pub async fn listar(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    _admin: crate::middleware::RequireAdmin,
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
    _admin: crate::middleware::RequireAdmin,
) -> impl IntoResponse {
    match db::kpis(&state.db, claims.0.company_id).await {
        Ok(data) => api::ok(data).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// GET /api/v1/nomina/{id}
pub async fn obtener(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    _admin: crate::middleware::RequireAdmin,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match db::obtener_por_id(&state.db, id).await {
        Ok(data) => api::ok(data).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}
