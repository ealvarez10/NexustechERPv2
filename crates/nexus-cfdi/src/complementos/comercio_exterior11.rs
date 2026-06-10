//! Complemento de Comercio Exterior 1.1
//! XSD: http://www.sat.gob.mx/ComercioExterior11
//!
//! Requerido en CFDIs de exportación definitiva.
//! Aplica cuando se exportan bienes o mercancías fuera del territorio nacional.
//!
//! Regla de uso: RFC MK = XEXX010101000 (extranjeros)
//! Documento normativo: Anexo 20 SAT + RMF vigente

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use crate::error::CfdiError;

// ─── Structs principales ──────────────────────────────────────────────────────

/// Complemento de Comercio Exterior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComercioExterior {
    /// Número de certificado de origen (emisor)
    pub cert_origen: Option<String>,
    /// Número de certificado de origen (exportación)
    pub num_exporter: Option<String>,
    /// Tipo de operación: 1=exportación, 2=importación (siempre 1 en CFDI)
    pub tipo_operacion: String,
    /// Clave de pedimento (A1=exportación definitiva, H1=retorno, etc.)
    pub clave_de_pedimento: Option<String>,
    /// Incoterm (EXW, FOB, CIF, DAP, etc.)
    pub incoterm: Option<String>,
    /// Indica si es subdivisión del CFDI: 0=No, 1=Sí
    pub subdivision: Option<bool>,
    /// Observaciones del exportador
    pub observaciones: Option<String>,
    /// Tipo de cambio (fecha del tipo de cambio SAT)
    pub tipo_cambio_usd: Option<Decimal>,
    /// Total en USD
    pub total_usd: Option<Decimal>,
    /// Emisor (exportador)
    pub emisor: Option<EmisorCE>,
    /// Receptor (importador en el extranjero)
    pub receptor: Option<ReceptorCE>,
    /// Destinatario (si difiere del receptor)
    pub destinatario: Option<Vec<DestinatarioCE>>,
    /// Mercancías exportadas
    pub mercancias: Vec<MercanciaCE>,
}

/// Datos del emisor para comercio exterior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmisorCE {
    /// CURP del exportador (personas físicas)
    pub curp: Option<String>,
}

/// Datos del receptor/importador en el extranjero
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceptorCE {
    /// Número de identificación fiscal del receptor en su país
    pub num_reg_id_trib: String,
    /// País de residencia del receptor (clave ISO 3166-1 alpha-3: USA, DEU, etc.)
    pub residencia_fiscal: String,
}

/// Destinatario (cuando la mercancía se entrega a un tercero)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinatarioCE {
    pub num_reg_id_trib: Option<String>,
    pub nombre: String,
    pub calle: Option<String>,
    pub num_exterior: Option<String>,
    pub colonia: Option<String>,
    pub localidad: Option<String>,
    pub municipio: Option<String>,
    pub estado: String,        // código de estado del país
    pub pais: String,          // ISO 3166-1 alpha-3
    pub codigo_postal: String,
    pub domicilios: Vec<DomicilioCE>,
}

/// Domicilio para destinatario CE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomicilioCE {
    pub calle: Option<String>,
    pub num_exterior: Option<String>,
    pub num_interior: Option<String>,
    pub colonia: Option<String>,
    pub localidad: Option<String>,
    pub referencia: Option<String>,
    pub municipio: Option<String>,
    pub estado: String,
    pub pais: String,
    pub codigo_postal: String,
}

/// Mercancía exportada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MercanciaCE {
    /// No. de identificación de la mercancía (correlaciona con Concepto)
    pub no_identificacion: String,
    /// Fracción arancelaria (10 dígitos TIGIE)
    pub fraccion_arancelaria: Option<String>,
    /// Cantidad con aduana
    pub cantidad_aduana: Option<Decimal>,
    /// Unidad de aduana (clave SAT c_UnidadAduana)
    pub unidad_aduana: Option<String>,
    /// Valor unitario en USD
    pub valor_unitario_aduana: Option<Decimal>,
    /// Valor total en USD
    pub valor_dolares: Decimal,
    /// Descripciones específicas (tallas, colores, etc.)
    pub descripciones_especificas: Vec<DescripcionEspecifica>,
}

