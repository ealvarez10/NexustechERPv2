//! Representación Impresa de CFDI 4.0
//!
//! Genera el PDF de la representación impresa del comprobante fiscal digital
//! conforme al Artículo 29-A del CFF y Anexo 20 del SAT.
//!
//! Secciones:
//! 1. Encabezado: datos del emisor, serie/folio, tipo de comprobante
//! 2. Datos del receptor
//! 3. Datos del comprobante (fecha, forma pago, moneda)
//! 4. Tabla de conceptos
//! 5. Resumen de impuestos + totales
//! 6. Información fiscal: UUID, no. certificados, sellos
//! 7. Código QR de verificación SAT
//! 8. Pie de página legal

use printpdf::*;
use std::io::BufWriter;
use rust_decimal::Decimal;

use crate::{
    builder::CfdiData,
    error::CfdiError,
    qr::{DatosQr, generar_url_verificacion},
};

// ─── Tipos públicos ────────────────────────────────────────────────────────────

/// Resultado de la generación de PDF
pub struct PdfCfdi {
    /// Bytes del PDF generado
    pub bytes: Vec<u8>,
    /// Nombre sugerido para el archivo
    pub nombre_archivo: String,
}

/// Opciones de configuración del PDF
#[derive(Debug, Clone)]
pub struct OpcionesPdf {
    /// Logo PNG/JPEG en bytes (opcional)
    pub logo_bytes: Option<Vec<u8>>,
    /// Mostrar cadena original completa
    pub mostrar_cadena_original: bool,
    /// Color de cabecera (R, G, B) en 0-255
    pub color_cabecera: (u8, u8, u8),
}

impl Default for OpcionesPdf {
    fn default() -> Self {
        Self {
            logo_bytes: None,
            mostrar_cadena_original: false,
            color_cabecera: (0, 51, 102), // Azul corporativo
        }
    }
}

// ─── Función principal ────────────────────────────────────────────────────────

