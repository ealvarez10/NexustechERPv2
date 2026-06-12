use axum::{
    extract::{Path, Query, State, Extension},
    response::IntoResponse,
    Json,
};
use nexus_core::db::sale_order as db;
use crate::state::AppState;
use crate::api::{self, PaginaParams, from_core_error};
use crate::middleware::JwtClaims;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FiltroParams {
    pub pagina:          Option<i64>,
    pub limite:          Option<i64>,
    pub estado:          Option<String>,
    pub buscar:          Option<String>,
    pub invoice_status:  Option<String>,
}

impl FiltroParams {
    fn p(&self) -> i64 { self.pagina.unwrap_or(1).max(1) }
    fn pp(&self) -> i64 { self.limite.unwrap_or(80).min(200) }
}

/// GET /ventas
pub async fn listar(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Query(params): Query<FiltroParams>,
) -> impl IntoResponse {
    let p = params.p(); let pp = params.pp(); let cid = claims.0.company_id;
    let estado = params.estado.as_deref();
    let buscar = params.buscar.as_deref().filter(|s| !s.is_empty());
    let inv    = params.invoice_status.as_deref().filter(|s| !s.is_empty());
    match db::listar(&state.db, cid, p, pp, estado, buscar, inv).await {
        Ok(data) => {
            let total = db::contar(&state.db, cid, estado, buscar, inv).await.unwrap_or(0);
            api::paginado(data, total, p, pp).into_response()
        }
        Err(e) => from_core_error(e).into_response(),
    }
}

