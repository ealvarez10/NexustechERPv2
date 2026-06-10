use axum::{extract::{State, Extension}, response::IntoResponse};
use nexus_core::db::{sale_order, account_move, stock};
use crate::state::AppState;
use crate::api::{self, from_core_error};
use crate::middleware::JwtClaims;

/// GET /api/v1/dashboard — KPIs consolidados del ERP
///
/// Ejecuta en paralelo los KPIs de ventas, facturación e inventario
/// y retorna un JSON unificado.
pub async fn kpis(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
) -> impl IntoResponse {
    let cid = claims.0.company_id;

    let (res_ventas, res_facturacion, res_inventario) = tokio::join!(
        sale_order::kpis(&state.db, cid),
        account_move::kpis_facturacion(&state.db, cid),
        stock::kpis(&state.db, cid),
    );

    match (res_ventas, res_facturacion, res_inventario) {
        (Ok(ventas), Ok(facturacion), Ok(inventario)) => {
            api::ok(serde_json::json!({
                "ventas":      ventas,
                "facturacion": facturacion,
                "inventario":  inventario,
            }))
            .into_response()
        }
        (Err(e), _, _) => from_core_error(e).into_response(),
        (_, Err(e), _) => from_core_error(e).into_response(),
        (_, _, Err(e)) => from_core_error(e).into_response(),
    }
}
