//! web — Handlers para servir la interfaz OWL nativa de Odoo.
//!
//! Contiene:
//! - Servidor estático: `/{addon}/static/{*path}`
//! - Servidor de attachments/assets: `/web/assets/{*path}`, `/web/content/{*path}`, `/web/image/{*path}`
//! - Bootstrap: `/web`
//! - JSON-RPC APIs: `version_info`, `bootstrap_translations`, `translations`, `load_menus`, `call_kw`, `search_read`

use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade, Message}, Path, Query, State, Form},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response, Redirect},
    Json,
};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};

use crate::state::AppState;

// ─── Constantes de sesión ────────────────────────────────────────────────────
const SESSION_COOKIE: &str = "nexustech_sid";
const SESSION_DURATION_SECS: u64 = 8 * 60 * 60; // 8 horas

/// Genera un token de sesión firmado: "uid:company_id:name:exp:hmac"
fn session_token_create(uid: i32, company_id: i32, name: &str, secret: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() + SESSION_DURATION_SECS;
    let name_b64 = {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode(name.as_bytes())
    };
    let payload = format!("{}:{}:{}:{}", uid, company_id, name_b64, exp);
    let sig = session_hmac(&payload, secret);
    format!("{}.{}", payload, sig)
}

/// Verifica y parsea un token de sesión. Retorna (uid, company_id, display_name) si es válido.
fn session_token_verify(token: &str, secret: &str) -> Option<(i32, i32, String)> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Formato: uid:company_id:name_b64:exp.hmac
    let dot = token.rfind('.')?;
    let (payload, sig) = token.split_at(dot);
    let sig = &sig[1..];
    // Verificar HMAC
    if session_hmac(payload, secret) != sig {
        return None;
    }
    let parts: Vec<&str> = payload.splitn(4, ':').collect();
    if parts.len() != 4 { return None; }
    let uid: i32 = parts[0].parse().ok()?;
    let company_id: i32 = parts[1].parse().ok()?;
    let exp: u64 = parts[3].parse().ok()?;
    if now > exp { return None; } // Expirado
    let name = {
        use base64::Engine;
        let bytes = base64::engine::general_purpose::STANDARD.decode(parts[2]).ok()?;
        String::from_utf8(bytes).ok()?
    };
    Some((uid, company_id, name))
}

/// HMAC-SHA256 simple usando ring
fn session_hmac(payload: &str, secret: &str) -> String {
    use base64::Engine;
    // ring ya está como dep workspace y disponible en el crate nexus-core
    // Lo usamos directamente ya que ring está en el Cargo.toml del workspace
    use ::ring::hmac;
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
    let sig = hmac::sign(&key, payload.as_bytes());
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(sig.as_ref())
}

/// Lee la cookie de sesión del header Cookie
fn extract_session_cookie(headers: &axum::http::HeaderMap) -> Option<String> {
    let cookie_header = headers.get(header::COOKIE)?.to_str().ok()?;
    for part in cookie_header.split(';') {
        let part = part.trim();
        if let Some(val) = part.strip_prefix(&format!("{}=", SESSION_COOKIE)) {
            return Some(val.to_string());
        }
    }
    None
}

#[derive(serde::Deserialize)]
pub struct LoginForm {
    pub login: String,
    pub password: String,
    pub redirect: Option<String>,
}

/// Mapea extensiones comunes a Mime types
fn guess_mime(path: &str) -> &'static str {
    let lower = path.to_lowercase();
    if lower.ends_with(".js") || lower.ends_with(".min.js") {
        "application/javascript"
    } else if lower.ends_with(".css") || lower.ends_with(".min.css") {
        "text/css"
    } else if lower.ends_with(".png") {
        "image/png"
    } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg"
    } else if lower.ends_with(".gif") {
        "image/gif"
    } else if lower.ends_with(".svg") {
        "image/svg+xml"
    } else if lower.ends_with(".woff") {
        "font/woff"
    } else if lower.ends_with(".woff2") {
        "font/woff2"
    } else if lower.ends_with(".ttf") {
        "font/ttf"
    } else if lower.ends_with(".otf") {
        "font/otf"
    } else if lower.ends_with(".json") {
        "application/json"
    } else if lower.ends_with(".xml") {
        "text/xml"
    } else if lower.ends_with(".ico") {
        "image/x-icon"
    } else {
        "application/octet-stream"
    }
}

/// GET /{addon}/static/{*path}
pub async fn serve_static(
    Path((addon, path)): Path<(String, String)>,
) -> impl IntoResponse {
    let mut resolved_addon = addon;
    if resolved_addon == "nexustech" {
        resolved_addon = "web".to_string();
    }
    let mut file_path = PathBuf::from("/home/ealvarez/workspace/NexustechERPv2/demo_addons")
        .join(&resolved_addon)
        .join("static")
        .join(&path);
        
    if !file_path.exists() {
        file_path = PathBuf::from("/home/ealvarez/workspace/erp/nexustech_produccion/core/nexustech/addons")
            .join(&resolved_addon)
            .join("static")
            .join(&path);
    }
    
    if !file_path.exists() {
        return StatusCode::NOT_FOUND.into_response();
    }
    
    match fs::read(&file_path).await {
        Ok(bytes) => {
            let mime = guess_mime(file_path.to_str().unwrap_or(""));
            let mut response = Response::new(axum::body::Body::from(bytes));
            response.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static(mime),
            );
            response
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// GET /web/bundle/{bundle_name}
/// GET /nexustech/bundle/{bundle_name}
///
/// Sirve el manifiesto JSON del bundle (lista de assets CSS/JS).
/// Si el bundle no está en nexustech_db, hace fallback al Odoo Python (port 8069),
/// persiste la URL encontrada en nexustech_db para futuras peticiones.
pub async fn serve_bundle(
    State(state): State<AppState>,
    Path(bundle_name): Path<String>,
) -> impl IntoResponse {
    let js_name = format!("{}.min.js", bundle_name);
    let css_name = format!("{}.min.css", bundle_name);

    let rows = sqlx::query!(
        "SELECT name, url FROM ir_attachment WHERE name IN ($1, $2) ORDER BY id ASC",
        js_name,
        css_name
    )
    .fetch_all(&state.db)
    .await;

    let mut list: Vec<serde_json::Value> = Vec::new();

    if let Ok(records) = rows {
        for r in records {
            let r_name = r.name;
            let r_url = r.url.unwrap_or_default();
            if r_name.ends_with(".js") {
                list.push(serde_json::json!({ "type": "script", "src": r_url }));
            } else if r_name.ends_with(".css") {
                list.push(serde_json::json!({ "type": "link", "src": r_url }));
            }
        }
    }

    // Si la DB no tiene el bundle, hacer fallback al Odoo Python y persistirlo
    if list.is_empty() {
        let odoo_url = format!(
            "http://localhost:8069/web/bundle/{}?lang=es_MX",
            bundle_name
        );
        tracing::info!("Bundle '{}' no encontrado en DB, haciendo proxy a Odoo: {}", bundle_name, odoo_url);

        if let Ok(resp) = reqwest::get(&odoo_url).await {
            if let Ok(odoo_list) = resp.json::<Vec<serde_json::Value>>().await {
                // Persistir cada asset en nexustech_db para evitar futuros fallbacks
                for item in &odoo_list {
                    let item_type = item.get("type").and_then(|v| v.as_str()).unwrap_or("");
                    let item_src  = item.get("src").and_then(|v| v.as_str()).unwrap_or("");
                    if item_src.is_empty() { continue; }

                    let att_name = if item_type == "script" { js_name.clone() } else { css_name.clone() };
                    let _ = sqlx::query!(
                        "INSERT INTO ir_attachment (name, url, type, res_model, create_uid, write_uid, create_date, write_date)
                         VALUES ($1, $2, 'url', 'ir.attachment', 1, 1, NOW(), NOW())
                         ON CONFLICT DO NOTHING",
                        att_name,
                        item_src
                    )
                    .execute(&state.db)
                    .await;
                }
                list = odoo_list;
                tracing::info!("Bundle '{}' obtenido de Odoo Python y persistido ({} items)", bundle_name, list.len());
            }
        } else {
            tracing::warn!("Bundle '{}' no encontrado ni en DB ni en Odoo Python", bundle_name);
        }
    }

    // Usar bytes directos con Content-Length explícito para evitar:
    // 1. Headers Content-Type duplicados (Json() ya los pone)
    // 2. transfer-encoding: chunked + gzip que puede causar body vacío en Chrome
    let body_bytes = serde_json::to_vec(&list).unwrap_or_else(|_| b"[]".to_vec());
    let body_len = body_bytes.len();

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::CONTENT_LENGTH, body_len)
        .header(header::CACHE_CONTROL, "no-cache, no-store, must-revalidate")
        .body(axum::body::Body::from(body_bytes))
        .unwrap()
}


/// GET /web/assets/{*path}
/// GET /web/content/{*path}
/// GET /web/image/{*path}
pub async fn serve_attachment(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
    uri: axum::http::Uri,
) -> impl IntoResponse {
    let mut uri_path = uri.path().to_string();
    if uri_path.starts_with("/nexustech/") {
        uri_path = uri_path.replacen("/nexustech/", "/web/", 1);
    }
    
    // 1. Buscar en ir_attachment por URL exacta
    let row = sqlx::query!(
        "SELECT store_fname, db_datas, mimetype FROM ir_attachment WHERE url = $1 ORDER BY id DESC LIMIT 1",
        uri_path
    )
    .fetch_optional(&state.db)
    .await;
    
    let mut record = None;
    if let Ok(Some(r)) = row {
        record = Some((r.store_fname, r.db_datas, r.mimetype));
    } else if uri_path.starts_with("/web/image") || uri_path.starts_with("/web/content") {
        let model = params.get("model").cloned().unwrap_or_default();
        let id_str = params.get("id").cloned().unwrap_or_default();
        let field = params.get("field").cloned().unwrap_or_default();

        // Campos computados de avatar — no existen como columnas en la DB,
        // siempre devolver PNG transparente directamente sin tocar la DB.
        let computed_avatar_fields = ["avatar_128", "avatar_256", "avatar_512",
            "avatar_1920", "image_128", "image_256", "image_512", "image_1920"];
        if computed_avatar_fields.contains(&field.as_str()) {
            let transparent_png: &[u8] = &[
                0x89,0x50,0x4E,0x47,0x0D,0x0A,0x1A,0x0A,
                0x00,0x00,0x00,0x0D,0x49,0x48,0x44,0x52,
                0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x01,
                0x08,0x06,0x00,0x00,0x00,0x1F,0x15,0xC4,0x89,
                0x00,0x00,0x00,0x0A,0x49,0x44,0x41,0x54,
                0x78,0x9C,0x62,0x00,0x01,0x00,0x00,0x05,0x00,
                0x01,0x0D,0x0A,0x2D,0xB4,
                0x00,0x00,0x00,0x00,0x49,0x45,0x4E,0x44,0xAE,0x42,0x60,0x82,
            ];
            return (StatusCode::OK, [(header::CONTENT_TYPE, "image/png")], transparent_png.to_vec()).into_response();
        }

        if !model.is_empty() && !id_str.is_empty() && !field.is_empty() {
            if let Ok(id) = id_str.parse::<i32>() {
                // 1. Buscar en ir_attachment
                let r = sqlx::query!(
                    "SELECT store_fname, db_datas, mimetype FROM ir_attachment WHERE res_model = $1 AND res_id = $2 AND res_field = $3 ORDER BY id DESC LIMIT 1",
                    model, id, field
                )
                .fetch_optional(&state.db)
                .await;

                if let Ok(Some(r)) = r {
                    record = Some((r.store_fname, r.db_datas, r.mimetype));
                } else {
                    // 2. Buscar como columna binaria directa en la tabla del modelo
                    let table = model.replace('.', "_");
                    if table.chars().all(|c| c.is_alphanumeric() || c == '_')
                        && field.chars().all(|c| c.is_alphanumeric() || c == '_')
                    {
                        let query_sql = format!("SELECT \"{}\" FROM {} WHERE id = $1", field, table);
                        let cell: Result<Option<Vec<u8>>, _> = sqlx::query_scalar(&query_sql)
                            .bind(id)
                            .fetch_optional(&state.db)
                            .await;
                        if let Ok(Some(bytes)) = cell {
                            if !bytes.is_empty() {
                                use base64::Engine;
                                let raw = if let Ok(dec) = base64::prelude::BASE64_STANDARD.decode(&bytes) { dec } else { bytes };
                                let mime = guess_mime(&field);
                                return (StatusCode::OK, [(header::CONTENT_TYPE, mime)], raw).into_response();
                            }
                        }
                    }
                }
            }
        }
    } else if uri_path.starts_with("/web/image/") {
        // Parsear path segments: /web/image/{model}/{id}/{field}
        // ej: /web/image/res.users/2/avatar_128
        let parts: Vec<&str> = uri_path.trim_start_matches("/web/image/").splitn(3, '/').collect();
        if parts.len() >= 2 {
            let m = parts[0].replace(',', "."); // res.users o res,users
            let id_s = parts[1];
            let f = if parts.len() >= 3 { parts[2] } else { "image_128" };
            // Quitar sufijos de tamaño tipo avatar_128, image_256, etc.
            let field_clean = f.split('?').next().unwrap_or(f);
            if let Ok(id) = id_s.parse::<i32>() {
                // Intentar primero campo directo en la tabla
                let table = m.replace('.', "_");
                if table.chars().all(|c| c.is_alphanumeric() || c == '_')
                    && field_clean.chars().all(|c| c.is_alphanumeric() || c == '_')
                {
                    let query_sql = format!("SELECT \"{}\" FROM {} WHERE id = $1", field_clean, table);
                    let cell: Result<Option<Vec<u8>>, _> = sqlx::query_scalar(&query_sql)
                        .bind(id)
                        .fetch_optional(&state.db)
                        .await;
                    if let Ok(Some(bytes)) = cell {
                        if !bytes.is_empty() {
                            use base64::Engine;
                            let raw = if let Ok(dec) = base64::prelude::BASE64_STANDARD.decode(&bytes) { dec } else { bytes };
                            return (StatusCode::OK, [(header::CONTENT_TYPE, "image/png")], raw).into_response();
                        }
                    }
                    // Fallback: ir_attachment
                    let att = sqlx::query!(
                        "SELECT store_fname, db_datas, mimetype FROM ir_attachment WHERE res_model = $1 AND res_id = $2 AND res_field = $3 ORDER BY id DESC LIMIT 1",
                        m, id, field_clean
                    ).fetch_optional(&state.db).await;
                    if let Ok(Some(r)) = att {
                        record = Some((r.store_fname, r.db_datas, r.mimetype));
                    }
                }
            }
        }
    }
    
    if let Some((store_fname, db_datas, mimetype)) = record {
        let mut mime = mimetype.unwrap_or_else(|| "application/octet-stream".to_string());
        if mime == "application/octet-stream" {
            let guessed = guess_mime(&uri_path);
            if guessed != "application/octet-stream" {
                mime = guessed.to_string();
            }
        }
        
        if let Some(fname) = store_fname {
            // 1. Buscar en filestore de nexustech_db
            let filestore_path = PathBuf::from("/home/ealvarez/.local/share/NEXUSTECH ERP/filestore/nexustech_db").join(&fname);
            if filestore_path.exists() {
                if let Ok(bytes) = fs::read(&filestore_path).await {
                    return (
                        StatusCode::OK,
                        [(header::CONTENT_TYPE, mime.as_str())],
                        bytes,
                    ).into_response();
                }
            }
            
            // 2. Fallback: buscar en filestore de la DB 'nexus' (Odoo Python)
            let nexus_filestore = PathBuf::from("/home/ealvarez/.local/share/NEXUSTECH ERP/filestore/nexus").join(&fname);
            if nexus_filestore.exists() {
                if let Ok(bytes) = fs::read(&nexus_filestore).await {
                    tracing::info!("Asset servido desde filestore/nexus: {}", fname);
                    return (
                        StatusCode::OK,
                        [(header::CONTENT_TYPE, mime.as_str())],
                        bytes,
                    ).into_response();
                }
            }
        }
        
        if let Some(datas) = db_datas {
            return (StatusCode::OK, [(header::CONTENT_TYPE, mime.as_str())], datas).into_response();
        }
    }
    
    // 3. Último recurso: proxy al Odoo Python para assets web
    if uri_path.starts_with("/web/assets/") || uri_path.starts_with("/web/content/") {
        let odoo_asset_url = format!("http://localhost:8069{}", uri_path);
        tracing::info!("Asset no encontrado localmente, proxy a Odoo: {}", odoo_asset_url);
        if let Ok(resp) = reqwest::get(&odoo_asset_url).await {
            if resp.status().is_success() {
                let content_type = resp.headers()
                    .get("content-type")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("application/octet-stream")
                    .to_string();
                if let Ok(bytes) = resp.bytes().await {
                    return Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, content_type)
                        .header(header::CONTENT_LENGTH, bytes.len())
                        .body(axum::body::Body::from(bytes))
                        .unwrap();
                }
            }
        }
    }
    
    // Fallback universal: PNG 1x1 transparente para imágenes no encontradas
    // Evita errores 404 visibles en el frontend
    if uri_path.contains("/image/") || uri_path.ends_with("/avatar_128")
        || uri_path.ends_with("/image_128") || uri_path.ends_with("/image_256")
    {
        // PNG 1×1 transparente (bytes exactos del formato PNG)
        let transparent_png: &[u8] = &[
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // signature
            0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
            0x08, 0x06, 0x00, 0x00, 0x00, 0x1F, 0x15, 0xC4,
            0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, // IDAT chunk
            0x54, 0x78, 0x9C, 0x62, 0x00, 0x01, 0x00, 0x00,
            0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00,
            0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, // IEND chunk
            0x42, 0x60, 0x82,
        ];
        return (StatusCode::OK, [(header::CONTENT_TYPE, "image/png")], transparent_png.to_vec()).into_response();
    }

    StatusCode::NOT_FOUND.into_response()
}

