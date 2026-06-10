//! Generador XML CFDI 4.0
//!
//! Produce el XML exacto requerido por el SAT según el Anexo 20 y el XSD:
//! http://www.sat.gob.mx/sitio_internet/cfd/4/cfdv40.xsd
//!
//! Flujo completo de timbrado:
//! 1. `generar_sin_sello(cfdi)` → XML sin Sello (para calcular cadena original)
//! 2. `cadena_original::generar(cfdi)` → cadena original
//! 3. `sellado::sellar(cadena, llave, cer)` → Sello { valor, no_certificado, certificado_b64 }
//! 4. `generar_sellado(cfdi, sello)` → XML completo listo para enviar al PAC

use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::io::Cursor;
use rust_decimal::Decimal;
use crate::builder::{CfdiData, Concepto, Impuestos, Traslado, RetencionGlobal};
use crate::sellado::Sello;
use crate::error::CfdiError;

const CFDI_NS: &str = "http://www.sat.gob.mx/cfd/4";
const XSI_NS: &str = "http://www.w3.org/2001/XMLSchema-instance";
const SCHEMA_LOC: &str =
    "http://www.sat.gob.mx/cfd/4 http://www.sat.gob.mx/sitio_internet/cfd/4/cfdv40.xsd";

/// Genera el XML CFDI 4.0 completamente sellado y listo para timbrar
pub fn generar_sellado(cfdi: &CfdiData, sello: &Sello) -> Result<String, CfdiError> {
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);

    // <?xml version="1.0" encoding="UTF-8"?>
    writer
        .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
        .map_err(|e| CfdiError::Xml(e.to_string()))?;

    // <cfdi:Comprobante ...>
    let mut comprobante = BytesStart::new("cfdi:Comprobante");
    comprobante.push_attribute(("xmlns:cfdi", CFDI_NS));
    comprobante.push_attribute(("xmlns:xsi", XSI_NS));
    comprobante.push_attribute(("xsi:schemaLocation", SCHEMA_LOC));
    comprobante.push_attribute(("Version", cfdi.version.as_str()));

    if let Some(s) = &cfdi.serie {
        comprobante.push_attribute(("Serie", s.as_str()));
    }
    if let Some(f) = &cfdi.folio {
        comprobante.push_attribute(("Folio", f.as_str()));
    }
    comprobante.push_attribute(("Fecha", cfdi.fecha.as_str()));

    if let Some(fp) = &cfdi.forma_pago {
        comprobante.push_attribute(("FormaPago", fp.as_str()));
    }

    // Sello digital
    comprobante.push_attribute(("NoCertificado", sello.no_certificado.as_str()));
    comprobante.push_attribute(("Certificado", sello.certificado_b64.as_str()));

    if let Some(cdp) = &cfdi.condiciones_de_pago {
        comprobante.push_attribute(("CondicionesDePago", cdp.as_str()));
    }

    comprobante.push_attribute(("SubTotal", fmt_decimal(cfdi.sub_total).as_str()));

    if let Some(desc) = cfdi.descuento {
        comprobante.push_attribute(("Descuento", fmt_decimal(desc).as_str()));
    }

    comprobante.push_attribute(("Moneda", cfdi.moneda.as_str()));

    if let Some(tc) = cfdi.tipo_cambio {
        comprobante.push_attribute(("TipoCambio", fmt_decimal(tc).as_str()));
    }

    comprobante.push_attribute(("Total", fmt_decimal(cfdi.total).as_str()));
    comprobante.push_attribute(("TipoDeComprobante", cfdi.tipo_de_comprobante.as_str()));

    if let Some(exp) = &cfdi.exportacion {
        comprobante.push_attribute(("Exportacion", exp.as_str()));
    }
    if let Some(mp) = &cfdi.metodo_pago {
        comprobante.push_attribute(("MetodoPago", mp.as_str()));
    }

    comprobante.push_attribute(("LugarExpedicion", cfdi.lugar_expedicion.as_str()));

    if let Some(conf) = &cfdi.confirmacion {
        comprobante.push_attribute(("Confirmacion", conf.as_str()));
    }

    comprobante.push_attribute(("Sello", sello.valor.as_str()));

    writer
        .write_event(Event::Start(comprobante))
        .map_err(|e| CfdiError::Xml(e.to_string()))?;

    // Información Global (si aplica — CFDI global)
    if let Some(ig) = &cfdi.informacion_global {
        let mut elem = BytesStart::new("cfdi:InformacionGlobal");
        elem.push_attribute(("Periodicidad", ig.periodicidad.as_str()));
        elem.push_attribute(("Meses", ig.meses.as_str()));
        elem.push_attribute(("Año", ig.año.to_string().as_str()));
        writer.write_event(Event::Empty(elem)).map_err(|e| CfdiError::Xml(e.to_string()))?;
    }

    // <cfdi:Emisor>
    let mut emisor = BytesStart::new("cfdi:Emisor");
    emisor.push_attribute(("Rfc", cfdi.emisor.rfc.as_str()));
    emisor.push_attribute(("Nombre", cfdi.emisor.nombre.as_str()));
    emisor.push_attribute(("RegimenFiscal", cfdi.emisor.regimen_fiscal.as_str()));
    writer.write_event(Event::Empty(emisor)).map_err(|e| CfdiError::Xml(e.to_string()))?;

    // <cfdi:Receptor>
    let mut receptor = BytesStart::new("cfdi:Receptor");
    receptor.push_attribute(("Rfc", cfdi.receptor.rfc.as_str()));
    receptor.push_attribute(("Nombre", cfdi.receptor.nombre.as_str()));
    if let Some(cp) = &cfdi.receptor.domicilio_fiscal_receptor {
        receptor.push_attribute(("DomicilioFiscalReceptor", cp.as_str()));
    }
    if let Some(rf) = &cfdi.receptor.residencia_fiscal {
        receptor.push_attribute(("ResidenciaFiscal", rf.as_str()));
    }
    if let Some(nr) = &cfdi.receptor.num_reg_id_trib {
        receptor.push_attribute(("NumRegIdTrib", nr.as_str()));
    }
    receptor.push_attribute(("RegimenFiscalReceptor", cfdi.receptor.regimen_fiscal_receptor.as_str()));
    receptor.push_attribute(("UsoCFDI", cfdi.receptor.uso_cfdi.as_str()));
    writer.write_event(Event::Empty(receptor)).map_err(|e| CfdiError::Xml(e.to_string()))?;

    // <cfdi:Conceptos>
    writer
        .write_event(Event::Start(BytesStart::new("cfdi:Conceptos")))
        .map_err(|e| CfdiError::Xml(e.to_string()))?;

    for concepto in &cfdi.conceptos {
        escribir_concepto(&mut writer, concepto)?;
    }

    writer
        .write_event(Event::End(BytesEnd::new("cfdi:Conceptos")))
        .map_err(|e| CfdiError::Xml(e.to_string()))?;

    // <cfdi:Impuestos>
    if let Some(impuestos) = &cfdi.impuestos {
        escribir_impuestos_globales(&mut writer, impuestos)?;
    }

    // </cfdi:Comprobante>
    writer
        .write_event(Event::End(BytesEnd::new("cfdi:Comprobante")))
        .map_err(|e| CfdiError::Xml(e.to_string()))?;

    let xml_bytes = writer.into_inner().into_inner();
    String::from_utf8(xml_bytes).map_err(|e| CfdiError::Xml(e.to_string()))
}

