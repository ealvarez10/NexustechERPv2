//! Handlers REST para Contactos / Clientes / Proveedores

use axum::{
    extract::{Path, Query, State, Extension},
    response::IntoResponse,
};
use nexus_core::db::partner as db;
use crate::state::AppState;
use crate::api::{self, PaginaParams, from_core_error};
use crate::middleware::JwtClaims;

/// GET /api/v1/partners?pagina=1&por_pagina=25
pub async fn listar(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Query(params): Query<PaginaParams>,
) -> impl IntoResponse {
    let p = params.pagina();
    let pp = params.por_pagina();
    let cid = claims.0.company_id;
    match (db::listar(&state.db, cid, p, pp).await, db::contar(&state.db, cid).await) {
        (Ok(data), Ok(total)) => api::paginado(data, total, p, pp).into_response(),
        (Err(e), _) | (_, Err(e)) => from_core_error(e).into_response(),
    }
}

/// GET /api/v1/partners/:id
pub async fn obtener(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match db::obtener_resumen(&state.db, id).await {
        Ok(data) => api::ok(data).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// GET /api/v1/clientes
pub async fn clientes(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Query(params): Query<PaginaParams>,
) -> impl IntoResponse {
    let p = params.pagina();
    let pp = params.por_pagina();
    let cid = claims.0.company_id;
    match db::listar_clientes(&state.db, cid, p, pp).await {
        Ok(data) => {
            let total = db::contar(&state.db, cid).await.unwrap_or(0);
            api::paginado(data, total, p, pp).into_response()
        }
        Err(e) => from_core_error(e).into_response(),
    }
}

/// GET /api/v1/proveedores
pub async fn proveedores(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Query(params): Query<PaginaParams>,
) -> impl IntoResponse {
    let p = params.pagina();
    let pp = params.por_pagina();
    let cid = claims.0.company_id;
    match db::listar_proveedores(&state.db, cid, p, pp).await {
        Ok(data) => {
            let total = db::contar(&state.db, cid).await.unwrap_or(0);
            api::paginado(data, total, p, pp).into_response()
        }
        Err(e) => from_core_error(e).into_response(),
    }
}

pub async fn crear(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    axum::extract::Json(payload): axum::extract::Json<nexus_core::db::partner::NuevoPartner>,
) -> impl IntoResponse {
    match db::crear(&state.db, &payload).await {
        Ok(id) => api::ok(id).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}
