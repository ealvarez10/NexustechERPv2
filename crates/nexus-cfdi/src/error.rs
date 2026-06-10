//! Tipos de error de nexus-cfdi

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CfdiError {
    #[error("Error de criptografía: {0}")]
    Crypto(String),

    #[error("Error al leer el certificado CSD: {0}")]
    Certificate(String),

    #[error("Error al leer la clave privada: {0}")]
    PrivateKey(String),

    #[error("Error al generar la cadena original: {0}")]
    CadenaOriginal(String),

    #[error("Error al generar el XML: {0}")]
    Xml(String),

    #[error("Error al comunicarse con el PAC: {0}")]
    Pac(String),

    #[error("El PAC rechazó el CFDI: código={codigo}, mensaje={mensaje}")]
    PacRejected { codigo: String, mensaje: String },

    #[error("Campo requerido faltante: {0}")]
    CampoRequerido(String),

    #[error("RFC inválido: {0}")]
    RfcInvalido(String),

    #[error("Error de I/O: {0}")]
    Io(#[from] std::io::Error),

    #[error("Error HTTP: {0}")]
    Http(#[from] reqwest::Error),
}