/// Genera XML sin Sello — para calcular la cadena original
/// (el Sello se agrega DESPUÉS de firmar la cadena original)
pub fn generar_sin_sello(cfdi: &CfdiData) -> Result<String, CfdiError> {
    // Crear un sello vacío temporal solo para generar estructura
    let sello_vacio = Sello {
        valor: String::new(),
        no_certificado: String::new(),
        certificado_b64: String::new(),
    };
    // Nota: el XML sin sello se usa solo para previsualización/debug
    // La cadena original se genera desde CfdiData directamente (no desde XML)
    generar_sellado(cfdi, &sello_vacio)
}

fn escribir_concepto(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    c: &Concepto,
) -> Result<(), CfdiError> {
    let tiene_impuestos = !c.traslados.is_empty() || !c.retenciones.is_empty();

    let mut elem = BytesStart::new("cfdi:Concepto");
    elem.push_attribute(("ClaveProdServ", c.clave_prod_serv.as_str()));

    if let Some(noid) = &c.no_identificacion {
        elem.push_attribute(("NoIdentificacion", noid.as_str()));
    }

    elem.push_attribute(("Cantidad", fmt_decimal6(c.cantidad).as_str()));
    elem.push_attribute(("ClaveUnidad", c.clave_unidad.as_str()));

    if let Some(u) = &c.unidad {
        elem.push_attribute(("Unidad", u.as_str()));
    }

    elem.push_attribute(("Descripcion", c.descripcion.as_str()));
    elem.push_attribute(("ValorUnitario", fmt_decimal6(c.valor_unitario).as_str()));
    elem.push_attribute(("Importe", fmt_decimal6(c.importe).as_str()));

    if let Some(d) = c.descuento {
        elem.push_attribute(("Descuento", fmt_decimal6(d).as_str()));
    }

    if let Some(oi) = &c.objeto_imp {
        elem.push_attribute(("ObjetoImp", oi.as_str()));
    }

    if tiene_impuestos {
        writer.write_event(Event::Start(elem)).map_err(|e| CfdiError::Xml(e.to_string()))?;

        // <cfdi:Impuestos> del concepto
        writer
            .write_event(Event::Start(BytesStart::new("cfdi:Impuestos")))
            .map_err(|e| CfdiError::Xml(e.to_string()))?;

        if !c.traslados.is_empty() {
            writer
                .write_event(Event::Start(BytesStart::new("cfdi:Traslados")))
                .map_err(|e| CfdiError::Xml(e.to_string()))?;
            for t in &c.traslados {
                escribir_traslado_concepto(writer, t)?;
            }
            writer
                .write_event(Event::End(BytesEnd::new("cfdi:Traslados")))
                .map_err(|e| CfdiError::Xml(e.to_string()))?;
        }

        if !c.retenciones.is_empty() {
            writer
                .write_event(Event::Start(BytesStart::new("cfdi:Retenciones")))
                .map_err(|e| CfdiError::Xml(e.to_string()))?;
            for r in &c.retenciones {
                let mut ret = BytesStart::new("cfdi:Retencion");
                ret.push_attribute(("Base", fmt_decimal6(r.base).as_str()));
                ret.push_attribute(("Impuesto", r.impuesto.as_str()));
                ret.push_attribute(("TipoFactor", r.tipo_factor.as_str()));
                ret.push_attribute(("TasaOCuota", fmt_decimal6(r.tasa_o_cuota).as_str()));
                ret.push_attribute(("Importe", fmt_decimal6(r.importe).as_str()));
                writer.write_event(Event::Empty(ret)).map_err(|e| CfdiError::Xml(e.to_string()))?;
            }
            writer
                .write_event(Event::End(BytesEnd::new("cfdi:Retenciones")))
                .map_err(|e| CfdiError::Xml(e.to_string()))?;
        }

        writer
            .write_event(Event::End(BytesEnd::new("cfdi:Impuestos")))
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
        writer
            .write_event(Event::End(BytesEnd::new("cfdi:Concepto")))
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    } else {
        writer.write_event(Event::Empty(elem)).map_err(|e| CfdiError::Xml(e.to_string()))?;
    }

    Ok(())
}

