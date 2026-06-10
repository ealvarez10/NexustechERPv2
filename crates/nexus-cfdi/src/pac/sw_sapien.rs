//! SW Sapien — Software del Rey PAC
//!
//! Integración con SW Sapien para timbrado con CSD propio (token-based).
//! Documentación: https://developers.sw.com.mx

use super::{Pac, TimbreResponse, CancelacionResponse};
use crate::error::CfdiError;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct SwSapienPac {
    client: Client,
    base_url: String,
    token: String,
}

impl SwSapienPac {
    pub fn produccion(token: String) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://services.sw.com.mx".into(),
            token,
        }
    }

    pub fn sandbox(token: String) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://services.test.sw.com.mx".into(),
            token,
        }
    }
}

#[derive(Deserialize)]
struct SwResponse {
    status: String,
    message: Option<String>,
    data: Option<SwData>,
}

#[derive(Deserialize)]
struct SwData {
    uuid: Option<String>,
    #[serde(rename = "cfdi")]
    xml_timbrado: Option<String>,
    #[serde(rename = "fechaTimbrado")]
    fecha_timbrado: Option<String>,
    #[serde(rename = "noCertificadoSat")]
    no_certificado_sat: Option<String>,
    #[serde(rename = "rfcProvCertif")]
    rfc_prov_certif: Option<String>,
    #[serde(rename = "selloSat")]
    sello_sat: Option<String>,
}

#[async_trait::async_trait]
impl Pac for SwSapienPac {
    async fn timbrar(&self, xml_sellado: &str) -> Result<TimbreResponse, CfdiError> {
        let url = format!("{}/cfdi33/stamp/xml/v4", self.base_url);

        let resp = self.client
            .post(&url)
            .header("Authorization", format!("bearer {}", self.token))
            .header("Content-Type", "application/xml")
            .body(xml_sellado.to_string())
            .send()
            .await?;

        let status = resp.status();
        let sw_resp: SwResponse = resp.json().await
            .map_err(|e| CfdiError::Pac(format!("Respuesta SW inválida: {}", e)))?;

        if sw_resp.status != "success" {
            return Err(CfdiError::PacRejected {
                codigo: status.as_str().into(),
                mensaje: sw_resp.message.unwrap_or_else(|| "Error desconocido SW".into()),
            });
        }

        let data = sw_resp.data.ok_or_else(|| CfdiError::Pac("SW no retornó data".into()))?;
        Ok(TimbreResponse {
            uuid: data.uuid.unwrap_or_default(),
            xml_timbrado: data.xml_timbrado.unwrap_or_default(),
            fecha_timbrado: data.fecha_timbrado.unwrap_or_default(),
            rfc_prov_certif: data.rfc_prov_certif.unwrap_or_default(),
            sello_sat: data.sello_sat.unwrap_or_default(),
            no_certificado_sat: data.no_certificado_sat.unwrap_or_default(),
        })
    }

    async fn cancelar(
        &self,
        rfc_emisor: &str,
        uuid: &str,
        motivo: &str,
        uuid_relacionado: Option<&str>,
    ) -> Result<CancelacionResponse, CfdiError> {
        #[derive(Serialize)]
        struct CancelReq<'a> {
            rfc: &'a str,
            uuid: &'a str,
            motivo: &'a str,
            #[serde(rename = "folioSustitucion", skip_serializing_if = "Option::is_none")]
            folio_sustitucion: Option<&'a str>,
        }

        let url = format!("{}/cfdi33/cancel/xml", self.base_url);
        let body = CancelReq {
            rfc: rfc_emisor,
            uuid,
            motivo,
            folio_sustitucion: uuid_relacionado,
        };

        let resp = self.client
            .post(&url)
            .header("Authorization", format!("bearer {}", self.token))
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            return Err(CfdiError::PacRejected {
                codigo: status.as_str().into(),
                mensaje: resp.text().await.unwrap_or_default(),
            });
        }

        Ok(CancelacionResponse {
            uuid: uuid.into(),
            estado_cancelacion: "201".into(),
            acuse: resp.text().await.ok(),
        })
    }
}