/// GET /web
pub async fn bootstrap(
    State(state): State<AppState>,
    uri: axum::http::Uri,
    headers: axum::http::HeaderMap,
) -> Response {
    let path = uri.path();
    let prefix = if path.starts_with("/nexustech") { "/nexustech" } else { "/web" };
    let login_url = format!("{}/login", prefix);

    // ── Verificar sesión ───────────────────────────────────────────────────
    let session = extract_session_cookie(&headers)
        .and_then(|tok| session_token_verify(&tok, &state.config.jwt_secret));

    let (session_uid, session_company, session_name) = match session {
        Some(s) => s,
        None => {
            // Sin sesión válida → redirigir al login
            return Redirect::to(&login_url).into_response();
        }
    };

    // Obtener datos del usuario autenticado para el session_info
    let user_data: Option<(String, i32, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT u.login, p.id, p.name::text, p.email \
         FROM res_users u \
         LEFT JOIN res_partner p ON p.id = u.partner_id \
         WHERE u.id = $1 AND u.active = true LIMIT 1"
    )
    .bind(session_uid)
    .fetch_optional(&state.db)
    .await
    .unwrap_or(None);

    let (username, partner_id, partner_name, partner_email) = if let Some((login, pid, pname, pemail)) = user_data {
        (
            login,
            pid,
            pname.unwrap_or_else(|| session_name.clone()),
            pemail.unwrap_or_else(|| session_name.clone()),
        )
    } else {
        ("admin".to_string(), 3_i32, session_name.clone(), session_name.clone())
    };

    let print_css = sqlx::query_scalar!(
        "SELECT url FROM ir_attachment WHERE name = 'web.assets_web_print.min.css' ORDER BY id DESC LIMIT 1"
    )
    .fetch_optional(&state.db)
    .await
    .unwrap_or(None)
    .flatten()
    .unwrap_or_else(|| "/web/assets/6293f7f/web.assets_web_print.min.css".to_string());
    
    let web_css = sqlx::query_scalar!(
        "SELECT url FROM ir_attachment WHERE name = 'web.assets_web.min.css' ORDER BY id DESC LIMIT 1"
    )
    .fetch_optional(&state.db)
    .await
    .unwrap_or(None)
    .flatten()
    .unwrap_or_else(|| "/web/assets/a9cd50e/web.assets_web.min.css".to_string());
    
    let web_js = sqlx::query_scalar!(
        "SELECT url FROM ir_attachment WHERE name = 'web.assets_web.min.js' ORDER BY id DESC LIMIT 1"
    )
    .fetch_optional(&state.db)
    .await
    .unwrap_or(None)
    .flatten()
    .unwrap_or_else(|| "/web/assets/f270e8d/web.assets_web.min.js".to_string());
    
    let group_rows = sqlx::query!(
        "SELECT CONCAT(module, '.', name) AS xml_id FROM ir_model_data WHERE model = 'res.groups'"
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let mut groups_map = serde_json::Map::new();
    groups_map.insert("base.group_allow_export".to_string(), serde_json::json!(true));
    for r in group_rows {
        if let Some(xml_id) = r.xml_id {
            groups_map.insert(xml_id.clone(), serde_json::json!(true));
            if xml_id.starts_with("base_nexustech.") {
                groups_map.insert(xml_id.replace("base_nexustech.", "base."), serde_json::json!(true));
            }
        }
    }

    let session_info = serde_json::json!({
        "uid": session_uid,
        "is_system": true,
        "is_admin": true,
        "is_public": false,
        "is_internal_user": true,
        "name": partner_name,
        "username": username,
        "user_context": {
            "lang": "es_MX",
            "tz": "America/Mexico_City",
            "uid": session_uid
        },
        "db": "nexustech_db",
        "registry_hash": "6293f7f",
        "user_settings": {
            "id": session_uid,
            "user_id": session_uid
        },
        "server_version": "19.0+nexustech+e",
        "server_version_info": [19, 0, 0, "final", 0, "e"],
        "support_url": "https://nexustech.internal/buy",
        "partner_display_name": partner_email,
        "partner_id": partner_id,
        "currencies": {
            "1": {
                "symbol": "$",
                "position": "before",
                "digits": [69, 2]
            }
        },
        "bundle_params": {
            "lang": "es_MX"
        },
        "active_ids_limit": 20000,
        "max_file_upload_size": 128000000,
        "groups": groups_map,
        "user_companies": {
            "current_company": session_company,
            "allowed_companies": {
                "1": {
                    "id": session_company,
                    "name": "NexusTech",
                    "sequence": 1,
                    "child_ids": [],
                    "parent_id": null,
                    "currency_id": 1
                }
            },
            "disallowed_ancestor_companies": {}
        },
        "show_effect": true,
        "view_info": {
            "list":      {"display_name": "Lista",          "icon": "oi oi-view-list",              "multi_record": true},
            "form":      {"display_name": "Formulario",     "icon": "fa fa-address-card",           "multi_record": false},
            "graph":     {"display_name": "Gráfico",        "icon": "fa fa-area-chart",             "multi_record": true},
            "pivot":     {"display_name": "Tabla dinámica", "icon": "oi oi-view-pivot",             "multi_record": true},
            "kanban":    {"display_name": "Kanban",         "icon": "oi oi-view-kanban",            "multi_record": true},
            "calendar":  {"display_name": "Calendario",     "icon": "fa fa-calendar",               "multi_record": true},
            "search":    {"display_name": "Búsqueda",       "icon": "oi oi-search",                 "multi_record": true},
            "gantt":     {"display_name": "Gantt",          "icon": "fa fa-tasks",                  "multi_record": true},
            "cohort":    {"display_name": "Cohorte",        "icon": "oi oi-view-cohort",            "multi_record": true},
            "hierarchy": {"display_name": "Jerarquía",      "icon": "fa fa-share-alt fa-rotate-90", "multi_record": true},
            "grid":      {"display_name": "Cuadrícula",     "icon": "fa fa-th",                     "multi_record": true},
            "activity":  {"display_name": "Actividad",      "icon": "fa fa-clock-o",                "multi_record": true},
            "map":       {"display_name": "Mapa",           "icon": "fa fa-map-marker",             "multi_record": true}
        }
    });
    
    let html = format!(
        r##"<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8"/>
        <meta http-equiv="X-UA-Compatible" content="IE=edge"/>
        <title>NEXUSTECH ERP</title>
        <link type="image/x-icon" rel="shortcut icon" href="/web/static/img/favicon.ico"/>
        <script id="web.layout.ntscript" type="text/javascript">
            var nexustech = {{
                csrf_token: "dummy_csrf_token",
                debug: "",
            }};
            window['odoo'] = nexustech;
        </script>
        <meta name="viewport" content="width=device-width, initial-scale=1, user-scalable=no"/>
        <meta name="theme-color" content="#71639e"/>
        <link rel="manifest" href="/web/manifest.webmanifest" crossorigin="use-credentials"/>
        <link rel="apple-touch-icon" href="/web/static/img/nexustech-icon.png"/>
        <script type="text/javascript">
            {{
                nexustech.__session_info__ = {};
                const {{ user_context }} = nexustech.__session_info__;
                const lang = new URLSearchParams(document.location.search).get("lang");
                let menuURL = "/web/webclient/load_menus";
                if (lang) {{
                    user_context.lang = lang;
                    menuURL += `&lang=${{lang}}`;
                }}
                nexustech.reloadMenus = () => fetch(menuURL, {{ cache: "no-store" }}).then(res => res.json()).then(data => data.result || data);
                nexustech.loadMenusPromise = nexustech.reloadMenus();
            }}
        </script>
        <link rel="stylesheet" href="{}" media="print"/>
        <script src="{}" defer="defer"></script>
        <link rel="stylesheet" href="{}" media="screen"/>
    </head>
    <body class="o_web_client">
    </body>
</html>"##,
        session_info.to_string(),
        print_css,
        web_js,
        web_css
    );
    
    let html = if prefix == "/nexustech" {
        html.replace("/web/", "/nexustech/")
    } else {
        html
    };
    
    Response::builder()
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(axum::body::Body::from(html))
        .unwrap()
        .into_response()
}

// ---------------------------------------------------------------------------
// Login / Logout — Autenticación Web
// ---------------------------------------------------------------------------

/// GET /web/login  |  GET /nexustech/login
/// Muestra la pantalla de inicio de sesión.
pub async fn login_page(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    // Si ya tiene sesión válida, redirigir al ERP
    let prefix = if uri.path().starts_with("/nexustech") { "/nexustech" } else { "/web" };
    if let Some(tok) = extract_session_cookie(&headers) {
        if session_token_verify(&tok, &state.config.jwt_secret).is_some() {
            return Redirect::to(prefix).into_response();
        }
    }
    let error_msg = params.get("error").map(|e| e.as_str()).unwrap_or("");
    let redirect = params.get("redirect").map(|r| r.as_str()).unwrap_or("");
    let error_html = if !error_msg.is_empty() {
        format!(r#"<div class="nt-login-error"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>{}</div>
"#, error_msg)
    } else {
        String::new()
    };
    let html = format!(r#"<!DOCTYPE html>
<html lang="es">
<head>
  <meta charset="utf-8"/>
  <meta name="viewport" content="width=device-width, initial-scale=1, user-scalable=no"/>
  <title>NexusTech ERP — Iniciar Sesión</title>
  <link rel="shortcut icon" type="image/x-icon" href="/web/static/img/favicon.ico"/>
  <style>
    *, *::before, *::after {{ box-sizing: border-box; margin: 0; padding: 0; }}
    @import url('https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap');
    :root {{
      --bg:        #0f1117;
      --surface:   #1a1d27;
      --surface2:  #21253a;
      --accent:    #7c5cbf;
      --accent-h:  #9b7de0;
      --accent-g:  linear-gradient(135deg, #7c5cbf 0%, #5b8dee 100%);
      --text:      #e8eaf6;
      --text-mute: #8b92b8;
      --border:    rgba(124,92,191,0.25);
      --danger:    #ef4444;
      --radius:    14px;
      --shadow:    0 25px 60px rgba(0,0,0,0.5);
    }}
    html, body {{ height: 100%; font-family: 'Inter', system-ui, sans-serif; background: var(--bg); color: var(--text); }}

    /* ── Background radial ── */
    body {{
      display: flex; align-items: center; justify-content: center; min-height: 100vh;
      background:
        radial-gradient(ellipse 900px 600px at 20% 20%, rgba(124,92,191,0.18) 0%, transparent 70%),
        radial-gradient(ellipse 700px 500px at 80% 80%, rgba(91,141,238,0.12) 0%, transparent 70%),
        var(--bg);
    }}

    /* ── Card ── */
    .nt-card {{
      width: min(420px, 95vw);
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: 20px;
      box-shadow: var(--shadow);
      padding: 48px 44px 40px;
      animation: fadeUp 0.4s cubic-bezier(.16,1,.3,1) both;
    }}
    @keyframes fadeUp {{
      from {{ opacity:0; transform: translateY(20px) scale(.98); }}
      to   {{ opacity:1; transform: translateY(0)   scale(1);    }}
    }}

    /* ── Logo ── */
    .nt-logo {{
      display: flex; align-items: center; gap: 12px; margin-bottom: 36px;
    }}
    .nt-logo-icon {{
      width: 44px; height: 44px; border-radius: 12px;
      background: var(--accent-g);
      display: flex; align-items: center; justify-content: center;
      box-shadow: 0 4px 20px rgba(124,92,191,0.4);
    }}
    .nt-logo-icon svg {{ width: 24px; height: 24px; fill: white; }}
    .nt-logo-text {{ line-height: 1; }}
    .nt-logo-text strong {{ display: block; font-size: 18px; font-weight: 700; letter-spacing: -.3px; color: #fff; }}
    .nt-logo-text span {{ font-size: 11px; color: var(--text-mute); font-weight: 400; letter-spacing: .5px; text-transform: uppercase; }}

    /* ── Heading ── */
    .nt-heading {{ margin-bottom: 28px; }}
    .nt-heading h1 {{ font-size: 22px; font-weight: 700; color: #fff; letter-spacing: -.4px; }}
    .nt-heading p  {{ margin-top: 4px; font-size: 13px; color: var(--text-mute); }}

    /* ── Error ── */
    .nt-login-error {{
      display: flex; align-items: center; gap: 8px;
      background: rgba(239,68,68,0.12);
      border: 1px solid rgba(239,68,68,0.35);
      border-radius: 10px; padding: 10px 14px;
      font-size: 13px; color: #fca5a5;
      margin-bottom: 20px;
    }}
    .nt-login-error svg {{ flex-shrink: 0; }}

    /* ── Form ── */
    .nt-form {{ display: flex; flex-direction: column; gap: 16px; }}
    .nt-field {{ display: flex; flex-direction: column; gap: 6px; }}
    .nt-field label {{ font-size: 12px; font-weight: 500; color: var(--text-mute); letter-spacing: .3px; text-transform: uppercase; }}
    .nt-field-wrap {{ position: relative; }}
    .nt-field-wrap .nt-icon {{
      position: absolute; left: 14px; top: 50%; transform: translateY(-50%);
      width: 16px; height: 16px; color: var(--text-mute); pointer-events: none;
    }}
    .nt-input {{
      width: 100%; padding: 12px 14px 12px 42px;
      background: var(--surface2); border: 1px solid rgba(255,255,255,0.08);
      border-radius: 10px; color: var(--text); font-size: 14px; font-family: inherit;
      outline: none; transition: border-color .2s, box-shadow .2s;
    }}
    .nt-input::placeholder {{ color: rgba(139,146,184,0.5); }}
    .nt-input:focus {{
      border-color: var(--accent); box-shadow: 0 0 0 3px rgba(124,92,191,0.2);
    }}

    /* ── Toggle password ── */
    .nt-eye {{
      position: absolute; right: 12px; top: 50%; transform: translateY(-50%);
      background: none; border: none; cursor: pointer; color: var(--text-mute);
      padding: 4px; border-radius: 6px; display: flex;
      transition: color .2s;
    }}
    .nt-eye:hover {{ color: var(--text); }}

    /* ── Submit ── */
    .nt-btn {{
      margin-top: 8px; padding: 13px;
      background: var(--accent-g);
      border: none; border-radius: 10px;
      color: #fff; font-size: 14px; font-weight: 600; font-family: inherit;
      cursor: pointer; transition: opacity .2s, transform .1s;
      box-shadow: 0 4px 20px rgba(124,92,191,0.35);
    }}
    .nt-btn:hover  {{ opacity: .9; }}
    .nt-btn:active {{ transform: scale(.98); }}
    .nt-btn:disabled {{ opacity: .6; cursor: not-allowed; }}

    /* ── Spinner inside button ── */
    .nt-spinner {{
      display: none; width: 18px; height: 18px; margin: 0 auto;
      border: 2px solid rgba(255,255,255,0.3); border-top-color: #fff;
      border-radius: 50%; animation: spin .7s linear infinite;
    }}
    @keyframes spin {{ to {{ transform: rotate(360deg); }} }}

    /* ── Footer ── */
    .nt-footer {{ margin-top: 28px; text-align: center; font-size: 11px; color: var(--text-mute); }}
    .nt-footer strong {{ color: rgba(139,146,184,0.7); }}
  </style>
</head>
<body>
  <div class="nt-card">
    <!-- Logo -->
    <div class="nt-logo">
      <div class="nt-logo-icon">
        <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
        </svg>
      </div>
      <div class="nt-logo-text">
        <strong>NexusTech ERP</strong>
        <span>Sistema Empresarial</span>
      </div>
    </div>

    <!-- Heading -->
    <div class="nt-heading">
      <h1>Bienvenido</h1>
      <p>Ingresa tus credenciales para continuar</p>
    </div>

    <!-- Error (si lo hay) -->
    {error_html}

    <!-- Form -->
    <form class="nt-form" method="POST" action="{prefix}/login" id="nt-login-form">
      <input type="hidden" name="redirect" value="{redirect}"/>

      <div class="nt-field">
        <label for="nt-login">Usuario o correo</label>
        <div class="nt-field-wrap">
          <svg class="nt-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
            <circle cx="12" cy="7" r="4"/>
          </svg>
          <input id="nt-login" class="nt-input" type="text" name="login"
                 placeholder="admin" autocomplete="username" required autofocus/>
        </div>
      </div>

      <div class="nt-field">
        <label for="nt-password">Contraseña</label>
        <div class="nt-field-wrap">
          <svg class="nt-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
            <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
          </svg>
          <input id="nt-password" class="nt-input" type="password" name="password"
                 placeholder="••••••••" autocomplete="current-password" required/>
          <button type="button" class="nt-eye" onclick="togglePass()" aria-label="Mostrar contraseña">
            <svg id="eye-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16">
              <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
              <circle cx="12" cy="12" r="3"/>
            </svg>
          </button>
        </div>
      </div>

      <button type="submit" class="nt-btn" id="nt-submit">
        <span id="nt-btn-text">Iniciar sesión</span>
        <div class="nt-spinner" id="nt-spinner"></div>
      </button>
    </form>

    <div class="nt-footer">
      <strong>NexusTech ERP</strong> &copy; 2025 — Todos los derechos reservados
    </div>
  </div>

  <script>
    function togglePass() {{
      const inp = document.getElementById('nt-password');
      const ico = document.getElementById('eye-icon');
      if (inp.type === 'password') {{
        inp.type = 'text';
        ico.innerHTML = '<path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/><line x1="1" y1="1" x2="23" y2="23"/>';
      }} else {{
        inp.type = 'password';
        ico.innerHTML = '<path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/>';
      }}
    }}
    document.getElementById('nt-login-form').addEventListener('submit', () => {{
      document.getElementById('nt-btn-text').style.display = 'none';
      document.getElementById('nt-spinner').style.display = 'block';
      document.getElementById('nt-submit').disabled = true;
    }});
  </script>
</body>
</html>
"#,
        error_html = error_html,
        prefix = prefix,
        redirect = redirect
    );
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(axum::body::Body::from(html))
        .unwrap()
        .into_response()
}

/// POST /web/login  |  POST /nexustech/login
/// Procesa las credenciales del formulario de login.
pub async fn web_login(
    State(state): State<AppState>,
    uri: axum::http::Uri,
    Form(form): Form<LoginForm>,
) -> Response {
    let prefix = if uri.path().starts_with("/nexustech") { "/nexustech" } else { "/web" };
    let login_fail_url = format!("{}/login?error=Credenciales+incorrectas", prefix);
    let redirect_to = if form.redirect.as_deref().unwrap_or("").is_empty() {
        prefix.to_string()
    } else {
        form.redirect.clone().unwrap_or_else(|| prefix.to_string())
    };

    // Autenticar contra la DB usando la lógica existente de nexus_core
    let datos = nexus_core::db::user::autenticar(&state.db, &form.login, &form.password).await;
    let datos = match datos {
        Ok(d) => d,
        Err(e) => {
            tracing::warn!("[Login] Fallo para '{}': {:?}", form.login, e);
            return Redirect::to(&login_fail_url).into_response();
        }
    };

    // Obtener nombre para mostrar en la UI
    let display_name: Option<String> = sqlx::query_scalar(
        "SELECT p.name::text FROM res_users u \
         LEFT JOIN res_partner p ON p.id = u.partner_id \
         WHERE u.id = $1 LIMIT 1"
    )
    .bind(datos.user_id)
    .fetch_optional(&state.db)
    .await
    .unwrap_or(None)
    .flatten();
    let name = display_name.unwrap_or_else(|| form.login.clone());

    // Crear token de sesión firmado
    let token = session_token_create(datos.user_id, datos.company_id, &name, &state.config.jwt_secret);

    tracing::info!("[Login] Sesión iniciada: user_id={} login='{}'  -> {}", datos.user_id, form.login, redirect_to);

    // Setear cookie HttpOnly, SameSite=Lax
    let cookie = format!(
        "{}={}; Path=/; Max-Age={}; HttpOnly; SameSite=Lax",
        SESSION_COOKIE, token, SESSION_DURATION_SECS
    );
    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, redirect_to)
        .header(header::SET_COOKIE, cookie)
        .body(axum::body::Body::empty())
        .unwrap()
        .into_response()
}

/// GET /web/logout  |  GET /nexustech/logout
/// Cierra la sesión borrando la cookie y redirige al login.
pub async fn web_logout(uri: axum::http::Uri) -> Response {
    let prefix = if uri.path().starts_with("/nexustech") { "/nexustech" } else { "/web" };
    let login_url = format!("{}/login", prefix);
    // Invalidar cookie (max-age=0)
    let cookie = format!("{}=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax", SESSION_COOKIE);
    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, login_url)
        .header(header::SET_COOKIE, cookie)
        .body(axum::body::Body::empty())
        .unwrap()
        .into_response()
}

// ---------------------------------------------------------------------------
// JSON-RPC Modelos
// ---------------------------------------------------------------------------

#[derive(serde::Deserialize)]
pub struct JsonRpcRequest {
    #[allow(dead_code)]
    pub jsonrpc: String,
    #[allow(dead_code)]
    pub method: String,
    pub params: serde_json::Value,
    pub id: Option<serde_json::Value>,
}

#[derive(serde::Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub result: serde_json::Value,
    pub id: Option<serde_json::Value>,
}

/// POST /web/webclient/version_info
pub async fn version_info(
    Json(payload): Json<JsonRpcRequest>,
) -> impl IntoResponse {
    let result = serde_json::json!({
        "server_version": "19.0+nexustech+e",
        "server_version_info": [19, 0, 0, "final", 0, "e"]
    });
    Json(JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result,
        id: payload.id,
    })
}

/// POST /web/webclient/bootstrap_translations
pub async fn bootstrap_translations(
    Json(payload): Json<JsonRpcRequest>,
) -> impl IntoResponse {
    let result = serde_json::json!({
        "modules": {},
        "lang_parameters": {
            "name": "Spanish (MX) / Español (MX)",
            "code": "es_MX",
            "direction": "ltr",
            "date_format": "%d/%m/%Y",
            "time_format": "%H:%M:%S",
            "grouping": "[3, 0]",
            "decimal_point": ".",
            "thousands_sep": ","
        }
    });
    Json(JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result,
        id: payload.id,
    })
}

/// GET /web/webclient/translations
pub async fn translations() -> impl IntoResponse {
    let result = serde_json::json!({
        "lang": "es_MX",
        "hash": "dummy_translations_hash",
        "modules": {},
        "lang_parameters": {
            "name": "Spanish (MX) / Español (MX)",
            "code": "es_MX",
            "direction": "ltr",
            "date_format": "%d/%m/%Y",
            "time_format": "%H:%M:%S",
            "grouping": "[3, 0]",
            "decimal_point": ".",
            "thousands_sep": ","
        },
        "multi_lang": false
    });
    Json(result)
}

/// GET /web/webclient/load_menus
pub async fn load_menus(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let rows = match sqlx::query!(
        r#"
        SELECT 
            m.id,
            m.parent_id,
            m.name,
            m.action,
            m.sequence,
            m.web_icon,
            d.name as "xml_id?"
        FROM ir_ui_menu m
        LEFT JOIN ir_model_data d ON d.res_id = m.id AND d.model = 'ir.ui.menu'
        WHERE m.active = true
        ORDER BY m.parent_id, m.sequence, m.id
        "#
    )
    .fetch_all(&state.db)
    .await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("Error querying ir_ui_menu: {:?}", e);
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::Body::from(format!("Database error: {:?}", e)))
                .unwrap()
                .into_response();
        }
    };
    
    let attachments = match sqlx::query!(
        "SELECT res_id, db_datas, mimetype FROM ir_attachment WHERE res_model = 'ir.ui.menu' AND res_field = 'web_icon_data'"
    )
    .fetch_all(&state.db)
    .await {
        Ok(a) => a,
        Err(e) => {
            tracing::error!("Error querying icon attachments: {:?}", e);
            Vec::new()
        }
    };
    
    use std::collections::HashMap;
    let mut attachment_map = HashMap::new();
    for att in attachments {
        if let Some(res_id) = att.res_id {
            attachment_map.insert(res_id, (att.db_datas, att.mimetype));
        }
    }
    
    let mut children_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut root_children: Vec<i32> = Vec::new();
    for row in &rows {
        if let Some(pid) = row.parent_id {
            children_map.entry(pid).or_default().push(row.id);
        } else {
            root_children.push(row.id);
        }
    }
    
    let mut app_map: HashMap<i32, i32> = HashMap::new();
    fn set_app_id(
        menu_id: i32,
        app_id: i32,
        children_map: &HashMap<i32, Vec<i32>>,
        app_map: &mut HashMap<i32, i32>,
    ) {
        app_map.insert(menu_id, app_id);
        if let Some(children) = children_map.get(&menu_id) {
            for &child_id in children {
                set_app_id(child_id, app_id, children_map, app_map);
            }
        }
    }
    for &root_id in &root_children {
        set_app_id(root_id, root_id, &children_map, &mut app_map);
    }
    
    #[allow(dead_code)]
    struct MenuInfo {
        id: i32,
        name: String,
        parent_id: Option<i32>,
        action: Option<String>,
        web_icon: Option<String>,
        xml_id: Option<String>,
    }
    let mut menu_info_map: HashMap<i32, MenuInfo> = HashMap::new();
    for row in &rows {
        let val = &row.name;
        let name_str = if let Some(s) = val.as_str() {
            s.to_string()
        } else if let Some(obj) = val.as_object() {
            obj.get("es_MX")
                .or_else(|| obj.get("en_US"))
                .or_else(|| obj.values().next())
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        } else {
            val.to_string()
        };
        menu_info_map.insert(row.id, MenuInfo {
            id: row.id,
            name: name_str,
            parent_id: row.parent_id,
            action: row.action.clone(),
            web_icon: row.web_icon.clone(),
            xml_id: row.xml_id.clone(),
        });
    }
    
    fn find_first_action(
        menu_id: i32,
        menu_info_map: &HashMap<i32, MenuInfo>,
        children_map: &HashMap<i32, Vec<i32>>,
    ) -> (Option<String>, Option<i32>) {
        if let Some(info) = menu_info_map.get(&menu_id) {
            if let Some(ref act) = info.action {
                if act.contains(',') {
                    let parts: Vec<&str> = act.split(',').collect();
                    if parts.len() >= 2 {
                        let model = parts[0].to_string();
                        let id = parts[1].parse::<i32>().ok();
                        return (Some(model), id);
                    }
                }
            }
        }
        if let Some(children) = children_map.get(&menu_id) {
            for &child_id in children {
                let res = find_first_action(child_id, menu_info_map, children_map);
                if res.0.is_some() {
                    return res;
                }
            }
        }
        (None, None)
    }

    
    let mut web_menus = serde_json::Map::new();
    
    // Root virtual menu
    web_menus.insert("root".to_string(), serde_json::json!({
        "id": "root",
        "name": "root",
        "children": root_children,
        "appID": false,
        "xmlid": "",
        "actionID": false,
        "actionModel": false,
        "actionPath": false,
        "webIcon": serde_json::Value::Null,
        "webIconData": serde_json::Value::Null,
        "webIconDataMimetype": serde_json::Value::Null,
    }));
    
    for row in &rows {
        let menu_id = row.id;
        let children = children_map.get(&menu_id).cloned().unwrap_or_default();
        let app_id = app_map.get(&menu_id).cloned();
        
        let val = &row.name;
        let name_str = if let Some(s) = val.as_str() {
            s.to_string()
        } else if let Some(obj) = val.as_object() {
            obj.get("es_MX")
                .or_else(|| obj.get("en_US"))
                .or_else(|| obj.values().next())
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        } else {
            val.to_string()
        };
        
        let mut action_model = serde_json::Value::Null;
        let mut action_id = serde_json::Value::Null;
        
        if let Some(ref act) = row.action {
            if act.contains(',') {
                let parts: Vec<&str> = act.split(',').collect();
                if parts.len() >= 2 {
                    action_model = serde_json::json!(parts[0]);
                    action_id = parts[1].parse::<i32>().map(|id| serde_json::json!(id)).unwrap_or(serde_json::Value::Null);
                }
            }
        }
        
        if Some(menu_id) == app_id {
            if action_id.is_null() {
                let (act_model, act_id) = find_first_action(menu_id, &menu_info_map, &children_map);
                if let (Some(m), Some(id)) = (act_model, act_id) {
                    action_model = serde_json::json!(m);
                    action_id = serde_json::json!(id);
                }
            }
        }
        
        let web_icon = row.web_icon.clone().map(|w| serde_json::json!(w)).unwrap_or(serde_json::Value::Null);
        let mut web_icon_data = serde_json::Value::Null;
        let mut web_icon_data_mimetype = serde_json::Value::Null;
        
        if let Some((db_datas, mimetype)) = attachment_map.get(&menu_id) {
            let mime = mimetype.clone().unwrap_or_else(|| "image/png".to_string());
            if let Some(ref bytes) = db_datas {
                use base64::Engine;
                let b64 = base64::engine::general_purpose::STANDARD.encode(bytes);
                web_icon_data = serde_json::json!(format!("data:{};base64,{}", mime, b64));
                web_icon_data_mimetype = serde_json::json!(mime);
            }
        }
        
        if Some(menu_id) == app_id && web_icon_data.is_null() {
            let mut has_bg = false;
            if let Some(ref icon) = row.web_icon {
                let parts: Vec<&str> = icon.split(',').collect();
                if parts.len() == 3 {
                    has_bg = true;
                }
            }
            if !has_bg {
                web_icon_data = serde_json::json!("/web/static/img/default_icon_app.png");
            }
        }
        
        web_menus.insert(menu_id.to_string(), serde_json::json!({
            "id": menu_id,
            "name": name_str,
            "children": children,
            "appID": app_id.map(|a| serde_json::json!(a)).unwrap_or(serde_json::Value::Null),
            "xmlid": row.xml_id.clone().unwrap_or_default(),
            "actionID": action_id,
            "actionModel": action_model,
            "actionPath": serde_json::Value::Null,
            "webIcon": web_icon,
            "webIconData": web_icon_data,
            "webIconDataMimetype": web_icon_data_mimetype,
        }));
    }
    
    Json(web_menus).into_response()
}