/// Genera PDF de representación impresa del CFDI
///
/// # Parámetros
/// - `cfdi`           — Datos del comprobante
/// - `uuid`           — Folio fiscal asignado por el PAC
/// - `sello_emisor`   — Sello digital del emisor (base64)
/// - `sello_sat`      — Sello digital del SAT (base64)
/// - `cadena_original`— Cadena original del timbre
/// - `cert_emisor`    — Número de certificado del emisor
/// - `cert_sat`       — Número de certificado del SAT
/// - `opciones`       — Configuración del PDF
pub fn generar_pdf(
    cfdi: &CfdiData,
    uuid: &str,
    sello_emisor: &str,
    sello_sat: &str,
    cadena_original: &str,
    cert_emisor: &str,
    cert_sat: &str,
    opciones: &OpcionesPdf,
) -> Result<PdfCfdi, CfdiError> {
    // Tamaño A4
    let ancho_mm: f32 = 210.0;
    let alto_mm:  f32 = 297.0;
    let margen: f64   = 15.0;
    let ancho_util: f64 = (ancho_mm as f64) - (margen * 2.0);

    let doc_title = format!(
        "{} {}-{}",
        tipo_label(&cfdi.tipo_de_comprobante),
        cfdi.serie.as_deref().unwrap_or(""),
        cfdi.folio.as_deref().unwrap_or(""),
    );

    let (doc, page1, layer1) = PdfDocument::new(
        &doc_title,
        Mm(ancho_mm),
        Mm(alto_mm),
        "Capa 1",
    );

    let page  = doc.get_page(page1);
    let layer = page.get_layer(layer1);

    // Fuentes embebidas (Helvetica built-in)
    let f_reg  = doc.add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| CfdiError::Pdf(format!("Fuente: {}", e)))?;
    let f_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| CfdiError::Pdf(format!("Fuente bold: {}", e)))?;
    let f_mono = doc.add_builtin_font(BuiltinFont::Courier)
        .map_err(|e| CfdiError::Pdf(format!("Fuente mono: {}", e)))?;

    let mut y: f64 = (alto_mm as f64) - margen;

    // ── TIPO DE COMPROBANTE ────────────────────────────────────────────────────
    texto(&layer, &f_bold, 14.0, margen, y, tipo_label(&cfdi.tipo_de_comprobante));
    let folio_str = format!(
        "Serie: {}  Folio: {}",
        cfdi.serie.as_deref().unwrap_or("—"),
        cfdi.folio.as_deref().unwrap_or("—"),
    );
    texto_derecha(&layer, &f_bold, 10.0, ancho_mm as f64 - margen, y, &folio_str);
    y -= 4.0;
    hr(&layer, margen, y, ancho_util);
    y -= 7.0;

    // ── EMISOR ─────────────────────────────────────────────────────────────────
    texto(&layer, &f_bold, 8.0, margen, y, "EMISOR");
    y -= 5.0;
    texto(&layer, &f_bold, 10.5, margen, y, &cfdi.emisor.nombre);
    y -= 5.5;
    texto(&layer, &f_reg, 8.5, margen, y,
        &format!("RFC: {}    Régimen: {}", cfdi.emisor.rfc, cfdi.emisor.regimen_fiscal));
    y -= 9.0;

    // ── RECEPTOR ──────────────────────────────────────────────────────────────
    texto(&layer, &f_bold, 8.0, margen, y, "RECEPTOR");
    y -= 5.0;
    texto(&layer, &f_bold, 10.5, margen, y, &cfdi.receptor.nombre);
    y -= 5.5;
    texto(&layer, &f_reg, 8.5, margen, y,
        &format!("RFC: {}    CP: {}    Uso CFDI: {}    Régimen: {}",
            cfdi.receptor.rfc,
            cfdi.receptor.domicilio_fiscal_receptor.as_deref().unwrap_or("—"),
            cfdi.receptor.uso_cfdi,
            cfdi.receptor.regimen_fiscal_receptor,
        ));
    y -= 9.0;

    // ── DATOS DEL COMPROBANTE ─────────────────────────────────────────────────
    hr(&layer, margen, y, ancho_util);
    y -= 6.0;

    let col2 = margen + ancho_util / 2.0;
    texto(&layer, &f_reg, 8.0, margen, y,
        &format!("Fecha expedición: {}", &cfdi.fecha));
    texto(&layer, &f_reg, 8.0, col2, y,
        &format!("Forma de pago: {}", cfdi.forma_pago.as_deref().unwrap_or("—")));
    y -= 5.0;
    texto(&layer, &f_reg, 8.0, margen, y,
        &format!("Método de pago: {}", cfdi.metodo_pago.as_deref().unwrap_or("—")));
    texto(&layer, &f_reg, 8.0, col2, y,
        &format!("Moneda: {}   T.C.: {}",
            cfdi.moneda,
            cfdi.tipo_cambio.map(|d| fmtd(d)).unwrap_or_else(|| "1".into()),
        ));
    y -= 5.0;
    texto(&layer, &f_reg, 8.0, margen, y,
        &format!("Exportación: {}", cfdi.exportacion.as_deref().unwrap_or("01")));
    y -= 9.0;

    // ── TABLA DE CONCEPTOS ────────────────────────────────────────────────────
    hr(&layer, margen, y, ancho_util);
    y -= 5.0;

    // Columnas (en mm desde la izquierda)
    let cx = [
        margen,                         // ClaveProdServ
        margen + 18.0,                  // Descripción
        margen + ancho_util * 0.54,     // Cantidad
        margen + ancho_util * 0.62,     // Unidad
        margen + ancho_util * 0.73,     // P.Unitario
        margen + ancho_util * 0.86,     // Importe
    ];

    texto(&layer, &f_bold, 7.5, cx[0], y, "ClaveProdServ");
    texto(&layer, &f_bold, 7.5, cx[1], y, "Descripción");
    texto(&layer, &f_bold, 7.5, cx[2], y, "Cant.");
    texto(&layer, &f_bold, 7.5, cx[3], y, "Unidad");
    texto(&layer, &f_bold, 7.5, cx[4], y, "P.Unitario");
    texto(&layer, &f_bold, 7.5, cx[5], y, "Importe");
    y -= 3.0;
    hr(&layer, margen, y, ancho_util);
    y -= 5.5;

    for c in &cfdi.conceptos {
        if y < 60.0 { break; } // Evitar overflow (multi-página: TODO v2)
        texto(&layer, &f_reg, 7.5, cx[0], y, &c.clave_prod_serv);
        texto(&layer, &f_reg, 7.5, cx[1], y, &trunc(&c.descripcion, 42));
        texto(&layer, &f_reg, 7.5, cx[2], y, &fmtd(c.cantidad));
        texto(&layer, &f_reg, 7.5, cx[3], y, &c.clave_unidad);
        texto(&layer, &f_reg, 7.5, cx[4], y, &fmtm(c.valor_unitario));
        texto(&layer, &f_reg, 7.5, cx[5], y, &fmtm(c.importe));
        y -= 5.0;
        if let Some(d) = c.descuento {
            if d > Decimal::ZERO {
                texto(&layer, &f_reg, 7.0, cx[1], y,
                    &format!("  Descuento: -{}", fmtm(d)));
                y -= 4.5;
            }
        }
    }

    hr(&layer, margen, y, ancho_util);
    y -= 8.0;

    // ── TOTALES ───────────────────────────────────────────────────────────────
    let xt = margen + ancho_util * 0.64;
    let xv = margen + ancho_util * 0.85;

    texto(&layer, &f_reg, 8.5, xt, y, "Subtotal:");
    texto(&layer, &f_reg, 8.5, xv, y, &fmtm(cfdi.sub_total));
    y -= 5.5;

    if let Some(d) = cfdi.descuento {
        if d > Decimal::ZERO {
            texto(&layer, &f_reg, 8.5, xt, y, "Descuento:");
            texto(&layer, &f_reg, 8.5, xv, y, &format!("-{}", fmtm(d)));
            y -= 5.5;
        }
    }

    // Traslados globales
    if let Some(ref imp) = cfdi.impuestos {
        for t in &imp.traslados {
            let etiq = impuesto_label(&t.impuesto, t.tasa_o_cuota);
            texto(&layer, &f_reg, 8.5, xt, y, &format!("{}:", etiq));
            texto(&layer, &f_reg, 8.5, xv, y, &fmtm(t.importe));
            y -= 5.5;
        }
        for r in &imp.retenciones {
            let etiq = if r.impuesto == "001" { "Ret. ISR" } else { "Ret. IVA" };
            texto(&layer, &f_reg, 8.5, xt, y, &format!("{}:", etiq));
            texto(&layer, &f_reg, 8.5, xv, y, &format!("-{}", fmtm(r.importe)));
            y -= 5.5;
        }
    }

    hr(&layer, xt - 2.0, y + 1.0, ancho_util - (xt - margen) + 2.0);
    y -= 1.5;
    texto(&layer, &f_bold, 10.0, xt, y, "TOTAL:");
    texto(&layer, &f_bold, 10.0, xv, y,
        &format!("{} {}", cfdi.moneda, fmtm(cfdi.total)));
    y -= 7.0;

    // Importe con letra
    texto(&layer, &f_reg, 7.5, margen, y,
        &format!("Son: {}", importe_letras(cfdi.total, &cfdi.moneda)));
    y -= 12.0;

    // ── DATOS FISCALES ────────────────────────────────────────────────────────
    hr(&layer, margen, y, ancho_util);
    y -= 5.0;

    texto(&layer, &f_bold, 8.5, margen, y, "INFORMACIÓN FISCAL");
    y -= 5.5;
    texto(&layer, &f_reg, 7.5, margen, y,
        &format!("Folio fiscal (UUID):  {}", uuid));
    y -= 4.5;
    texto(&layer, &f_reg, 7.5, margen, y,
        &format!("No. Certificado emisor: {}    No. Certificado SAT: {}", cert_emisor, cert_sat));
    y -= 4.5;
    texto(&layer, &f_reg, 7.5, margen, y,
        &format!("Fecha/hora certificación: {}", &cfdi.fecha));
    y -= 7.0;

    // Sello emisor
    texto(&layer, &f_bold, 7.0, margen, y, "Sello digital del emisor:");
    y -= 4.0;
    let (l1, l2) = split_sello(sello_emisor);
    texto(&layer, &f_mono, 6.0, margen, y, &l1);
    if !l2.is_empty() { y -= 4.0; texto(&layer, &f_mono, 6.0, margen, y, &l2); }
    y -= 7.0;

    // Sello SAT
    texto(&layer, &f_bold, 7.0, margen, y, "Sello digital del SAT:");
    y -= 4.0;
    let (s1, s2) = split_sello(sello_sat);
    texto(&layer, &f_mono, 6.0, margen, y, &s1);
    if !s2.is_empty() { y -= 4.0; texto(&layer, &f_mono, 6.0, margen, y, &s2); }
    y -= 7.0;

    // Cadena original opcional
    if opciones.mostrar_cadena_original && !cadena_original.is_empty() {
        texto(&layer, &f_bold, 7.0, margen, y, "Cadena original del timbre:");
        y -= 4.0;
        texto(&layer, &f_mono, 5.5, margen, y, &trunc(cadena_original, 130));
        y -= 8.0;
    }

    // ── QR SAT ────────────────────────────────────────────────────────────────
    let datos_qr = DatosQr {
        rfc_emisor:   cfdi.emisor.rfc.clone(),
        rfc_receptor: cfdi.receptor.rfc.clone(),
        total:        cfdi.total,
        uuid:         uuid.to_string(),
        sello:        sello_emisor.to_string(),
    };
    let url_sat = generar_url_verificacion(&datos_qr)?;

    // QR de verificación SAT — se muestra como URL textual
    // (La imagen QR será añadida en versión posterior cuando las dependencias estén alineadas)
    texto(&layer, &f_bold, 7.0, margen, y, "Verificar CFDI en:");
    y -= 4.5;
    texto(&layer, &f_mono, 5.5, margen, y, &trunc(&url_sat, 110));
    y -= 5.0;


    // ── PIE DE PÁGINA ─────────────────────────────────────────────────────────
    hr(&layer, margen, 12.0, ancho_util);
    texto(&layer, &f_reg, 6.0, margen, 8.0,
        "Este documento es una representación impresa de un CFDI. \
         Verifique la autenticidad en: www.sat.gob.mx");

    // ── EXPORTAR ──────────────────────────────────────────────────────────────
    let mut buf = BufWriter::new(Vec::new());
    doc.save(&mut buf)
        .map_err(|e| CfdiError::Pdf(format!("Exportando PDF: {}", e)))?;
    let bytes = buf.into_inner()
        .map_err(|e| CfdiError::Pdf(format!("Serializando PDF: {}", e)))?;

    let nombre = format!(
        "CFDI_{}_{}{}.pdf",
        cfdi.emisor.rfc,
        cfdi.serie.as_deref().unwrap_or("S"),
        cfdi.folio.as_deref().unwrap_or("0"),
    );

    Ok(PdfCfdi { bytes, nombre_archivo: nombre })
}

