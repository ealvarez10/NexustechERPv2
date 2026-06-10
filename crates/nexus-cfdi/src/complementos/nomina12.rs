//! Complemento de Nómina 1.2
//! XSD: http://www.sat.gob.mx/nomina12
//!
//! Requerido para el pago de sueldos y salarios.
//! TODO: Implementación completa pendiente
//! Prioridad: Media

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nomina {
    pub version: String,          // "1.2"
    pub tipo_nomina: String,       // "O"=ordinaria, "E"=extraordinaria
    pub fecha_pago: String,
    pub fecha_inicial_pago: String,
    pub fecha_final_pago: String,
    pub num_dias_pagados: f64,
    pub total_percepciones: Option<f64>,
    pub total_deducciones: Option<f64>,
    pub total_otros_pagos: Option<f64>,
    pub emisor: EmisorNomina,
    pub receptor: ReceptorNomina,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmisorNomina {
    pub registro_patronal: Option<String>,
    pub rfc_patron_origen: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceptorNomina {
    pub curp: String,
    pub num_seguridad_social: Option<String>,
    pub fecha_inicio_rel_laboral: Option<String>,
    pub antigüedad: Option<String>,
    pub tipo_contrato: String,
    pub sindicalizado: Option<String>,
    pub tipo_jornada: Option<String>,
    pub tipo_regimen: String,
    pub num_empleado: String,
    pub departamento: Option<String>,
    pub puesto: Option<String>,
    pub riesgo_puesto: Option<String>,
    pub periodicidad_pago: String,
    pub banco: Option<String>,
    pub cuenta_bancaria: Option<String>,
    pub salario_base_cot_apor: Option<f64>,
    pub salario_diario_integrado: Option<f64>,
    pub clave_ent_fed: String,
}

// TODO: generar_xml() — pendiente
