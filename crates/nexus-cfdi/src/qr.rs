//! Generador de URL de verificación SAT y QR para representación impresa CFDI
//!
//! El SAT requiere que la representación impresa incluya un código QR con la URL
//! de verificación del CFDI.
//!
//! URL format:
//! https://verificacfdi.facturaelectronica.sat.gob.mx/default.aspx
//!   ?id={UUID}
//!   &re={RFC_EMISOR}
//!   &rr={RFC_RECEPTOR}
//!   &tt={TOTAL_CON_8_DECIMALES}
//!   &fe={ULTIMOS_8_CHARS_SELLO}
//!
//! Ejemplo:
//! https://verificacfdi.facturaelectronica.sat.gob.mx/default.aspx?id=550e8400-e29b-41d4-a716-446655440000&re=IBS120101AA1&rr=XAXX010101000&tt=116.00000000&fe=AbCdEfGh

use crate::error::CfdiError;
use rust_decimal::Decimal;

/// URL base de verificación SAT
pub const SAT_VERIFICA_URL: &str =
    "https://verificacfdi.facturaelectronica.sat.gob.mx/default.aspx";

/// Datos necesarios para generar el QR del CFDI
#[derive(Debug, Clone)]
pub struct DatosQr {
    pub uuid: String,
    pub rfc_emisor: String,
    pub rfc_receptor: String,
    pub total: Decimal,
    pub sello: String, // sello digital del emisor (completo)
}

/// Genera la URL de verificación del SAT para el CFDI
///
/// Esta URL se incluye en el código QR de la representación impresa.
pub fn generar_url_verificacion(datos: &DatosQr) -> Result<String, CfdiError> {
    // Validar UUID
    if datos.uuid.len() != 36 || datos.uuid.chars().filter(|c| *c == '-').count() != 4 {
        return Err(CfdiError::CampoRequerido(
            format!("UUID inválido para QR: '{}'", datos.uuid)
        ));
    }

    // Los últimos 8 caracteres del sello (fe = firma electrónica snippet)
    let fe = if datos.sello.len() >= 8 {
        &datos.sello[datos.sello.len() - 8..]
    } else {
        &datos.sello
    };

    // Total con 8 decimales (requisito SAT)
    let total_str = format!("{:.8}", datos.total);

    let url = format!(
        "{}?id={}&re={}&rr={}&tt={}&fe={}",
        SAT_VERIFICA_URL,
        datos.uuid,
        datos.rfc_emisor,
        datos.rfc_receptor,
        total_str,
        fe,
    );

    Ok(url)
}

/// Genera el QR como SVG string (sin dependencias externas)
///
/// Usa la crate `qrcode` para generar la imagen QR.
/// El SVG se puede embeber directamente en la representación impresa.
#[cfg(feature = "qr")]
pub fn generar_qr_svg(datos: &DatosQr) -> Result<String, CfdiError> {
    use qrcode::QrCode;
    use qrcode::render::svg;

    let url = generar_url_verificacion(datos)?;
    let code = QrCode::new(url.as_bytes())
        .map_err(|e| CfdiError::Xml(format!("Error QR: {}", e)))?;

    let svg = code.render::<svg::Color>()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();

    Ok(svg)
}

/// Genera el QR como imagen PNG en bytes (sin dependencias externas)
#[cfg(feature = "qr")]
pub fn generar_qr_png(datos: &DatosQr) -> Result<Vec<u8>, CfdiError> {
    use qrcode::QrCode;
    use qrcode::render::unicode;
    use image::Luma;

    let url = generar_url_verificacion(datos)?;
    let code = QrCode::new(url.as_bytes())
        .map_err(|e| CfdiError::Xml(format!("Error QR: {}", e)))?;

    let image = code.render::<Luma<u8>>()
        .min_dimensions(256, 256)
        .build();

    let mut png_bytes = Vec::new();
    image.write_to(
        &mut std::io::Cursor::new(&mut png_bytes),
        image::ImageFormat::Png,
    ).map_err(|e| CfdiError::Xml(format!("Error PNG: {}", e)))?;

    Ok(png_bytes)
}

/// Genera representación ASCII del QR (útil para debug, no requiere dependencias)
pub fn generar_qr_texto(datos: &DatosQr) -> Result<String, CfdiError> {
    let url = generar_url_verificacion(datos)?;
    // Retorna la URL como texto (el QR real se genera con feature "qr")
    Ok(format!("[QR: {}]", url))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    fn datos_prueba() -> DatosQr {
        DatosQr {
            uuid: "550e8400-e29b-41d4-a716-446655440000".into(),
            rfc_emisor: "IBS120101AA1".into(),
            rfc_receptor: "XAXX010101000".into(),
            total: Decimal::from_str("116.00").unwrap(),
            sello: "ZHAKBDejFGHIJKLMNOPQRSTUVWXYZAbCdEfGh".into(),
        }
    }

    #[test]
    fn test_url_verificacion_formato() {
        let datos = datos_prueba();
        let url = generar_url_verificacion(&datos).unwrap();

        assert!(url.starts_with("https://verificacfdi.facturaelectronica.sat.gob.mx"));
        assert!(url.contains("id=550e8400-e29b-41d4-a716-446655440000"));
        assert!(url.contains("re=IBS120101AA1"));
        assert!(url.contains("rr=XAXX010101000"));
        assert!(url.contains("tt=116.00000000"));
        assert!(url.contains("fe=AbCdEfGh"));
        println!("URL SAT: {}", url);
    }

    #[test]
    fn test_uuid_invalido() {
        let mut datos = datos_prueba();
        datos.uuid = "invalido".into();
        assert!(generar_url_verificacion(&datos).is_err());
    }

    #[test]
    fn test_qr_texto() {
        let datos = datos_prueba();
        let qr = generar_qr_texto(&datos).unwrap();
        assert!(qr.contains("[QR:"));
        assert!(qr.contains("verificacfdi"));
    }
}
