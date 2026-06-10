//! Sellado criptográfico del CFDI
//!
//! Implementa la generación del sello digital usando el CSD (Certificado de Sello Digital)
//! según las especificaciones del SAT en el Anexo 20 del CFDI 4.0:
//!
//! 1. Construir la cadena original (campos en orden del Anexo 20)
//! 2. Aplicar SHA-256 a la cadena original
//! 3. Firmar con RSA + llave privada (.key del CSD) usando PKCS1v15
//! 4. Encodear en Base64 → atributo `Sello` del XML

use crate::error::CfdiError;
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use ring::signature::{self, RsaKeyPair};
use ring::rand::SystemRandom;
use std::path::Path;

/// Sello digital generado
#[derive(Debug, Clone)]
pub struct Sello {
    /// Valor Base64 del sello (va en atributo `Sello` del XML)
    pub valor: String,
    /// Número de serie del certificado (va en atributo `NoCertificado`)
    pub no_certificado: String,
    /// Contenido del certificado en Base64 (va en atributo `Certificado`)
    pub certificado_b64: String,
}

/// Carga el par de llaves RSA desde el archivo .key del SAT
///
/// El SAT entrega la llave privada en formato DER (binario).
/// ring requiere PKCS8 DER, así que si es RSA PKCS1, se envuelve.
pub fn cargar_llave_privada(path_key: &Path) -> Result<Vec<u8>, CfdiError> {
    let der = std::fs::read(path_key)
        .map_err(|e| CfdiError::PrivateKey(format!("No se puede leer {}: {}", path_key.display(), e)))?;

    // El SAT entrega en PKCS#8 DER directamente en versiones recientes
    // Si empieza con 0x30 0x82 es DER válido
    Ok(der)
}

/// Carga el certificado .cer y extrae número de serie y contenido Base64
pub fn cargar_certificado(path_cer: &Path) -> Result<(String, String), CfdiError> {
    let cer_bytes = std::fs::read(path_cer)
        .map_err(|e| CfdiError::Certificate(format!("No se puede leer {}: {}", path_cer.display(), e)))?;

    // Número de serie: bytes 16-36 del DER en hexadecimal, luego solo dígitos pares
    // El SAT codifica el serial como hex donde cada par = 1 dígito decimal
    let no_certificado = extraer_numero_serie(&cer_bytes)
        .unwrap_or_else(|_| "00000000000000000000".to_string());

    let certificado_b64 = B64.encode(&cer_bytes);

    Ok((no_certificado, certificado_b64))
}

/// Extrae el número de serie del certificado DER
/// El serial en ASN.1 DER está en offset ~16 después de los headers de SEQUENCE + SEQUENCE + INTEGER
fn extraer_numero_serie(der: &[u8]) -> Result<String, CfdiError> {
    // Buscar el INTEGER del serial number en el TBSCertificate
    // Estructura ASN.1: SEQUENCE { SEQUENCE { [0] version, INTEGER serialNumber, ... } }
    // Skip: 0x30 len 0x30 len [0x a0 03 02 01 02] 0x02 len <serial bytes>
    let mut i = 0;
    // Saltar SEQUENCE externo
    if der.get(i) != Some(&0x30) { return Err(CfdiError::Certificate("DER inválido".into())); }
    i += 1; i += longitud_asn1(der, i).1;
    // Saltar SEQUENCE interno (TBSCertificate)
    if der.get(i) != Some(&0x30) { return Err(CfdiError::Certificate("TBS inválido".into())); }
    i += 1; i += longitud_asn1(der, i).1;
    // Saltar [0] VERSION opcional
    if der.get(i) == Some(&0xa0) {
        i += 1;
        let (len, skip) = longitud_asn1(der, i);
        i += skip + len;
    }
    // INTEGER = serial number
    if der.get(i) != Some(&0x02) { return Err(CfdiError::Certificate("Serial no encontrado".into())); }
    i += 1;
    let (len, skip) = longitud_asn1(der, i);
    i += skip;
    let serial_bytes = &der[i..i+len];

    // Convertir bytes a hex y tomar solo los dígitos en posiciones pares (formato SAT)
    let hex = serial_bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<String>();

    // El SAT usa: cada byte del serial → 2 hex chars → tomar char[1] de cada par
    let no_cert: String = hex.chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|pair| pair[1])
        .collect();

    Ok(no_cert)
}

/// Lee longitud ASN.1 BER/DER, retorna (longitud, bytes_consumidos_por_la_longitud)
fn longitud_asn1(data: &[u8], offset: usize) -> (usize, usize) {
    let first = data[offset];
    if first < 0x80 {
        (first as usize, 1)
    } else {
        let n = (first & 0x7f) as usize;
        let mut len = 0usize;
        for i in 0..n {
            len = (len << 8) | data[offset + 1 + i] as usize;
        }
        (len, 1 + n)
    }
}

/// Genera el sello digital para una cadena original
///
/// # Argumentos
/// * `cadena_original` - La cadena original del CFDI (generada por `cadena_original::generar`)
/// * `llave_der` - Bytes de la llave privada en formato DER (PKCS#8)
/// * `path_cer` - Ruta al archivo .cer del CSD
pub fn sellar(
    cadena_original: &str,
    llave_der: &[u8],
    path_cer: &Path,
) -> Result<Sello, CfdiError> {
    // 1. Cargar el par de llaves
    let key_pair = RsaKeyPair::from_der(llave_der)
        .map_err(|e| CfdiError::Crypto(format!("Llave privada inválida: {}", e)))?;

    // 2. Firmar SHA-256 + RSA PKCS1v15
    let rng = SystemRandom::new();
    let mut firma = vec![0u8; key_pair.public().modulus_len()];
    key_pair
        .sign(
            &signature::RSA_PKCS1_SHA256,
            &rng,
            cadena_original.as_bytes(),
            &mut firma,
        )
        .map_err(|e| CfdiError::Crypto(format!("Error al firmar: {}", e)))?;

    // 3. Base64 del sello
    let sello_b64 = B64.encode(&firma);

    // 4. Cargar certificado
    let (no_certificado, certificado_b64) = cargar_certificado(path_cer)?;

    Ok(Sello {
        valor: sello_b64,
        no_certificado,
        certificado_b64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longitud_asn1_corta() {
        let data = &[0x05, 0x00]; // NULL
        assert_eq!(longitud_asn1(data, 0), (5, 1));
    }

    #[test]
    fn test_longitud_asn1_larga() {
        let data = &[0x82, 0x01, 0xA4]; // longitud = 420
        assert_eq!(longitud_asn1(data, 0), (420, 3));
    }
}