// ─── Helpers internos ─────────────────────────────────────────────────────────

fn texto(layer: &PdfLayerReference, font: &IndirectFontRef, sz: f64, x: f64, y: f64, t: &str) {
    layer.use_text(t, sz as f32, Mm(x as f32), Mm(y as f32), font);
}

fn texto_derecha(layer: &PdfLayerReference, font: &IndirectFontRef, sz: f64, x_r: f64, y: f64, t: &str) {
    let w = t.len() as f64 * sz * 0.35 * 0.352;
    let x = (x_r - w).max(0.0);
    layer.use_text(t, sz as f32, Mm(x as f32), Mm(y as f32), font);
}

fn hr(layer: &PdfLayerReference, x: f64, y: f64, w: f64) {
    let pts = vec![
        (Point::new(Mm(x as f32), Mm(y as f32)), false),
        (Point::new(Mm((x + w) as f32), Mm(y as f32)), false),
    ];
    layer.add_line(Line { points: pts, is_closed: false });
}

fn fmtm(d: Decimal) -> String { format!("{:.2}", d) }

fn fmtd(d: Decimal) -> String {
    let s = format!("{}", d);
    if s.contains('.') { s.trim_end_matches('0').trim_end_matches('.').into() }
    else { s }
}

fn trunc(s: &str, n: usize) -> String {
    if s.len() <= n { s.into() }
    else { format!("{}…", &s[..n]) }
}