/// GET /ventas/{id}
pub async fn obtener(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match db::obtener_por_id(&state.db, id).await {
        Ok(orden) => api::ok(orden).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// GET /ventas/kpis
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

/// PUT /ventas/{id}  — Actualizar campos del encabezado
pub async fn actualizar(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
    Json(body): Json<db::ActualizarOrden>,
) -> impl IntoResponse {
    match db::actualizar(&state.db, id, &body).await {
        Ok(()) => api::ok(serde_json::json!({ "ok": true })).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// PUT /ventas/{id}/confirmar
pub async fn confirmar(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match db::confirmar(&state.db, id).await {
        Ok(Some(row)) => {
            let cid = claims.0.company_id;
            let uid = claims.0.user_id;
            let pool = state.db.clone();
            tokio::spawn(async move {
                if let Err(e) = nexus_core::db::stock_rules::ejecutar_scheduler(&pool, cid, uid).await {
                    tracing::error!("Error ejecutando scheduler tras venta: {}", e);
                }
            });
            api::ok(serde_json::json!({ "ok": true, "state": row.state })).into_response()
        }
        Ok(None) => api::error(axum::http::StatusCode::CONFLICT, "No se puede confirmar: estado inválido").into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// PUT /ventas/{id}/cancelar
pub async fn cancelar(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match db::cancelar(&state.db, id).await {
        Ok(Some(_)) => api::ok(serde_json::json!({ "ok": true })).into_response(),
        Ok(None) => api::error(axum::http::StatusCode::CONFLICT, "No se puede cancelar").into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// PUT /ventas/{id}/enviar  — Cotización enviada (draft → sent)
pub async fn enviar(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match db::enviar(&state.db, id).await {
        Ok(Some(row)) => api::ok(serde_json::json!({ "ok": true, "state": row.state })).into_response(),
        Ok(None) => api::error(axum::http::StatusCode::CONFLICT, "Solo borradores se pueden enviar").into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// PUT /ventas/{id}/borrador  — Restaurar a borrador (cancel/sent → draft)
pub async fn restaurar_borrador(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match db::restaurar_borrador(&state.db, id).await {
        Ok(Some(_)) => api::ok(serde_json::json!({ "ok": true, "state": "draft" })).into_response(),
        Ok(None) => api::error(axum::http::StatusCode::CONFLICT, "Solo órdenes canceladas o enviadas pueden restaurarse a borrador").into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// PUT /ventas/{id}/bloquear
pub async fn bloquear(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    let locked = body["locked"].as_bool().unwrap_or(true);
    match db::bloquear(&state.db, id, locked).await {
        Ok(()) => api::ok(serde_json::json!({ "ok": true, "locked": locked })).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// POST /ventas  — Crear nueva orden de venta
pub async fn crear(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    let partner_id = match body["partner_id"].as_i64() {
        Some(v) if v > 0 => v as i32,
        _ => return api::bad_request("partner_id es requerido").into_response(),
    };
    let partner_invoice_id = body["partner_invoice_id"].as_i64().map(|v| v as i32);
    let partner_shipping_id = body["partner_shipping_id"].as_i64().map(|v| v as i32);
    let nota = body["note"].as_str().unwrap_or("");
    let client_order_ref = body["client_order_ref"].as_str();
    let validity_days = body["validity_days"].as_i64().map(|v| v as i32);
    let company_id = claims.0.company_id;
    match db::crear(&state.db, company_id, partner_id, partner_invoice_id, partner_shipping_id, nota, client_order_ref, validity_days).await {
        Ok(row) => api::creado(serde_json::json!({ "id": row.id, "name": row.name })).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// POST /ventas/{id}/lineas  — Agregar línea de pedido
pub async fn agregar_linea(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
    Json(body): Json<db::NuevaLinea>,
) -> impl IntoResponse {
    match db::agregar_linea(&state.db, id, &body).await {
        Ok(lid) => api::creado(serde_json::json!({ "id": lid, "order_id": id })).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// PUT /ventas/{id}/lineas/{lid}  — Editar línea de pedido
pub async fn actualizar_linea(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path((id, lid)): Path<(i32, i32)>,
    Json(body): Json<db::ActualizarLinea>,
) -> impl IntoResponse {
    match db::actualizar_linea(&state.db, id, lid, &body).await {
        Ok(()) => api::ok(serde_json::json!({ "ok": true })).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// DELETE /ventas/{id}/lineas/{lid}  — Eliminar línea de pedido
pub async fn eliminar_linea(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path((id, lid)): Path<(i32, i32)>,
) -> impl IntoResponse {
    match db::eliminar_linea(&state.db, id, lid).await {
        Ok(()) => api::ok(serde_json::json!({ "ok": true })).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

#[derive(Deserialize)]
pub struct BuscarParams {
    pub q: Option<String>,
    pub limit: Option<i64>,
}

/// GET /ventas/buscar-clientes?q=...
pub async fn buscar_clientes(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Query(params): Query<BuscarParams>,
) -> impl IntoResponse {
    let q = params.q.as_deref().unwrap_or("");
    let limit = params.limit.unwrap_or(20);
    match db::buscar_clientes(&state.db, q, limit).await {
        Ok(rows) => {
            let data: Vec<_> = rows.into_iter().map(|(id, name, email)| {
                serde_json::json!({ "id": id, "name": name, "email": email })
            }).collect();
            api::ok(data).into_response()
        }
        Err(e) => from_core_error(e).into_response(),
    }
}

/// GET /ventas/buscar-productos?q=...
pub async fn buscar_productos(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Query(params): Query<BuscarParams>,
) -> impl IntoResponse {
    let q = params.q.as_deref().unwrap_or("");
    let limit = params.limit.unwrap_or(20);
    match db::buscar_productos(&state.db, q, limit).await {
        Ok(data) => api::ok(data).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

// ─── FLUJO VENTAS → FACTURACIÓN ──────────────────────────────────────────────

#[derive(Deserialize)]
pub struct CrearFacturaBody {
    pub advance_payment_method: Option<String>,  // "delivered" | "percentage" | "fixed"
    pub amount:                 Option<f64>,      // porcentaje (1-100) si method=percentage
    pub fixed_amount:           Option<f64>,      // monto fijo si method=fixed
}

/// POST /ventas/{id}/crear-factura — Crea factura desde pedido confirmado
pub async fn crear_factura(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
    Json(body): Json<CrearFacturaBody>,
) -> impl IntoResponse {
    let method = body.advance_payment_method.as_deref().unwrap_or("delivered");
    match db::crear_factura_desde_venta(&state.db, id, claims.0.company_id, method, body.amount, body.fixed_amount).await {
        Ok(r) => api::creado(serde_json::json!({
            "factura_id":   r.factura_id,
            "factura_name": r.factura_name,
            "order_name":   r.order_name,
        })).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// GET /ventas/{id}/facturas — Facturas vinculadas al pedido
pub async fn facturas_de_venta(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match db::facturas_de_venta(&state.db, id).await {
        Ok(data) => api::ok(data).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

// ─── FLUJO VENTAS → ALMACÉN ───────────────────────────────────────────────────

/// GET /ventas/{id}/entrega — Estado de entrega de un pedido
pub async fn entrega_de_venta(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match db::entrega_de_venta(&state.db, id).await {
        Ok(data) => api::ok(data).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

#[derive(Deserialize)]
pub struct ValidarEntregaBody {
    pub lineas: Vec<(i32, f64)>,
}

/// PUT /ventas/{id}/validar-entrega — Marcar productos como entregados
pub async fn validar_entrega(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
    Json(body): Json<ValidarEntregaBody>,
) -> impl IntoResponse {
    match db::validar_entrega(&state.db, id, body.lineas).await {
        Ok(()) => api::ok(serde_json::json!({ "ok": true })).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}

/// GET /ventas/{id}/picking — Devuelve el ID del stock_picking para navegar al módulo Almacén
pub async fn picking_de_venta(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match db::get_picking_for_order(&state.db, id).await {
        Ok(Some((picking_id, name, state_pick, count))) => api::ok(serde_json::json!({
            "picking_id": picking_id,
            "name":       name,
            "state":      state_pick,
            "count_moves": count,
        })).into_response(),
        Ok(None) => api::ok(serde_json::json!(null)).into_response(),
        Err(e)   => from_core_error(e).into_response(),
    }
}

/// POST /ventas/{id}/duplicar — Duplicar una orden de venta (igual que Odoo Duplicate)
pub async fn duplicar(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match db::duplicar(&state.db, id, claims.0.company_id).await {
        Ok(nueva) => api::creado(serde_json::json!({
            "id":   nueva.id,
            "name": nueva.name,
        })).into_response(),
        Err(e) => from_core_error(e).into_response(),
    }
}