fn handle_mock_orm_fallback(_model: &str, method: &str, args: &serde_json::Value) -> Option<serde_json::Value> {
    match method {
        "web_save" => {
            let ids = args.get(0).and_then(|v| v.as_array());
            let values = args.get(1).unwrap_or(&serde_json::Value::Null);
            let is_create = ids.map(|a| a.is_empty()).unwrap_or(true);
            let mut record = serde_json::Map::new();
            if is_create {
                let random_id = 1000 + (chrono::Utc::now().timestamp_millis() % 10000);
                record.insert("id".to_string(), serde_json::json!(random_id));
            } else if let Some(first_id) = ids.and_then(|a| a.first()) {
                record.insert("id".to_string(), first_id.clone());
            } else {
                record.insert("id".to_string(), serde_json::json!(1001));
            }
            if let Some(val_obj) = values.as_object() {
                for (k, v) in val_obj {
                    record.insert(k.clone(), v.clone());
                }
            }
            if !record.contains_key("display_name") {
                let name = record.get("name").and_then(|n| n.as_str()).unwrap_or("Nuevo");
                record.insert("display_name".to_string(), serde_json::json!(name));
            }
            Some(serde_json::json!([record]))
        }
        "create" => {
            let random_id = 1000 + (chrono::Utc::now().timestamp_millis() % 10000);
            Some(serde_json::json!(random_id))
        }
        "write" => {
            Some(serde_json::json!(true))
        }
        "unlink" => {
            Some(serde_json::json!(true))
        }
        _ => None,
    }
}

