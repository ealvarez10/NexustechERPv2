//! Cadena original del CFDI 4.0
//!
//! Implementa la concatenación de campos según el Anexo 20 del SAT.
//! La cadena original es el insumo para el sello digital RSA-SHA256.
//!
//! Formato: `||campo1|campo2|...|campoN||`
//! - Separador de inicio y fin: `||`
//! - Separador entre campos: `|`
//! - Campos vacíos/nulos se omiten
//! - Sin espacios extra, sin saltos de línea

use crate::builder::CfdiData;

/// Genera la cadena original de un CFDI 4.0 según el Anexo 20 del SAT
///
/// La cadena original sigue el orden exacto definido en el XSD del SAT.
/// NO se usa XSLT — se implementa directamente el algoritmo de concatenación.
pub fn generar(cfdi: &CfdiData) -> String {
    let mut campos: Vec<String> = Vec::with_capacity(64);

    // ─── Comprobante ───
    push(&mut campos, &cfdi.version);           // Version
    push_opt(&mut campos, &cfdi.serie);         // Serie
    push_opt(&mut campos, &cfdi.folio);         // Folio
    push(&mut campos, &cfdi.fecha);             // Fecha (ISO 8601: 2024-01-15T12:00:00)
    push(&mut campos, &cfdi.forma_pago.as_deref().unwrap_or(""));       // FormaPago
    push(&mut campos, &cfdi.no_certificado);    // NoCertificado
    push(&mut campos, &cfdi.condiciones_de_pago.as_deref().unwrap_or("")); // CondicionesDePago
    push(&mut campos, &cfdi.sub_total.to_string()); // SubTotal
    if let Some(d) = &cfdi.descuento {
        push(&mut campos, &d.to_string());      // Descuento (solo si existe)
    }
    push(&mut campos, &cfdi.moneda);            // Moneda
    if let Some(tc) = &cfdi.tipo_cambio {
        push(&mut campos, &tc.to_string());     // TipoCambio
    }
    push(&mut campos, &cfdi.total.to_string()); // Total
    push(&mut campos, &cfdi.tipo_de_comprobante); // TipoDeComprobante
    push_opt(&mut campos, &cfdi.exportacion);   // Exportacion
    push_opt(&mut campos, &cfdi.metodo_pago);   // MetodoPago
    push(&mut campos, &cfdi.lugar_expedicion);  // LugarExpedicion
    push_opt(&mut campos, &cfdi.confirmacion);  // Confirmacion

    // ─── InformacionGlobal (si aplica) ───
    if let Some(ig) = &cfdi.informacion_global {
        push(&mut campos, &ig.periodicidad);
        push(&mut campos, &ig.meses);
        push(&mut campos, &ig.año.to_string());
    }

    // ─── Emisor ───
    push(&mut campos, &cfdi.emisor.rfc);
    push(&mut campos, &cfdi.emisor.nombre);
    push(&mut campos, &cfdi.emisor.regimen_fiscal);

    // ─── Receptor ───
    push(&mut campos, &cfdi.receptor.rfc);
    push(&mut campos, &cfdi.receptor.nombre);
    push_opt(&mut campos, &cfdi.receptor.domicilio_fiscal_receptor);
    push_opt(&mut campos, &cfdi.receptor.residencia_fiscal);
    push_opt(&mut campos, &cfdi.receptor.num_reg_id_trib);
    push(&mut campos, &cfdi.receptor.regimen_fiscal_receptor);
    push(&mut campos, &cfdi.receptor.uso_cfdi);

    // ─── Conceptos ───
    for concepto in &cfdi.conceptos {
        push(&mut campos, &concepto.clave_prod_serv);
        push_opt(&mut campos, &concepto.no_identificacion);
        push(&mut campos, &concepto.cantidad.to_string());
        push(&mut campos, &concepto.clave_unidad);
        push_opt(&mut campos, &concepto.unidad);
        push(&mut campos, &concepto.descripcion);
        push(&mut campos, &concepto.valor_unitario.to_string());
        push(&mut campos, &concepto.importe.to_string());
        if let Some(d) = &concepto.descuento {
            push(&mut campos, &d.to_string());
        }
        push_opt(&mut campos, &concepto.objeto_imp);

        // Impuestos del concepto
        for traslado in &concepto.traslados {
            push(&mut campos, &traslado.base.to_string());
            push(&mut campos, &traslado.impuesto);
            push(&mut campos, &traslado.tipo_factor);
            push(&mut campos, &traslado.tasa_o_cuota.to_string());
            push(&mut campos, &traslado.importe.to_string());
        }
        for retencion in &concepto.retenciones {
            push(&mut campos, &retencion.base.to_string());
            push(&mut campos, &retencion.impuesto);
            push(&mut campos, &retencion.tipo_factor);
            push(&mut campos, &retencion.tasa_o_cuota.to_string());
            push(&mut campos, &retencion.importe.to_string());
        }
    }

    // ─── Impuestos globales ───
    if let Some(imp) = &cfdi.impuestos {
        if let Some(total_ret) = imp.total_impuestos_retenidos {
            push(&mut campos, &total_ret.to_string());
        }
        if let Some(total_tras) = imp.total_impuestos_trasladados {
            push(&mut campos, &total_tras.to_string());
        }
        for traslado in &imp.traslados {
            push(&mut campos, &traslado.base.to_string());
            push(&mut campos, &traslado.impuesto);
            push(&mut campos, &traslado.tipo_factor);
            push(&mut campos, &traslado.tasa_o_cuota.to_string());
            push(&mut campos, &traslado.importe.to_string());
        }
        for retencion in &imp.retenciones {
            push(&mut campos, &retencion.impuesto);
            push(&mut campos, &retencion.importe.to_string());
        }
    }

    // Formatear: ||campo1|campo2|...||
    format!("||{}||", campos.join("|"))
}

#[inline]
fn push(campos: &mut Vec<String>, valor: &str) {
    if !valor.is_empty() {
        campos.push(valor.to_string());
    }
}

#[inline]
fn push_opt(campos: &mut Vec<String>, valor: &Option<String>) {
    if let Some(v) = valor {
        if !v.is_empty() {
            campos.push(v.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::{CfdiData, Emisor, Receptor};
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn test_cadena_original_formato() {
        // Cadena minimal de prueba
        let cfdi = CfdiData {
            version: "4.0".into(),
            serie: Some("A".into()),
            folio: Some("1".into()),
            fecha: "2024-01-15T12:00:00".into(),
            forma_pago: Some("01".into()),
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
                nombre: "ID & BARCODE SOLUTIONS SA DE CV".into(),
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
            conceptos: vec![],
            impuestos: None,
        };

        let cadena = generar(&cfdi);
        assert!(cadena.starts_with("||"));
        assert!(cadena.ends_with("||"));
        assert!(cadena.contains("IBS120101AA1"));
        assert!(cadena.contains("4.0"));
        println!("Cadena original: {}", cadena);
    }
}
