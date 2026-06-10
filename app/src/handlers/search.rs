//! Handlers del motor de búsqueda integrado
//!
//! POST /api/v1/search/sync   — Sincroniza todos los índices ERP
//! GET  /api/v1/search/status — Estado y métricas del motor

use axum::{extract::State, response::IntoResponse, Json};
use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
pub struct SyncResponse {
    pub success: bool,
    pub total_indexados: u64,
    pub total_errores: u64,
    pub mensaje: String,
}

/// POST /api/v1/search/sync
///
/// Dispara la sincronización completa de productos y partners
/// hacia el motor de búsqueda. Usar con precaución en producción
/// (puede tardar minutos con catálogos grandes).
pub async fn sync(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let company_id = 1i32; // Configurable en futuras versiones via claims

    match nexus_search::indexer::sincronizar_todos(
        &state.db,
        &state.search_client,
        company_id,
    )
    .await
    {
        Ok(stats) => Json(SyncResponse {
            success: true,
            total_indexados: stats.total_indexados,
            total_errores: stats.total_errores,
            mensaje: format!(
                "Sincronizados {} documentos ({} errores)",
                stats.total_indexados, stats.total_errores
            ),
        }),
        Err(e) => Json(SyncResponse {
            success: false,
            total_indexados: 0,
            total_errores: 0,
            mensaje: format!("Error en sincronización: {}", e),
        }),
    }
}

/// GET /api/v1/search/status
///
/// Verifica conectividad y retorna estadísticas de los índices.
pub async fn status(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let disponible = state.search_client.health_check().await;

    if disponible {
        Json(serde_json::json!({
            "status": "ok",
            "motor": "activo",
            "url": state.search_client.url,
        }))
    } else {
        Json(serde_json::json!({
            "status": "degradado",
            "motor": "inaccesible",
            "url": state.search_client.url,
        }))
    }
}