/// POST /web/dataset/call_kw
/// POST /web/dataset/search_read
/// POST /web/dataset/call_kw/{model}/{method}  (Odoo 17 variant)
pub async fn dispatch_jsonrpc(
    State(state): State<AppState>,
    path_params: Option<axum::extract::Path<(String, String)>>,
    Json(payload): Json<JsonRpcRequest>,
) -> impl IntoResponse {
    let payload_id = payload.id.clone();
    let params = &payload.params;

    // Modelo y método pueden venir en el body (JSON-RPC clásico) o en la URL
    // (formato Odoo 17: /call_kw/{model}/{method})
    let (path_model, path_method) = path_params
        .map(|axum::extract::Path((m, mt))| (m, mt))
        .unwrap_or_default();

    let model = params.get("model").and_then(|m| m.as_str())
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or(path_model);
    let method = params.get("method").and_then(|m| m.as_str())
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or(path_method);

    let args = params.get("args").cloned().unwrap_or_else(|| serde_json::json!([]));
    let kwargs = params.get("kwargs").cloned().unwrap_or_else(|| serde_json::json!({}));

    tracing::info!("JSON-RPC Request: model={}, method={}, args={}", model, method, args);

    // Interceptor Rust: métodos ORM estándar que los modelos Python mini no tienen
    if let Some(rust_result) = dispatch_orm_rust(&state, &model, &method, &args, &kwargs).await {
        return match rust_result {
            Ok(val) => Json(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: val,
                id: payload_id,
            }).into_response(),
            Err(err_msg) => {
                tracing::warn!("orm_rust {}.{}: {}", model, method, err_msg);
                let error_val = serde_json::json!({
                    "code": 200, "message": "Odoo Server Error",
                    "data": { "name": "odoo.exceptions.UserError", "arguments": [&err_msg], "message": &err_msg }
                });
                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::from(serde_json::json!({
                        "jsonrpc": "2.0", "error": error_val, "id": payload_id,
                    }).to_string()))
                    .unwrap()
                    .into_response()
            }
        };
    }

    // ── Guard pre-Python: modelos del sistema no cargados en el registry ──────
    // Evita el KeyError de PyO3 para modelos que Python mini no tiene.
    // Retorna respuestas seguras en lugar de loguear ERROR.
    let safe_fallback = match (model.as_str(), method.as_str()) {
        // Seguridad / permisos — siempre permitir
        (_, "check") | (_, "check_access_rights") | (_, "check_access_rule") => Some(serde_json::json!(true)),
        // Contexto de usuario
        ("res.users", "context_get") | (_, "context_get") => Some(serde_json::json!({
            "lang": "es_MX", "tz": "America/Mexico_City", "uid": 2
        })),
        // ir.actions.client — stubs de escritura
        ("ir.actions.client", "create") | ("ir.act.client", "create") => Some(serde_json::json!(296)),
        ("ir.actions.client", "write") | ("ir.act.client", "write") => Some(serde_json::json!(true)),
        ("ir.actions.client", "unlink") | ("ir.act.client", "unlink") => Some(serde_json::json!(true)),
        ("ir.actions.client", "fields_get") | ("ir.act.client", "fields_get") => Some(serde_json::json!({
            "id": {"type": "integer", "string": "ID"},
            "name": {"type": "char", "string": "Name"},
            "tag": {"type": "char", "string": "Tag"},
            "path": {"type": "char", "string": "Path"},
            "type": {"type": "char", "string": "Type"}
        })),
        // Mail / Discuss
        ("mail.message", _) | ("mail.notification", _) | ("discuss.channel", _) |
        ("discuss.channel.member", _) | ("mail.followers", _) => {
            match method.as_str() {
                "create" | "write" | "unlink" => Some(serde_json::json!(true)),
                _ => Some(serde_json::json!([])),
            }
        }
        // ir.model.access — siempre permitir
        ("ir.model.access", _) => Some(serde_json::json!(true)),
        ("ir.model", "search_read") | ("ir.model", "search") => Some(serde_json::json!([])),
        ("ir.rule", "search_read") | ("ir.rule", "search") => Some(serde_json::json!([])),
        // ir.module.module — gestión de módulos del sistema
        // check_module_update, button_install, button_uninstall, etc.
        // no están en el registry Python mini, retornar stub seguro
        ("ir.module.module", "check_module_update") => Some(serde_json::json!(null)),
        ("ir.module.module", "button_install") |
        ("ir.module.module", "button_immediate_install") |
        ("ir.module.module", "button_uninstall") |
        ("ir.module.module", "button_immediate_uninstall") |
        ("ir.module.module", "button_upgrade") => Some(serde_json::json!({"type": "ir.actions.act_window_close"})),
        ("ir.module.module", "install_from_urls") => Some(serde_json::json!(null)),
        ("ir.module.module", "update_list") => Some(serde_json::json!(true)),
        // Cualquier otro método de ir.module.module que no sea lectura
        ("ir.module.module", m) if !["search_read","search","read","fields_get","get_views","onchange","name_search","web_read"].contains(&m) => {
            Some(serde_json::json!(null))
        },
        // Cualquier modelo que claramente es del sistema y no de negocio
        _ => None,
    };
    if let Some(safe_val) = safe_fallback {
        return Json(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: safe_val,
            id: payload_id,
        }).into_response();
    }

    // Fallback: llamar a Python via PyO3
    let model_log = model.clone();
    let method_log = method.clone();
    let args_py_move = args.clone();
    let result = tokio::task::spawn_blocking(move || {
        Python::with_gil(|py| -> Result<serde_json::Value, PyErr> {
            let context = kwargs.get("context").cloned().unwrap_or_else(|| serde_json::json!({}));
            let api_mod = py.import_bound("odoo.api")?;
            let json_mod = py.import_bound("json")?;
            let context_str = context.to_string();
            let context_py = json_mod.call_method1("loads", (context_str,))?;
            let env_obj = api_mod.getattr("Environment")?.call1((1u64, 2i32, context_py))?;
            let model_obj = env_obj.call_method1("__getitem__", (model.as_str(),))?;
            let ids: Vec<i64> = if let Some(arr) = args_py_move.as_array() {
                if !arr.is_empty() && arr[0].is_array() {
                    arr[0].as_array().unwrap().iter().filter_map(|i| i.as_i64()).collect()
                } else if !arr.is_empty() && arr[0].is_number() {
                    vec![arr[0].as_i64().unwrap_or(0)]
                } else { Vec::new() }
            } else { Vec::new() };
            let recordset = if !ids.is_empty() { model_obj.call_method1("browse", (ids,))? } else { model_obj };
            let args_str = args_py_move.to_string();
            let args_py: Bound<'_, PyTuple> = json_mod.call_method1("loads", (args_str,))?.downcast::<PyList>()?.to_tuple();
            let kwargs_str = kwargs.to_string();
            let kwargs_py: Bound<'_, PyDict> = json_mod.call_method1("loads", (kwargs_str,))?.downcast::<PyDict>()?.clone();
            let py_result = recordset.getattr(method.as_str())?.call(args_py, Some(&kwargs_py))?;
            let result_json_str: String = json_mod.call_method1("dumps", (py_result,))?.extract()?;
            let val: serde_json::Value = serde_json::from_str(&result_json_str)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("JSON: {:?}", e)))?;
            Ok(val)
        })
    }).await;

    match result {
        Ok(Ok(val)) => Json(JsonRpcResponse { jsonrpc: "2.0".to_string(), result: val, id: payload_id }).into_response(),
        Ok(Err(e)) => {
            let err_str = format!("{:?}", e);
            if err_str.contains("KeyError") && err_str.contains("no existe en el registry") {
                if let Some(mock_val) = handle_mock_orm_fallback(&model_log, &method_log, &args) {
                    return Json(JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        result: mock_val,
                        id: payload_id,
                    }).into_response();
                }
            }
            tracing::error!("call_kw {}.{}: {:?}", model_log, method_log, e);
            let error_val = serde_json::json!({
                "code": 200, "message": "Odoo Server Error",
                "data": { "name": "odoo.exceptions.UserError", "arguments": [format!("{:?}", e)], "message": format!("{:?}", e) }
            });
            Response::builder().status(StatusCode::OK).header(header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(serde_json::json!({"jsonrpc":"2.0","error":error_val,"id":payload_id}).to_string()))
                .unwrap().into_response()
        }
        Err(e) => Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::from(format!("Join error: {:?}", e))).unwrap().into_response(),
    }
}

