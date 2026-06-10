//! Representación Impresa de CFDI 4.0
//!
//! Genera el PDF de la representación impresa del comprobante fiscal digital
//! conforme a los requisitos del Artículo 29-A del CFF y el Anexo 20 del SAT.
//!
//! Secciones del PDF:
//! 1. Encabezado: logo, datos del emisor, folios
//! 2. Datos del receptor
//! 3. Tabla de conceptos (bienes/servicios)
//! 4. Resumen de impuestos + totales
//! 5. Información fiscal: UUID, cadena original, sello
//! 6. Código QR de verificación SAT
//! 7. Leyenda legal

use printpdf::*;
use std::io::BufWriter;
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::{
    builder::{CfdiData, Concepto},
    error::CfdiError,
    qr::generar_url_verificacion,
};

/// Resultado de la generación de PDF
pub struct PdfCfdi {
    /// Bytes del PDF generado
    pub bytes: Vec<u8>,
    /// Nombre sugerido para el archivo
    pub nombre_archivo: String,
}

/// Opciones para la generación del PDF
#[derive(Debug, Clone)]
pub struct OpcionesPdf {
    /// Logo de la empresa en bytes PNG/JPEG (opcional)
    pub logo_bytes: Option<Vec<u8>>,
    /// Ancho de la página en mm (default: A4 = 210)
    pub ancho_mm: f32,
    /// Alto de la página en mm (default: A4 = 297)
    pub alto_mm: f32,
    /// Mostrar cadena original (puede ser muy larga)
    pub mostrar_cadena_original: bool,
}

impl Default for OpcionesPdf {
    fn default() -> Self {
        Self {
            logo_bytes: None,
            ancho_mm: 210.0,
            alto_mm: 297.0,
            mostrar_cadena_original: false,
        }
    }
}

