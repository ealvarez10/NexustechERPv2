//! orm_rpc — Endpoint ORM genérico para cualquier modelo Odoo.
//!
//! Permite al storefront hacer write/create/unlink/search_read/read sobre
//! CUALQUIER modelo, ejecutando SQL directo en PostgreSQL.
//!
//! Ruta: POST /api/v1/orm/{model}/{method}

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{debug, warn};

use crate::state::AppState;
use crate::api;

#[derive(Deserialize, Default)]
pub struct OrmPayload {
    #[serde(default)] pub ids: Vec<i64>,
    #[serde(default)] pub vals: Option<serde_json::Map<String, Value>>,
    #[serde(default)] pub domain: Option<Value>,
    #[serde(default)] pub fields: Vec<String>,
    #[serde(default)] pub limit: Option<u32>,
    #[serde(default)] pub offset: Option<u32>,
    #[serde(default)] pub order: Option<String>,
}

#[derive(Serialize)]
pub struct OrmResponse {
    pub success: bool,
    pub result: Value,
}

/// POST /api/v1/orm/{model}/{method}
pub async fn orm_rpc(
    State(state): State<AppState>,
    Path((model, method)): Path<(String, String)>,
    Json(payload): Json<OrmPayload>,
) -> Result<Json<OrmResponse>, (StatusCode, Json<api::ApiError>)> {

    let table = model.replace('.', "_");
    debug!("orm_rpc: {model}.{method} table={table}");

    match method.as_str() {

        // ── WRITE ──────────────────────────────────────────────────────────
        "write" => {
            let ids = &payload.ids;
            let vals = payload.vals.ok_or_else(|| {
                api::error(StatusCode::BAD_REQUEST, "write requiere 'vals'")
            })?;

            if ids.is_empty() {
                return Err(api::error(StatusCode::BAD_REQUEST, "write requiere 'ids'"));
            }
            if vals.is_empty() {
                return Ok(Json(OrmResponse { success: true, result: json!(true) }));
            }

            let affected = execute_dynamic_update(&state.db, &table, &vals, ids).await
                .map_err(|e| api::error(StatusCode::INTERNAL_SERVER_ERROR, &format!("Error en write: {e}")))?;

            debug!("orm_rpc write: {model} ids={:?} → {affected} rows updated", ids);
            Ok(Json(OrmResponse { success: true, result: json!(true) }))
        }

        // ── CREATE ─────────────────────────────────────────────────────────
        "create" => {
            let vals = payload.vals.ok_or_else(|| {
                api::error(StatusCode::BAD_REQUEST, "create requiere 'vals'")
            })?;

            let new_id = execute_dynamic_create(&state.db, &table, &vals).await
                .map_err(|e| api::error(StatusCode::INTERNAL_SERVER_ERROR, &format!("Error en create: {e}")))?;

            debug!("orm_rpc create: {model} → new_id={new_id}");
            Ok(Json(OrmResponse { success: true, result: json!(new_id) }))
        }

        // ── UNLINK ─────────────────────────────────────────────────────────
        "unlink" => {
            let ids = &payload.ids;
            if ids.is_empty() {
                return Err(api::error(StatusCode::BAD_REQUEST, "unlink requiere 'ids'"));
            }

            let ids_list = ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
            let sql = format!("DELETE FROM {} WHERE id IN ({})", table, ids_list);

            sqlx::query(&sql)
                .execute(&state.db)
                .await
                .map_err(|e| api::error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

            Ok(Json(OrmResponse { success: true, result: json!(true) }))
        }

        // ── READ ───────────────────────────────────────────────────────────
        "read" => {
            let ids = &payload.ids;
            if ids.is_empty() {
                return Ok(Json(OrmResponse { success: true, result: json!([]) }));
            }

            let valid_cols = get_table_columns(&state.db, &table).await;
            let col_types = get_table_column_types(&state.db, &table).await;

            let fields: Vec<String> = if payload.fields.is_empty() {
                vec!["id".to_string(), "name".to_string()]
            } else {
                payload.fields.iter()
                    .filter(|f| valid_cols.contains(&f.to_string()))
                    .cloned()
                    .collect()
            };

            if fields.is_empty() {
                return Ok(Json(OrmResponse { success: true, result: json!([]) }));
            }

            let ids_list = ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
            // Usar extracción inteligente de JSONB para 'name'
            let select_expr = fields.iter()
                .map(|f| jsonb_aware_col(f, &col_types))
                .collect::<Vec<_>>()
                .join(", ");

            let sql = format!(
                "SELECT {} FROM {} WHERE id IN ({}) ORDER BY id",
                select_expr, table, ids_list
            );

            debug!("orm_rpc read SQL: {}", &sql[..sql.len().min(300)]);

            let rows = sqlx::query(&sql)
                .fetch_all(&state.db)
                .await
                .map_err(|e| api::error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

            let result = rows_to_json(rows, &fields);
            Ok(Json(OrmResponse { success: true, result: json!(result) }))
        }

        // ── SEARCH_READ ────────────────────────────────────────────────────
        "search_read" | "search" => {
            let limit = payload.limit.unwrap_or(80);
            let offset = payload.offset.unwrap_or(0);
            let order = payload.order.as_deref().unwrap_or("id desc");

            // Obtener columnas reales de la tabla
            let valid_cols = get_table_columns(&state.db, &table).await;

            if valid_cols.is_empty() {
                debug!("orm_rpc search_read: tabla '{}' no existe en PostgreSQL (modelo externo/addon no instalado)", table);
                return Ok(Json(OrmResponse { success: true, result: json!([]) }));
            }

            // Determinar campos a retornar
            let fields: Vec<String> = if payload.fields.is_empty() {
                valid_cols.iter()
                    .filter(|c| matches!(c.as_str(), "id" | "name" | "active"))
                    .cloned()
                    .collect()
            } else {
                payload.fields.iter()
                    .filter(|f| valid_cols.contains(&f.to_string()))
                    .cloned()
                    .collect()
            };

            if fields.is_empty() {
                return Ok(Json(OrmResponse { success: true, result: json!([]) }));
            }

            let col_types = get_table_column_types(&state.db, &table).await;
            // SELECT con extracción JSONB para campos multiidioma
            let select_expr = fields.iter()
                .map(|f| jsonb_aware_col(f, &col_types))
                .collect::<Vec<_>>()
                .join(", ");

            // Parsear domain para extraer filtro de ID o condición simple
            let where_clause = build_where_from_domain(&payload.domain, &valid_cols);

            // Validar ORDER BY
            let safe_order = sanitize_order(order, &valid_cols);

            let sql = format!(
                "SELECT {} FROM {} {} ORDER BY {} LIMIT {} OFFSET {}",
                select_expr, table, where_clause, safe_order, limit, offset
            );

            debug!("orm_rpc search_read SQL: {}", &sql[..sql.len().min(300)]);

            let rows = sqlx::query(&sql)
                .fetch_all(&state.db)
                .await
                .map_err(|e| api::error(StatusCode::INTERNAL_SERVER_ERROR, &format!("search_read: {e}")))?;

            let result = rows_to_json(rows, &fields);
            Ok(Json(OrmResponse { success: true, result: json!(result) }))
        }

        // ── SEARCH_COUNT ───────────────────────────────────────────────────
        "search_count" => {
            let valid_cols = get_table_columns(&state.db, &table).await;
            let where_clause = if valid_cols.contains(&"active".to_string()) {
                "WHERE active IS NOT FALSE"
            } else {
                ""
            };
            let sql = format!("SELECT COUNT(*) as cnt FROM {} {}", table, where_clause);
            let row = sqlx::query(&sql)
                .fetch_one(&state.db)
                .await
                .map_err(|e| api::error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

            use sqlx::Row;
            let cnt: i64 = row.try_get("cnt").unwrap_or(0);
            Ok(Json(OrmResponse { success: true, result: json!(cnt) }))
        }

        // ── FIELDS_GET ─────────────────────────────────────────────────────
        "fields_get" => {
            let sql = r#"
                SELECT column_name, data_type, is_nullable
                FROM information_schema.columns
                WHERE table_name = $1
                  AND table_schema = 'public'
                ORDER BY ordinal_position
            "#;

            let rows = sqlx::query(sql)
                .bind(&table)
                .fetch_all(&state.db)
                .await
                .map_err(|e| api::error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

            use sqlx::Row;
            let mut fields_map = serde_json::Map::new();
            for row in &rows {
                let col: String = row.try_get("column_name").unwrap_or_default();
                let dtype: String = row.try_get("data_type").unwrap_or_default();
                let nullable: String = row.try_get("is_nullable").unwrap_or_default();

                // Detectar campos many2one por convención de nombre (_id suffix)
                let odoo_type = if col.ends_with("_id") && dtype.contains("int") {
                    "many2one"
                } else {
                    match dtype.as_str() {
                        "integer" | "bigint" | "smallint" => "integer",
                        "numeric" | "real" | "double precision" => "float",
                        "boolean" => "boolean",
                        "timestamp without time zone" | "timestamp with time zone" => "datetime",
                        "date" => "date",
                        "jsonb" | "json" => "char",  // JSONB names → tratar como char en el form
                        _ => "char",
                    }
                };

                // Label legible: snake_case → Title Case
                let label = col.replace('_', " ")
                    .split_whitespace()
                    .map(|w| {
                        let mut c = w.chars();
                        match c.next() {
                            None => String::new(),
                            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ");

                fields_map.insert(col.clone(), json!({
                    "type": odoo_type,
                    "string": label,
                    "required": nullable == "NO",
                    "readonly": false,
                    "store": true,
                }));
            }

            Ok(Json(OrmResponse { success: true, result: Value::Object(fields_map) }))
        }

        other => {
            warn!("orm_rpc: método no soportado: {}", other);
            Err(api::error(
                StatusCode::NOT_IMPLEMENTED,
                &format!("Método ORM '{}' no implementado. Soportados: write, create, unlink, read, search_read, search_count, fields_get", other),
            ))
        }
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn is_readonly_field(field: &str) -> bool {
    matches!(
        field,
        "id" | "create_date" | "create_uid" | "write_date" | "write_uid" | "__last_update" | "display_name"
    )
}

/// Devuelve expresión SQL para un campo, aplicando extracción JSONB solo si el tipo es jsonb.
/// Requiere el mapa de columnas->tipo obtenido con get_table_columns.
pub fn jsonb_aware_col(col: &str, col_types: &std::collections::HashMap<String, String>) -> String {
    let dtype = col_types.get(col).map(|s| s.as_str()).unwrap_or("");
    if dtype == "jsonb" || dtype == "json" {
        // Extraer texto con fallback de idiomas
        format!(
            "COALESCE(\"{col}\"->>'es_MX', \"{col}\"->>'en_US', \"{col}\"->>(0)::text, \"{col}\"::text, '') AS \"{col}\""
        )
    } else {
        // Tipo normal: solo columna (con alias para consistencia en AS)
        format!("\"{col}\" AS \"{col}\"")
    }
}

/// Alias para uso donde solo se necesitan los nombres de columna (retro-compatibilidad)
#[allow(dead_code)]
pub async fn get_table_column_names(db: &sqlx::PgPool, table: &str) -> Vec<String> {
    get_table_columns(db, table).await
}

/// Obtiene columnas reales de una tabla de PostgreSQL (solo nombres).
pub async fn get_table_columns(db: &sqlx::PgPool, table: &str) -> Vec<String> {
    use sqlx::Row;
    let sql = r#"
        SELECT column_name FROM information_schema.columns
        WHERE table_name = $1 AND table_schema = 'public'
        ORDER BY ordinal_position
    "#;
    sqlx::query(sql)
        .bind(table)
        .fetch_all(db)
        .await
        .unwrap_or_default()
        .iter()
        .map(|r| r.try_get::<String, _>("column_name").unwrap_or_default())
        .collect()
}

/// Obtiene columnas con tipo de dato PostgreSQL.
/// Retorna HashMap<nombre_columna, tipo_dato_pg>
pub async fn get_table_column_types(db: &sqlx::PgPool, table: &str) -> std::collections::HashMap<String, String> {
    use sqlx::Row;
    let sql = r#"
        SELECT column_name, data_type FROM information_schema.columns
        WHERE table_name = $1 AND table_schema = 'public'
        ORDER BY ordinal_position
    "#;
    sqlx::query(sql)
        .bind(table)
        .fetch_all(db)
        .await
        .unwrap_or_default()
        .iter()
        .map(|r| {
            let name = r.try_get::<String, _>("column_name").unwrap_or_default();
            let dtype = r.try_get::<String, _>("data_type").unwrap_or_default();
            (name, dtype)
        })
        .collect()
}

/// Sanitiza ORDER BY: solo permite columnas reales + ASC/DESC
/// Construye cláusula WHERE desde un domain Odoo.
/// Soporta condiciones simples: [["campo", "operador", valor]]
/// Siempre incluye active IS NOT FALSE si el campo existe.
pub fn build_where_from_domain(domain: &Option<Value>, valid_cols: &[String]) -> String {
    let mut conditions: Vec<String> = vec![];

    // Filtrar activos si el campo existe — incluir NULL como "activo"
    if valid_cols.contains(&"active".to_string()) {
        conditions.push("(\"active\" IS NOT FALSE)".to_string());
    }

    // Parsear domain: puede ser [[...], [...]] o []
    if let Some(domain_val) = domain {
        if let Some(arr) = domain_val.as_array() {
            for cond in arr {
                if let Some(triple) = cond.as_array() {
                    if triple.len() == 3 {
                        let field = triple[0].as_str().unwrap_or("");
                        let op = triple[1].as_str().unwrap_or("=");
                        let val = &triple[2];

                        // Solo campos reales y operadores seguros
                        if valid_cols.contains(&field.to_string()) && is_safe_operator(op) {
                            let sql_val = match val {
                                Value::Number(n) => n.to_string(),
                                Value::String(s) => format!("'{}'", s.replace('\'', "''")),
                                Value::Bool(b) => if *b { "TRUE".to_string() } else { "FALSE".to_string() },
                                Value::Null => "NULL".to_string(),
                                _ => continue,
                            };
                            // Para el field 'active', ya lo tenemos arriba — saltar
                            if field == "active" { continue; }
                            conditions.push(format!("\"{}\" {} {}", field, op, sql_val));
                        }
                    }
                }
            }
        }
    }

    if conditions.is_empty() {
        "WHERE true".to_string()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    }
}

fn is_safe_operator(op: &str) -> bool {
    matches!(op, "=" | "!=" | "<" | ">" | "<=" | ">=" | "like" | "ilike" | "in" | "not in")
}

pub fn sanitize_order(order: &str, valid_cols: &[String]) -> String {
    let parts: Vec<&str> = order.split(',').collect();
    let safe: Vec<String> = parts.iter().filter_map(|part| {
        let tokens: Vec<&str> = part.trim().splitn(2, ' ').collect();
        let col = tokens[0].trim_matches('"');
        let dir = tokens.get(1).map(|d| d.trim().to_uppercase()).unwrap_or_else(|| "ASC".to_string());
        if valid_cols.contains(&col.to_string()) && matches!(dir.as_str(), "ASC" | "DESC") {
            Some(format!("\"{}\" {}", col, dir))
        } else {
            None
        }
    }).collect();

    if safe.is_empty() { "id DESC".to_string() } else { safe.join(", ") }
}

async fn execute_dynamic_update(
    db: &sqlx::PgPool,
    table: &str,
    vals: &serde_json::Map<String, Value>,
    ids: &[i64],
) -> anyhow::Result<u64> {
    let valid_cols = get_table_columns(db, table).await;

    let filterable: Vec<(&String, &Value)> = vals.iter()
        .filter(|(k, _)| !k.starts_with('_') && !is_readonly_field(k) && valid_cols.contains(&k.to_string()))
        .collect();

    if filterable.is_empty() || ids.is_empty() {
        return Ok(0);
    }

    let mut set_parts = Vec::new();
    for (field, val) in &filterable {
        let sql_val = value_to_sql_literal(val);
        set_parts.push(format!("\"{}\" = {}", field, sql_val));
    }
    set_parts.push("write_date = NOW()".to_string());

    let ids_list = ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let sql = format!(
        "UPDATE {} SET {} WHERE id IN ({})",
        table, set_parts.join(", "), ids_list
    );

    debug!("execute_dynamic_update SQL: {}", &sql[..sql.len().min(200)]);

    let result = sqlx::query(&sql).execute(db).await?;
    Ok(result.rows_affected())
}

async fn execute_dynamic_create(
    db: &sqlx::PgPool,
    table: &str,
    vals: &serde_json::Map<String, Value>,
) -> anyhow::Result<i64> {
    let valid_cols = get_table_columns(db, table).await;

    let filterable: Vec<(&String, &Value)> = vals.iter()
        .filter(|(k, _)| !k.starts_with('_') && !is_readonly_field(k) && valid_cols.contains(&k.to_string()))
        .collect();

    if filterable.is_empty() {
        anyhow::bail!("No hay campos válidos para crear");
    }

    let cols = filterable.iter()
        .map(|(k, _)| format!("\"{}\"", k))
        .collect::<Vec<_>>()
        .join(", ");
    let sql_vals = filterable.iter()
        .map(|(_, v)| value_to_sql_literal(v))
        .collect::<Vec<_>>()
        .join(", ");

    let sql = format!(
        "INSERT INTO {} ({}, create_date, write_date) VALUES ({}, NOW(), NOW()) RETURNING id",
        table, cols, sql_vals
    );

    debug!("execute_dynamic_create SQL: {}", &sql[..sql.len().min(200)]);

    let row = sqlx::query(&sql).fetch_one(db).await?;
    use sqlx::Row;
    Ok(row.try_get::<i64, _>("id").unwrap_or(0))
}

fn value_to_sql_literal(val: &Value) -> String {
    match val {
        Value::Null => "NULL".to_string(),
        Value::Bool(b) => if *b { "TRUE".to_string() } else { "FALSE".to_string() },
        Value::Number(n) => n.to_string(),
        Value::String(s) => {
            let escaped = s.replace('\'', "''");
            format!("'{}'", escaped)
        }
        Value::Array(_) | Value::Object(_) => {
            let json_str = val.to_string().replace('\'', "''");
            format!("'{}'::jsonb", json_str)
        }
    }
}

/// Convierte rows de sqlx a Vec<Value> — intenta todos los tipos posibles.
/// Retorna `false` (no `null`) para valores NULL — OWL RelationalModel requiere
/// `false` como valor vacío; `null` causa crash en _loadRecords (.length de null).
pub fn rows_to_json(rows: Vec<sqlx::postgres::PgRow>, fields: &[String]) -> Vec<Value> {
    use sqlx::Row;
    use rust_decimal::Decimal;
    let none = Value::Bool(false);  // Odoo-compatible "empty" value
    rows.iter().map(|row| {
        let mut map = serde_json::Map::new();
        for field in fields {
            let val = if let Ok(v) = row.try_get::<Option<String>, _>(field.as_str()) {
                v.map(Value::String).unwrap_or_else(|| none.clone())
            } else if let Ok(v) = row.try_get::<Option<bool>, _>(field.as_str()) {
                v.map(Value::Bool).unwrap_or_else(|| none.clone())
            } else if let Ok(v) = row.try_get::<Option<i64>, _>(field.as_str()) {
                v.map(|n| Value::Number(n.into())).unwrap_or_else(|| none.clone())
            } else if let Ok(v) = row.try_get::<Option<i32>, _>(field.as_str()) {
                v.map(|n| Value::Number(n.into())).unwrap_or_else(|| none.clone())
            } else if let Ok(v) = row.try_get::<Option<Decimal>, _>(field.as_str()) {
                v.map(|d| Value::String(d.to_string())).unwrap_or_else(|| none.clone())
            } else if let Ok(v) = row.try_get::<Option<f64>, _>(field.as_str()) {
                v.and_then(|f| serde_json::Number::from_f64(f))
                    .map(Value::Number)
                    .unwrap_or_else(|| none.clone())
            } else if let Ok(v) = row.try_get::<Option<chrono::NaiveDateTime>, _>(field.as_str()) {
                v.map(|dt| Value::String(dt.format("%Y-%m-%d %H:%M:%S").to_string()))
                    .unwrap_or_else(|| none.clone())
            } else if let Ok(v) = row.try_get::<Option<chrono::NaiveDate>, _>(field.as_str()) {
                v.map(|d| Value::String(d.to_string())).unwrap_or_else(|| none.clone())
            } else if let Ok(v) = row.try_get::<Option<serde_json::Value>, _>(field.as_str()) {
                v.unwrap_or_else(|| none.clone())
            } else {
                none.clone()
            };
            map.insert(field.clone(), val);
        }
        Value::Object(map)
    }).collect()
}