/// Interceptor Rust-side para métodos ORM estándar de Odoo.
/// Retorna Some(Ok(val)) si maneja el método, None para pasar a Python.
async fn dispatch_orm_rust(
    state: &AppState,
    model: &str,
    method: &str,
    args: &serde_json::Value,
    kwargs: &serde_json::Value,
) -> Option<Result<serde_json::Value, String>> {
    use crate::handlers::orm_rpc::{get_table_columns, get_table_column_types, rows_to_json, build_where_from_domain, jsonb_aware_col, sanitize_order};
    use sqlx::Row;

    let table = model.replace('.', "_");

    match method {
        "check_field_access_rights" => Some(Ok(serde_json::json!(true))),

        "button_install" | "button_immediate_install" | "button_upgrade" | "button_immediate_upgrade" => {
            if model == "ir.module.module" {
                let ids: Vec<i64> = if let Some(arr) = args.as_array() {
                    if !arr.is_empty() && arr[0].is_array() {
                        arr[0].as_array().unwrap().iter().filter_map(|i| i.as_i64()).collect()
                    } else if !arr.is_empty() && arr[0].is_number() {
                        vec![arr[0].as_i64().unwrap_or(0)]
                    } else { Vec::new() }
                } else { Vec::new() };

                for id in ids {
                    let _ = sqlx::query("UPDATE ir_module_module SET state = 'installed' WHERE id = $1")
                        .bind(id)
                        .execute(&state.db)
                        .await;
                }
            }
            Some(Ok(serde_json::json!({
                "type": "ir.actions.client",
                "tag": "reload"
            })))
        }

        "button_uninstall" | "button_immediate_uninstall" => {
            if model == "ir.module.module" {
                let ids: Vec<i64> = if let Some(arr) = args.as_array() {
                    if !arr.is_empty() && arr[0].is_array() {
                        arr[0].as_array().unwrap().iter().filter_map(|i| i.as_i64()).collect()
                    } else if !arr.is_empty() && arr[0].is_number() {
                        vec![arr[0].as_i64().unwrap_or(0)]
                    } else { Vec::new() }
                } else { Vec::new() };

                for id in ids {
                    let _ = sqlx::query("UPDATE ir_module_module SET state = 'uninstalled' WHERE id = $1")
                        .bind(id)
                        .execute(&state.db)
                        .await;
                }
            }
            Some(Ok(serde_json::json!({
                "type": "ir.actions.client",
                "tag": "reload"
            })))
        }

        "onchange" => Some(Ok(serde_json::json!({ "value": {}, "warning": null, "domain": {} }))),

        "name_search" => {
            let name = args.get(0).and_then(|v| v.as_str())
                .or_else(|| kwargs.get("name").and_then(|v| v.as_str())).unwrap_or("");
            let limit = kwargs.get("limit").and_then(|v| v.as_u64()).unwrap_or(8) as i64;
            let valid = get_table_columns(&state.db, &table).await;
            if valid.is_empty() { return Some(Ok(serde_json::json!([]))); }
            let name_col = if valid.contains(&"name".to_string()) { "\"name\"::text" } else { "id::text" };
            let sql = format!("SELECT id, {} as name FROM {} WHERE {}::text ILIKE $1 LIMIT $2", name_col, table, name_col);
            let rows = sqlx::query(&sql).bind(format!("%{}%", name)).bind(limit).fetch_all(&state.db).await.unwrap_or_default();
            let result: Vec<serde_json::Value> = rows.iter().map(|r| {
                let id: i64 = r.try_get("id").unwrap_or(0);
                let nm: String = r.try_get("name").unwrap_or_default();
                serde_json::json!([id, nm])
            }).collect();
            Some(Ok(serde_json::json!(result)))
        }

        "name_get" => {
            let ids: Vec<i64> = args.get(0).and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|i| i.as_i64()).collect()).unwrap_or_default();
            if ids.is_empty() { return Some(Ok(serde_json::json!([]))); }
            let valid = get_table_columns(&state.db, &table).await;
            let col_types = get_table_column_types(&state.db, &table).await;
            let name_col = if valid.contains(&"name".to_string()) {
                jsonb_aware_col("name", &col_types)
            } else { "id::text AS name".to_string() };
            let ids_str = ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
            let sql = format!("SELECT id, {} FROM {} WHERE id IN ({}) ORDER BY id", name_col, table, ids_str);
            let rows = sqlx::query(&sql).fetch_all(&state.db).await.unwrap_or_default();
            let result: Vec<serde_json::Value> = rows.iter().map(|r| {
                let id: i64 = r.try_get("id").unwrap_or(0);
                let nm: String = r.try_get("name").unwrap_or_else(|_| id.to_string());
                serde_json::json!([id, nm])
            }).collect();
            Some(Ok(serde_json::json!(result)))
        }

        "web_search_read" => {
            let domain = kwargs.get("domain").cloned()
                .or_else(|| args.get(0).cloned()).unwrap_or_else(|| serde_json::json!([]));
            let fields_req: Vec<String> = kwargs.get("fields").and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|f| f.as_str().map(String::from)).collect()).unwrap_or_default();
            let limit = kwargs.get("limit").and_then(|v| v.as_u64()).unwrap_or(80) as u32;
            let offset = kwargs.get("offset").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
            let order = kwargs.get("order").and_then(|v| v.as_str()).unwrap_or("id desc").to_string();

            let valid = get_table_columns(&state.db, &table).await;
            if valid.is_empty() { return Some(Ok(serde_json::json!({ "records": [], "length": 0 }))); }
            let col_types = get_table_column_types(&state.db, &table).await;

            // Separar campos SQL reales de campos computados pedidos por el frontend.
            // Los computados no existen en la tabla pero el frontend los necesita en la respuesta.
            let (sql_fields, computed_fields): (Vec<String>, Vec<String>) = if fields_req.is_empty() {
                // Sin campos explícitos: seleccionar columnas útiles para la vista
                let defaults: Vec<String> = valid.iter()
                    .filter(|c| matches!(c.as_str(),
                        "id"|"name"|"active"|"state"|"shortdesc"|"summary"|"author"|
                        "website"|"icon"|"category_id"|"sequence"|"application"|"to_buy"|
                        "installed_version"|"latest_version"|"description"|"license"
                    )).cloned().collect();
                (defaults, vec![])
            } else {
                let sql_f: Vec<String> = fields_req.iter().filter(|f| valid.contains(f)).cloned().collect();
                let comp_f: Vec<String> = fields_req.iter().filter(|f| !valid.contains(f)).cloned().collect();
                (sql_f, comp_f)
            };

            // Garantizar que siempre se incluye 'id'
            let mut final_sql_fields = sql_fields;
            if !final_sql_fields.contains(&"id".to_string()) {
                final_sql_fields.insert(0, "id".to_string());
            }
            // Incluir 'icon' si hay campo icon_image computado (necesario para generar el valor)
            let needs_icon = computed_fields.contains(&"icon_image".to_string());
            if needs_icon && valid.contains(&"icon".to_string())
                && !final_sql_fields.contains(&"icon".to_string()) {
                final_sql_fields.push("icon".to_string());
            }

            let select_expr = final_sql_fields.iter()
                .map(|f| jsonb_aware_col(f, &col_types)).collect::<Vec<_>>().join(", ");
            let domain_opt = Some(domain);
            let where_clause = build_where_from_domain(&domain_opt, &valid);
            let safe_order = sanitize_order(&order, &valid);

            let count_sql = format!("SELECT COUNT(*) FROM {} {}", table, where_clause);
            let total: i64 = sqlx::query_scalar(&count_sql).fetch_one(&state.db).await.unwrap_or(0);
            let sql = format!("SELECT {} FROM {} {} ORDER BY {} LIMIT {} OFFSET {}",
                select_expr, table, where_clause, safe_order, limit, offset);
            tracing::debug!("web_search_read SQL: {}", &sql[..sql.len().min(500)]);
            let rows = match sqlx::query(&sql).fetch_all(&state.db).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!("web_search_read SQL error: {:?}", e);
                    return Some(Ok(serde_json::json!({ "records": [], "length": total })));
                }
            };
            let mut records = rows_to_json(rows, &final_sql_fields);

            // Añadir campos computados a cada record.
            // Odoo espera que estén presentes (aunque sea como false) o el RelationalModel crashea.
            for record in &mut records {
                for cf in &computed_fields {
                    if !record.as_object().map(|o| o.contains_key(cf.as_str())).unwrap_or(false) {
                        let stub_val = match cf.as_str() {
                            // icon_image: intentar leer del campo 'icon' (path al PNG del módulo)
                            // Si no se puede, devolver false (Odoo muestra icono genérico)
                            "icon_image" => {
                                let icon_path = record.get("icon")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();
                                if icon_path.is_empty() {
                                    serde_json::Value::Bool(false)
                                } else {
                                    // Buscar en addons_path relativo
                                    let clean = icon_path.trim_start_matches('/');
                                    let full_path = format!("/home/ealvarez/workspace/NexustechERPv2/{}", clean);
                                    if let Ok(bytes) = std::fs::read(&full_path) {
                                        use base64::Engine;
                                        serde_json::Value::String(base64::prelude::BASE64_STANDARD.encode(&bytes))
                                    } else {
                                        serde_json::Value::Bool(false)
                                    }
                                }
                            },
                            // display_name = name en Odoo básico
                            "display_name" => record.get("name")
                                .cloned().unwrap_or(serde_json::Value::Bool(false)),
                            // installed_version / latest_version — alias
                            "installed_version" => record.get("latest_version")
                                .cloned().unwrap_or(serde_json::Value::Bool(false)),
                            // Si el nombre sugiere una relación X2many, retornar []
                            name if name.ends_with("_ids") || name.ends_with("_lines") || name == "dependencies_id" || name.contains("_line") => {
                                serde_json::Value::Array(vec![])
                            },
                            // Cualquier otro campo computado no soportado → false
                            _ => serde_json::Value::Bool(false),
                        };
                        if let Some(obj) = record.as_object_mut() {
                            obj.insert(cf.clone(), stub_val);
                        }
                    }
                }
            }

            // ── Convertir campos many2one: integer → [id, "display_name"] ──────────
            convert_many2one_fields(&state.db, &table, &mut records, &final_sql_fields, &col_types).await;

            tracing::debug!("web_search_read: {} records returned (total={})", records.len(), total);
            Some(Ok(serde_json::json!({ "records": records, "length": total })))

        }

        "read_group" => {
            let domain = kwargs.get("domain").cloned()
                .or_else(|| args.get(0).cloned()).unwrap_or_else(|| serde_json::json!([]));
            let fields_req: Vec<String> = kwargs.get("fields").and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|f| f.as_str().map(String::from)).collect()).unwrap_or_default();
            let groupby: Vec<String> = kwargs.get("groupby").and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|f| f.as_str().map(String::from)).collect()).unwrap_or_default();

            let valid = get_table_columns(&state.db, &table).await;
            if valid.is_empty() { return Some(Ok(serde_json::json!([]))); }

            let groupby_cols: Vec<String> = groupby.iter()
                .map(|g| g.split(':').next().unwrap_or(g).to_string()).collect();
            let valid_groupby: Vec<String> = groupby_cols.iter()
                .filter(|c| valid.contains(c)).cloned().collect();

            let domain_opt = Some(domain);
            let where_clause = build_where_from_domain(&domain_opt, &valid);

            if valid_groupby.is_empty() {
                let count_sql = format!("SELECT COUNT(*) FROM {} {}", table, where_clause);
                let cnt: i64 = sqlx::query_scalar(&count_sql).fetch_one(&state.db).await.unwrap_or(0);
                return Some(Ok(serde_json::json!([{ "__count": cnt }])));
            }

            let group_expr = valid_groupby.iter().map(|c| format!("\"{}\"", c)).collect::<Vec<_>>().join(", ");
            let mut select_parts = valid_groupby.iter().map(|c| format!("\"{}\"", c)).collect::<Vec<_>>();
            select_parts.push("COUNT(*) AS __count".to_string());
            for f in &fields_req {
                let base = f.split(':').next().unwrap_or(f);
                if valid.contains(&base.to_string()) && !valid_groupby.contains(&base.to_string()) {
                    select_parts.push(format!("SUM(\"{}\") AS \"{}\"", base, base));
                }
            }

            let sql = format!("SELECT {} FROM {} {} GROUP BY {} ORDER BY __count DESC LIMIT 200",
                select_parts.join(", "), table, where_clause, group_expr);
            let rows = sqlx::query(&sql).fetch_all(&state.db).await.unwrap_or_default();

            let mut result = Vec::new();
            for row in &rows {
                let mut map = serde_json::Map::new();
                let cnt: i64 = row.try_get("__count").unwrap_or(0);
                map.insert("__count".to_string(), serde_json::json!(cnt));
                for col in &valid_groupby {
                    let val = if let Ok(v) = row.try_get::<Option<String>, _>(col.as_str()) {
                        v.map(|s| serde_json::json!(s)).unwrap_or(serde_json::Value::Null)
                    } else if let Ok(v) = row.try_get::<Option<i64>, _>(col.as_str()) {
                        v.map(|n| serde_json::json!(n)).unwrap_or(serde_json::Value::Null)
                    } else if let Ok(v) = row.try_get::<Option<bool>, _>(col.as_str()) {
                        v.map(|b| serde_json::json!(b)).unwrap_or(serde_json::Value::Null)
                    } else { serde_json::Value::Null };
                    map.insert(col.clone(), val.clone());
                    let domain_filter = if val.is_null() { serde_json::json!([[col,"=",false]]) }
                        else { serde_json::json!([[col,"=",val]]) };
                    map.insert(format!("{}_count", col), serde_json::json!(cnt));
                    map.insert("__domain".to_string(), domain_filter);
                }
                for f in &fields_req {
                    let base = f.split(':').next().unwrap_or(f);
                    if valid.contains(&base.to_string()) && !valid_groupby.contains(&base.to_string()) {
                        if let Ok(v) = row.try_get::<Option<f64>, _>(base) {
                            map.insert(base.to_string(), v.map(|n| serde_json::json!(n)).unwrap_or(serde_json::json!(0)));
                        }
                    }
                }
                result.push(serde_json::Value::Object(map));
            }
            Some(Ok(serde_json::json!(result)))
        }

        "get_views" | "load_views" => {
            let views_req: Vec<(Option<i64>, String)> = kwargs.get("views").and_then(|v| v.as_array())
                .map(|arr| arr.iter().map(|item| {
                    let id = item.get(0).and_then(|i| i.as_i64());
                    let vt = item.get(1).and_then(|t| t.as_str()).unwrap_or("list").to_string();
                    (id, vt)
                }).collect())
                .unwrap_or_else(|| vec![(None, "list".to_string()), (None, "form".to_string())]);

            let py_res = tokio::task::spawn_blocking({
                let model = model.to_string();
                let kwargs = kwargs.clone();
                move || {
                    Python::with_gil(|py| -> Result<serde_json::Value, PyErr> {
                        let context = kwargs.get("context").cloned().unwrap_or_else(|| serde_json::json!({}));
                        let api_mod = py.import_bound("odoo.api")?;
                        let json_mod = py.import_bound("json")?;
                        let context_str = context.to_string();
                        let context_py = json_mod.call_method1("loads", (context_str,))?;
                        let env_obj = api_mod.getattr("Environment")?.call1((1u64, 2i32, context_py))?;
                        let model_obj = env_obj.call_method1("__getitem__", (model.as_str(),))?;
                        let py_fields = model_obj.call_method0("fields_get")?;
                        let result_json_str: String = json_mod.call_method1("dumps", (py_fields,))?.extract()?;
                        let val: serde_json::Value = serde_json::from_str(&result_json_str)
                            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("JSON: {:?}", e)))?;
                        Ok(val)
                    })
                }
            }).await;

            let mut fields_map = serde_json::Map::new();
            let mut got_py_fields = false;
            let mut cache_views = None;

            if let Ok(Ok(serde_json::Value::Object(map))) = py_res {
                fields_map = map;
                got_py_fields = true;
            }

            if !got_py_fields {
                let fields_sql = r#"SELECT column_name, data_type, is_nullable
                    FROM information_schema.columns
                    WHERE table_name = $1 AND table_schema = 'public'
                    ORDER BY ordinal_position"#;
                let field_rows = sqlx::query(fields_sql).bind(&table).fetch_all(&state.db).await.unwrap_or_default();

                for row in &field_rows {
                    use sqlx::Row;
                    let col: String = row.try_get("column_name").unwrap_or_default();
                    let dtype: String = row.try_get("data_type").unwrap_or_default();
                    let nullable: String = row.try_get("is_nullable").unwrap_or_else(|_| "YES".to_string());
                    let odoo_type = if col.ends_with("_id") && dtype.contains("int") { "many2one" }
                        else { match dtype.as_str() {
                            "integer"|"bigint"|"smallint" => "integer",
                            "numeric"|"real"|"double precision" => "float",
                            "boolean" => "boolean",
                            "timestamp without time zone"|"timestamp with time zone" => "datetime",
                            "date" => "date",
                            _ => "char",
                        }};
                    let label = col.replace('_', " ").split_whitespace()
                        .map(|w| { let mut c = w.chars(); c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default() })
                        .collect::<Vec<_>>().join(" ");
                    fields_map.insert(col, serde_json::json!({
                        "type": odoo_type, "string": label, "required": nullable == "NO", "readonly": false, "store": true
                    }));
                }
            }

            tracing::info!("get_views: Checking cache for model '{}'", model);
            match std::fs::read_to_string("model_views_cache.json") {
                Ok(cache_content) => {
                    tracing::info!("get_views: Read cache file ({} bytes)", cache_content.len());
                    match serde_json::from_str::<serde_json::Value>(&cache_content) {
                        Ok(serde_json::Value::Object(cache_map)) => {
                            if let Some(serde_json::Value::Object(model_data)) = cache_map.get(&model[..]) {
                                tracing::info!("get_views: Found model '{}' in cache", model);
                                if let Some(serde_json::Value::Object(fields_obj)) = model_data.get("fields") {
                                    tracing::info!("get_views: Found {} fields in cache", fields_obj.len());
                                    for (field_name, field_def) in fields_obj {
                                        if !fields_map.contains_key(field_name) {
                                            fields_map.insert(field_name.clone(), field_def.clone());
                                        }
                                    }
                                } else {
                                    tracing::warn!("get_views: 'fields' key not found for model '{}' in cache", model);
                                }
                                if cache_views.is_none() {
                                    if let Some(serde_json::Value::Object(views_obj)) = model_data.get("views") {
                                        cache_views = Some(views_obj.clone());
                                    }
                                }
                            } else {
                                tracing::warn!("get_views: Model '{}' not found in cache", model);
                            }
                        }
                        Ok(_) => tracing::error!("get_views: Cache content is not a JSON object"),
                        Err(e) => tracing::error!("get_views: Failed to parse cache JSON: {:?}", e),
                    }
                }
                Err(e) => tracing::error!("get_views: Failed to read model_views_cache.json: {:?}", e),
            }


            if !fields_map.contains_key("id") {
                fields_map.insert("id".to_string(), serde_json::json!({
                    "type": "integer", "string": "ID", "required": false, "readonly": true, "store": true
                }));
            }
            if !fields_map.contains_key("display_name") {
                fields_map.insert("display_name".to_string(), serde_json::json!({
                    "type": "char", "string": "Display Name", "required": false, "readonly": true, "store": false
                }));
            }

            if model == "ir.module.module" {
                fields_map.insert("icon_image".to_string(), serde_json::json!({
                    "type": "binary", "string": "Icon", "required": false, "readonly": true, "store": false
                }));
                fields_map.insert("icon".to_string(), serde_json::json!({
                    "type": "char", "string": "Icon Path", "required": false, "readonly": true, "store": true
                }));
                fields_map.insert("installed_version".to_string(), serde_json::json!({
                    "type": "char", "string": "Installed Version", "required": false, "readonly": true, "store": false
                }));
                fields_map.insert("state".to_string(), serde_json::json!({
                    "type": "selection",
                    "string": "Status",
                    "required": false,
                    "readonly": true,
                    "store": true,
                    "selection": [
                        ["uninstallable", "Uninstallable"],
                        ["uninstalled", "Not Installed"],
                        ["installed", "Installed"],
                        ["to upgrade", "To be upgraded"],
                        ["to remove", "To be removed"],
                        ["to install", "To be installed"]
                    ]
                }));
            }

            // Post-process fields_map to ensure all fields of type "selection" have a non-empty "selection" option list
            for (_, fdef) in fields_map.iter_mut() {
                if let Some(ftype) = fdef.get("type").and_then(|t| t.as_str()) {
                    if ftype == "selection" && fdef.get("selection").is_none() {
                        if let Some(fdef_obj) = fdef.as_object_mut() {
                            fdef_obj.insert("selection".to_string(), serde_json::json!([["normal", "Normal"]]));
                        }
                    }
                }
            }

            let mut views_map = serde_json::Map::new();
            let mut models_map = serde_json::json!({ model: { "fields": fields_map.clone() } });

            // Auto-inject co-models (relational models) into models_map recursively to prevent client crash
            if let Some(models_obj) = models_map.as_object_mut() {
                if let Ok(cache_content) = std::fs::read_to_string("model_views_cache.json") {
                    if let Ok(serde_json::Value::Object(cache_map)) = serde_json::from_str::<serde_json::Value>(&cache_content) {
                        let mut resolved_models = std::collections::HashSet::new();
                        resolved_models.insert(model.to_string());

                        let mut queue = Vec::new();
                        for (_, fval) in &fields_map {
                            if let Some(rel) = fval.get("relation").and_then(|r| r.as_str()) {
                                queue.push(rel.to_string());
                            }
                        }

                        // Process queue recursively up to a depth limit or until empty
                        while let Some(rel) = queue.pop() {
                            if resolved_models.contains(&rel) {
                                continue;
                            }
                            if let Some(serde_json::Value::Object(rel_model_data)) = cache_map.get(&rel) {
                                if let Some(rel_fields) = rel_model_data.get("fields") {
                                    models_obj.insert(rel.clone(), serde_json::json!({ "fields": rel_fields }));
                                    resolved_models.insert(rel.clone());
                                    if let Some(fields_obj) = rel_fields.as_object() {
                                        for (_, fval) in fields_obj {
                                            if let Some(sub_rel) = fval.get("relation").and_then(|r| r.as_str()) {
                                                if !resolved_models.contains(sub_rel) {
                                                    queue.push(sub_rel.to_string());
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                // Graceful fallback: insert basic/empty fields schema so it does not crash on undefined model.fields
                                models_obj.insert(rel.clone(), serde_json::json!({
                                    "fields": {
                                        "id": { "type": "integer", "string": "ID", "required": false, "readonly": true, "store": true },
                                        "display_name": { "type": "char", "string": "Display Name", "required": false, "readonly": true, "store": false }
                                    }
                                }));
                                resolved_models.insert(rel.clone());
                            }
                        }
                    }
                }
            }

            // Ensure all fields of type "selection" in all models inside models_map have a non-empty "selection" option list
            if let Some(models_obj) = models_map.as_object_mut() {
                for (_, mdef) in models_obj.iter_mut() {
                    if let Some(fields_obj) = mdef.get_mut("fields").and_then(|f| f.as_object_mut()) {
                        for (_, fdef) in fields_obj.iter_mut() {
                            if let Some(ftype) = fdef.get("type").and_then(|t| t.as_str()) {
                                if ftype == "selection" && fdef.get("selection").is_none() {
                                    if let Some(fdef_obj) = fdef.as_object_mut() {
                                        fdef_obj.insert("selection".to_string(), serde_json::json!([["normal", "Normal"]]));
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let module_arches: std::collections::HashMap<&str, &str> = if model == "ir.module.module" {
                let mut m = std::collections::HashMap::new();
                m.insert("list",
                    r#"<list string="Apps" create="false">
  <field name="icon_image" widget="image" invisible="1"/>
  <field name="shortdesc" string="Name"/>
  <field name="name" string="Technical Name"/>
  <field name="state" string="Status" widget="badge"/>
  <field name="author"/>
  <field name="website" widget="url"/>
  <field name="installed_version" string="Version"/>
</list>"#);
                m.insert("kanban",
                    r#"<kanban class="o_modules_kanban" create="false">
  <field name="icon_image"/>
  <field name="icon"/>
  <field name="name"/>
  <field name="shortdesc"/>
  <field name="summary"/>
  <field name="state"/>
  <field name="to_buy"/>
  <field name="sequence"/>
  <field name="website"/>
  <field name="author"/>
  <templates>
    <t t-name="card">
      <div class="oe_module_vignette">
        <img t-att-src="record.icon.value or '/nexustech/base/static/description/icon.png'" class="o_module_icon" />
        <div class="oe_module_desc">
          <div class="o_kanban_record_title"><field name="shortdesc"/></div>
          <p class="o_module_desc"><field name="summary"/></p>
          <div class="o_kanban_record_bottom">
            <div class="oe_kanban_bottom_left"><field name="state"/></div>
          </div>
        </div>
      </div>
    </t>
  </templates>
</kanban>"#);
                m.insert("search",
                    r#"<search string="Apps">
  <field name="name" string="Technical Name"/>
  <field name="shortdesc" string="Name"/>
  <field name="description" string="Description"/>
  <field name="author"/>
  <field name="category_id" string="Category"/>
  <separator/>
  <filter string="Installed" name="installed" domain="[[&apos;state&apos;,&apos;=&apos;,&apos;installed&apos;]]"/>
  <filter string="Not Installed" name="uninstalled" domain="[[&apos;state&apos;,&apos;=&apos;,&apos;uninstalled&apos;]]"/>
  <filter string="Applications" name="apps" domain="[[&apos;application&apos;,&apos;=&apos;,True]]"/>
  <group string="Group By">
    <filter string="Category" name="category" context="{&apos;group_by&apos;:&apos;category_id&apos;}"/>
    <filter string="Author" name="author" context="{&apos;group_by&apos;:&apos;author&apos;}"/>
  </group>
</search>"#);
                m.insert("form",
                    r#"<form string="Module" create="false" edit="false" delete="false">
  <header>
    <button name="button_immediate_install" string="Install" type="object" class="oe_highlight" invisible="state != 'uninstalled'"/>
    <button name="button_immediate_upgrade" string="Upgrade" type="object" class="oe_highlight" invisible="state != 'installed'"/>
    <button name="button_immediate_uninstall" string="Uninstall" type="object" invisible="state != 'installed'"/>
  </header>
  <sheet>
    <div class="oe_title">
      <h1><field name="shortdesc"/></h1>
      <h2><field name="name"/></h2>
    </div>
    <group>
      <field name="state"/>
      <field name="author"/>
      <field name="website"/>
      <field name="installed_version"/>
      <field name="category_id"/>
      <field name="application"/>
      <field name="to_buy"/>
    </group>
    <field name="description"/>
  </sheet>
</form>"#);
                m
            } else {
                std::collections::HashMap::new()
            };

            let generate_dynamic_arch = |vtype: &str, fmap: &serde_json::Map<String, serde_json::Value>| -> String {
                let mut sorted_fields: Vec<(String, String)> = Vec::new();
                for (k, v) in fmap {
                    if let Some(t) = v.get("type").and_then(|t| t.as_str()) {
                        sorted_fields.push((k.clone(), t.to_string()));
                    }
                }
                sorted_fields.sort_by(|a, b| a.0.cmp(&b.0));

                match vtype {
                    "list" => {
                        let list_field_types = ["char", "integer", "float", "monetary", "selection", "many2one", "boolean", "date", "datetime"];
                        let mut list_fields = Vec::new();
                        if sorted_fields.iter().any(|(k, _)| k == "name") {
                            list_fields.push("name".to_string());
                        } else if sorted_fields.iter().any(|(k, _)| k == "display_name") {
                            list_fields.push("display_name".to_string());
                        }
                        for (k, t) in &sorted_fields {
                            if k == "name" || k == "display_name" || k == "id" {
                                continue;
                            }
                            if list_field_types.contains(&t.as_str()) {
                                list_fields.push(k.clone());
                            }
                            if list_fields.len() >= 8 {
                                break;
                            }
                        }
                        let list_cols = list_fields.iter()
                            .map(|f| format!("  <field name=\"{}\"/>", f))
                            .collect::<Vec<_>>().join("\n");
                        format!("<list string=\"{}\">\n{}\n</list>", model, list_cols)
                    }
                    "form" => {
                        let mut title_field = None;
                        if sorted_fields.iter().any(|(k, _)| k == "name") {
                            title_field = Some("name");
                        } else if sorted_fields.iter().any(|(k, _)| k == "title") {
                            title_field = Some("title");
                        }

                        let mut group1_fields = Vec::new();
                        let mut group2_fields = Vec::new();
                        let mut notebook_pages = Vec::new();

                        let standard_types = ["char", "integer", "float", "monetary", "selection", "many2one", "boolean", "date", "datetime"];
                        let relational_types = ["one2many", "many2many"];
                        let text_types = ["text", "html"];

                        let mut standard_count = 0;
                        for (k, t) in &sorted_fields {
                            if let Some(tf) = title_field {
                                if k == tf {
                                    continue;
                                }
                            }
                            if k == "id" || k == "display_name" {
                                continue;
                            }

                            if standard_types.contains(&t.as_str()) {
                                if standard_count % 2 == 0 {
                                    group1_fields.push(k.clone());
                                } else {
                                    group2_fields.push(k.clone());
                                }
                                standard_count += 1;
                            } else if relational_types.contains(&t.as_str()) {
                                let label = k.replace('_', " ").split_whitespace()
                                    .map(|w| { let mut c = w.chars(); c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default() })
                                    .collect::<Vec<_>>().join(" ");
                                notebook_pages.push((label, k.clone()));
                            } else if text_types.contains(&t.as_str()) {
                                let label = k.replace('_', " ").split_whitespace()
                                    .map(|w| { let mut c = w.chars(); c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default() })
                                    .collect::<Vec<_>>().join(" ");
                                notebook_pages.push((label, k.clone()));
                            }
                        }

                        let mut form_arch = String::new();
                        form_arch.push_str(&format!("<form string=\"{}\">\n", model));
                        form_arch.push_str("  <sheet>\n");
                        if let Some(tf) = title_field {
                            form_arch.push_str("    <div class=\"oe_title\">\n");
                            form_arch.push_str(&format!("      <h1><field name=\"{}\" placeholder=\"Name...\"/></h1>\n", tf));
                            form_arch.push_str("    </div>\n");
                        }
                        form_arch.push_str("    <group>\n");
                        form_arch.push_str("      <group>\n");
                        for f in group1_fields {
                            form_arch.push_str(&format!("        <field name=\"{}\"/>\n", f));
                        }
                        form_arch.push_str("      </group>\n");
                        form_arch.push_str("      <group>\n");
                        for f in group2_fields {
                            form_arch.push_str(&format!("        <field name=\"{}\"/>\n", f));
                        }
                        form_arch.push_str("      </group>\n");
                        form_arch.push_str("    </group>\n");

                        if !notebook_pages.is_empty() {
                            form_arch.push_str("    <notebook>\n");
                            for (label, f) in notebook_pages {
                                form_arch.push_str(&format!("      <page string=\"{}\" name=\"{}\">\n", label, f));
                                form_arch.push_str(&format!("        <field name=\"{}\"/>\n", f));
                                form_arch.push_str("      </page>\n");
                            }
                            form_arch.push_str("    </notebook>\n");
                        }
                        form_arch.push_str("  </sheet>\n");
                        form_arch.push_str("</form>");
                        form_arch
                    }
                    "search" => {
                        let mut search_fields = Vec::new();
                        if sorted_fields.iter().any(|(k, _)| k == "name") {
                            search_fields.push("name".to_string());
                        }
                        for (k, t) in &sorted_fields {
                            if k == "name" || k == "id" {
                                continue;
                            }
                            if t == "char" || t == "many2one" {
                                search_fields.push(k.clone());
                            }
                            if search_fields.len() >= 5 {
                                break;
                            }
                        }
                        let search_cols = search_fields.iter()
                            .map(|f| format!("  <field name=\"{}\"/>", f))
                            .collect::<Vec<_>>().join("\n");
                        format!("<search string=\"{}\">\n  <field name=\"id\"/>\n{}\n</search>", model, search_cols)
                    }
                    "kanban" => {
                        let kfields = sorted_fields.iter()
                            .take(5)
                            .map(|(k, _)| format!("  <field name=\"{}\"/>", k))
                            .collect::<Vec<_>>().join("\n");
                        format!("<kanban string=\"{}\">\n{}\n  <templates>\n    <t t-name=\"card\">\n      <div class=\"oe_kanban_global_click\">\n        <field name=\"display_name\"/>\n      </div>\n    </t>\n  </templates>\n</kanban>", model, kfields)
                    }
                    _ => format!("<{} string=\"{}\"></{}>" , vtype, model, vtype),
                }
            };

            for (view_id, view_type) in &views_req {
                let view_row = if let Some(id) = view_id {
                    sqlx::query("SELECT id, arch, name FROM ir_ui_view WHERE id = $1 LIMIT 1")
                        .bind(id).fetch_optional(&state.db).await.unwrap_or(None)
                } else {
                    sqlx::query("SELECT id, arch, name FROM ir_ui_view WHERE model = $1 AND type = $2 AND active = true ORDER BY priority ASC, id ASC LIMIT 1")
                        .bind(model).bind(view_type).fetch_optional(&state.db).await.unwrap_or(None)
                };

                let (vid, arch, vname) = if let Some(r) = view_row {
                    let vid: i64 = r.try_get("id").unwrap_or(0);
                    let db_arch: String = r.try_get::<Option<serde_json::Value>, _>("arch")
                        .ok().flatten()
                        .and_then(|v| if v.is_string() { v.as_str().map(String::from) } else { Some(v.to_string()) })
                        .unwrap_or_default();
                    let vname: String = r.try_get("name").unwrap_or_else(|_| model.to_string());
                    let arch = if db_arch.len() > 20 {
                        db_arch
                    } else if let Some(hardcoded) = module_arches.get(view_type.as_str()) {
                        hardcoded.to_string()
                    } else if let Some(cached_arch) = cache_views.as_ref().and_then(|m| m.get(view_type.as_str()).and_then(|v| v.as_str())) {
                        cached_arch.to_string()
                    } else {
                        generate_dynamic_arch(view_type.as_str(), &fields_map)
                    };
                    (vid, arch, vname)
                } else {
                    let arch = if let Some(hardcoded) = module_arches.get(view_type.as_str()) {
                        hardcoded.to_string()
                    } else if let Some(cached_arch) = cache_views.as_ref().and_then(|m| m.get(view_type.as_str()).and_then(|v| v.as_str())) {
                        cached_arch.to_string()
                    } else {
                        generate_dynamic_arch(view_type.as_str(), &fields_map)
                    };
                    (0i64, arch, model.to_string())
                };

                let arch = arch
                    .replace("widget=\"slide_category_one2many\"", "")
                    .replace("widget=\"website_redirect_button\"", "");

                views_map.insert(view_type.clone(), serde_json::json!({
                    "id": vid, "type": view_type, "name": vname, "arch": arch,
                    "model": model, "fields": fields_map,
                    "toolbar": { "action": [], "print": [], "relate": [] }, "view_ref": false,
                }));
            }

            Some(Ok(serde_json::json!({ "views": views_map, "models": models_map })))
        }

        "load" => Some(Ok(serde_json::json!({ "fields": [], "records": [] }))),
        "copy_data" | "copy" => Some(Ok(serde_json::json!({}))),
        "get_formview_id" | "get_formview_action" => Some(Ok(serde_json::json!(false))),
        "default_get" => Some(Ok(serde_json::json!({}))),
        "read_progress_bar" => Some(Ok(serde_json::json!({}))),
        "activity_format" | "activity_schedule" => Some(Ok(serde_json::json!([]))),
        "get_mention_suggestions" => Some(Ok(serde_json::json!([]))),
        "has_group" => Some(Ok(serde_json::json!(true))),

        // ── Seguridad / permisos ─────────────────────────────────────────────
        // ir.model.access.check — siempre retorna true (admin mode)
        "check" => Some(Ok(serde_json::json!(true))),
        // check_access_rights, check_access_rule — siempre permitir
        "check_access_rights" | "check_access_rule" => Some(Ok(serde_json::json!(true))),

        // ── Contexto de usuario ──────────────────────────────────────────────
        "context_get" => Some(Ok(serde_json::json!({
            "lang": "es_MX",
            "tz": "America/Mexico_City",
            "uid": 2
        }))),

        // ── Filtros / favoritos ──────────────────────────────────────────────
        "get_filters" => Some(Ok(serde_json::json!([]))),
        "create_or_replace" => Some(Ok(serde_json::json!(1))),
        "unlink_action" => Some(Ok(serde_json::json!(true))),

        // ── Actividades / chatter ────────────────────────────────────────────
        "mail_activity_count" | "get_activity_data" => Some(Ok(serde_json::json!({}))),
        "message_fetch" | "message_post" | "message_subscribe" => Some(Ok(serde_json::json!({}))),
        "message_format" | "message_unread_counter" => Some(Ok(serde_json::json!({}))),
        "get_discuss_sidebar_initvalues" => Some(Ok(serde_json::json!({ "channels": [], "needaction_inbox_counter": 0 }))),

        // ── Misceláneos ──────────────────────────────────────────────────────
        "action_open_record" | "action_open_website" => Some(Ok(serde_json::json!(false))),
        "get_base_url" => Some(Ok(serde_json::json!("http://localhost:8090"))),
        "get_url" => Some(Ok(serde_json::json!("http://localhost:8090"))),
        "search_panel_select_range" | "search_panel_select_multi_range" => Some(Ok(serde_json::json!({ "values": [] }))),
        "get_available_actions" => Some(Ok(serde_json::json!([]))),
        "systray_get_activities" => Some(Ok(serde_json::json!([]))),

        // ── search_read / search / read / web_read ───────────────────────────
        "search_read" | "search" => {
            let domain = args.get(0).cloned()
                .or_else(|| kwargs.get("domain").cloned())
                .unwrap_or_else(|| serde_json::json!([]));
            let fields_req: Vec<String> = kwargs.get("fields").and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|f| f.as_str().map(String::from)).collect())
                .unwrap_or_default();
            let limit = kwargs.get("limit").and_then(|v| v.as_u64()).unwrap_or(80) as u32;
            let offset = kwargs.get("offset").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
            let order = kwargs.get("order").and_then(|v| v.as_str()).unwrap_or("id desc").to_string();

            let valid = get_table_columns(&state.db, &table).await;
            if valid.is_empty() { return Some(Ok(serde_json::json!([]))); }
            let col_types = get_table_column_types(&state.db, &table).await;

            let (sql_fields, computed_fields): (Vec<String>, Vec<String>) = if fields_req.is_empty() {
                (valid.iter().filter(|c| matches!(c.as_str(), "id"|"name"|"active")).cloned().collect(), vec![])
            } else {
                let sql_f: Vec<String> = fields_req.iter().filter(|f| valid.contains(f)).cloned().collect();
                let comp_f: Vec<String> = fields_req.iter().filter(|f| !valid.contains(f)).cloned().collect();
                (sql_f, comp_f)
            };
            let mut final_sql_fields = sql_fields;
            if final_sql_fields.is_empty() {
                final_sql_fields.push("id".to_string());
            }

            let select_expr = final_sql_fields.iter().map(|f| jsonb_aware_col(f, &col_types)).collect::<Vec<_>>().join(", ");
            let domain_opt = Some(domain);
            let where_clause = build_where_from_domain(&domain_opt, &valid);
            let safe_order = sanitize_order(&order, &valid);

            let sql = format!("SELECT {} FROM {} {} ORDER BY {} LIMIT {} OFFSET {}",
                select_expr, table, where_clause, safe_order, limit, offset);
            tracing::debug!("search_read interceptor SQL: {}", &sql[..sql.len().min(400)]);
            let rows = match sqlx::query(&sql).fetch_all(&state.db).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!("search_read SQL error for {}: {:?}", model, e);
                    return Some(Ok(serde_json::json!([])));
                }
            };
            let mut records = rows_to_json(rows, &final_sql_fields);

            // Stub computed/missing fields to prevent Odoo crashes
            for record in &mut records {
                for cf in &computed_fields {
                    if !record.as_object().map(|o| o.contains_key(cf.as_str())).unwrap_or(false) {
                        let stub_val = match cf.as_str() {
                            "display_name" => record.get("name").cloned().unwrap_or(serde_json::Value::Bool(false)),
                            name if name.ends_with("_ids") || name.ends_with("_lines") || name == "dependencies_id" || name.contains("_line") => {
                                serde_json::Value::Array(vec![])
                            },
                            _ => serde_json::Value::Bool(false),
                        };
                        if let Some(obj) = record.as_object_mut() {
                            obj.insert(cf.clone(), stub_val);
                        }
                    }
                }
            }

            convert_many2one_fields(&state.db, &table, &mut records, &final_sql_fields, &col_types).await;
            Some(Ok(serde_json::json!(records)))
        }

        "read" => {
            let ids: Vec<i64> = args.get(0).and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|i| i.as_i64()).collect()).unwrap_or_default();
            if ids.is_empty() { return Some(Ok(serde_json::json!([]))); }
            let fields_req: Vec<String> = args.get(1).and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|f| f.as_str().map(String::from)).collect())
                .or_else(|| kwargs.get("fields").and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|f| f.as_str().map(String::from)).collect()))
                .unwrap_or_default();

            let valid = get_table_columns(&state.db, &table).await;
            if valid.is_empty() { return Some(Ok(serde_json::json!([]))); }
            let col_types = get_table_column_types(&state.db, &table).await;

            let (sql_fields, computed_fields): (Vec<String>, Vec<String>) = if fields_req.is_empty() {
                (vec!["id".to_string()], vec![])
            } else {
                let sql_f: Vec<String> = fields_req.iter().filter(|f| valid.contains(f)).cloned().collect();
                let comp_f: Vec<String> = fields_req.iter().filter(|f| !valid.contains(f)).cloned().collect();
                (sql_f, comp_f)
            };
            let mut final_sql_fields = sql_fields;
            if !final_sql_fields.contains(&"id".to_string()) {
                final_sql_fields.insert(0, "id".to_string());
            }

            let select_expr = final_sql_fields.iter().map(|f| jsonb_aware_col(f, &col_types)).collect::<Vec<_>>().join(", ");
            let ids_str = ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
            let sql = format!("SELECT {} FROM {} WHERE id IN ({}) ORDER BY id", select_expr, table, ids_str);
            let rows = match sqlx::query(&sql).fetch_all(&state.db).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!("read SQL error for {}: {:?}", model, e);
                    return Some(Ok(serde_json::json!([])));
                }
            };
            let mut records = rows_to_json(rows, &final_sql_fields);

            // Stub computed/missing fields to prevent Odoo crashes
            for record in &mut records {
                for cf in &computed_fields {
                    if !record.as_object().map(|o| o.contains_key(cf.as_str())).unwrap_or(false) {
                        let stub_val = match cf.as_str() {
                            "display_name" => record.get("name").cloned().unwrap_or(serde_json::Value::Bool(false)),
                            name if name.ends_with("_ids") || name.ends_with("_lines") || name == "dependencies_id" || name.contains("_line") => {
                                serde_json::Value::Array(vec![])
                            },
                            _ => serde_json::Value::Bool(false),
                        };
                        if let Some(obj) = record.as_object_mut() {
                            obj.insert(cf.clone(), stub_val);
                        }
                    }
                }
            }

            convert_many2one_fields(&state.db, &table, &mut records, &final_sql_fields, &col_types).await;
            Some(Ok(serde_json::json!(records)))
        }

        "web_read" => {
            let ids: Vec<i64> = args.get(0).and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|i| i.as_i64()).collect()).unwrap_or_default();
            if ids.is_empty() { return Some(Ok(serde_json::json!([]))); }
            
            let spec = args.get(1).and_then(|v| v.as_object())
                .or_else(|| kwargs.get("specification").and_then(|v| v.as_object()));
                
            let fields_req: Vec<String> = match spec {
                Some(s) => s.keys().cloned().collect(),
                None => vec!["id".to_string()]
            };

            let valid = get_table_columns(&state.db, &table).await;
            if valid.is_empty() { return Some(Ok(serde_json::json!([]))); }
            let col_types = get_table_column_types(&state.db, &table).await;

            let (sql_fields, computed_fields): (Vec<String>, Vec<String>) = if fields_req.is_empty() {
                (vec!["id".to_string()], vec![])
            } else {
                let sql_f: Vec<String> = fields_req.iter().filter(|f| valid.contains(f)).cloned().collect();
                let comp_f: Vec<String> = fields_req.iter().filter(|f| !valid.contains(f)).cloned().collect();
                (sql_f, comp_f)
            };

            let mut final_sql_fields = sql_fields;
            if !final_sql_fields.contains(&"id".to_string()) {
                final_sql_fields.insert(0, "id".to_string());
            }

            let select_expr = final_sql_fields.iter().map(|f| jsonb_aware_col(f, &col_types)).collect::<Vec<_>>().join(", ");
            let ids_str = ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
            let sql = format!("SELECT {} FROM {} WHERE id IN ({}) ORDER BY id", select_expr, table, ids_str);
            let rows = match sqlx::query(&sql).fetch_all(&state.db).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!("web_read SQL error for {}: {:?}", model, e);
                    return Some(Ok(serde_json::json!([])));
                }
            };
            let mut records = rows_to_json(rows, &final_sql_fields);

            // Stub computed/missing fields to prevent Odoo crashes
            for record in &mut records {
                for cf in &computed_fields {
                    if !record.as_object().map(|o| o.contains_key(cf.as_str())).unwrap_or(false) {
                        let stub_val = match cf.as_str() {
                            "display_name" => record.get("name").cloned().unwrap_or(serde_json::Value::Bool(false)),
                            name if name.ends_with("_ids") || name.ends_with("_lines") || name == "dependencies_id" || name.contains("_line") => {
                                serde_json::Value::Array(vec![])
                            },
                            _ => serde_json::Value::Bool(false),
                        };
                        if let Some(obj) = record.as_object_mut() {
                            obj.insert(cf.clone(), stub_val);
                        }
                    }
                }
            }

            convert_many2one_fields(&state.db, &table, &mut records, &final_sql_fields, &col_types).await;
            Some(Ok(serde_json::json!(records)))
        }

        _ => None,
    }
}