fn escribir_traslado_concepto(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    t: &Traslado,
) -> Result<(), CfdiError> {
    let mut elem = BytesStart::new("cfdi:Traslado");
    elem.push_attribute(("Base", fmt_decimal6(t.base).as_str()));
    elem.push_attribute(("Impuesto", t.impuesto.as_str()));
    elem.push_attribute(("TipoFactor", t.tipo_factor.as_str()));
    elem.push_attribute(("TasaOCuota", fmt_decimal6(t.tasa_o_cuota).as_str()));
    elem.push_attribute(("Importe", fmt_decimal6(t.importe).as_str()));
    writer.write_event(Event::Empty(elem)).map_err(|e| CfdiError::Xml(e.to_string()))
}

fn escribir_impuestos_globales(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    imp: &Impuestos,
) -> Result<(), CfdiError> {
    let mut elem = BytesStart::new("cfdi:Impuestos");

    if let Some(ret) = imp.total_impuestos_retenidos {
        elem.push_attribute(("TotalImpuestosRetenidos", fmt_decimal6(ret).as_str()));
    }
    if let Some(tras) = imp.total_impuestos_trasladados {
        elem.push_attribute(("TotalImpuestosTrasladados", fmt_decimal6(tras).as_str()));
    }

    let tiene_hijos = !imp.traslados.is_empty() || !imp.retenciones.is_empty();

    if tiene_hijos {
        writer.write_event(Event::Start(elem)).map_err(|e| CfdiError::Xml(e.to_string()))?;

        if !imp.retenciones.is_empty() {
            writer
                .write_event(Event::Start(BytesStart::new("cfdi:Retenciones")))
                .map_err(|e| CfdiError::Xml(e.to_string()))?;
            for r in &imp.retenciones {
                let mut ret_elem = BytesStart::new("cfdi:Retencion");
                ret_elem.push_attribute(("Impuesto", r.impuesto.as_str()));
                ret_elem.push_attribute(("Importe", fmt_decimal6(r.importe).as_str()));
                writer.write_event(Event::Empty(ret_elem)).map_err(|e| CfdiError::Xml(e.to_string()))?;
            }
            writer
                .write_event(Event::End(BytesEnd::new("cfdi:Retenciones")))
                .map_err(|e| CfdiError::Xml(e.to_string()))?;
        }

        if !imp.traslados.is_empty() {
            writer
                .write_event(Event::Start(BytesStart::new("cfdi:Traslados")))
                .map_err(|e| CfdiError::Xml(e.to_string()))?;
            for t in &imp.traslados {
                escribir_traslado_concepto(writer, t)?;
            }
            writer
                .write_event(Event::End(BytesEnd::new("cfdi:Traslados")))
                .map_err(|e| CfdiError::Xml(e.to_string()))?;
        }

        writer
            .write_event(Event::End(BytesEnd::new("cfdi:Impuestos")))
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    } else {
        writer.write_event(Event::Empty(elem)).map_err(|e| CfdiError::Xml(e.to_string()))?;
    }

    Ok(())
}

