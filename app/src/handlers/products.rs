use axum::{extract::{Path, Query, State, Extension}, response::IntoResponse};
use nexus_core::db::product as db;
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
        Ok(data) => api::ok(data).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}