async fn convert_many2one_fields(
    db: &sqlx::PgPool,
    table: &str,
    records: &mut Vec<serde_json::Value>,
    fields: &[String],
    col_types: &std::collections::HashMap<String, String>,
) {
    let m2o_fields: Vec<String> = fields.iter()
        .filter(|f| {
            f.ends_with("_id") && col_types.get(f.as_str())
                .map(|t| t.contains("int"))
                .unwrap_or(false)
        })
        .cloned()
        .collect();

    for m2o in &m2o_fields {
        // Buscar la tabla relacionada real vía FK de PostgreSQL
        let fk_sql = "
            SELECT ccu.table_name AS foreign_table
            FROM information_schema.table_constraints AS tc
            JOIN information_schema.key_column_usage AS kcu
              ON tc.constraint_name = kcu.constraint_name
            JOIN information_schema.constraint_column_usage AS ccu
              ON ccu.constraint_name = tc.constraint_name
            WHERE tc.constraint_type = 'FOREIGN KEY'
              AND tc.table_name = $1
              AND kcu.column_name = $2
            LIMIT 1
        ";
        let rel_table: String = match sqlx::query(fk_sql)
            .bind(table)
            .bind(m2o)
            .fetch_optional(db)
            .await
        {
            Ok(Some(r)) => {
                use sqlx::Row;
                r.try_get("foreign_table").unwrap_or_default()
            }
            _ => String::new(),
        };

        if rel_table.is_empty() {
            // No FK encontrado, dejar el valor como false
            for record in records.iter_mut() {
                if let Some(obj) = record.as_object_mut() {
                    if obj.get(m2o).and_then(|v| v.as_i64()).is_some() {
                        obj.insert(m2o.clone(), serde_json::Value::Bool(false));
                    }
                }
            }
            continue;
        }

        // Obtener todos los IDs únicos no-false de este campo en los records
        let ids: Vec<i64> = records.iter()
            .filter_map(|r| r.get(m2o).and_then(|v| v.as_i64()))
            .collect::<std::collections::HashSet<i64>>()
            .into_iter()
            .collect();

        if ids.is_empty() {
            for record in records.iter_mut() {
                if let Some(obj) = record.as_object_mut() {
                    if obj.get(m2o).map(|v| v.is_null()).unwrap_or(true) {
                        obj.insert(m2o.clone(), serde_json::Value::Bool(false));
                    }
                }
            }
            continue;
        }

        // Buscar display_name en la tabla relacionada
        let ids_list = ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
        let name_map: std::collections::HashMap<i64, String> = {
            let rel_sql = format!(
                "SELECT id::bigint, CAST(COALESCE(name->>'es_MX', name->>'en_US', name::text, id::text) AS TEXT) AS dname FROM {} WHERE id IN ({}) LIMIT 500",
                rel_table, ids_list
            );
            match sqlx::query_as::<_, (i64, Option<String>)>(&rel_sql)
                .fetch_all(db)
                .await
            {
                Ok(rows) => {
                    rows.into_iter().map(|(id, name)| {
                        (id, name.unwrap_or_else(|| format!("#{}", id)))
                    }).collect()
                }
                Err(e) => {
                    tracing::warn!("m2o name lookup failed for '{}': {:?}", rel_table, e);
                    std::collections::HashMap::new()
                }
            }
        };

        // Reemplazar integer → [id, "display_name"] en cada record
        for record in records.iter_mut() {
            if let Some(obj) = record.as_object_mut() {
                if let Some(val) = obj.get(m2o) {
                    let new_val = if let Some(id) = val.as_i64() {
                        let dname = name_map.get(&id)
                            .cloned()
                            .unwrap_or_else(|| format!("#{}", id));
                        serde_json::json!([id, dname])
                    } else {
                        serde_json::Value::Bool(false)
                    };
                    obj.insert(m2o.clone(), new_val);
                } else {
                    obj.insert(m2o.clone(), serde_json::Value::Bool(false));
                }
            }
        }
    }
}

