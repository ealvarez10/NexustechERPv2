//! Builder y tipos de datos del CFDI 4.0

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Datos completos de un CFDI 4.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfdiData {
    // Atributos del Comprobante
    pub version: String,
    pub serie: Option<String>,
    pub folio: Option<String>,
    pub fecha: String,           // ISO 8601: YYYY-MM-DDTHH:MM:SS
    pub forma_pago: Option<String>,
    pub no_certificado: String,
    pub condiciones_de_pago: Option<String>,
    pub sub_total: Decimal,
    pub descuento: Option<Decimal>,
    pub moneda: String,          // MXN, USD, etc.
    pub tipo_cambio: Option<Decimal>,
    pub total: Decimal,
    pub tipo_de_comprobante: String, // I=ingreso, E=egreso, T=traslado, N=nómina, P=pago
    pub exportacion: Option<String>,
    pub metodo_pago: Option<String>, // PUE, PPD
    pub lugar_expedicion: String,    // Código postal
    pub confirmacion: Option<String>,
    pub informacion_global: Option<InformacionGlobal>,
    pub emisor: Emisor,
    pub receptor: Receptor,
    pub conceptos: Vec<Concepto>,
    pub impuestos: Option<Impuestos>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InformacionGlobal {
    pub periodicidad: String, // 01=diario, 02=semanal, 03=quincenal, 04=mensual, 05=bimestral
    pub meses: String,        // 01-12
    pub año: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emisor {
    pub rfc: String,
    pub nombre: String,
    pub regimen_fiscal: String, // 601, 612, 626, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receptor {
    pub rfc: String,
    pub nombre: String,
    pub domicilio_fiscal_receptor: Option<String>, // CP del receptor
    pub residencia_fiscal: Option<String>,
    pub num_reg_id_trib: Option<String>,
    pub regimen_fiscal_receptor: String,
    pub uso_cfdi: String, // G01, G03, S01, CP01, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concepto {
    pub clave_prod_serv: String,  // Catálogo SAT c_ClaveProdServ
    pub no_identificacion: Option<String>,
    pub cantidad: Decimal,
    pub clave_unidad: String,     // Catálogo SAT c_ClaveUnidad (E48, H87, etc.)
    pub unidad: Option<String>,
    pub descripcion: String,
    pub valor_unitario: Decimal,
    pub importe: Decimal,
    pub descuento: Option<Decimal>,
    pub objeto_imp: Option<String>, // 01=no objeto, 02=sí objeto, 03=sí objeto no obligado
    pub traslados: Vec<Traslado>,
    pub retenciones: Vec<Retencion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Traslado {
    pub base: Decimal,
    pub impuesto: String,      // 001=ISR, 002=IVA, 003=IEPS
    pub tipo_factor: String,   // Tasa, Cuota, Exento
    pub tasa_o_cuota: Decimal, // 0.160000 para IVA 16%
    pub importe: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Retencion {
    pub base: Decimal,
    pub impuesto: String,
    pub tipo_factor: String,
    pub tasa_o_cuota: Decimal,
    pub importe: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Impuestos {
    pub total_impuestos_retenidos: Option<Decimal>,
    pub total_impuestos_trasladados: Option<Decimal>,
    pub traslados: Vec<Traslado>,
    pub retenciones: Vec<RetencionGlobal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetencionGlobal {
    pub impuesto: String,
    pub importe: Decimal,
}

/// Builder fluido para construir un CFDI paso a paso
#[derive(Default)]
pub struct CfdiBuilder {
    #[allow(dead_code)]
    data: Option<CfdiData>,
    serie: Option<String>,
    folio: Option<String>,
    emisor: Option<Emisor>,
    receptor: Option<Receptor>,
    conceptos: Vec<Concepto>,
    forma_pago: Option<String>,
    metodo_pago: Option<String>,
    moneda: String,
    lugar_expedicion: String,
    tipo_de_comprobante: String,
}

impl CfdiBuilder {
    pub fn new() -> Self {
        Self {
            moneda: "MXN".into(),
            tipo_de_comprobante: "I".into(),
            ..Default::default()
        }
    }

    pub fn serie(mut self, serie: impl Into<String>) -> Self {
        self.serie = Some(serie.into()); self
    }

    pub fn folio(mut self, folio: impl Into<String>) -> Self {
        self.folio = Some(folio.into()); self
    }

    pub fn emisor(mut self, emisor: Emisor) -> Self {
        self.emisor = Some(emisor); self
    }

    pub fn receptor(mut self, receptor: Receptor) -> Self {
        self.receptor = Some(receptor); self
    }

    pub fn concepto(mut self, concepto: Concepto) -> Self {
        self.conceptos.push(concepto); self
    }

    pub fn forma_pago(mut self, forma: impl Into<String>) -> Self {
        self.forma_pago = Some(forma.into()); self
    }

    pub fn metodo_pago(mut self, metodo: impl Into<String>) -> Self {
        self.metodo_pago = Some(metodo.into()); self
    }

    pub fn moneda(mut self, moneda: impl Into<String>) -> Self {
        self.moneda = moneda.into(); self
    }

    pub fn lugar_expedicion(mut self, cp: impl Into<String>) -> Self {
        self.lugar_expedicion = cp.into(); self
    }

    pub fn tipo_comprobante(mut self, tipo: impl Into<String>) -> Self {
        self.tipo_de_comprobante = tipo.into(); self
    }

    /// Construye el CfdiData calculando subtotal, impuestos y total automáticamente
    pub fn build(self, no_certificado: String) -> Result<CfdiData, crate::error::CfdiError> {
        let emisor = self.emisor.ok_or_else(|| crate::error::CfdiError::CampoRequerido("emisor".into()))?;
        let receptor = self.receptor.ok_or_else(|| crate::error::CfdiError::CampoRequerido("receptor".into()))?;
        if self.lugar_expedicion.is_empty() {
            return Err(crate::error::CfdiError::CampoRequerido("lugar_expedicion".into()));
        }

        // Calcular subtotal (suma de importes de conceptos)
        let sub_total: Decimal = self.conceptos.iter().map(|c| c.importe).sum();

        // Calcular total de traslados
        let total_traslados: Decimal = self.conceptos.iter()
            .flat_map(|c| c.traslados.iter())
            .map(|t| t.importe)
            .sum();

        let total_retenciones: Decimal = self.conceptos.iter()
            .flat_map(|c| c.retenciones.iter())
            .map(|r| r.importe)
            .sum();

        let total = sub_total + total_traslados - total_retenciones;

        // Agrupar traslados globales
        let traslados_globales = agrupar_traslados(&self.conceptos);
        let retenciones_globales = agrupar_retenciones(&self.conceptos);

        let impuestos = if !traslados_globales.is_empty() || !retenciones_globales.is_empty() {
            Some(Impuestos {
                total_impuestos_trasladados: if total_traslados > Decimal::ZERO { Some(total_traslados) } else { None },
                total_impuestos_retenidos: if total_retenciones > Decimal::ZERO { Some(total_retenciones) } else { None },
                traslados: traslados_globales,
                retenciones: retenciones_globales,
            })
        } else { None };

        // Fecha actual en formato SAT
        let fecha = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();

        Ok(CfdiData {
            version: "4.0".into(),
            serie: self.serie,
            folio: self.folio,
            fecha,
            forma_pago: self.forma_pago,
            no_certificado,
            condiciones_de_pago: None,
            sub_total,
            descuento: None,
            moneda: self.moneda,
            tipo_cambio: None,
            total,
            tipo_de_comprobante: self.tipo_de_comprobante,
            exportacion: Some("01".into()),
            metodo_pago: self.metodo_pago,
            lugar_expedicion: self.lugar_expedicion,
            confirmacion: None,
            informacion_global: None,
            emisor,
            receptor,
            conceptos: self.conceptos,
            impuestos,
        })
    }
}

fn agrupar_traslados(conceptos: &[Concepto]) -> Vec<Traslado> {
    use std::collections::HashMap;
    let mut mapa: HashMap<String, (Decimal, Decimal, Decimal, String, String)> = HashMap::new();
    for c in conceptos {
        for t in &c.traslados {
            let key = format!("{}_{}", t.impuesto, t.tipo_factor);
            let entry = mapa.entry(key).or_insert((Decimal::ZERO, t.tasa_o_cuota, Decimal::ZERO, t.impuesto.clone(), t.tipo_factor.clone()));
            entry.0 += t.base;
            entry.2 += t.importe;
        }
    }
    mapa.into_values().map(|(base, tasa, importe, impuesto, tipo)| Traslado {
        base, impuesto, tipo_factor: tipo, tasa_o_cuota: tasa, importe,
    }).collect()
}

fn agrupar_retenciones(conceptos: &[Concepto]) -> Vec<RetencionGlobal> {
    use std::collections::HashMap;
    let mut mapa: HashMap<String, Decimal> = HashMap::new();
    for c in conceptos {
        for r in &c.retenciones {
            *mapa.entry(r.impuesto.clone()).or_insert(Decimal::ZERO) += r.importe;
        }
    }
    mapa.into_iter().map(|(impuesto, importe)| RetencionGlobal { impuesto, importe }).collect()
}