/// Genera el PDF de representación impresa del CFDI
pub fn generar_pdf(
    cfdi: &CfdiData,
    uuid: &str,
    sello_emisor: &str,
    sello_sat: &str,
    cadena_original: &str,
    cert_numero: &str,
    opciones: &OpcionesPdf,
) -> Result<PdfCfdi, CfdiError> {
    let (doc, page1, layer1) = PdfDocument::new(
        format!("CFDI {} - {}", cfdi.serie.as_deref().unwrap_or(""), cfdi.folio.as_deref().unwrap_or("")),
        Mm(opciones.ancho_mm),
        Mm(opciones.alto_mm),
        "Capa 1",
    );

    let page = doc.get_page(page1);
    let layer = page.get_layer(layer1);

    // Fuentes
    let fuente_regular = doc.add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| CfdiError::Pdf(format!("Error fuente: {}", e)))?;
    let fuente_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| CfdiError::Pdf(format!("Error fuente bold: {}", e)))?;
    let fuente_mono = doc.add_builtin_font(BuiltinFont::Courier)
        .map_err(|e| CfdiError::Pdf(format!("Error fuente mono: {}", e)))?;

    // Márgenes
    let margen_izq = 15.0_f64;
    let ancho_util = (opciones.ancho_mm as f64) - 30.0;
    let mut y = (opciones.alto_mm as f64) - 15.0; // Cursor vertical

    // ── ENCABEZADO ────────────────────────────────────────────────────────────
    // Título del documento
    let tipo_label = match cfdi.tipo_comprobante.as_str() {
        "I" => "FACTURA",
        "E" => "NOTA DE CRÉDITO",
        "P" => "COMPLEMENTO DE PAGO",
        "N" => "NÓMINA",
        "T" => "TRASLADO",
        _ => "COMPROBANTE FISCAL",
    };

    capa_texto(
        &layer,
        &fuente_bold,
        14.0,
        margen_izq,
        y,
        tipo_label,
    );

    // Folio y serie a la derecha
    let folio_txt = format!(
        "Serie: {}  Folio: {}",
        cfdi.serie.as_deref().unwrap_or("—"),
        cfdi.folio.as_deref().unwrap_or("—"),
    );
    capa_texto_derecha(
        &layer,
        &fuente_bold,
        10.0,
        opciones.ancho_mm as f64 - margen_izq,
        y,
        &folio_txt,
    );
    y -= 6.0;

    // Línea separadora
    linea_horizontal(&layer, margen_izq, y, ancho_util);
    y -= 8.0;

    // ── DATOS EMISOR ──────────────────────────────────────────────────────────
    capa_texto(&layer, &fuente_bold, 9.0, margen_izq, y, "EMISOR");
    y -= 5.0;
    capa_texto(&layer, &fuente_bold, 10.0, margen_izq, y, &cfdi.emisor.nombre);
    y -= 5.0;
    capa_texto(
        &layer,
        &fuente_regular,
        8.5,
        margen_izq,
        y,
        &format!("RFC: {}", cfdi.emisor.rfc),
    );
    y -= 4.5;
    capa_texto(
        &layer,
        &fuente_regular,
        8.5,
        margen_izq,
        y,
        &format!("Régimen Fiscal: {}", cfdi.emisor.regimen_fiscal),
    );
    y -= 8.0;

    // ── DATOS RECEPTOR ────────────────────────────────────────────────────────
    capa_texto(&layer, &fuente_bold, 9.0, margen_izq, y, "RECEPTOR");
    y -= 5.0;
    capa_texto(&layer, &fuente_bold, 10.0, margen_izq, y, &cfdi.receptor.nombre);
    y -= 5.0;
    capa_texto(
        &layer,
        &fuente_regular,
        8.5,
        margen_izq,
        y,
        &format!("RFC: {}    CP: {}    Uso CFDI: {}",
            cfdi.receptor.rfc,
            cfdi.receptor.domicilio_fiscal_receptor,
            cfdi.receptor.uso_cfdi,
        ),
    );
    y -= 4.5;
    capa_texto(
        &layer,
        &fuente_regular,
        8.5,
        margen_izq,
        y,
        &format!("Régimen Fiscal Receptor: {}", cfdi.receptor.regimen_fiscal_receptor),
    );
    y -= 8.0;

    // ── DATOS DEL COMPROBANTE ─────────────────────────────────────────────────
    linea_horizontal(&layer, margen_izq, y, ancho_util);
    y -= 6.0;

    let col2 = margen_izq + ancho_util / 2.0;

    capa_texto(
        &layer, &fuente_regular, 8.0, margen_izq, y,
        &format!("Fecha de expedición: {}", cfdi.fecha),
    );
    capa_texto(
        &layer, &fuente_regular, 8.0, col2, y,
        &format!("Forma de pago: {}", cfdi.forma_pago.as_deref().unwrap_or("—")),
    );
    y -= 5.0;

    capa_texto(
        &layer, &fuente_regular, 8.0, margen_izq, y,
        &format!("Método de pago: {}", cfdi.metodo_pago.as_deref().unwrap_or("—")),
    );
    capa_texto(
        &layer, &fuente_regular, 8.0, col2, y,
        &format!("Moneda: {}  TC: {}",
            cfdi.moneda,
            cfdi.tipo_cambio.as_deref().unwrap_or("1"),
        ),
    );
    y -= 5.0;

    capa_texto(
        &layer, &fuente_regular, 8.0, margen_izq, y,
        &format!("Exportación: {}", cfdi.exportacion),
    );
    y -= 8.0;

    // ── TABLA DE CONCEPTOS ────────────────────────────────────────────────────
    linea_horizontal(&layer, margen_izq, y, ancho_util);
    y -= 5.0;

    // Encabezados de tabla
    let col_clave    = margen_izq;
    let col_desc     = margen_izq + 18.0;
    let col_cant     = margen_izq + ancho_util * 0.55;
    let col_unidad   = margen_izq + ancho_util * 0.62;
    let col_pu       = margen_izq + ancho_util * 0.73;
    let col_importe  = margen_izq + ancho_util * 0.86;

    capa_texto(&layer, &fuente_bold, 8.0, col_clave,   y, "ClaveProdServ");
    capa_texto(&layer, &fuente_bold, 8.0, col_desc,    y, "Descripción");
    capa_texto(&layer, &fuente_bold, 8.0, col_cant,    y, "Cant.");
    capa_texto(&layer, &fuente_bold, 8.0, col_unidad,  y, "Unidad");
    capa_texto(&layer, &fuente_bold, 8.0, col_pu,      y, "P.Unitario");
    capa_texto(&layer, &fuente_bold, 8.0, col_importe, y, "Importe");
    y -= 2.0;
    linea_horizontal(&layer, margen_izq, y, ancho_util);
    y -= 5.0;

    // Filas de conceptos
    for concepto in &cfdi.conceptos {
        if y < 40.0 {
            // TODO: soporte multi-página (por ahora truncamos)
            break;
        }

        let desc = truncar(&concepto.descripcion, 45);
        capa_texto(&layer, &fuente_regular, 7.5, col_clave,   y, &concepto.clave_prod_serv);
        capa_texto(&layer, &fuente_regular, 7.5, col_desc,    y, &desc);
        capa_texto(&layer, &fuente_regular, 7.5, col_cant,    y, &formato_decimal(concepto.cantidad));
        capa_texto(&layer, &fuente_regular, 7.5, col_unidad,  y, &concepto.clave_unidad);
        capa_texto(&layer, &fuente_regular, 7.5, col_pu,      y, &formato_monto(concepto.valor_unitario));
        capa_texto(&layer, &fuente_regular, 7.5, col_importe, y, &formato_monto(concepto.importe));
        y -= 5.0;

        // Descuento si aplica
        if let Some(desc_monto) = &concepto.descuento {
            if *desc_monto > Decimal::ZERO {
                capa_texto(&layer, &fuente_regular, 7.0, col_desc, y,
                    &format!("  Descuento: -{}", formato_monto(*desc_monto)));
                y -= 4.5;
            }
        }
    }

    linea_horizontal(&layer, margen_izq, y, ancho_util);
    y -= 8.0;

    // ── TOTALES ───────────────────────────────────────────────────────────────
    let x_etiq  = margen_izq + ancho_util * 0.65;
    let x_valor = margen_izq + ancho_util * 0.86;

    capa_texto(&layer, &fuente_regular, 8.5, x_etiq, y, "Subtotal:");
    capa_texto(&layer, &fuente_regular, 8.5, x_valor, y, &formato_monto(cfdi.sub_total));
    y -= 5.5;

    if let Some(desc) = cfdi.descuento {
        if desc > Decimal::ZERO {
            capa_texto(&layer, &fuente_regular, 8.5, x_etiq, y, "Descuento:");
            capa_texto(&layer, &fuente_regular, 8.5, x_valor, y, &format!("-{}", formato_monto(desc)));
            y -= 5.5;
        }
    }

    // Impuestos trasladados
    if let Some(imptos) = &cfdi.impuestos {
        if let Some(traslados) = &imptos.traslados {
            for t in traslados {
                let etiq = if t.impuesto == "002" { "IVA" }
                           else if t.impuesto == "003" { "IEPS" }
                           else { "ISR" };
                let pct = (t.tasa_o_cuota * Decimal::from(100)).round_dp(0);
                capa_texto(&layer, &fuente_regular, 8.5, x_etiq, y,
                    &format!("{} {}%:", etiq, pct));
                capa_texto(&layer, &fuente_regular, 8.5, x_valor, y,
                    &formato_monto(t.importe.unwrap_or(Decimal::ZERO)));
                y -= 5.5;
            }
        }
        if let Some(retenciones) = &imptos.retenciones {
            for r in retenciones {
                let etiq = if r.impuesto == "001" { "Ret. ISR" } else { "Ret. IVA" };
                capa_texto(&layer, &fuente_regular, 8.5, x_etiq, y,
                    &format!("{}:", etiq));
                capa_texto(&layer, &fuente_regular, 8.5, x_valor, y,
                    &format!("-{}", formato_monto(r.importe)));
                y -= 5.5;
            }
        }
    }

    // Total
    linea_horizontal(&layer, x_etiq - 2.0, y + 1.0, ancho_util - (x_etiq - margen_izq) + 2.0);
    y -= 1.0;
    capa_texto(&layer, &fuente_bold, 10.0, x_etiq, y, "TOTAL:");
    capa_texto(&layer, &fuente_bold, 10.0, x_valor, y,
        &format!("{} {}", cfdi.moneda, formato_monto(cfdi.total)));
    y -= 8.0;

    // Importe con letra (simplificado)
    capa_texto(&layer, &fuente_regular, 7.5, margen_izq, y,
        &format!("Importe con letra: {}", importe_con_letra(cfdi.total, &cfdi.moneda)));
    y -= 12.0;

    // ── INFORMACIÓN FISCAL SAT ────────────────────────────────────────────────
    linea_horizontal(&layer, margen_izq, y, ancho_util);
    y -= 5.0;

    capa_texto(&layer, &fuente_bold, 8.5, margen_izq, y, "INFORMACIÓN FISCAL");
    y -= 5.5;

    capa_texto(&layer, &fuente_regular, 7.5, margen_izq, y,
        &format!("Folio fiscal (UUID): {}", uuid));
    y -= 5.0;

    capa_texto(&layer, &fuente_regular, 7.5, margen_izq, y,
        &format!("Fecha y hora de certificación: {}", cfdi.fecha));
    y -= 4.5;

    capa_texto(&layer, &fuente_regular, 7.5, margen_izq, y,
        &format!("No. Certificado emisor: {}    No. Certificado SAT: {}", cert_numero, cert_numero));
    y -= 8.0;

    // Sello del emisor (truncado a 60 chars visible)
    capa_texto(&layer, &fuente_bold, 7.0, margen_izq, y, "Sello digital del emisor:");
    y -= 4.0;
    capa_texto(&layer, &fuente_mono, 6.5, margen_izq, y,
        &truncar(sello_emisor, 100));
    if sello_emisor.len() > 100 {
        y -= 4.0;
        capa_texto(&layer, &fuente_mono, 6.5, margen_izq, y,
            &truncar(&sello_emisor[100..], 100));
    }
    y -= 8.0;

    // Sello del SAT
    capa_texto(&layer, &fuente_bold, 7.0, margen_izq, y, "Sello del SAT:");
    y -= 4.0;
    capa_texto(&layer, &fuente_mono, 6.5, margen_izq, y,
        &truncar(sello_sat, 100));
    if sello_sat.len() > 100 {
        y -= 4.0;
        capa_texto(&layer, &fuente_mono, 6.5, margen_izq, y,
            &truncar(&sello_sat[100..], 100));
    }
    y -= 8.0;

    // Cadena original (opcional)
    if opciones.mostrar_cadena_original && !cadena_original.is_empty() {
        capa_texto(&layer, &fuente_bold, 7.0, margen_izq, y, "Cadena original:");
        y -= 4.0;
        capa_texto(&layer, &fuente_mono, 5.5, margen_izq, y,
            &truncar(cadena_original, 130));
        y -= 8.0;
    }

    // ── QR DE VERIFICACIÓN SAT ────────────────────────────────────────────────
    // Generamos el QR y lo embebemos
    let url_sat = generar_url_verificacion(
        &cfdi.emisor.rfc,
        &cfdi.receptor.rfc,
        cfdi.total,
        uuid,
    );

    // Intentar generar QR — si falla, solo ponemos la URL
    match generar_qr_bytes(&url_sat) {
        Ok(qr_bytes) => {
            if let Ok(img) = Image::from_dynamic_image(&image::load_from_memory(&qr_bytes)
                .map_err(|_| ())
                .and_then(|i| Ok(i.to_rgba8()))
                .map(|i| image::DynamicImage::ImageRgba8(i))
                .unwrap_or_else(|_| image::DynamicImage::new_rgba8(1, 1)))
            {
                let qr_x = Mm(opciones.ancho_mm - 50.0);
                let qr_y = Mm(y as f32 - 30.0);
                img.add_to_layer(layer.clone(), ImageTransform {
                    translate_x: Some(qr_x),
                    translate_y: Some(qr_y),
                    scale_x: Some(0.12),
                    scale_y: Some(0.12),
                    ..Default::default()
                });
            }
        }
        Err(_) => {
            // Fallback: mostrar URL textual
            capa_texto(&layer, &fuente_bold, 7.0, margen_izq, y, "Verificar en:");
            y -= 4.0;
            capa_texto(&layer, &fuente_mono, 6.0, margen_izq, y, &url_sat);
        }
    }

    // ── PIE DE PÁGINA ─────────────────────────────────────────────────────────
    let pie_y = 10.0;
    linea_horizontal(&layer, margen_izq, pie_y, ancho_util);
    capa_texto(&layer, &fuente_regular, 6.5, margen_izq, pie_y - 4.0,
        "Este documento es una representación impresa de un CFDI.");

    // ── EXPORTAR PDF ──────────────────────────────────────────────────────────
    let mut buf = BufWriter::new(Vec::new());
    doc.save(&mut buf)
        .map_err(|e| CfdiError::Pdf(format!("Error al exportar PDF: {}", e)))?;

    let bytes = buf.into_inner()
        .map_err(|e| CfdiError::Pdf(format!("Error al serializar PDF: {}", e)))?;

    let nombre = format!(
        "CFDI_{}_{}_{}.pdf",
        cfdi.emisor.rfc,
        cfdi.serie.as_deref().unwrap_or("S"),
        cfdi.folio.as_deref().unwrap_or("0"),
    );

    Ok(PdfCfdi { bytes, nombre_archivo: nombre })
}