/// GET /websocket/health
pub async fn websocket_health() -> impl IntoResponse {
    let data = serde_json::json!({
        "status": "pass"
    });
    Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::CACHE_CONTROL, "no-store")
        .body(axum::body::Body::from(data.to_string()))
        .unwrap()
}

/// POST /websocket/peek_notifications
pub async fn websocket_peek_notifications(
    Json(payload): Json<JsonRpcRequest>,
) -> impl IntoResponse {
    let result = serde_json::json!({
        "channels": [],
        "notifications": []
    });
    Json(JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result,
        id: payload.id,
    })
}

/// POST /websocket/on_closed
pub async fn websocket_on_closed(
    Json(payload): Json<JsonRpcRequest>,
) -> impl IntoResponse {
    Json(JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result: serde_json::json!(true),
        id: payload.id,
    })
}

/// GET /websocket
pub async fn serve_websocket(
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
    tracing::info!("WebSocket client connected");
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Close(_) => {
                tracing::info!("WebSocket client disconnected");
                break;
            }
            Message::Ping(payload) => {
                if socket.send(Message::Pong(payload)).await.is_err() {
                    break;
                }
            }
            _ => {}
        }
    }
}

/// GET /bus/websocket_worker_bundle
pub async fn serve_websocket_worker_bundle() -> impl IntoResponse {
    let files = [
        "/home/ealvarez/workspace/erp/nexustech_produccion/core/nexustech/addons/web/static/src/module_loader.js",
        "/home/ealvarez/workspace/erp/nexustech_produccion/core/nexustech/addons/bus/static/src/workers/bus_worker_utils.js",
        "/home/ealvarez/workspace/erp/nexustech_produccion/core/nexustech/addons/bus/static/src/workers/base_worker.js",
        "/home/ealvarez/workspace/erp/nexustech_produccion/core/nexustech/addons/bus/static/src/workers/election_worker.js",
        "/home/ealvarez/workspace/erp/nexustech_produccion/core/nexustech/addons/bus/static/src/workers/websocket_worker.js",
        "/home/ealvarez/workspace/erp/nexustech_produccion/core/nexustech/addons/bus/static/src/workers/bus_worker_script.js",
    ];
    
    let mut combined_js = String::new();
    for file in &files {
        match fs::read_to_string(file).await {
            Ok(content) => {
                for line in content.lines() {
                    let trimmed = line.trim_start();
                    if trimmed.starts_with("import ") {
                        continue;
                    }
                    if trimmed.starts_with("export ") {
                        let indent_len = line.len() - trimmed.len();
                        let indent = &line[..indent_len];
                        let after_export = &trimmed[7..];
                        combined_js.push_str(indent);
                        combined_js.push_str(after_export);
                        combined_js.push('\n');
                    } else {
                        combined_js.push_str(line);
                        combined_js.push('\n');
                    }
                }
                combined_js.push_str("\n\n");
            }
            Err(e) => {
                tracing::error!("Error reading worker asset file {}: {:?}", file, e);
            }
        }
    }
    
    Response::builder()
        .header(header::CONTENT_TYPE, "application/javascript")
        .body(axum::body::Body::from(combined_js))
        .unwrap()
}

/// POST /web/action/load
pub async fn action_load(
    Json(payload): Json<JsonRpcRequest>,
) -> impl IntoResponse {
    let payload_id = payload.id.clone();
    let result = tokio::task::spawn_blocking(move || {
        Python::with_gil(|py| -> Result<serde_json::Value, PyErr> {
            let params = &payload.params;
            let action_id = params.get("action_id").cloned().unwrap_or(serde_json::Value::Null);
            let context = params.get("context").cloned().unwrap_or_else(|| serde_json::json!({}));
            
            let api_mod = py.import_bound("odoo.api")?;
            let json_mod = py.import_bound("json")?;
            
            let context_str = context.to_string();
            let context_py = json_mod.call_method1("loads", (context_str,))?;
            
            let env_obj = api_mod.getattr("Environment")?.call1((1, 2, context_py))?;
            
            let code = r#"
def localize_field(val, env):
    if isinstance(val, dict):
        lang = env.context.get("lang") or "es_MX"
        return val.get(lang) or val.get("es_MX") or val.get("en_US") or next(iter(val.values()), "")
    return val

def generate_views(action):
    view_id = action.get('view_id') or False
    if isinstance(view_id, (list, tuple)):
        view_id = view_id[0]
    view_mode = action.get('view_mode') or 'list,form'
    view_modes = view_mode.split(',')
    if len(view_modes) > 1:
        if view_id:
            action['views'] = [(view_id, view_modes[0])] + [(False, mode) for mode in view_modes[1:]]
            return
        action['views'] = [(False, mode) for mode in view_modes]
        return
    action['views'] = [(view_id, view_modes[0])]

def clean_action(action, env):
    if not action:
        return False
    action_type = action.setdefault('type', 'ir.actions.act_window_close')
    if action_type == 'ir.actions.act_window' and (not action.get('views')):
        generate_views(action)
    try:
        model = env[action_type]
        readable_fields = set(model._fields.keys())
        action_type_fields = model._fields.keys()
    except Exception:
        return action
    cleaned_action = {field: value for (field, value) in action.items() if field in readable_fields or field not in action_type_fields}
    return cleaned_action

def get_action_data(env, action_id):
    if not action_id:
        return None
    res_id = None
    act_type = None
    try:
        res_id = int(action_id)
        env.cr.execute("SELECT type FROM ir_actions WHERE id = %s", [res_id])
        row = env.cr.fetchone()
        if row:
            act_type = row[0]
    except (ValueError, TypeError):
        action_id_str = str(action_id)
        if '.' in action_id_str:
            module, name = action_id_str.split('.', 1)
            env.cr.execute("SELECT model, res_id FROM ir_model_data WHERE module = %s AND name = %s", [module, name])
            row = env.cr.fetchone()
            if row:
                act_type, res_id = row[0], row[1]
        else:
            env.cr.execute("SELECT id, type FROM ir_actions WHERE path = %s", [action_id_str])
            row = env.cr.fetchone()
            if row:
                res_id, act_type = row[0], row[1]
            else:
                # Fallback: buscar por tag en ir_act_client
                # (ej: 'action_spreadsheet_dashboard')
                env.cr.execute("SELECT id FROM ir_act_client WHERE tag = %s", [action_id_str])
                row = env.cr.fetchone()
                if row:
                    res_id = row[0]
                    act_type = 'ir.actions.client'
    if not res_id or not act_type:
        try:
            res_id = int(action_id)
            env.cr.execute("SELECT type FROM ir_actions WHERE id = %s", [res_id])
            row = env.cr.fetchone()
            if row:
                act_type = row[0]
        except Exception:
            pass
    if not res_id or not act_type:
        return None
    MODEL_TO_TABLE = {
        'ir.actions.act_window': 'ir_act_window',
        'ir.actions.server': 'ir_act_server',
        'ir.actions.client': 'ir_act_client',
        'ir.actions.report': 'ir_act_report',
        'ir.actions.url': 'ir_act_url',
        'ir.actions.act_url': 'ir_act_url',
    }
    table_name = MODEL_TO_TABLE.get(act_type, act_type.replace('.', '_'))
    try:
        env.cr.execute(f"SELECT * FROM {table_name} WHERE id = %s", [res_id])
        rows = env.cr.dictfetchall()
        if not rows:
            return None
        action = rows[0]
    except Exception:
        try:
            env.cr.execute("SELECT * FROM ir_actions WHERE id = %s", [res_id])
            rows = env.cr.dictfetchall()
            if not rows:
                return None
            action = rows[0]
        except Exception:
            return None
    action['type'] = act_type
    if 'name' in action:
        action['name'] = localize_field(action['name'], env)
    if 'help' in action:
        action['help'] = localize_field(action['help'], env)
    def resolve_m2o(field_id, model_table):
        if not field_id or not isinstance(field_id, int):
            return field_id
        try:
            if model_table == 'ir_model':
                env.cr.execute("SELECT model, name FROM ir_model WHERE id = %s", [field_id])
                r = env.cr.fetchone()
                if r:
                    return [field_id, r[0]]
            elif model_table == 'ir_ui_view':
                env.cr.execute("SELECT name FROM ir_ui_view WHERE id = %s", [field_id])
                r = env.cr.fetchone()
                if r:
                    return [field_id, r[0]]
        except Exception:
            pass
        return field_id
    if 'binding_model_id' in action:
        action['binding_model_id'] = resolve_m2o(action['binding_model_id'], 'ir_model')
    if 'model_id' in action:
        action['model_id'] = resolve_m2o(action['model_id'], 'ir_model')
    if 'view_id' in action:
        action['view_id'] = resolve_m2o(action['view_id'], 'ir_ui_view')
    if 'search_view_id' in action:
        action['search_view_id'] = resolve_m2o(action['search_view_id'], 'ir_ui_view')
    if act_type == 'ir.actions.act_window':
        try:
            env.cr.execute("SELECT view_id, view_mode FROM ir_act_window_view WHERE act_window_id = %s ORDER BY sequence", [res_id])
            view_rows = env.cr.fetchall()
            if view_rows:
                action['views'] = [[v[0] if v[0] else False, v[1]] for v in view_rows]
        except Exception:
            pass
    return action

def run_load(env, action_id):
    act = get_action_data(env, action_id)
    if not act:
        return False
    return clean_action(act, env)
"#;
            let globals = PyDict::new_bound(py);
            py.run_bound(code, Some(&globals), None)?;
            let run_load_fn = globals.get_item("run_load")?.unwrap();
            
            let action_id_py = match action_id {
                serde_json::Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        i.into_py(py)
                    } else if let Some(f) = n.as_f64() {
                        f.into_py(py)
                    } else {
                        py.None()
                    }
                }
                serde_json::Value::String(s) => s.into_py(py),
                _ => py.None(),
            };
            
            let py_result = run_load_fn.call1((env_obj, action_id_py))?;
            let result_json_str: String = json_mod.call_method1("dumps", (py_result,))?.extract()?;
            let val: serde_json::Value = serde_json::from_str(&result_json_str)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("JSON parse error: {:?}", e)))?;
            Ok(val)
        })
    }).await;
    
    let inner_result = match result {
        Ok(res) => res,
        Err(e) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::Body::from(format!("Join error: {:?}", e)))
                .unwrap()
                .into_response();
        }
    };
    
    match inner_result {
        Ok(val) => {
            Json(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: val,
                id: payload_id,
            }).into_response()
        }
        Err(e) => {
            tracing::error!("Error in action_load: {:?}", e);
            Json(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: serde_json::json!({
                    "error": format!("{:?}", e)
                }),
                id: payload_id,
            }).into_response()
        }
    }
}