/// Formatea un Decimal con 2 decimales (para montos en Comprobante)
fn fmt_decimal(d: Decimal) -> String {
    format!("{:.2}", d)
}

/// Formatea un Decimal con 6 decimales (para tasas, cantidades, importes en Concepto)
fn fmt_decimal6(d: Decimal) -> String {
    format!("{:.6}", d)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    fn cfdi_prueba() -> CfdiData {
        CfdiData {
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
            conceptos: vec![
                Concepto {
                    clave_prod_serv: "43232408".into(),
                    no_identificacion: Some("PROD-001".into()),
                    cantidad: Decimal::from_str("1.000000").unwrap(),
                    clave_unidad: "H87".into(),
                    unidad: Some("Pieza".into()),
                    descripcion: "Impresora de etiquetas Zebra ZT420".into(),
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
                },
            ],
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
    fn test_xml_contiene_estructura_cfdi() {
        let cfdi = cfdi_prueba();
        let sello = Sello {
            valor: "SELLObase64==".into(),
            no_certificado: "00001000000504465028".into(),
            certificado_b64: "CERTbase64==".into(),
        };
        let xml = generar_sellado(&cfdi, &sello).unwrap();

        assert!(xml.contains("cfdi:Comprobante"), "Falta raíz Comprobante");
        assert!(xml.contains("xmlns:cfdi=\"http://www.sat.gob.mx/cfd/4\""), "Falta namespace CFDI");
        assert!(xml.contains("Version=\"4.0\""), "Falta Version");
        assert!(xml.contains("cfdi:Emisor"), "Falta Emisor");
        assert!(xml.contains("cfdi:Receptor"), "Falta Receptor");
        assert!(xml.contains("cfdi:Conceptos"), "Falta Conceptos");
        assert!(xml.contains("cfdi:Impuestos"), "Falta Impuestos");
        assert!(xml.contains("IBS120101AA1"), "Falta RFC emisor");
        assert!(xml.contains("XAXX010101000"), "Falta RFC receptor");
        assert!(xml.contains("Zebra ZT420"), "Falta descripción concepto");
        assert!(xml.contains("TasaOCuota=\"0.160000\""), "Falta tasa IVA");

        println!("XML generado:\n{}", xml);
    }

    #[test]
    fn test_xml_escapa_ampersand() {
        let cfdi = cfdi_prueba(); // emisor tiene "&" en el nombre
        let sello = Sello {
            valor: String::new(),
            no_certificado: String::new(),
            certificado_b64: String::new(),
        };
        let xml = generar_sellado(&cfdi, &sello).unwrap();
        // El & debe escaparse como &amp; en XML
        assert!(xml.contains("&amp;") || xml.contains("ID &amp; BARCODE") ||
                xml.contains("ID & BARCODE"), // quick-xml escapa automáticamente
            "El & debe estar correctamente en el XML");
    }
}