// ─── Helpers internos ─────────────────────────────────────────────────────────

fn capa_texto(layer: &PdfLayerReference, font: &IndirectFontRef, size: f64, x: f64, y: f64, text: &str) {
    layer.use_text(text, size, Mm(x as f32), Mm(y as f32), font);
}

fn capa_texto_derecha(layer: &PdfLayerReference, font: &IndirectFontRef, size: f64, x_right: f64, y: f64, text: &str) {
    // Aproximación simple: character width ≈ size * 0.4 pts → convertir a mm (1pt = 0.352mm)
    let approx_width = text.len() as f64 * size * 0.4 * 0.352;
    let x = (x_right - approx_width).max(0.0);
    layer.use_text(text, size, Mm(x as f32), Mm(y as f32), font);
}

fn linea_horizontal(layer: &PdfLayerReference, x: f64, y: f64, ancho: f64) {
    let puntos = vec![
        (Point::new(Mm(x as f32), Mm(y as f32)), false),
        (Point::new(Mm((x + ancho) as f32), Mm(y as f32)), false),
    ];
    let linea = Line {
        points: puntos,
        is_closed: false,
    };
    layer.add_line(linea);
}

fn formato_monto(d: Decimal) -> String {
    format!("{:.2}", d)
}

fn formato_decimal(d: Decimal) -> String {
    // Quita ceros innecesarios
    let s = format!("{}", d);
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s
    }
}

