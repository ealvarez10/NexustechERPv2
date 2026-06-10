//! Handlers de cotizaciones / Sale
//!
//! GET    /api/v1/cotizaciones              — Lista cotizaciones (draft/sent) paginado
//! GET    /api/v1/cotizaciones/kpis         — KPIs de cotizaciones
//! GET    /api/v1/cotizaciones/:id          — Detalle con líneas
//! POST   /api/v1/cotizaciones              — Crear nueva cotización
//! PUT    /api/v1/cotizaciones/:id/confirmar — Confirmar (draft→sale)
//! PUT    /api/v1/cotizaciones/:id/cancelar  — Cancelar
//! PUT    /api/v1/cotizaciones/:id          — Actualizar nota/referencia
//! POST   /api/v1/cotizaciones/:id/lineas   — Agregar línea
//! DELETE /api/v1/cotizaciones/:id/lineas/:linea_id — Eliminar línea

use axum::{
    extract::{Json, Path, Query, State, Extension},
    http::StatusCode,
    response::IntoResponse,
};
use nexus_sale::db::ordenes::{self, NuevaOrden};
use nexus_sale::db::lineas::{self, NuevaLinea};
use nexus_sale::SaleError;
use crate::state::AppState;
use crate::api::{self, PaginaParams};
use crate::middleware::JwtClaims;
use serde::Deserialize;

// ─── Mapeo de SaleError a respuesta HTTP ──────────────────────────────────────

fn sale_error(e: SaleError) -> (StatusCode, axum::Json<crate::api::ApiError>) {
    match e {
        SaleError::NoEncontrada(id) => api::not_found(&format!("Orden {} no encontrada", id)),
        SaleError::Db(err) => {
            tracing::error!("Error de base de datos en sale: {}", err);
            api::internal_error()
        }
        SaleError::PrecioInvalido(msg) => api::bad_request(&msg),
        SaleError::DescuentoInvalido => api::bad_request("Descuento debe estar entre 0 y 100"),
    }
}

// ─── Handlers ─────────────────────────────────────────────────────────────────

/// GET /api/v1/cotizaciones
pub async fn listar_cotizaciones(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Query(params): Query<PaginaParams>,
) -> impl IntoResponse {
    let p = params.pagina();
    let pp = params.por_pagina();
    let cid = claims.0.company_id;
    match ordenes::listar_cotizaciones(&state.db, cid, p, pp).await {
        Ok(data) => {
            let total = ordenes::contar_cotizaciones(&state.db, cid).await.unwrap_or(0);
            api::paginado(data, total, p, pp).into_response()
        }
        Err(e) => sale_error(e).into_response(),
    }
}

/// GET /api/v1/cotizaciones/kpis
pub async fn kpis_cotizaciones(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
) -> impl IntoResponse {
    match ordenes::kpis_cotizaciones(&state.db, claims.0.company_id).await {
        Ok(data) => api::ok(data).into_response(),
        Err(e) => sale_error(e).into_response(),
    }
}

/// GET /api/v1/cotizaciones/:id
pub async fn obtener_cotizacion(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let cid = claims.0.company_id;
    match ordenes::por_id(&state.db, id, cid).await {
        Ok(orden) => {
            let lineas = lineas::por_orden(&state.db, id).await.unwrap_or_default();
            api::ok(serde_json::json!({ "orden": orden, "lineas": lineas })).into_response()
        }
        Err(e) => sale_error(e).into_response(),
    }
}

/// POST /api/v1/cotizaciones
pub async fn crear_cotizacion(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Json(body): Json<NuevaOrden>,
) -> impl IntoResponse {
    match ordenes::crear(&state.db, &body, claims.0.company_id).await {
        Ok(id) => api::creado(serde_json::json!({ "id": id })).into_response(),
        Err(e) => sale_error(e).into_response(),
    }
}

/// PUT /api/v1/cotizaciones/:id/confirmar
pub async fn confirmar_cotizacion(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match ordenes::confirmar(&state.db, id, claims.0.company_id).await {
        Ok(()) => api::ok(serde_json::json!({ "id": id, "state": "sale" })).into_response(),
        Err(e) => sale_error(e).into_response(),
    }
}

/// PUT /api/v1/cotizaciones/:id/cancelar
pub async fn cancelar_cotizacion(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match ordenes::cancelar(&state.db, id, claims.0.company_id).await {
        Ok(()) => api::ok(serde_json::json!({ "id": id, "state": "cancel" })).into_response(),
        Err(e) => sale_error(e).into_response(),
    }
}

/// Cuerpo para actualizar cotización
#[derive(Debug, Deserialize)]
pub struct ActualizarBody {
    pub note: Option<String>,
    pub client_order_ref: Option<String>,
}

/// PUT /api/v1/cotizaciones/:id
pub async fn actualizar_cotizacion(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
    Json(body): Json<ActualizarBody>,
) -> impl IntoResponse {
    match ordenes::actualizar(&state.db, id, body.note, body.client_order_ref, claims.0.company_id).await {
        Ok(()) => api::ok(serde_json::json!({ "id": id })).into_response(),
        Err(e) => sale_error(e).into_response(),
    }
}

/// POST /api/v1/cotizaciones/:id/lineas
pub async fn agregar_linea(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
    Json(body): Json<NuevaLinea>,
) -> impl IntoResponse {
    match lineas::agregar(&state.db, &body, id).await {
        Ok(linea_id) => api::creado(serde_json::json!({ "id": linea_id })).into_response(),
        Err(e) => sale_error(e).into_response(),
    }
}

/// DELETE /api/v1/cotizaciones/:id/lineas/:linea_id
pub async fn eliminar_linea(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path((id, linea_id)): Path<(i32, i32)>,
) -> impl IntoResponse {
    match lineas::eliminar(&state.db, linea_id, id).await {
        Ok(()) => api::ok(serde_json::json!({ "eliminado": true })).into_response(),
        Err(e) => sale_error(e).into_response(),
    }
}
