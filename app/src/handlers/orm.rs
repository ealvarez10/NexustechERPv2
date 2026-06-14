use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::state::AppState;
use nexus_orm::prelude::*;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct CallKwPayload {
    pub args: Vec<Value>,
    #[serde(default)]
    pub kwargs: Value,
}

#[allow(dead_code)]
#[derive(Serialize)]
pub struct CallKwResponse {
    pub result: Value,
}

#[allow(dead_code)]
pub async fn call_kw(
    State(state): State<AppState>,
    Path((model_name, method_name)): Path<(String, String)>,
    Json(payload): Json<CallKwPayload>,
) -> Result<Json<CallKwResponse>, (StatusCode, Json<crate::api::ApiError>)> {
    let registry = state.registry.as_ref().ok_or_else(|| {
        crate::api::error(StatusCode::INTERNAL_SERVER_ERROR, "ORM Registry no inicializado")
    })?;

    // Inicializamos el entorno transaccional
    let env = Env::mock(registry.clone());
    
    // Convertir de JSON a OVal (usando from_json nativo del ORM)
    let mut args_oval = Vec::new();
    for v in &payload.args {
        args_oval.push(OVal::from_json(v));
    }

    // Identificar record IDs para browse
    let mut record_ids = vec![];
    if let Some(first_arg) = payload.args.first() {
        if let Value::Array(arr) = first_arg {
            for v in arr {
                if let Value::Number(n) = v {
                    if let Some(num) = n.as_i64() {
                        record_ids.push(num);
                    }
                }
            }
        }
    }

    let rs = env.browse(&model_name, record_ids).map_err(|e| crate::api::error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    // Llamamos al método
    let _result = rs.call(&method_name, &args_oval).await.map_err(|e| crate::api::error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    // Convertir OVal a JSON (Aproximación simple para prueba de concepto)
    // Odoo suele retornar diccionarios con tag y params.
    let result_json = Value::Object({
        let mut map = serde_json::Map::new();
        map.insert("type".to_string(), Value::String("ir.actions.client".to_string()));
        map.insert("tag".to_string(), Value::String("display_notification".to_string()));
        let mut params = serde_json::Map::new();
        params.insert("title".to_string(), Value::String("Éxito".to_string()));
        params.insert("message".to_string(), Value::String("¡Conexión a Mercadily establecida desde Python!".to_string()));
        params.insert("type".to_string(), Value::String("success".to_string()));
        map.insert("params".to_string(), Value::Object(params));
        map
    });

    Ok(Json(CallKwResponse { result: result_json }))
}
