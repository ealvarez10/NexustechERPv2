//! Handlers de emisión y gestión de CFDIs
//!
//! POST /api/v1/cfdi/timbrar      — Genera + sella + timbra un CFDI
//! POST /api/v1/cfdi/cancelar     — Cancela un CFDI por UUID
//! GET  /api/v1/cfdi/{uuid}/pdf   — Descarga representación impresa en PDF

use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use base64::Engine;
use serde::{Deserialize, Serialize};

use crate::state::AppState;

// ── TIMBRADO ─────────────────────────────────────────────────────────────────

/// Payload para timbrar un CFDI
#[derive(Debug, Deserialize)]
pub struct TimbrarRequest {
    /// Datos estructurados del comprobante
    pub cfdi: nexus_cfdi::CfdiData,
    /// Certificado CSD en base64 (.cer en DER)
    pub cert_b64: String,
    /// Clave privada CSD en base64 (.key en DER)
    pub key_b64: String,
    /// Contraseña del CSD (no usada en ring, pero se recibe para compatibilidad futura)
    #[allow(dead_code)]
    pub key_password: String,
}

#[derive(Debug, Serialize)]
pub struct TimbrarResponse {
    pub success: bool,
    pub uuid: Option<String>,
    pub xml_timbrado: Option<String>,
    pub fecha_timbrado: Option<String>,
    pub error: Option<String>,
}

/// POST /api/v1/cfdi/timbrar
///
/// Flujo:
/// 1. Genera cadena original
/// 2. Decodifica CSD de base64 y escribe a archivos temporales
/// 3. Sella con RSA-SHA256 usando el CSD
/// 4. Genera XML CFDI 4.0 sellado
/// 5. Timbra vía PAC configurado
pub async fn timbrar(
    State(state): State<AppState>,
    Json(req): Json<TimbrarRequest>,
) -> Response {
    // 1. Generar cadena original
    let cadena = nexus_cfdi::cadena_original::generar(&req.cfdi);

    // 2. Decodificar CSD
    let cert_bytes = match base64::engine::general_purpose::STANDARD.decode(&req.cert_b64) {
        Ok(b) => b,
        Err(e) => {
            return Json(TimbrarResponse {
                success: false,
                uuid: None,
                xml_timbrado: None,
                fecha_timbrado: None,
                error: Some(format!("CER inválido: {}", e)),
            })
            .into_response();
        }
    };

    let key_bytes = match base64::engine::general_purpose::STANDARD.decode(&req.key_b64) {
        Ok(b) => b,
        Err(e) => {
            return Json(TimbrarResponse {
                success: false,
                uuid: None,
                xml_timbrado: None,
                fecha_timbrado: None,
                error: Some(format!("KEY inválido: {}", e)),
            })
            .into_response();
        }
    };

    // 3. Escribir archivos temporales para el CSD
    let tmp_dir = std::env::temp_dir();
    let cer_path = tmp_dir.join(format!("cfdi_{}.cer", uuid::Uuid::new_v4()));
    let key_path = tmp_dir.join(format!("cfdi_{}.key", uuid::Uuid::new_v4()));

    if let Err(e) = std::fs::write(&cer_path, &cert_bytes) {
        return Json(TimbrarResponse {
            success: false,
            uuid: None,
            xml_timbrado: None,
            fecha_timbrado: None,
            error: Some(format!("Error escribiendo CER temporal: {}", e)),
        })
        .into_response();
    }

    if let Err(e) = std::fs::write(&key_path, &key_bytes) {
        let _ = std::fs::remove_file(&cer_path);
        return Json(TimbrarResponse {
            success: false,
            uuid: None,
            xml_timbrado: None,
            fecha_timbrado: None,
            error: Some(format!("Error escribiendo KEY temporal: {}", e)),
        })
        .into_response();
    }

    // 4. Sellar con CSD
    let sello = nexus_cfdi::sellado::sellar(&cadena, &key_bytes, &cer_path);

    // Limpiar archivos temporales
    let _ = std::fs::remove_file(&cer_path);
    let _ = std::fs::remove_file(&key_path);

    let sello = match sello {
        Ok(s) => s,
        Err(e) => {
            return Json(TimbrarResponse {
                success: false,
                uuid: None,
                xml_timbrado: None,
                fecha_timbrado: None,
                error: Some(format!("Sellado: {}", e)),
            })
            .into_response();
        }
    };

    // 5. Generar XML sellado
    let xml = match nexus_cfdi::xml::generar_sellado(&req.cfdi, &sello) {
        Ok(x) => x,
        Err(e) => {
            return Json(TimbrarResponse {
                success: false,
                uuid: None,
                xml_timbrado: None,
                fecha_timbrado: None,
                error: Some(format!("Generación XML: {}", e)),
            })
            .into_response();
        }
    };

    match state.pac.timbrar(&xml).await {
        Ok(timbre) => {
            use nexus_core::db::cfdi::{insertar, NuevoCfdi};
            let nuevo_cfdi = NuevoCfdi {
                uuid: timbre.uuid.clone(),
                folio: req.cfdi.folio.clone(),
                serie: req.cfdi.serie.clone(),
                fecha_emision: Some(req.cfdi.fecha.clone()),
                rfc_emisor: req.cfdi.emisor.rfc.clone(),
                rfc_receptor: req.cfdi.receptor.rfc.clone(),
                nombre_emisor: Some(req.cfdi.emisor.nombre.clone()),
                nombre_receptor: Some(req.cfdi.receptor.nombre.clone()),
                total: Some(req.cfdi.total),
                tipo_cfdi: Some(req.cfdi.tipo_de_comprobante.clone()),
                xml_timbrado: Some(timbre.xml_timbrado.clone()),
                fecha_timbrado: Some(timbre.fecha_timbrado.clone()),
                account_move_id: None,
            };
            if let Err(e) = insertar(&state.db, &nuevo_cfdi).await {
                tracing::error!("Error guardando CFDI timbrado en BD: {}", e);
            }

            Json(TimbrarResponse {
                success: true,
                uuid: Some(timbre.uuid),
                xml_timbrado: Some(timbre.xml_timbrado),
                fecha_timbrado: Some(timbre.fecha_timbrado),
                error: None,
            })
            .into_response()
        },
        Err(e) => Json(TimbrarResponse {
            success: false,
            uuid: None,
            xml_timbrado: None,
            fecha_timbrado: None,
            error: Some(format!("PAC: {}", e)),
        })
        .into_response(),
    }
}