/// Descripción específica de la mercancía
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescripcionEspecifica {
    pub marca: String,
    pub modelo: Option<String>,
    pub sub_modelo: Option<String>,
    pub num_serie: Option<String>,
}

// ─── Validaciones ─────────────────────────────────────────────────────────────

/// Valida el complemento de comercio exterior
pub fn validar(ce: &ComercioExterior) -> Result<(), CfdiError> {
    // Tipo de operación debe ser "1" o "2"
    if ce.tipo_operacion != "1" && ce.tipo_operacion != "2" {
        return Err(CfdiError::ValidacionFallida(
            format!("TipoOperacion inválido: {}. Debe ser 1 o 2", ce.tipo_operacion)
        ));
    }

    // Debe haber al menos una mercancía
    if ce.mercancias.is_empty() {
        return Err(CfdiError::ValidacionFallida(
            "ComercioExterior requiere al menos una mercancía".into()
        ));
    }

    // Fracción arancelaria: 10 dígitos si existe
    for m in &ce.mercancias {
        if let Some(ref fa) = m.fraccion_arancelaria {
            if fa.len() != 10 || !fa.chars().all(|c| c.is_ascii_digit()) {
                return Err(CfdiError::ValidacionFallida(
                    format!("FraccionArancelaria inválida: {} (debe ser 10 dígitos)", fa)
                ));
            }
        }
        if m.valor_dolares <= Decimal::ZERO {
            return Err(CfdiError::ValidacionFallida(
                format!("ValorDolares debe ser > 0 para mercancía {}", m.no_identificacion)
            ));
        }
    }

    // Incoterm: 3 letras mayúsculas
    if let Some(ref inc) = ce.incoterm {
        if inc.len() != 3 || !inc.chars().all(|c| c.is_ascii_uppercase()) {
            return Err(CfdiError::ValidacionFallida(
                format!("Incoterm inválido: {} (ej. FOB, CIF, EXW)", inc)
            ));
        }
    }

    Ok(())
}

// ─── Generador XML ────────────────────────────────────────────────────────────

