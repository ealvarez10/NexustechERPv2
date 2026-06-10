//! Validación pre-timbrado de CFDI 4.0
//!
//! Verifica que todos los campos del CFDI sean válidos según las reglas SAT
//! ANTES de enviar al PAC, para evitar rechazos innecesarios.
//!
//! Valida:
//! - RFC emisor/receptor (usando rfc::validar_rfc)
//! - Catálogos: FormaPago, MetodoPago, UsoCFDI, RegimenFiscal, etc.
//! - Reglas de negocio: PUE requiere FormaPago, PPD no la requiere
//! - Totales matemáticos: SubTotal + Impuestos = Total
//! - Conceptos: importe = cantidad × valor_unitario

use crate::builder::CfdiData;
use crate::rfc::validar_rfc;
use crate::catalogs::*;
use crate::error::CfdiError;
use rust_decimal::Decimal;
use std::str::FromStr;

/// Error de validación con campo y mensaje
#[derive(Debug, Clone)]
pub struct ErrorValidacion {
    pub campo: String,
    pub mensaje: String,
}

impl std::fmt::Display for ErrorValidacion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.campo, self.mensaje)
    }
}

/// Resultado de la validación con todos los errores encontrados
#[derive(Debug)]
pub struct ResultadoValidacion {
    pub errores: Vec<ErrorValidacion>,
}

impl ResultadoValidacion {
    pub fn es_valido(&self) -> bool {
        self.errores.is_empty()
    }

    pub fn a_error_cfdi(&self) -> CfdiError {
        let msgs: Vec<String> = self.errores.iter().map(|e| e.to_string()).collect();
        CfdiError::ValidacionFallida(msgs.join("; "))
    }
}

/// Valida un CfdiData completo antes de timbrar
///
/// Retorna Ok(()) si es válido, Err con todos los errores encontrados si no.
pub fn validar(cfdi: &CfdiData) -> Result<(), CfdiError> {
    let resultado = validar_completo(cfdi);
    if resultado.es_valido() {
        Ok(())
    } else {
        Err(resultado.a_error_cfdi())
    }
}