fn split_sello(s: &str) -> (String, String) {
    if s.len() > 100 {
        (s[..100].to_string(), trunc(&s[100..], 100))
    } else {
        (s.to_string(), String::new())
    }
}

fn tipo_label(t: &str) -> &'static str {
    match t {
        "I" => "FACTURA",
        "E" => "NOTA DE CRÉDITO",
        "P" => "COMPLEMENTO DE PAGO",
        "N" => "COMPROBANTE DE NÓMINA",
        "T" => "CARTA PORTE",
        _   => "COMPROBANTE FISCAL",
    }
}

fn impuesto_label(impuesto: &str, tasa: Decimal) -> String {
    let nombre = match impuesto {
        "002" => "IVA",
        "003" => "IEPS",
        "001" => "ISR",
        _     => "Impuesto",
    };
    let pct = (tasa * Decimal::from(100)).round_dp(0);
    format!("{} {}%", nombre, pct)
}

fn importe_letras(monto: Decimal, moneda: &str) -> String {
    let entero = monto.trunc().to_string().parse::<u64>().unwrap_or(0);
    let frac   = monto.fract();
    let centavos = (frac * Decimal::from(100)).round_dp(0)
        .to_string().parse::<u32>().unwrap_or(0);
    let m_label = if moneda == "MXN" { "PESOS M.N." } else { moneda };
    format!("{} {}/{}/100 {}", n_letras(entero), centavos, 100, m_label)
}

