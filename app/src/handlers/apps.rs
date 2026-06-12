use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use serde_json::json;

use crate::state::AppState;

#[derive(Serialize, Deserialize)]
pub struct AppModel {
    pub id: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub icono: Option<String>,
    pub gradiente: Option<String>,
    pub kpi_url: Option<String>,
    pub kpi_field: Option<String>,
    pub estado: String,
}

pub async fn listar_apps(
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Si queremos filtrar por estado, podríamos aceptar un query param, 
    // pero por simplicidad listamos todos y que el frontend filtre si lo necesita,
    // o enviamos todo al frontend y el frontend decide qué mostrar.
    let apps = match sqlx::query!("SELECT id, nombre, descripcion, icono, gradiente, kpi_url, kpi_field, estado FROM nexus_apps ORDER BY nombre")
        .fetch_all(&state.db)
        .await
    {
        Ok(filas) => filas,
        Err(e) => {
            tracing::error!("Error consultando apps: {}", e);
            return Json(json!({"error": "Error interno al consultar apps"}));
        }
    };

    let result: Vec<AppModel> = apps.into_iter().map(|f| AppModel {
        id: f.id,
        nombre: f.nombre,
        descripcion: f.descripcion,
        icono: f.icono,
        gradiente: f.gradiente,
        kpi_url: f.kpi_url,
        kpi_field: f.kpi_field,
        estado: f.estado.unwrap_or_else(|| "uninstalled".to_string()),
    }).collect();

    Json(json!(result))
}

pub async fn instalar_app(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match sqlx::query!("UPDATE nexus_apps SET estado = 'installed' WHERE id = $1 RETURNING id", id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => Json(json!({"success": true, "message": "Aplicación instalada"})),
        Ok(None) => Json(json!({"error": "Aplicación no encontrada"})),
        Err(e) => {
            tracing::error!("Error instalando app {}: {}", id, e);
            Json(json!({"error": "Error interno al instalar aplicación"}))
        }
    }
}

pub async fn desinstalar_app(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match sqlx::query!("UPDATE nexus_apps SET estado = 'uninstalled' WHERE id = $1 RETURNING id", id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => Json(json!({"success": true, "message": "Aplicación desinstalada"})),
        Ok(None) => Json(json!({"error": "Aplicación no encontrada"})),
        Err(e) => {
            tracing::error!("Error desinstalando app {}: {}", id, e);
            Json(json!({"error": "Error interno al desinstalar aplicación"}))
        }
    }
}
