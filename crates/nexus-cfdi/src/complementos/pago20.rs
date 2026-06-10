//! Complemento de Pagos 2.0 (pago20)
//!
//! Implementación del complemento para pagos en parcialidades o diferido (PPD).
//! XSD oficial: http://www.sat.gob.mx/esquemas/ContabilidadE/1_3/Pagos20/Pagos20.xsd
//!
//! Flujo PPD:
//! 1. Emitir CFDI tipo "I" con MetodoPago="PPD" y FormaPago ausente
//! 2. Cuando el cliente paga: emitir CFDI tipo "P" (pago) con este complemento
//! 3. El CFDI de pago referencia al CFDI original por UUID

use crate::error::CfdiError;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Writer;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

const PAGO20_NS: &str = "http://www.sat.gob.mx/Pagos20";
const PAGO20_SCHEMA_LOC: &str =
    "http://www.sat.gob.mx/Pagos20 http://www.sat.gob.mx/esquemas/ContabilidadE/1_3/Pagos20/Pagos20.xsd";

/// Datos completos del complemento de pagos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplementoPago {
    /// Lista de pagos recibidos (normalmente 1 por CFDI de pago)
    pub pagos: Vec<Pago>,
}

/// Un pago individual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pago {
    /// Fecha y hora del pago — ISO 8601: 2024-01-20T10:00:00
    pub fecha_pago: String,
    /// Forma de pago del complemento (c_FormaPago)
    pub forma_de_pago_p: String,
    /// Moneda del pago (MXN, USD, etc.)
    pub moneda_p: String,
    /// Tipo de cambio (requerido si moneda != MXN)
    pub tipo_cambio_p: Option<Decimal>,
    /// Monto total del pago
    pub monto: Decimal,
    /// Número de operación del banco (referencia)
    pub num_operacion: Option<String>,
    /// RFC del banco ordenante
    pub rfc_emisor_cta_ord: Option<String>,
    /// Nombre del banco ordenante
    pub nom_banco_ord_ext: Option<String>,
    /// Cuenta bancaria origen
    pub cta_ordenante: Option<String>,
    /// RFC del banco beneficiario
    pub rfc_emisor_cta_ben: Option<String>,
    /// Cuenta bancaria destino
    pub cta_beneficiario: Option<String>,
    /// Documentos relacionados (CFDIs que se están pagando)
    pub documentos_relacionados: Vec<DoctoRelacionado>,
}

/// CFDI que se está pagando con este complemento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctoRelacionado {
    /// UUID del CFDI original que se paga
    pub id_documento: String,
    /// Serie del CFDI original
    pub serie: Option<String>,
    /// Folio del CFDI original
    pub folio: Option<String>,
    /// Moneda del CFDI original
    pub moneda_dr: String,
    /// Equivalencia de moneda (1 si misma moneda)
    pub equivalencia_dr: Decimal,
    /// Número de parcialidad (1, 2, 3...)
    pub num_parcialidad: u32,
    /// Importe saldo anterior
    pub imp_saldo_ant: Decimal,
    /// Importe pagado en esta operación
    pub imp_pagado: Decimal,
    /// Importe saldo insoluto después del pago
    pub imp_saldo_insoluto: Decimal,
    /// Objeto de impuesto del documento
    pub objeto_imp_dr: String,
    /// Impuestos del documento relacionado
    pub impuestos_dr: Option<ImpuestosDr>,
}