fn n_letras(n: u64) -> String {
    const U: &[&str] = &["", "UN", "DOS", "TRES", "CUATRO", "CINCO", "SEIS",
        "SIETE", "OCHO", "NUEVE", "DIEZ", "ONCE", "DOCE", "TRECE", "CATORCE",
        "QUINCE", "DIECISÉIS", "DIECISIETE", "DIECIOCHO", "DIECINUEVE"];
    const D: &[&str] = &["", "DIEZ", "VEINTE", "TREINTA", "CUARENTA", "CINCUENTA",
        "SESENTA", "SETENTA", "OCHENTA", "NOVENTA"];
    const C: &[&str] = &["", "CIENTO", "DOSCIENTOS", "TRESCIENTOS", "CUATROCIENTOS",
        "QUINIENTOS", "SEISCIENTOS", "SETECIENTOS", "OCHOCIENTOS", "NOVECIENTOS"];
    if n == 0   { return "CERO".into(); }
    if n == 100 { return "CIEN".into(); }
    if n < 20   { return U[n as usize].into(); }
    if n < 100  {
        let (d, u) = (n / 10, n % 10);
        return if u == 0 { D[d as usize].into() }
               else { format!("{} Y {}", D[d as usize], U[u as usize]) };
    }
    if n < 1_000 {
        let (c, r) = (n / 100, n % 100);
        return if r == 0 { C[c as usize].into() }
               else { format!("{} {}", C[c as usize], n_letras(r)) };
    }
    if n < 1_000_000 {
        let (m, r) = (n / 1_000, n % 1_000);
        let p = if m == 1 { "MIL".into() } else { format!("{} MIL", n_letras(m)) };
        return if r == 0 { p } else { format!("{} {}", p, n_letras(r)) };
    }
    if n < 1_000_000_000 {
        let (m, r) = (n / 1_000_000, n % 1_000_000);
        let p = if m == 1 { "UN MILLÓN".into() } else { format!("{} MILLONES", n_letras(m)) };
        return if r == 0 { p } else { format!("{} {}", p, n_letras(r)) };
    }
    format!("{}", n)
}


// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_n_letras_basicos() {
        assert_eq!(n_letras(0),    "CERO");
        assert_eq!(n_letras(1),    "UN");
        assert_eq!(n_letras(15),   "QUINCE");
        assert_eq!(n_letras(21),   "VEINTE Y UN");
        assert_eq!(n_letras(100),  "CIEN");
        assert_eq!(n_letras(101),  "CIENTO UN");
        assert_eq!(n_letras(1000), "MIL");
        assert_eq!(n_letras(1234), "MIL DOSCIENTOS TREINTA Y CUATRO");
        assert_eq!(n_letras(1_000_000), "UN MILLÓN");
    }

    #[test]
    fn test_importe_letras_mxn() {
        let s = importe_letras(dec!(1234.56), "MXN");
        assert!(s.contains("MIL"), "Debe contener MIL: {}", s);
        assert!(s.contains("56/100"), "Debe contener centavos: {}", s);
        assert!(s.contains("PESOS"), "Debe contener moneda: {}", s);
    }

    #[test]
    fn test_fmtm() {
        assert_eq!(fmtm(dec!(1234.5)),  "1234.50");
        assert_eq!(fmtm(dec!(0.01)),    "0.01");
        assert_eq!(fmtm(dec!(100)),     "100.00");
    }

    #[test]
    fn test_tipo_label() {
        assert_eq!(tipo_label("I"), "FACTURA");
        assert_eq!(tipo_label("E"), "NOTA DE CRÉDITO");
        assert_eq!(tipo_label("P"), "COMPLEMENTO DE PAGO");
    }
}