/// Valida y retorna todos los errores (no solo el primero)
pub fn validar_completo(cfdi: &CfdiData) -> ResultadoValidacion {
    let mut errores = Vec::new();
    let mut e = |campo: &str, msg: &str| {
        errores.push(ErrorValidacion {
            campo: campo.to_string(),
            mensaje: msg.to_string(),
        });
    };

    // ─── Versión ───
    if cfdi.version != "4.0" {
        e("Version", "Debe ser '4.0'");
    }

    // ─── Fecha ───
    if cfdi.fecha.is_empty() {
        e("Fecha", "Requerida");
    } else if !cfdi.fecha.contains('T') || cfdi.fecha.len() != 19 {
        e("Fecha", "Formato inválido. Debe ser YYYY-MM-DDTHH:MM:SS");
    }

    // ─── Emisor ───
    if let Err(err) = validar_rfc(&cfdi.emisor.rfc) {
        e("Emisor.Rfc", &err.to_string());
    }
    if cfdi.emisor.nombre.trim().is_empty() {
        e("Emisor.Nombre", "Requerido");
    }
    if !regimen_fiscal::es_valido(&cfdi.emisor.regimen_fiscal) {
        e("Emisor.RegimenFiscal", &format!("Clave '{}' no válida en c_RegimenFiscal", cfdi.emisor.regimen_fiscal));
    }

    // ─── Receptor ───
    if let Err(err) = validar_rfc(&cfdi.receptor.rfc) {
        e("Receptor.Rfc", &err.to_string());
    }
    if cfdi.receptor.nombre.trim().is_empty() {
        e("Receptor.Nombre", "Requerido");
    }
    if !regimen_fiscal::es_valido(&cfdi.receptor.regimen_fiscal_receptor) {
        e("Receptor.RegimenFiscalReceptor", &format!("Clave '{}' no válida", cfdi.receptor.regimen_fiscal_receptor));
    }
    if !uso_cfdi::es_valido(&cfdi.receptor.uso_cfdi) {
        e("Receptor.UsoCFDI", &format!("Clave '{}' no válida en c_UsoCFDI", cfdi.receptor.uso_cfdi));
    }
    // Si receptor no es genérico, requiere DomicilioFiscalReceptor
    if cfdi.receptor.rfc != "XAXX010101000" && cfdi.receptor.rfc != "XEXX010101000" {
        if cfdi.receptor.domicilio_fiscal_receptor.as_deref().unwrap_or("").is_empty() {
            e("Receptor.DomicilioFiscalReceptor", "Requerido cuando RFC no es genérico");
        }
    }

    // ─── TipoDeComprobante ───
    if !tipo_comprobante::es_valido(&cfdi.tipo_de_comprobante) {
        e("TipoDeComprobante", &format!("Clave '{}' no válida en c_TipoDeComprobante", cfdi.tipo_de_comprobante));
    }

    // ─── Exportación ───
    if let Some(exp) = &cfdi.exportacion {
        if !exportacion::es_valida(exp) {
            e("Exportacion", &format!("Clave '{}' no válida en c_Exportacion", exp));
        }
    }

    // ─── Método y Forma de Pago (regla cruzada) ───
    match cfdi.metodo_pago.as_deref() {
        Some("PUE") => {
            if cfdi.forma_pago.is_none() {
                e("FormaPago", "Requerida cuando MetodoPago = PUE");
            } else if let Some(fp) = &cfdi.forma_pago {
                if !forma_pago::es_valida(fp) {
                    e("FormaPago", &format!("Clave '{}' no válida en c_FormaPago", fp));
                }
            }
        }
        Some("PPD") => {
            // PPD no debe tener FormaPago (SAT regla 2.7.4.2)
            // Aunque algunos PACs lo aceptan, el SAT lo rechaza estrictamente
        }
        Some(mp) if !metodo_pago::es_valido(mp) => {
            e("MetodoPago", &format!("Clave '{}' no válida en c_MetodoPago", mp));
        }
        _ => {}
    }

    // ─── Moneda ───
    if cfdi.moneda.is_empty() {
        e("Moneda", "Requerida");
    }
    if cfdi.moneda != "MXN" && cfdi.tipo_cambio.is_none() {
        e("TipoCambio", "Requerido cuando Moneda != MXN");
    }

    // ─── LugarExpedicion ───
    if cfdi.lugar_expedicion.len() != 5 || !cfdi.lugar_expedicion.chars().all(|c| c.is_ascii_digit()) {
        e("LugarExpedicion", "Debe ser un código postal de 5 dígitos");
    }

    // ─── Conceptos ───
    if cfdi.conceptos.is_empty() {
        e("Conceptos", "Debe tener al menos un concepto");
    }

    for (i, concepto) in cfdi.conceptos.iter().enumerate() {
        let prefix = format!("Conceptos[{}]", i);

        if concepto.clave_prod_serv.is_empty() {
            e(&format!("{}.ClaveProdServ", prefix), "Requerida");
        }
        if concepto.descripcion.trim().is_empty() {
            e(&format!("{}.Descripcion", prefix), "Requerida");
        }
        if concepto.clave_unidad.is_empty() {
            e(&format!("{}.ClaveUnidad", prefix), "Requerida");
        }
        if concepto.cantidad <= Decimal::ZERO {
            e(&format!("{}.Cantidad", prefix), "Debe ser mayor a cero");
        }
        if concepto.valor_unitario < Decimal::ZERO {
            e(&format!("{}.ValorUnitario", prefix), "No puede ser negativo");
        }

        // Verificar que importe = cantidad × valor_unitario (tolerancia de $0.01)
        let importe_calculado = concepto.cantidad * concepto.valor_unitario;
        let diferencia = (importe_calculado - concepto.importe).abs();
        let tolerancia = Decimal::from_str("0.01").unwrap();
        if diferencia > tolerancia {
            e(&format!("{}.Importe", prefix),
                &format!("No coincide: {} × {} = {}, declarado: {}",
                    concepto.cantidad, concepto.valor_unitario, importe_calculado, concepto.importe));
        }

        // Validar catálogos de impuestos del concepto
        for (j, t) in concepto.traslados.iter().enumerate() {
            let tp = format!("{}.Traslados[{}]", prefix, j);
            if !impuesto::es_valido(&t.impuesto) {
                e(&format!("{}.Impuesto", tp), &format!("Clave '{}' no válida en c_Impuesto", t.impuesto));
            }
            if !tipo_factor::es_valido(&t.tipo_factor) {
                e(&format!("{}.TipoFactor", tp), &format!("Clave '{}' no válida en c_TipoFactor", t.tipo_factor));
            }
        }
    }

    // ─── Totales ───
    let subtotal_calculado: Decimal = cfdi.conceptos.iter().map(|c| c.importe).sum();
    let tolerancia = Decimal::from_str("0.01").unwrap();
    if (subtotal_calculado - cfdi.sub_total).abs() > tolerancia {
        e("SubTotal", &format!("No coincide: suma de conceptos = {}, declarado: {}", subtotal_calculado, cfdi.sub_total));
    }

    // Total = SubTotal + Traslados - Retenciones
    let total_traslados: Decimal = cfdi.impuestos.as_ref()
        .and_then(|i| i.total_impuestos_trasladados)
        .unwrap_or(Decimal::ZERO);
    let total_retenciones: Decimal = cfdi.impuestos.as_ref()
        .and_then(|i| i.total_impuestos_retenidos)
        .unwrap_or(Decimal::ZERO);
    let total_calculado = cfdi.sub_total + total_traslados - total_retenciones;

    if (total_calculado - cfdi.total).abs() > tolerancia {
        e("Total", &format!("No coincide: {} + {} - {} = {}, declarado: {}",
            cfdi.sub_total, total_traslados, total_retenciones, total_calculado, cfdi.total));
    }

    ResultadoValidacion { errores }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    fn cfdi_valido() -> CfdiData {
        CfdiData {
            version: "4.0".into(),
            serie: Some("A".into()),
            folio: Some("1".into()),
            fecha: "2024-01-15T12:00:00".into(),
            forma_pago: Some("03".into()),
            no_certificado: "00001000000504465028".into(),
            condiciones_de_pago: None,
            sub_total: Decimal::from_str("100.00").unwrap(),
            descuento: None,
            moneda: "MXN".into(),
            tipo_cambio: None,
            total: Decimal::from_str("116.00").unwrap(),
            tipo_de_comprobante: "I".into(),
            exportacion: Some("01".into()),
            metodo_pago: Some("PUE".into()),
            lugar_expedicion: "64000".into(),
            confirmacion: None,
            informacion_global: None,
            emisor: Emisor {
                rfc: "IBS120101AA1".into(),
                nombre: "ID BARCODE SOLUTIONS SA DE CV".into(),
                regimen_fiscal: "601".into(),
            },
            receptor: Receptor {
                rfc: "XAXX010101000".into(),
                nombre: "PUBLICO EN GENERAL".into(),
                domicilio_fiscal_receptor: Some("64000".into()),
                residencia_fiscal: None,
                num_reg_id_trib: None,
                regimen_fiscal_receptor: "616".into(),
                uso_cfdi: "S01".into(),
            },
            conceptos: vec![Concepto {
                clave_prod_serv: "43232408".into(),
                no_identificacion: None,
                cantidad: Decimal::from_str("1.000000").unwrap(),
                clave_unidad: "H87".into(),
                unidad: None,
                descripcion: "Producto de prueba".into(),
                valor_unitario: Decimal::from_str("100.000000").unwrap(),
                importe: Decimal::from_str("100.000000").unwrap(),
                descuento: None,
                objeto_imp: Some("02".into()),
                traslados: vec![Traslado {
                    base: Decimal::from_str("100.000000").unwrap(),
                    impuesto: "002".into(),
                    tipo_factor: "Tasa".into(),
                    tasa_o_cuota: Decimal::from_str("0.160000").unwrap(),
                    importe: Decimal::from_str("16.000000").unwrap(),
                }],
                retenciones: vec![],
            }],
            impuestos: Some(Impuestos {
                total_impuestos_trasladados: Some(Decimal::from_str("16.00").unwrap()),
                total_impuestos_retenidos: None,
                traslados: vec![Traslado {
                    base: Decimal::from_str("100.000000").unwrap(),
                    impuesto: "002".into(),
                    tipo_factor: "Tasa".into(),
                    tasa_o_cuota: Decimal::from_str("0.160000").unwrap(),
                    importe: Decimal::from_str("16.000000").unwrap(),
                }],
                retenciones: vec![],
            }),
        }
    }

    #[test]
    fn test_cfdi_valido_sin_errores() {
        let cfdi = cfdi_valido();
        let resultado = validar_completo(&cfdi);
        if !resultado.es_valido() {
            for e in &resultado.errores { eprintln!("ERROR: {}", e); }
        }
        assert!(resultado.es_valido(), "CFDI de prueba debe ser válido");
    }

    #[test]
    fn test_rfc_invalido_detectado() {
        let mut cfdi = cfdi_valido();
        cfdi.emisor.rfc = "INVALIDO".into();
        let resultado = validar_completo(&cfdi);
        assert!(!resultado.es_valido());
        assert!(resultado.errores.iter().any(|e| e.campo.contains("Rfc")));
    }

    #[test]
    fn test_total_incorrecto_detectado() {
        let mut cfdi = cfdi_valido();
        cfdi.total = rust_decimal::Decimal::from_str("999.00").unwrap();
        let resultado = validar_completo(&cfdi);
        assert!(!resultado.es_valido());
        assert!(resultado.errores.iter().any(|e| e.campo.contains("Total")));
    }

    #[test]
    fn test_uso_cfdi_invalido() {
        let mut cfdi = cfdi_valido();
        cfdi.receptor.uso_cfdi = "ZZ9".into();
        let resultado = validar_completo(&cfdi);
        assert!(!resultado.es_valido());
        assert!(resultado.errores.iter().any(|e| e.campo.contains("UsoCFDI")));
    }
}
