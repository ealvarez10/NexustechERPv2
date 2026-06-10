//! PAC — Proveedores Autorizados de Certificación
//!
//! Integración con PACs mexicanos para timbrar y cancelar CFDIs.
//! Soporta: Facturama, SW Sapien (Software del Rey)

pub mod facturama;
pub mod sw_sapien;

use crate::error::CfdiError;
use serde::{Deserialize, Serialize};

/// Respuesta del timbrado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimbreResponse {
    /// UUID asignado por el SAT (Timbre Fiscal Digital)
    pub uuid: String,
    /// XML timbrado completo con el TFD
    pub xml_timbrado: String,
    /// Fecha de timbrado
    pub fecha_timbrado: String,
    /// RFC del proveedor de certificación (PAC)
    pub rfc_prov_certif: String,
    /// Sello del SAT
    pub sello_sat: String,
    /// No. de certificado del SAT
    pub no_certificado_sat: String,
}

/// Respuesta de cancelación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelacionResponse {
    pub uuid: String,
    pub estado_cancelacion: String, // 201=cancelado, 202=pendiente aceptación
    pub acuse: Option<String>,
}

/// Trait que deben implementar todos los PACs
#[async_trait::async_trait]
pub trait Pac: Send + Sync {
    /// Timbra un XML sellado y retorna el XML con TFD
    async fn timbrar(&self, xml_sellado: &str) -> Result<TimbreResponse, CfdiError>;

    /// Cancela un CFDI por UUID
    async fn cancelar(
        &self,
        rfc_emisor: &str,
        uuid: &str,
        motivo: &str,               // 01=comprobante emitido con errores con relación
        uuid_relacionado: Option<&str>,
    ) -> Result<CancelacionResponse, CfdiError>;
}