/// Impuestos del documento relacionado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpuestosDr {
    pub traslados: Vec<TrasladoDr>,
    pub retenciones: Vec<RetencionDr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrasladoDr {
    pub base_dr: Decimal,
    pub impuesto_dr: String,      // 002=IVA, 003=IEPS
    pub tipo_factor_dr: String,   // Tasa, Cuota, Exento
    pub tasa_o_cuota_dr: Decimal,
    pub importe_dr: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetencionDr {
    pub base_dr: Decimal,
    pub impuesto_dr: String,
    pub tipo_factor_dr: String,
    pub tasa_o_cuota_dr: Decimal,
    pub importe_dr: Decimal,
}

/// Totales calculados del complemento (los calcula automáticamente)
#[derive(Debug, Clone)]
pub struct TotalesPago {
    pub monto_total_pagos: Decimal,
    // IVA 16%
    pub total_traslados_base_iva16: Option<Decimal>,
    pub total_traslados_impuesto_iva16: Option<Decimal>,
    // IVA 8%
    pub total_traslados_base_iva8: Option<Decimal>,
    pub total_traslados_impuesto_iva8: Option<Decimal>,
    // IVA 0%
    pub total_traslados_base_iva0: Option<Decimal>,
    pub total_traslados_impuesto_iva0: Option<Decimal>,
    // Exento
    pub total_traslados_base_iva_exento: Option<Decimal>,
    // Retenciones
    pub total_retenciones_iva: Option<Decimal>,
    pub total_retenciones_isr: Option<Decimal>,
    pub total_retenciones_ieps: Option<Decimal>,
}

impl ComplementoPago {
    /// Calcula los totales automáticamente desde los documentos relacionados
    pub fn calcular_totales(&self) -> TotalesPago {
        let monto_total: Decimal = self.pagos.iter().map(|p| p.monto).sum();
        let mut base16 = Decimal::ZERO;
        let mut imp16 = Decimal::ZERO;
        let mut base0 = Decimal::ZERO;
        let mut imp0 = Decimal::ZERO;
        let mut base_exento = Decimal::ZERO;

        for pago in &self.pagos {
            for dr in &pago.documentos_relacionados {
                if let Some(imps) = &dr.impuestos_dr {
                    for t in &imps.traslados {
                        if t.impuesto_dr == "002" {
                            match t.tipo_factor_dr.as_str() {
                                "Tasa" if t.tasa_o_cuota_dr.to_string().starts_with("0.16") => {
                                    base16 += t.base_dr;
                                    imp16 += t.importe_dr;
                                }
                                "Tasa" if t.tasa_o_cuota_dr.to_string().starts_with("0.00") => {
                                    base0 += t.base_dr;
                                    imp0 += t.importe_dr;
                                }
                                "Exento" => { base_exento += t.base_dr; }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        TotalesPago {
            monto_total_pagos: monto_total,
            total_traslados_base_iva16: if base16 > Decimal::ZERO { Some(base16) } else { None },
            total_traslados_impuesto_iva16: if imp16 > Decimal::ZERO { Some(imp16) } else { None },
            total_traslados_base_iva8: None,
            total_traslados_impuesto_iva8: None,
            total_traslados_base_iva0: if base0 > Decimal::ZERO { Some(base0) } else { None },
            total_traslados_impuesto_iva0: if imp0 > Decimal::ZERO { Some(imp0) } else { None },
            total_traslados_base_iva_exento: if base_exento > Decimal::ZERO { Some(base_exento) } else { None },
            total_retenciones_iva: None,
            total_retenciones_isr: None,
            total_retenciones_ieps: None,
        }
    }
}

/// Genera el XML del complemento de pagos 2.0
pub fn generar_xml(complemento: &ComplementoPago) -> Result<String, CfdiError> {
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 4);
    let totales = complemento.calcular_totales();

    // <pago20:Pagos>
    let mut pagos_elem = BytesStart::new("pago20:Pagos");
    pagos_elem.push_attribute(("xmlns:pago20", PAGO20_NS));
    pagos_elem.push_attribute(("xsi:schemaLocation", PAGO20_SCHEMA_LOC));
    pagos_elem.push_attribute(("Version", "2.0"));
    writer.write_event(Event::Start(pagos_elem)).map_err(xml_err)?;

    // <pago20:Totales>
    let mut totales_elem = BytesStart::new("pago20:Totales");
    totales_elem.push_attribute(("MontoTotalPagos", fmt6(totales.monto_total_pagos).as_str()));
    if let (Some(b), Some(i)) = (totales.total_traslados_base_iva16, totales.total_traslados_impuesto_iva16) {
        totales_elem.push_attribute(("TotalTrasladosBaseIVA16", fmt6(b).as_str()));
        totales_elem.push_attribute(("TotalTrasladosImpuestoIVA16", fmt6(i).as_str()));
    }
    if let (Some(b), Some(i)) = (totales.total_traslados_base_iva0, totales.total_traslados_impuesto_iva0) {
        totales_elem.push_attribute(("TotalTrasladosBaseIVA0", fmt6(b).as_str()));
        totales_elem.push_attribute(("TotalTrasladosImpuestoIVA0", fmt6(i).as_str()));
    }
    if let Some(b) = totales.total_traslados_base_iva_exento {
        totales_elem.push_attribute(("TotalTrasladosBaseIVAExento", fmt6(b).as_str()));
    }
    writer.write_event(Event::Empty(totales_elem)).map_err(xml_err)?;

    // Cada <pago20:Pago>
    for pago in &complemento.pagos {
        escribir_pago(&mut writer, pago)?;
    }

    writer.write_event(Event::End(BytesEnd::new("pago20:Pagos"))).map_err(xml_err)?;

    let bytes = writer.into_inner().into_inner();
    String::from_utf8(bytes).map_err(|e| CfdiError::Xml(e.to_string()))
}

fn escribir_pago(writer: &mut Writer<Cursor<Vec<u8>>>, pago: &Pago) -> Result<(), CfdiError> {
    let mut elem = BytesStart::new("pago20:Pago");
    elem.push_attribute(("FechaPago", pago.fecha_pago.as_str()));
    elem.push_attribute(("FormaDePagoP", pago.forma_de_pago_p.as_str()));
    elem.push_attribute(("MonedaP", pago.moneda_p.as_str()));
    if let Some(tc) = pago.tipo_cambio_p {
        elem.push_attribute(("TipoCambioP", fmt6(tc).as_str()));
    }
    elem.push_attribute(("Monto", fmt6(pago.monto).as_str()));
    if let Some(op) = &pago.num_operacion {
        elem.push_attribute(("NumOperacion", op.as_str()));
    }
    if let Some(rfc) = &pago.rfc_emisor_cta_ord {
        elem.push_attribute(("RfcEmisorCtaOrd", rfc.as_str()));
    }
    if let Some(banco) = &pago.nom_banco_ord_ext {
        elem.push_attribute(("NomBancoOrdExt", banco.as_str()));
    }
    if let Some(cta) = &pago.cta_ordenante {
        elem.push_attribute(("CtaOrdenante", cta.as_str()));
    }
    if let Some(rfc) = &pago.rfc_emisor_cta_ben {
        elem.push_attribute(("RfcEmisorCtaBen", rfc.as_str()));
    }
    if let Some(cta) = &pago.cta_beneficiario {
        elem.push_attribute(("CtaBeneficiario", cta.as_str()));
    }

    writer.write_event(Event::Start(elem)).map_err(xml_err)?;

    // Documentos relacionados
    for dr in &pago.documentos_relacionados {
        escribir_docto(writer, dr)?;
    }

    // ImpuestosP (agregados del pago)
    let impuestos_p = agregar_impuestos_pago(&pago.documentos_relacionados);
    if !impuestos_p.traslados.is_empty() || !impuestos_p.retenciones.is_empty() {
        writer.write_event(Event::Start(BytesStart::new("pago20:ImpuestosP"))).map_err(xml_err)?;

        if !impuestos_p.traslados.is_empty() {
            writer.write_event(Event::Start(BytesStart::new("pago20:TrasladosP"))).map_err(xml_err)?;
            for t in &impuestos_p.traslados {
                let mut te = BytesStart::new("pago20:TrasladoP");
                te.push_attribute(("BaseP", fmt6(t.base_dr).as_str()));
                te.push_attribute(("ImpuestoP", t.impuesto_dr.as_str()));
                te.push_attribute(("TipoFactorP", t.tipo_factor_dr.as_str()));
                te.push_attribute(("TasaOCuotaP", fmt6(t.tasa_o_cuota_dr).as_str()));
                te.push_attribute(("ImporteP", fmt6(t.importe_dr).as_str()));
                writer.write_event(Event::Empty(te)).map_err(xml_err)?;
            }
            writer.write_event(Event::End(BytesEnd::new("pago20:TrasladosP"))).map_err(xml_err)?;
        }

        writer.write_event(Event::End(BytesEnd::new("pago20:ImpuestosP"))).map_err(xml_err)?;
    }

    writer.write_event(Event::End(BytesEnd::new("pago20:Pago"))).map_err(xml_err)
}

fn escribir_docto(writer: &mut Writer<Cursor<Vec<u8>>>, dr: &DoctoRelacionado) -> Result<(), CfdiError> {
    let mut elem = BytesStart::new("pago20:DoctoRelacionado");
    elem.push_attribute(("IdDocumento", dr.id_documento.as_str()));
    if let Some(s) = &dr.serie { elem.push_attribute(("Serie", s.as_str())); }
    if let Some(f) = &dr.folio { elem.push_attribute(("Folio", f.as_str())); }
    elem.push_attribute(("MonedaDR", dr.moneda_dr.as_str()));
    elem.push_attribute(("EquivalenciaDR", fmt6(dr.equivalencia_dr).as_str()));
    elem.push_attribute(("NumParcialidad", dr.num_parcialidad.to_string().as_str()));
    elem.push_attribute(("ImpSaldoAnt", fmt6(dr.imp_saldo_ant).as_str()));
    elem.push_attribute(("ImpPagado", fmt6(dr.imp_pagado).as_str()));
    elem.push_attribute(("ImpSaldoInsoluto", fmt6(dr.imp_saldo_insoluto).as_str()));
    elem.push_attribute(("ObjetoImpDR", dr.objeto_imp_dr.as_str()));

    if let Some(imps) = &dr.impuestos_dr {
        if !imps.traslados.is_empty() || !imps.retenciones.is_empty() {
            writer.write_event(Event::Start(elem)).map_err(xml_err)?;
            writer.write_event(Event::Start(BytesStart::new("pago20:ImpuestosDR"))).map_err(xml_err)?;

            if !imps.traslados.is_empty() {
                writer.write_event(Event::Start(BytesStart::new("pago20:TrasladosDR"))).map_err(xml_err)?;
                for t in &imps.traslados {
                    let mut te = BytesStart::new("pago20:TrasladoDR");
                    te.push_attribute(("BaseDR", fmt6(t.base_dr).as_str()));
                    te.push_attribute(("ImpuestoDR", t.impuesto_dr.as_str()));
                    te.push_attribute(("TipoFactorDR", t.tipo_factor_dr.as_str()));
                    te.push_attribute(("TasaOCuotaDR", fmt6(t.tasa_o_cuota_dr).as_str()));
                    te.push_attribute(("ImporteDR", fmt6(t.importe_dr).as_str()));
                    writer.write_event(Event::Empty(te)).map_err(xml_err)?;
                }
                writer.write_event(Event::End(BytesEnd::new("pago20:TrasladosDR"))).map_err(xml_err)?;
            }

            writer.write_event(Event::End(BytesEnd::new("pago20:ImpuestosDR"))).map_err(xml_err)?;
            writer.write_event(Event::End(BytesEnd::new("pago20:DoctoRelacionado"))).map_err(xml_err)?;
        } else {
            writer.write_event(Event::Empty(elem)).map_err(xml_err)?;
        }
    } else {
        writer.write_event(Event::Empty(elem)).map_err(xml_err)?;
    }
    Ok(())
}

/// Agrega impuestos de todos los documentos relacionados para el ImpuestosP
fn agregar_impuestos_pago(docs: &[DoctoRelacionado]) -> ImpuestosDr {
    use std::collections::HashMap;
    let mut mapa: HashMap<String, TrasladoDr> = HashMap::new();

    for dr in docs {
        if let Some(imps) = &dr.impuestos_dr {
            for t in &imps.traslados {
                let key = format!("{}_{}", t.impuesto_dr, t.tipo_factor_dr);
                let entry = mapa.entry(key).or_insert_with(|| t.clone());
                if !std::ptr::eq(entry as *const _, t as *const _) {
                    entry.base_dr += t.base_dr;
                    entry.importe_dr += t.importe_dr;
                }
            }
        }
    }

    ImpuestosDr {
        traslados: mapa.into_values().collect(),
        retenciones: vec![],
    }
}

fn fmt6(d: Decimal) -> String { format!("{:.6}", d) }
fn xml_err(e: quick_xml::Error) -> CfdiError { CfdiError::Xml(e.to_string()) }

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    fn pago_prueba() -> ComplementoPago {
        ComplementoPago {
            pagos: vec![Pago {
                fecha_pago: "2024-01-20T10:00:00".into(),
                forma_de_pago_p: "03".into(),
                moneda_p: "MXN".into(),
                tipo_cambio_p: None,
                monto: Decimal::from_str("1160.00").unwrap(),
                num_operacion: Some("TRANS001".into()),
                rfc_emisor_cta_ord: None,
                nom_banco_ord_ext: None,
                cta_ordenante: None,
                rfc_emisor_cta_ben: Some("IBS120101AA1".into()),
                cta_beneficiario: Some("012XXXXXXXXX01234567890".into()),
                documentos_relacionados: vec![DoctoRelacionado {
                    id_documento: "550e8400-e29b-41d4-a716-446655440000".into(),
                    serie: Some("A".into()),
                    folio: Some("1".into()),
                    moneda_dr: "MXN".into(),
                    equivalencia_dr: Decimal::ONE,
                    num_parcialidad: 1,
                    imp_saldo_ant: Decimal::from_str("1160.00").unwrap(),
                    imp_pagado: Decimal::from_str("1160.00").unwrap(),
                    imp_saldo_insoluto: Decimal::ZERO,
                    objeto_imp_dr: "02".into(),
                    impuestos_dr: Some(ImpuestosDr {
                        traslados: vec![TrasladoDr {
                            base_dr: Decimal::from_str("1000.00").unwrap(),
                            impuesto_dr: "002".into(),
                            tipo_factor_dr: "Tasa".into(),
                            tasa_o_cuota_dr: Decimal::from_str("0.160000").unwrap(),
                            importe_dr: Decimal::from_str("160.00").unwrap(),
                        }],
                        retenciones: vec![],
                    }),
                }],
            }],
        }
    }

    #[test]
    fn test_pago20_xml_generado() {
        let comp = pago_prueba();
        let xml = generar_xml(&comp).unwrap();
        assert!(xml.contains("pago20:Pagos"), "Falta raíz Pagos");
        assert!(xml.contains("Version=\"2.0\""), "Falta Version");
        assert!(xml.contains("pago20:Totales"), "Falta Totales");
        assert!(xml.contains("pago20:Pago"), "Falta Pago");
        assert!(xml.contains("pago20:DoctoRelacionado"), "Falta DoctoRelacionado");
        assert!(xml.contains("550e8400"), "Falta UUID del documento");
        assert!(xml.contains("MontoTotalPagos"), "Falta MontoTotalPagos");
        println!("XML Pago20:\n{}", xml);
    }

    #[test]
    fn test_totales_calculados() {
        let comp = pago_prueba();
        let totales = comp.calcular_totales();
        assert_eq!(totales.monto_total_pagos, Decimal::from_str("1160.00").unwrap());
        assert!(totales.total_traslados_base_iva16.is_some());
        assert_eq!(
            totales.total_traslados_base_iva16.unwrap(),
            Decimal::from_str("1000.00").unwrap()
        );
    }
}