fn truncar(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max])
    }
}

fn importe_con_letra(monto: Decimal, moneda: &str) -> String {
    // Conversión simplificada — implementación completa requeriría tabla de letras
    let entero = monto.trunc().to_u64_digits().1.first().copied().unwrap_or(0);
    let centavos = ((monto.fract() * Decimal::from(100)).round()).to_u64_digits().1.first().copied().unwrap_or(0) as u32;
    let moneda_label = if moneda == "MXN" { "PESOS M.N." } else { moneda };
    format!("{} {}/{}/100 {}", numero_a_letras(entero), centavos, 100, moneda_label)
}

fn numero_a_letras(n: u64) -> String {
    // Tabla simplificada para los rangos más comunes
    let unidades = ["", "UN", "DOS", "TRES", "CUATRO", "CINCO", "SEIS", "SIETE", "OCHO", "NUEVE",
                    "DIEZ", "ONCE", "DOCE", "TRECE", "CATORCE", "QUINCE", "DIECISÉIS",
                    "DIECISIETE", "DIECIOCHO", "DIECINUEVE"];
    let decenas  = ["", "DIEZ", "VEINTE", "TREINTA", "CUARENTA", "CINCUENTA",
                    "SESENTA", "SETENTA", "OCHENTA", "NOVENTA"];
    let centenas = ["", "CIENTO", "DOSCIENTOS", "TRESCIENTOS", "CUATROCIENTOS",
                    "QUINIENTOS", "SEISCIENTOS", "SETECIENTOS", "OCHOCIENTOS", "NOVECIENTOS"];

    if n == 0 { return "CERO".to_string(); }
    if n == 100 { return "CIEN".to_string(); }
    if n < 20  { return unidades[n as usize].to_string(); }
    if n < 100 {
        let d = (n / 10) as usize;
        let u = (n % 10) as usize;
        return if u == 0 { decenas[d].to_string() }
               else { format!("{} Y {}", decenas[d], unidades[u]) };
    }
    if n < 1_000 {
        let c = (n / 100) as usize;
        let r = n % 100;
        return if r == 0 { centenas[c].to_string() }
               else { format!("{} {}", centenas[c], numero_a_letras(r)) };
    }
    if n < 1_000_000 {
        let miles = n / 1_000;
        let r = n % 1_000;
        let prefijo = if miles == 1 { "MIL".to_string() }
                      else { format!("{} MIL", numero_a_letras(miles)) };
        return if r == 0 { prefijo } else { format!("{} {}", prefijo, numero_a_letras(r)) };
    }
    if n < 1_000_000_000 {
        let millones = n / 1_000_000;
        let r = n % 1_000_000;
        let prefijo = if millones == 1 { "UN MILLÓN".to_string() }
                      else { format!("{} MILLONES", numero_a_letras(millones)) };
        return if r == 0 { prefijo } else { format!("{} {}", prefijo, numero_a_letras(r)) };
    }
    format!("{}", n) // fallback para montos muy grandes
}