// ── CANCELACIÓN ───────────────────────────────────────────────────────────────

/// Motivos de cancelación SAT:
/// 01 = Error con relación (requiere UUID relacionado)
/// 02 = Emitido con errores sin relación
/// 03 = No se llevó a cabo la operación
/// 04 = Operación nominativa relacionada
#[derive(Debug, Deserialize)]
pub struct CancelarRequest {
    pub rfc_emisor: String,
    pub uuid: String,
    pub motivo: String,
    pub uuid_relacionado: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CancelarResponse {
    pub success: bool,
    pub estado_cancelacion: Option<String>,
    pub acuse: Option<String>,
    pub error: Option<String>,
}

/// POST /api/v1/cfdi/cancelar
pub async fn cancelar(
    State(state): State<AppState>,
    Json(req): Json<CancelarRequest>,
) -> impl IntoResponse {
    match state
        .pac
        .cancelar(
            &req.rfc_emisor,
            &req.uuid,
            &req.motivo,
            req.uuid_relacionado.as_deref(),
        )
        .await
    {
        Ok(resp) => Json(CancelarResponse {
            success: true,
            estado_cancelacion: Some(resp.estado_cancelacion),
            acuse: resp.acuse,
            error: None,
        })
        .into_response(),
        Err(e) => Json(CancelarResponse {
            success: false,
            estado_cancelacion: None,
            acuse: None,
            error: Some(e.to_string()),
        })
        .into_response(),
    }
}

// ── PDF ───────────────────────────────────────────────────────────────────────

/// GET /api/v1/cfdi/{uuid}/pdf
///
/// En una implementación completa se consulta la DB por el UUID,
/// se obtiene el XML timbrado y se genera el PDF. En esta fase
/// retorna un mensaje estructurado indicando la consulta pendiente.
pub async fn pdf_por_uuid(
    State(_state): State<AppState>,
    Path(uuid): Path<String>,
) -> impl IntoResponse {
    // TODO: Consultar DB por UUID, obtener XML timbrado, generar PDF
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "success": false,
            "uuid": uuid,
            "mensaje": "Consulta por UUID requiere integración con tabla de CFDIs timbrados. Próxima fase."
        })),
    )
        .into_response()
}

/// POST /api/v1/cfdi/pdf — Genera PDF con datos completos del comprobante
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct PdfRequest {
    pub cfdi: nexus_cfdi::CfdiData,
    pub uuid: String,
    pub sello_emisor: String,
    pub sello_sat: String,
    pub cadena_original: String,
    pub cert_emisor: String,
    pub cert_sat: String,
}

#[allow(dead_code)]
pub async fn pdf(
    Json(req): Json<PdfRequest>,
) -> impl IntoResponse {
    let opciones = nexus_cfdi::OpcionesPdf::default();
    match nexus_cfdi::generar_pdf(
        &req.cfdi,
        &req.uuid,
        &req.sello_emisor,
        &req.sello_sat,
        &req.cadena_original,
        &req.cert_emisor,
        &req.cert_sat,
        &opciones,
    ) {
        Ok(pdf_cfdi) => {
            let headers = [
                (header::CONTENT_TYPE, "application/pdf".to_string()),
                (
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", pdf_cfdi.nombre_archivo),
                ),
            ];
            (StatusCode::OK, headers, pdf_cfdi.bytes).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "success": false, "error": e.to_string() })),
        )
            .into_response(),
    }
}
