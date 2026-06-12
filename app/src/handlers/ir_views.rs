//! ir_views — Lee `ir_ui_view` directamente desde PostgreSQL.
//!
//! Endpoint usado por el storefront para precargar el ViewRegistry
//! con TODOS los modelos Odoo instalados al iniciar.

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::Serialize;
use tracing::info;

use crate::state::AppState;
use crate::api;

#[derive(Serialize)]
pub struct IrViewRow {
    pub model: String,
    pub view_type: String,
    pub arch: String,
    pub priority: i32,
    pub name: String,
}

/// GET /api/v1/ir-views
///
/// Devuelve todas las vistas base activas de ir_ui_view (form, list, kanban).
/// Una fila por (model, type) — la de menor prioridad (más específica).
pub async fn list_views(
    State(state): State<AppState>,
) -> Result<Json<Vec<IrViewRow>>, (StatusCode, Json<api::ApiError>)> {

    let rows = sqlx::query!(
        r#"
        SELECT DISTINCT ON (model, type)
            model,
            type   AS view_type,
            COALESCE(
                arch_db->>'en_US',
                arch_db->>'es_MX',
                arch_db->>'es',
                arch_fs,
                arch_prev,
                ''
            )  AS arch,
            priority,
            COALESCE(name, '')  AS name
        FROM ir_ui_view
        WHERE active = true
          AND inherit_id IS NULL
          AND type IN ('form', 'list', 'tree', 'kanban')
          AND model IS NOT NULL
          AND model != ''
          AND (arch_db IS NOT NULL OR arch_fs IS NOT NULL OR arch_prev IS NOT NULL)
        ORDER BY model, type, priority ASC
        "#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| api::error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let result: Vec<IrViewRow> = rows
        .into_iter()
        .filter_map(|r| {
            let model = r.model?;
            let view_type = r.view_type?;
            let arch = r.arch?;
            if model.is_empty() || arch.is_empty() { return None; }
            Some(IrViewRow {
                model,
                view_type,
                arch,
                priority: r.priority,
                name: r.name.unwrap_or_default(),
            })
        })
        .collect();

    info!("📐 /ir-views: {} vistas únicas enviadas al storefront", result.len());
    Ok(Json(result))
}