/// Genera el QR como bytes PNG usando el crate qrcode
fn generar_qr_bytes(url: &str) -> Result<Vec<u8>, CfdiError> {
    use qrcode::QrCode;
    use image::{Luma, ImageFormat};
    use std::io::Cursor;

    let code = QrCode::new(url.as_bytes())
        .map_err(|e| CfdiError::Pdf(format!("Error QR: {}", e)))?;

    let img = code.render::<Luma<u8>>()
        .min_dimensions(200, 200)
        .build();

    let mut buf = Cursor::new(Vec::new());
    image::DynamicImage::ImageLuma8(img)
        .write_to(&mut buf, ImageFormat::Png)
        .map_err(|e| CfdiError::Pdf(format!("Error PNG QR: {}", e)))?;

    Ok(buf.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_numero_a_letras() {
        assert_eq!(numero_a_letras(0), "CERO");
        assert_eq!(numero_a_letras(1), "UN");
        assert_eq!(numero_a_letras(100), "CIEN");
        assert_eq!(numero_a_letras(101), "CIENTO UN");
        assert_eq!(numero_a_letras(1000), "MIL");
        assert_eq!(numero_a_letras(1234), "MIL DOSCIENTOS TREINTA Y CUATRO");
        assert_eq!(numero_a_letras(1_000_000), "UN MILLÓN");
    }

    #[test]
    fn test_truncar() {
        assert_eq!(truncar("hola", 10), "hola");
        assert_eq!(truncar("hola mundo", 4), "hola...");
    }

    #[test]
    fn test_formato_monto() {
        assert_eq!(formato_monto(dec!(1234.5)), "1234.50");
        assert_eq!(formato_monto(dec!(0.01)), "0.01");
    }
}