/// Genera el XML del complemento Comercio Exterior 1.1
pub fn generar_xml(ce: &ComercioExterior) -> Result<String, CfdiError> {
    validar(ce)?;

    let mut xml = String::new();

    write!(&mut xml, r#"<cce11:ComercioExterior
    xmlns:cce11="http://www.sat.gob.mx/ComercioExterior11"
    xsi:schemaLocation="http://www.sat.gob.mx/ComercioExterior11 http://www.sat.gob.mx/sitio_internet/cfd/ComercioExterior11/ComercioExterior11.xsd"
    Version="1.1"
    TipoOperacion="{}""#,
        ce.tipo_operacion,
    ).map_err(|e| CfdiError::Xml(e.to_string()))?;

    if let Some(ref co) = ce.cert_origen {
        write!(&mut xml, r#" CertificadoOrigen="{}""#, co)
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }
    if let Some(ref ne) = ce.num_exporter {
        write!(&mut xml, r#" NumExportador="{}""#, ne)
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }
    if let Some(ref cp) = ce.clave_de_pedimento {
        write!(&mut xml, r#" ClaveDePedimento="{}""#, cp)
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }
    if let Some(ref inc) = ce.incoterm {
        write!(&mut xml, r#" Incoterm="{}""#, inc)
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }
    if let Some(sub) = ce.subdivision {
        write!(&mut xml, r#" Subdivision="{}""#, if sub { "1" } else { "0" })
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }
    if let Some(ref obs) = ce.observaciones {
        write!(&mut xml, r#" Observaciones="{}""#, xml_escape(obs))
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }
    if let Some(tc) = ce.tipo_cambio_usd {
        write!(&mut xml, r#" TipoCambioUSD="{:.6}""#, tc)
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }
    if let Some(tu) = ce.total_usd {
        write!(&mut xml, r#" TotalUSD="{:.2}""#, tu)
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }

    writeln!(&mut xml, ">").map_err(|e| CfdiError::Xml(e.to_string()))?;

    // Emisor
    if let Some(ref em) = ce.emisor {
        write!(&mut xml, "  <cce11:Emisor").map_err(|e| CfdiError::Xml(e.to_string()))?;
        if let Some(ref curp) = em.curp {
            write!(&mut xml, r#" Curp="{}""#, curp)
                .map_err(|e| CfdiError::Xml(e.to_string()))?;
        }
        writeln!(&mut xml, "/>").map_err(|e| CfdiError::Xml(e.to_string()))?;
    }

    // Receptor
    if let Some(ref rec) = ce.receptor {
        writeln!(&mut xml,
            r#"  <cce11:Receptor NumRegIdTrib="{}" ResidenciaFiscal="{}"/>"#,
            rec.num_reg_id_trib, rec.residencia_fiscal
        ).map_err(|e| CfdiError::Xml(e.to_string()))?;
    }

    // Destinatarios
    if let Some(ref dests) = ce.destinatario {
        for d in dests {
            write!(&mut xml, r#"  <cce11:Destinatario Nombre="{}""#,
                xml_escape(&d.nombre)
            ).map_err(|e| CfdiError::Xml(e.to_string()))?;
            if let Some(ref nr) = d.num_reg_id_trib {
                write!(&mut xml, r#" NumRegIdTrib="{}""#, nr)
                    .map_err(|e| CfdiError::Xml(e.to_string()))?;
            }
            writeln!(&mut xml, ">").map_err(|e| CfdiError::Xml(e.to_string()))?;
            for dom in &d.domicilios {
                xml_domicilio(&mut xml, dom)?;
            }
            writeln!(&mut xml, "  </cce11:Destinatario>")
                .map_err(|e| CfdiError::Xml(e.to_string()))?;
        }
    }

    // Mercancías
    writeln!(&mut xml, "  <cce11:Mercancias>")
        .map_err(|e| CfdiError::Xml(e.to_string()))?;
    for m in &ce.mercancias {
        write!(&mut xml,
            r#"    <cce11:Mercancia NoIdentificacion="{}" ValorDolares="{:.2}""#,
            m.no_identificacion, m.valor_dolares
        ).map_err(|e| CfdiError::Xml(e.to_string()))?;
        if let Some(ref fa) = m.fraccion_arancelaria {
            write!(&mut xml, r#" FraccionArancelaria="{}""#, fa)
                .map_err(|e| CfdiError::Xml(e.to_string()))?;
        }
        if let Some(ca) = m.cantidad_aduana {
            write!(&mut xml, r#" CantidadAduana="{:.3}""#, ca)
                .map_err(|e| CfdiError::Xml(e.to_string()))?;
        }
        if let Some(ref ua) = m.unidad_aduana {
            write!(&mut xml, r#" UnidadAduana="{}""#, ua)
                .map_err(|e| CfdiError::Xml(e.to_string()))?;
        }
        if let Some(vu) = m.valor_unitario_aduana {
            write!(&mut xml, r#" ValorUnitarioAduana="{:.2}""#, vu)
                .map_err(|e| CfdiError::Xml(e.to_string()))?;
        }

        if m.descripciones_especificas.is_empty() {
            writeln!(&mut xml, "/>").map_err(|e| CfdiError::Xml(e.to_string()))?;
        } else {
            writeln!(&mut xml, ">").map_err(|e| CfdiError::Xml(e.to_string()))?;
            for de in &m.descripciones_especificas {
                write!(&mut xml,
                    r#"      <cce11:DescripcionEspecifica Marca="{}""#,
                    xml_escape(&de.marca)
                ).map_err(|e| CfdiError::Xml(e.to_string()))?;
                if let Some(ref mo) = de.modelo {
                    write!(&mut xml, r#" Modelo="{}""#, xml_escape(mo))
                        .map_err(|e| CfdiError::Xml(e.to_string()))?;
                }
                if let Some(ref sm) = de.sub_modelo {
                    write!(&mut xml, r#" SubModelo="{}""#, xml_escape(sm))
                        .map_err(|e| CfdiError::Xml(e.to_string()))?;
                }
                if let Some(ref ns) = de.num_serie {
                    write!(&mut xml, r#" NumSerie="{}""#, xml_escape(ns))
                        .map_err(|e| CfdiError::Xml(e.to_string()))?;
                }
                writeln!(&mut xml, "/>").map_err(|e| CfdiError::Xml(e.to_string()))?;
            }
            writeln!(&mut xml, "    </cce11:Mercancia>")
                .map_err(|e| CfdiError::Xml(e.to_string()))?;
        }
    }
    writeln!(&mut xml, "  </cce11:Mercancias>")
        .map_err(|e| CfdiError::Xml(e.to_string()))?;
    writeln!(&mut xml, "</cce11:ComercioExterior>")
        .map_err(|e| CfdiError::Xml(e.to_string()))?;

    Ok(xml)
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn xml_domicilio(xml: &mut String, dom: &DomicilioCE) -> Result<(), CfdiError> {
    write!(xml,
        r#"    <cce11:Domicilio Estado="{}" Pais="{}" CodigoPostal="{}""#,
        dom.estado, dom.pais, dom.codigo_postal
    ).map_err(|e| CfdiError::Xml(e.to_string()))?;
    if let Some(ref c) = dom.calle {
        write!(xml, r#" Calle="{}""#, xml_escape(c))
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }
    if let Some(ref ne) = dom.num_exterior {
        write!(xml, r#" NumeroExterior="{}""#, ne)
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }
    if let Some(ref ni) = dom.num_interior {
        write!(xml, r#" NumeroInterior="{}""#, ni)
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }
    if let Some(ref col) = dom.colonia {
        write!(xml, r#" Colonia="{}""#, xml_escape(col))
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }
    if let Some(ref mun) = dom.municipio {
        write!(xml, r#" Municipio="{}""#, xml_escape(mun))
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }
    writeln!(xml, "/>").map_err(|e| CfdiError::Xml(e.to_string()))?;
    Ok(())
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&apos;")
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn ce_basico() -> ComercioExterior {
        ComercioExterior {
            cert_origen: None,
            num_exporter: None,
            tipo_operacion: "1".into(),
            clave_de_pedimento: Some("A1".into()),
            incoterm: Some("FOB".into()),
            subdivision: Some(false),
            observaciones: None,
            tipo_cambio_usd: Some(dec!(17.5432)),
            total_usd: Some(dec!(1000.00)),
            emisor: None,
            receptor: Some(ReceptorCE {
                num_reg_id_trib: "123456789".into(),
                residencia_fiscal: "USA".into(),
            }),
            destinatario: None,
            mercancias: vec![
                MercanciaCE {
                    no_identificacion: "SKU-001".into(),
                    fraccion_arancelaria: Some("8471300000".into()),
                    cantidad_aduana: Some(dec!(10.000)),
                    unidad_aduana: Some("06".into()),
                    valor_unitario_aduana: Some(dec!(100.00)),
                    valor_dolares: dec!(1000.00),
                    descripciones_especificas: vec![],
                }
            ],
        }
    }

    #[test]
    fn test_validacion_ok() {
        assert!(validar(&ce_basico()).is_ok());
    }

    #[test]
    fn test_fraccion_arancelaria_invalida() {
        let mut ce = ce_basico();
        ce.mercancias[0].fraccion_arancelaria = Some("12345".into()); // solo 5 dígitos
        assert!(validar(&ce).is_err());
    }

    #[test]
    fn test_incoterm_invalido() {
        let mut ce = ce_basico();
        ce.incoterm = Some("fob".into()); // minúsculas
        assert!(validar(&ce).is_err());
    }

    #[test]
    fn test_xml_generado() {
        let xml = generar_xml(&ce_basico()).expect("XML generado");
        assert!(xml.contains("cce11:ComercioExterior"), "Debe tener nodo raíz");
        assert!(xml.contains("TipoOperacion=\"1\""));
        assert!(xml.contains("Incoterm=\"FOB\""));
        assert!(xml.contains("8471300000"), "Debe tener fracción arancelaria");
        assert!(xml.contains("ValorDolares=\"1000.00\""));
        assert!(xml.contains("ResidenciaFiscal=\"USA\""));
    }

    #[test]
    fn test_xml_escape() {
        assert_eq!(xml_escape("AT&T"), "AT&amp;T");
        assert_eq!(xml_escape("<tag>"), "&lt;tag&gt;");
        assert_eq!(xml_escape("say \"hi\""), "say &quot;hi&quot;");
    }
}
