//! Integración con Facturama Multiemisor
//!
//! API REST de Facturama para timbrado y cancelación de CFDIs.
//! Documentación: https://apisandbox.facturama.mx/docs

use super::{Pac, TimbreResponse, CancelacionResponse};
use crate::error::CfdiError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose::STANDARD as B64, Engine};

pub struct FacturamaPac {
    client: Client,
    base_url: String,
    api_user: String,
    api_secret: String,
}

impl FacturamaPac {
    /// Crear instancia de producción
    pub fn produccion(api_user: String, api_secret: String) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.facturama.mx".into(),
            api_user,
            api_secret,
        }
    }

    /// Crear instancia de sandbox (pruebas)
    pub fn sandbox(api_user: String, api_secret: String) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://apisandbox.facturama.mx".into(),
            api_user,
            api_secret,
        }
    }

    fn auth_header(&self) -> String {
        let credenciales = format!("{}:{}", self.api_user, self.api_secret);
        format!("Basic {}", B64.encode(credenciales.as_bytes()))
    }
}

#[derive(Serialize)]
struct TimbreRequest {
    #[serde(rename = "Content")]
    content: String,
}

#[derive(Deserialize)]
struct TimbreRaw {
    #[serde(rename = "NoCertificadoSAT")]
    no_certificado_sat: Option<String>,
    #[serde(rename = "NoCertificadoCFDI")]
    _no_certificado_cfdi: Option<String>,
    #[serde(rename = "UUID")]
    uuid: Option<String>,
    #[serde(rename = "SelloSAT")]
    sello_sat: Option<String>,
    #[serde(rename = "SelloCFDI")]
    _sello_cfdi: Option<String>,
    #[serde(rename = "FechaTimbrado")]
    fecha_timbrado: Option<String>,
    #[serde(rename = "RfcProvCertif")]
    rfc_prov_certif: Option<String>,
    #[serde(rename = "Data")]
    data: Option<String>,    // XML timbrado en Base64
    #[serde(rename = "Warnings")]
    warnings: Option<Vec<String>>,
}

#[async_trait::async_trait]
impl Pac for FacturamaPac {
    async fn timbrar(&self, xml_sellado: &str) -> Result<TimbreResponse, CfdiError> {
        // Facturama espera el XML en Base64
        let xml_b64 = B64.encode(xml_sellado.as_bytes());

        let url = format!("{}/api-lite/3/cfdis", self.base_url);
        let resp = self.client
            .post(&url)
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json")
            .json(&TimbreRequest { content: xml_b64 })
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(CfdiError::PacRejected {
                codigo: status.as_str().into(),
                mensaje: body,
            });
        }

        let raw: TimbreRaw = resp.json().await
            .map_err(|e| CfdiError::Pac(format!("Respuesta inválida de Facturama: {}", e)))?;

        // Decodificar XML timbrado de Base64
        let xml_b64_resp = raw.data.ok_or_else(|| CfdiError::Pac("Facturama no retornó XML".into()))?;
        let xml_bytes = B64.decode(&xml_b64_resp)
            .map_err(|e| CfdiError::Pac(format!("Base64 inválido en respuesta: {}", e)))?;
        let xml_timbrado = String::from_utf8(xml_bytes)
            .map_err(|e| CfdiError::Pac(format!("XML no es UTF-8: {}", e)))?;

        Ok(TimbreResponse {
            uuid: raw.uuid.unwrap_or_default(),
            xml_timbrado,
            fecha_timbrado: raw.fecha_timbrado.unwrap_or_default(),
            rfc_prov_certif: raw.rfc_prov_certif.unwrap_or_default(),
            sello_sat: raw.sello_sat.unwrap_or_default(),
            no_certificado_sat: raw.no_certificado_sat.unwrap_or_default(),
        })
    }

    async fn cancelar(
        &self,
        rfc_emisor: &str,
        uuid: &str,
        motivo: &str,
        uuid_relacionado: Option<&str>,
    ) -> Result<CancelacionResponse, CfdiError> {
        let mut url = format!(
            "{}/api-lite/3/cfdis/{}/{}/{}",
            self.base_url, rfc_emisor, uuid, motivo
        );
        if let Some(rel) = uuid_relacionado {
            url = format!("{}?uuidRelacionado={}", url, rel);
        }

        let resp = self.client
            .delete(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(CfdiError::PacRejected {
                codigo: status.as_str().into(),
                mensaje: body,
            });
        }

        Ok(CancelacionResponse {
            uuid: uuid.into(),
            estado_cancelacion: status.as_str().into(),
            acuse: resp.text().await.ok(),
        })
    }
}