/// POST /web/action/run
pub async fn action_run(
    Json(payload): Json<JsonRpcRequest>,
) -> impl IntoResponse {
    let payload_id = payload.id.clone();
    let result = tokio::task::spawn_blocking(move || {
        Python::with_gil(|py| -> Result<serde_json::Value, PyErr> {
            let params = &payload.params;
            let action_id = params.get("action_id").cloned().unwrap_or(serde_json::Value::Null);
            let context = params.get("context").cloned().unwrap_or_else(|| serde_json::json!({}));
            
            let api_mod = py.import_bound("odoo.api")?;
            let json_mod = py.import_bound("json")?;
            
            let context_str = context.to_string();
            let context_py = json_mod.call_method1("loads", (context_str,))?;
            
            let env_obj = api_mod.getattr("Environment")?.call1((1, 2, context_py))?;
            
            let code = r#"
def localize_field(val, env):
    if isinstance(val, dict):
        lang = env.context.get("lang") or "es_MX"
        return val.get(lang) or val.get("es_MX") or val.get("en_US") or next(iter(val.values()), "")
    return val

def generate_views(action):
    view_id = action.get('view_id') or False
    if isinstance(view_id, (list, tuple)):
        view_id = view_id[0]
    view_mode = action.get('view_mode') or 'list,form'
    view_modes = view_mode.split(',')
    if len(view_modes) > 1:
        if view_id:
            action['views'] = [(view_id, view_modes[0])] + [(False, mode) for mode in view_modes[1:]]
            return
        action['views'] = [(False, mode) for mode in view_modes]
        return
    action['views'] = [(view_id, view_modes[0])]

def clean_action(action, env):
    if not action:
        return False
    action_type = action.setdefault('type', 'ir.actions.act_window_close')
    if action_type == 'ir.actions.act_window' and (not action.get('views')):
        generate_views(action)
    try:
        model = env[action_type]
        readable_fields = set(model._fields.keys())
        action_type_fields = model._fields.keys()
    except Exception:
        return action
    cleaned_action = {field: value for (field, value) in action.items() if field in readable_fields or field not in action_type_fields}
    return cleaned_action

def get_action_data(env, action_id):
    if not action_id:
        return None
    res_id = None
    act_type = None
    try:
        res_id = int(action_id)
        env.cr.execute("SELECT type FROM ir_actions WHERE id = %s", [res_id])
        row = env.cr.fetchone()
        if row:
            act_type = row[0]
    except (ValueError, TypeError):
        action_id_str = str(action_id)
        if '.' in action_id_str:
            module, name = action_id_str.split('.', 1)
            env.cr.execute("SELECT model, res_id FROM ir_model_data WHERE module = %s AND name = %s", [module, name])
            row = env.cr.fetchone()
            if row:
                act_type, res_id = row[0], row[1]
        else:
            env.cr.execute("SELECT id, type FROM ir_actions WHERE path = %s", [action_id_str])
            row = env.cr.fetchone()
            if row:
                res_id, act_type = row[0], row[1]
    if not res_id or not act_type:
        try:
            res_id = int(action_id)
            env.cr.execute("SELECT type FROM ir_actions WHERE id = %s", [res_id])
            row = env.cr.fetchone()
            if row:
                act_type = row[0]
        except Exception:
            pass
    if not res_id or not act_type:
        return None
    MODEL_TO_TABLE = {
        'ir.actions.act_window': 'ir_act_window',
        'ir.actions.server': 'ir_act_server',
        'ir.actions.client': 'ir_act_client',
        'ir.actions.report': 'ir_act_report',
        'ir.actions.url': 'ir_act_url',
        'ir.actions.act_url': 'ir_act_url',
    }
    table_name = MODEL_TO_TABLE.get(act_type, act_type.replace('.', '_'))
    try:
        env.cr.execute(f"SELECT * FROM {table_name} WHERE id = %s", [res_id])
        rows = env.cr.dictfetchall()
        if not rows:
            return None
        action = rows[0]
    except Exception:
        try:
            env.cr.execute("SELECT * FROM ir_actions WHERE id = %s", [res_id])
            rows = env.cr.dictfetchall()
            if not rows:
                return None
            action = rows[0]
        except Exception:
            return None
    action['type'] = act_type
    if 'name' in action:
        action['name'] = localize_field(action['name'], env)
    if 'help' in action:
        action['help'] = localize_field(action['help'], env)
    def resolve_m2o(field_id, model_table):
        if not field_id or not isinstance(field_id, int):
            return field_id
        try:
            if model_table == 'ir_model':
                env.cr.execute("SELECT model, name FROM ir_model WHERE id = %s", [field_id])
                r = env.cr.fetchone()
                if r:
                    return [field_id, r[0]]
            elif model_table == 'ir_ui_view':
                env.cr.execute("SELECT name FROM ir_ui_view WHERE id = %s", [field_id])
                r = env.cr.fetchone()
                if r:
                    return [field_id, r[0]]
        except Exception:
            pass
        return field_id
    if 'binding_model_id' in action:
        action['binding_model_id'] = resolve_m2o(action['binding_model_id'], 'ir_model')
    if 'model_id' in action:
        action['model_id'] = resolve_m2o(action['model_id'], 'ir_model')
    if 'view_id' in action:
        action['view_id'] = resolve_m2o(action['view_id'], 'ir_ui_view')
    if 'search_view_id' in action:
        action['search_view_id'] = resolve_m2o(action['search_view_id'], 'ir_ui_view')
    if act_type == 'ir.actions.act_window':
        try:
            env.cr.execute("SELECT view_id, view_mode FROM ir_act_window_view WHERE act_window_id = %s ORDER BY sequence", [res_id])
            view_rows = env.cr.fetchall()
            if view_rows:
                action['views'] = [[v[0] if v[0] else False, v[1]] for v in view_rows]
        except Exception:
            pass
    return action

def run_action(env, action_id):
    try:
        action = env['ir.actions.server'].browse(int(action_id))
        result = action.run()
        return clean_action(result, env=env) if result else False
    except Exception:
        act = get_action_data(env, action_id)
        if act:
            return clean_action(act, env)
        return False
"#;
            let globals = PyDict::new_bound(py);
            py.run_bound(code, Some(&globals), None)?;
            let run_action_fn = globals.get_item("run_action")?.unwrap();
            
            let action_id_py = match action_id {
                serde_json::Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        i.into_py(py)
                    } else if let Some(f) = n.as_f64() {
                        f.into_py(py)
                    } else {
                        py.None()
                    }
                }
                serde_json::Value::String(s) => s.into_py(py),
                _ => py.None(),
            };
            
            let py_result = run_action_fn.call1((env_obj, action_id_py))?;
            let result_json_str: String = json_mod.call_method1("dumps", (py_result,))?.extract()?;
            let val: serde_json::Value = serde_json::from_str(&result_json_str)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("JSON parse error: {:?}", e)))?;
            Ok(val)
        })
    }).await;
    
    let inner_result = match result {
        Ok(res) => res,
        Err(e) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::Body::from(format!("Join error: {:?}", e)))
                .unwrap()
                .into_response();
        }
    };
    
    match inner_result {
        Ok(val) => {
            Json(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: val,
                id: payload_id,
            }).into_response()
        }
        Err(e) => {
            tracing::error!("Error in action_run: {:?}", e);
            Json(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: serde_json::json!({
                    "error": format!("{:?}", e)
                }),
                id: payload_id,
            }).into_response()
        }
    }
}

/// POST /web/action/load_breadcrumbs
pub async fn action_load_breadcrumbs(
    Json(payload): Json<JsonRpcRequest>,
) -> impl IntoResponse {
    let payload_id = payload.id.clone();
    let result = tokio::task::spawn_blocking(move || {
        Python::with_gil(|py| -> Result<serde_json::Value, PyErr> {
            let params = &payload.params;
            let actions = params.get("actions").cloned().unwrap_or_else(|| serde_json::json!([]));
            let context = params.get("context").cloned().unwrap_or_else(|| serde_json::json!({}));
            
            let api_mod = py.import_bound("odoo.api")?;
            let json_mod = py.import_bound("json")?;
            
            let context_str = context.to_string();
            let context_py = json_mod.call_method1("loads", (context_str,))?;
            
            let env_obj = api_mod.getattr("Environment")?.call1((1, 2, context_py))?;
            
            let code = r#"
def localize_field(val, env):
    if isinstance(val, dict):
        lang = env.context.get("lang") or "es_MX"
        return val.get(lang) or val.get("es_MX") or val.get("en_US") or next(iter(val.values()), "")
    return val

def generate_views(action):
    view_id = action.get('view_id') or False
    if isinstance(view_id, (list, tuple)):
        view_id = view_id[0]
    view_mode = action.get('view_mode') or 'list,form'
    view_modes = view_mode.split(',')
    if len(view_modes) > 1:
        if view_id:
            action['views'] = [(view_id, view_modes[0])] + [(False, mode) for mode in view_modes[1:]]
            return
        action['views'] = [(False, mode) for mode in view_modes]
        return
    action['views'] = [(view_id, view_modes[0])]

def clean_action(action, env):
    if not action:
        return False
    action_type = action.setdefault('type', 'ir.actions.act_window_close')
    if action_type == 'ir.actions.act_window' and (not action.get('views')):
        generate_views(action)
    try:
        model = env[action_type]
        readable_fields = set(model._fields.keys())
        action_type_fields = model._fields.keys()
    except Exception:
        return action
    cleaned_action = {field: value for (field, value) in action.items() if field in readable_fields or field not in action_type_fields}
    return cleaned_action

def get_action_data(env, action_id):
    if not action_id:
        return None
    res_id = None
    act_type = None
    try:
        res_id = int(action_id)
        env.cr.execute("SELECT type FROM ir_actions WHERE id = %s", [res_id])
        row = env.cr.fetchone()
        if row:
            act_type = row[0]
    except (ValueError, TypeError):
        action_id_str = str(action_id)
        if '.' in action_id_str:
            module, name = action_id_str.split('.', 1)
            env.cr.execute("SELECT model, res_id FROM ir_model_data WHERE module = %s AND name = %s", [module, name])
            row = env.cr.fetchone()
            if row:
                act_type, res_id = row[0], row[1]
        else:
            env.cr.execute("SELECT id, type FROM ir_actions WHERE path = %s", [action_id_str])
            row = env.cr.fetchone()
            if row:
                res_id, act_type = row[0], row[1]
    if not res_id or not act_type:
        try:
            res_id = int(action_id)
            env.cr.execute("SELECT type FROM ir_actions WHERE id = %s", [res_id])
            row = env.cr.fetchone()
            if row:
                act_type = row[0]
        except Exception:
            pass
    if not res_id or not act_type:
        return None
    MODEL_TO_TABLE = {
        'ir.actions.act_window': 'ir_act_window',
        'ir.actions.server': 'ir_act_server',
        'ir.actions.client': 'ir_act_client',
        'ir.actions.report': 'ir_act_report',
        'ir.actions.url': 'ir_act_url',
        'ir.actions.act_url': 'ir_act_url',
    }
    table_name = MODEL_TO_TABLE.get(act_type, act_type.replace('.', '_'))
    try:
        env.cr.execute(f"SELECT * FROM {table_name} WHERE id = %s", [res_id])
        rows = env.cr.dictfetchall()
        if not rows:
            return None
        action = rows[0]
    except Exception:
        try:
            env.cr.execute("SELECT * FROM ir_actions WHERE id = %s", [res_id])
            rows = env.cr.dictfetchall()
            if not rows:
                return None
            action = rows[0]
        except Exception:
            return None
    action['type'] = act_type
    if 'name' in action:
        action['name'] = localize_field(action['name'], env)
    if 'help' in action:
        action['help'] = localize_field(action['help'], env)
    def resolve_m2o(field_id, model_table):
        if not field_id or not isinstance(field_id, int):
            return field_id
        try:
            if model_table == 'ir_model':
                env.cr.execute("SELECT model, name FROM ir_model WHERE id = %s", [field_id])
                r = env.cr.fetchone()
                if r:
                    return [field_id, r[0]]
            elif model_table == 'ir_ui_view':
                env.cr.execute("SELECT name FROM ir_ui_view WHERE id = %s", [field_id])
                r = env.cr.fetchone()
                if r:
                    return [field_id, r[0]]
        except Exception:
            pass
        return field_id
    if 'binding_model_id' in action:
        action['binding_model_id'] = resolve_m2o(action['binding_model_id'], 'ir_model')
    if 'model_id' in action:
        action['model_id'] = resolve_m2o(action['model_id'], 'ir_model')
    if 'view_id' in action:
        action['view_id'] = resolve_m2o(action['view_id'], 'ir_ui_view')
    if 'search_view_id' in action:
        action['search_view_id'] = resolve_m2o(action['search_view_id'], 'ir_ui_view')
    if act_type == 'ir.actions.act_window':
        try:
            env.cr.execute("SELECT view_id, view_mode FROM ir_act_window_view WHERE act_window_id = %s ORDER BY sequence", [res_id])
            view_rows = env.cr.fetchall()
            if view_rows:
                action['views'] = [[v[0] if v[0] else False, v[1]] for v in view_rows]
        except Exception:
            pass
    return action

def run_breadcrumbs(env, actions):
    results = []
    for idx, action_info in enumerate(actions):
        record_id = action_info.get('resId')
        try:
            if action_info.get('action'):
                act_id = action_info.get('action')
                act = get_action_data(env, act_id)
                if not act:
                    results.append({'error': f"Action {act_id} not found"})
                    continue
                    
                if act['type'] == 'ir.actions.server':
                    if act.get('path'):
                        try:
                            run_res = env['ir.actions.server'].browse(act['id']).run()
                            if run_res:
                                act = clean_action(run_res, env=env)
                        except Exception:
                            pass
                    else:
                        results.append({'error': 'A server action must have a path to be restored'})
                        continue
                if not act.get('display_name'):
                    act['display_name'] = act.get('name') or ''
                if act['type'] == 'ir.actions.client' and idx + 1 < len(actions) and action_info.get('action') == actions[idx + 1].get('action'):
                    results.append({'error': 'Client actions don\'t have multi-record views'})
                    continue
                if record_id:
                    if record_id == 'new':
                        results.append({'display_name': "Nuevo"})
                    elif act.get('res_model'):
                        try:
                            display_name = env[act['res_model']].browse(record_id).display_name
                        except Exception:
                            display_name = f"{act['res_model']}({record_id})"
                        results.append({'display_name': display_name})
                    else:
                        results.append({'display_name': act['display_name']})
                else:
                    if act.get('res_model') and act['type'] != 'ir.actions.client':
                        name = act['display_name'] if any(view[1] != 'form' and view[1] != 'search' for view in act.get('views', [])) else None
                    else:
                        name = act['display_name']
                    results.append({'display_name': name})
            elif action_info.get('model'):
                model_name = action_info.get('model')
                if record_id:
                    if record_id == 'new':
                        results.append({'display_name': "Nuevo"})
                    else:
                        try:
                            display_name = env[model_name].browse(record_id).display_name
                        except Exception:
                            display_name = f"{model_name}({record_id})"
                        results.append({'display_name': display_name})
                else:
                    results.append({'error': 'Actions with a model should also have a resId'})
            else:
                results.append({'error': 'Actions should have either an action (id or path) or a model'})
        except Exception as exc:
            results.append({'error': str(exc)})
    return results
"#;
            let globals = PyDict::new_bound(py);
            py.run_bound(code, Some(&globals), None)?;
            let run_breadcrumbs_fn = globals.get_item("run_breadcrumbs")?.unwrap();
            
            let actions_str = actions.to_string();
            let actions_py = json_mod.call_method1("loads", (actions_str,))?;
            
            let py_result = run_breadcrumbs_fn.call1((env_obj, actions_py))?;
            let result_json_str: String = json_mod.call_method1("dumps", (py_result,))?.extract()?;
            let val: serde_json::Value = serde_json::from_str(&result_json_str)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("JSON parse error: {:?}", e)))?;
            Ok(val)
        })
    }).await;
    
    let inner_result = match result {
        Ok(res) => res,
        Err(e) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::Body::from(format!("Join error: {:?}", e)))
                .unwrap()
                .into_response();
        }
    };
    
    match inner_result {
        Ok(val) => {
            Json(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: val,
                id: payload_id,
            }).into_response()
        }
        Err(e) => {
            tracing::error!("Error in action_load_breadcrumbs: {:?}", e);
            Json(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: serde_json::json!({
                    "error": format!("{:?}", e)
                }),
                id: payload_id,
            }).into_response()
        }
    }
}



// ─────────────────────────────────────────────────────────────────────────────
// Mail bus / longpolling stub
// El frontend Odoo llama a /mail/data esperando notificaciones en tiempo real.
// Retornamos una respuesta vacía válida para evitar el ConnectionLostError.
// ─────────────────────────────────────────────────────────────────────────────
pub async fn mail_data(
    body: Option<Json<serde_json::Value>>,
) -> impl IntoResponse {
    let id = body.as_ref()
        .and_then(|b| b.get("id").cloned())
        .unwrap_or(serde_json::json!(1));

    Json(serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": {
            "Store": {
                "messages": [],
                "Message": [],
                "Discuss.Channel": [],
                "mail.notification": [],
                "res.partner": [],
                "bus.bus": []
            }
        }
    }))
}

// ─────────────────────────────────────────────────────────────────────────────
// PWA Web Manifest
// ─────────────────────────────────────────────────────────────────────────────
pub async fn serve_manifest() -> impl IntoResponse {
    let manifest = serde_json::json!({
        "name": "NexusTech ERP",
        "short_name": "NexusTech",
        "start_url": "/nexustech",
        "display": "standalone",
        "background_color": "#1a1a2e",
        "theme_color": "#6c5ce7",
        "icons": []
    });
    (
        [(axum::http::header::CONTENT_TYPE, "application/manifest+json")],
        Json(manifest),
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// Service Worker stub (vacío pero válido para que el navegador no marque 404)
// ─────────────────────────────────────────────────────────────────────────────
pub async fn serve_service_worker() -> impl IntoResponse {
    // Service-Worker-Allowed: / permite que el SW registrado desde /web/
    // controle el scope /nexustech (que está en diferente path raíz).
    let body = "// NexusTech ERP Service Worker\nself.addEventListener('install', () => self.skipWaiting());\nself.addEventListener('activate', e => e.waitUntil(clients.claim()));\n";
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/javascript")
        .header("service-worker-allowed", "/")
        .body(axum::body::Body::from(body))
        .unwrap()
        .into_response()
}
