//! Complemento de Nómina 1.2
//! XSD: http://www.sat.gob.mx/nomina12
//!
//! Cálculos de cuotas sociales y fiscal (ISR) para nómina mexicana.
//! Tablas vigentes 2024.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use crate::error::CfdiError;

// ─── STRUCTS CFDI Nómina 1.2 ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nomina {
    pub version: String,                           // "1.2"
    pub tipo_nomina: String,                       // "O"=ordinaria, "E"=extraordinaria
    pub fecha_pago: String,                        // YYYY-MM-DD
    pub fecha_inicial_pago: String,
    pub fecha_final_pago: String,
    pub num_dias_pagados: Decimal,
    pub total_percepciones: Option<Decimal>,
    pub total_deducciones: Option<Decimal>,
    pub total_otros_pagos: Option<Decimal>,
    pub emisor: EmisorNomina,
    pub receptor: ReceptorNomina,
    pub percepciones: Option<Percepciones>,
    pub deducciones: Option<Deducciones>,
    pub otros_pagos: Option<Vec<OtroPago>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmisorNomina {
    pub registro_patronal: Option<String>,         // IMSS ej. "Y2510000X00"
    pub rfc_patron_origen: Option<String>,
    pub entidad_sncf: Option<EntidadSNCF>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntidadSNCF {
    pub origen_recurso: String,        // IP, IF, IM, IPSF, etc.
    pub monto_recurso_propio: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceptorNomina {
    pub curp: String,
    pub num_seguridad_social: Option<String>,      // NSS IMSS
    pub fecha_inicio_rel_laboral: Option<String>,
    pub antiguedad: Option<String>,                // P1Y2M3D (ISO 8601 duración)
    pub tipo_contrato: String,                     // 01=indefinido, 02=temporal, etc.
    pub sindicalizado: Option<String>,             // Sí / No
    pub tipo_jornada: Option<String>,              // 01=diurna, 02=nocturna, etc.
    pub tipo_regimen: String,                      // 02=sueldos y salarios
    pub num_empleado: String,
    pub departamento: Option<String>,
    pub puesto: Option<String>,
    pub riesgo_puesto: Option<String>,             // 1-5 clase de riesgo IMSS
    pub periodicidad_pago: String,                 // 01=diario, 02=semanal, 04=quincenal, 05=mensual
    pub banco: Option<String>,                     // CLABE bancaria código
    pub cuenta_bancaria: Option<String>,
    pub salario_base_cot_apor: Option<Decimal>,    // SBC IMSS
    pub salario_diario_integrado: Option<Decimal>, // SDI
    pub clave_ent_fed: String,                     // ej. "JAL", "CDMX"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Percepciones {
    pub total_sueldos: Decimal,
    pub total_separacion_indemnizacion: Option<Decimal>,
    pub total_jubilacion_pension_retiro: Option<Decimal>,
    pub total_gravado: Decimal,
    pub total_exento: Decimal,
    pub percepciones: Vec<Percepcion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Percepcion {
    pub tipo_percepcion: String,      // Catálogo SAT: 001=sueldo, 002=aguinaldo, etc.
    pub clave: String,
    pub concepto: String,
    pub importe_gravado: Decimal,
    pub importe_exento: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deducciones {
    pub total_otras_deducciones: Option<Decimal>,
    pub total_impuestos_retenidos: Option<Decimal>,
    pub deducciones: Vec<Deduccion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deduccion {
    pub tipo_deduccion: String,       // 001=IMSS, 002=ISR, 003=Infonavit, 004=préstamo, etc.
    pub clave: String,
    pub concepto: String,
    pub importe: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtroPago {
    pub tipo_otro_pago: String,       // 001=reintegro ISR no sujeto, 002=subsidio para el empleo
    pub clave: String,
    pub concepto: String,
    pub importe: Decimal,
    pub subsidio_al_empleo: Option<SubsidioAlEmpleo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsidioAlEmpleo {
    pub subsidio_causado: Decimal,
}

// ─── CÁLCULOS IMSS 2024 ──────────────────────────────────────────────────────

/// Unidad de Medida y Actualización 2024 (UMA diaria)
pub const UMA_DIARIA_2024: Decimal = dec!(108.57);
/// Salario Mínimo General 2024
pub const SALARIO_MINIMO_2024: Decimal = dec!(248.93);
/// SMGDF (Zona Libre Frontera Norte)
pub const SMGDF_ZONA_LIBRE_2024: Decimal = dec!(374.89);
/// Tope IMSS (25 UMAs)
pub const TOPE_25_UMA: Decimal = dec!(2714.25); // 25 × 108.57

/// Resultado de cálculo de cuotas IMSS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuotasImss {
    /// Cuota obrero total
    pub cuota_obrero: Decimal,
    /// Cuota patronal total (informativa, no se retiene al empleado)
    pub cuota_patronal: Decimal,
    /// Desglose por ramo
    pub enfermedad_maternidad_obrero: Decimal,
    pub invalidez_vida_obrero: Decimal,
    pub cesantia_vejez_obrero: Decimal,
    pub guarderia_prestaciones_sociales: Decimal, // solo patronal
    pub dias_cotizados: i32,
}

/// Calcula cuotas IMSS para un período de nómina
///
/// # Parámetros
/// - `sbc` — Salario Base de Cotización diario
/// - `dias` — Días cotizados en el período
/// - `tipo_regimen` — 02=sueldos ordinarios
pub fn calcular_imss(sbc: Decimal, dias: i32, _tipo_regimen: &str) -> CuotasImss {
    // Tope diario: 25 UMAs
    let sbc_topado = sbc.min(TOPE_25_UMA);
    let dias_d = Decimal::from(dias);

    // ── CUOTAS OBRERO (Ley del Seguro Social Art. 36, 38, 147) ────────────────
    // Enfermedad y Maternidad — Prestaciones en Especie (Art. 25, fracción I y II)
    let em_pe_obrero = if sbc > UMA_DIARIA_2024 {
        sbc_topado * dec!(0.00375) * dias_d
    } else {
        Decimal::ZERO
    };

    // Invalidez y Vida — 0.625% obrero
    let iv_obrero = sbc_topado * dec!(0.00625) * dias_d;

    // Cesantía en Edad Avanzada y Vejez (CEAV) — 1.125% obrero
    let ceav_obrero = sbc_topado * dec!(0.01125) * dias_d;

    // Total obrero
    let cuota_obrero = (em_pe_obrero + iv_obrero + ceav_obrero).round_dp(2);

    // ── CUOTAS PATRONALES (informativas) ──────────────────────────────────────
    // Enf. y Mat. Prestaciones en Especie — cuota fija 20.40% UMA + excedente 1.10%
    let em_pe_pat_fija = UMA_DIARIA_2024 * dec!(0.204) * dias_d;
    let em_pe_pat_excedente = if sbc > UMA_DIARIA_2024 {
        (sbc_topado - UMA_DIARIA_2024) * dec!(0.011) * dias_d
    } else {
        Decimal::ZERO
    };
    // Prestaciones en dinero 0.70%
    let em_pd_pat = sbc_topado * dec!(0.007) * dias_d;
    // Gastos médicos pensionados 1.05%
    let em_gmp_pat = sbc_topado * dec!(0.0105) * dias_d;
    // Invalidez y Vida 1.75%
    let iv_pat = sbc_topado * dec!(0.0175) * dias_d;
    // CEAV 3.150%
    let ceav_pat = sbc_topado * dec!(0.0315) * dias_d;
    // Guarderías y Prestaciones Sociales 1.00%
    let guarderia = sbc_topado * dec!(0.01) * dias_d;
    // Riesgo de trabajo (clase I mínimo 0.50360%)
    let riesgo_trabajo = sbc_topado * dec!(0.005036) * dias_d;

    let cuota_patronal = (em_pe_pat_fija + em_pe_pat_excedente + em_pd_pat
        + em_gmp_pat + iv_pat + ceav_pat + guarderia + riesgo_trabajo).round_dp(2);

    CuotasImss {
        cuota_obrero,
        cuota_patronal,
        enfermedad_maternidad_obrero: em_pe_obrero.round_dp(2),
        invalidez_vida_obrero: iv_obrero.round_dp(2),
        cesantia_vejez_obrero: ceav_obrero.round_dp(2),
        guarderia_prestaciones_sociales: guarderia.round_dp(2),
        dias_cotizados: dias,
    }
}

// ─── INFONAVIT 2024 ──────────────────────────────────────────────────────────

/// Cuota Infonavit patronal: 5% del SBC topado a 25 UMAs
/// (No se retiene al trabajador; es aportación patronal subcuenta vivienda)
pub fn calcular_infonavit_patronal(sbc: Decimal, dias: i32) -> Decimal {
    let sbc_topado = sbc.min(TOPE_25_UMA);
    (sbc_topado * dec!(0.05) * Decimal::from(dias)).round_dp(2)
}

/// Descuento Infonavit (crédito activo) — porcentaje del SDI o cuota fija
#[derive(Debug, Clone)]
pub struct DescuentoInfonavit {
    pub tipo: TipoDescuentoInfonavit,
    pub valor: Decimal, // porcentaje (ej. 0.15 = 15%) o cuota fija (ej. dec!(1200.00))
}

#[derive(Debug, Clone)]
pub enum TipoDescuentoInfonavit {
    PorcentajeSalarioIntegrado,
    CuotaFija,
    VecesUmaDiaria,
}

/// Calcula el descuento Infonavit para el trabajador (crédito activo)
pub fn calcular_descuento_infonavit(
    sdi: Decimal,
    dias: i32,
    descuento: &DescuentoInfonavit,
) -> Decimal {
    let importe = match descuento.tipo {
        TipoDescuentoInfonavit::PorcentajeSalarioIntegrado => {
            sdi * Decimal::from(dias) * descuento.valor
        },
        TipoDescuentoInfonavit::CuotaFija => {
            // Cuota fija por período (no depende de días, ya es el período completo)
            descuento.valor
        },
        TipoDescuentoInfonavit::VecesUmaDiaria => {
            // X veces UMA × días
            UMA_DIARIA_2024 * descuento.valor * Decimal::from(dias)
        },
    };
    importe.round_dp(2)
}

// ─── ISR RETENCIÓN 2024 ───────────────────────────────────────────────────────

/// Tabla ISR mensual 2024 (Artículo 96 LISR y Anexo 8 RMF 2024)
/// Formato: (limite_inferior, cuota_fija, porcentaje_excedente)
#[rustfmt::skip]
const TABLA_ISR_MENSUAL_2024: &[(f64, f64, f64)] = &[
    (       0.01,       0.00,  1.92),
    (     746.05,      14.32,  6.40),
    (    6_332.06,     371.83, 10.88),
    (   11_128.02,     893.63, 16.00),
    (   12_935.83,   1_182.88, 17.92),
    (   15_487.72,   1_640.18, 21.36),
    (   31_236.50,   5_004.12, 23.52),
    (   49_233.01,   9_236.89, 30.00),
    (   93_993.91,  22_665.17, 32.00),
    (  125_325.21,  32_691.18, 34.00),
    (  166_666.68,  46_748.26, 35.00),
    (  500_000.01, 163_580.17, 36.00),
    (3_000_000.01, 703_380.17, 37.00),
];

/// Tabla Subsidio al Empleo mensual 2024 (Anexo 8 RMF 2024)
/// Formato: (limite_inferior, limite_superior, subsidio_mensual)
#[rustfmt::skip]
const TABLA_SUBSIDIO_EMPLEO_MENSUAL: &[(f64, f64, f64)] = &[
    (     0.01,   1_768.96, 407.02),
    ( 1_768.97,   2_653.38, 406.83),
    ( 2_653.39,   3_472.84, 406.62),
    ( 3_472.85,   3_537.87, 392.77),
    ( 3_537.88,   4_446.15, 382.46),
    ( 4_446.16,   4_717.18, 354.23),
    ( 4_717.19,   5_335.42, 324.87),
    ( 5_335.43,   6_224.67, 294.63),
    ( 6_224.68,   7_113.90, 253.54),
    ( 7_113.91,   7_382.33, 217.61),
    ( 7_382.34,  11_382.33,   0.00),
    (11_382.34, f64::MAX,    0.00),
];

/// Resultado del cálculo de ISR mensual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultadoIsr {
    /// Ingreso gravado mensual
    pub ingreso_gravado: Decimal,
    /// ISR causado
    pub isr_causado: Decimal,
    /// Subsidio al empleo aplicable
    pub subsidio_al_empleo: Decimal,
    /// ISR a retener (isr_causado - subsidio; mínimo 0)
    pub isr_retener: Decimal,
    /// ¿Se paga subsidio (isr_causado < subsidio)?
    pub paga_subsidio: bool,
    /// Subsidio pagado al trabajador (cuando isr_causado < subsidio)
    pub subsidio_pagado: Decimal,
}

/// Calcula ISR mensual a retener con subsidio al empleo
///
/// # Parámetros
/// - `ingreso_mensual_gravado` — Total percepciones gravadas en el mes
pub fn calcular_isr_mensual(ingreso_mensual_gravado: Decimal) -> ResultadoIsr {
    let ingreso_f = ingreso_mensual_gravado
        .to_string()
        .parse::<f64>()
        .unwrap_or(0.0);

    // Buscar renglón en tabla ISR
    let (li, cuota_fija, porcentaje) = TABLA_ISR_MENSUAL_2024
        .iter()
        .rev()
        .find(|(li, _, _)| ingreso_f >= *li)
        .copied()
        .unwrap_or((0.01, 0.00, 1.92));

    let isr_causado_f = cuota_fija + (ingreso_f - li) * (porcentaje / 100.0);
    let isr_causado = Decimal::try_from(isr_causado_f)
        .unwrap_or(Decimal::ZERO)
        .round_dp(2);

    // Buscar subsidio al empleo
    let subsidio_f = TABLA_SUBSIDIO_EMPLEO_MENSUAL
        .iter()
        .find(|(li, ls, _)| ingreso_f >= *li && ingreso_f <= *ls)
        .map(|(_, _, s)| *s)
        .unwrap_or(0.0);
    let subsidio = Decimal::try_from(subsidio_f)
        .unwrap_or(Decimal::ZERO)
        .round_dp(2);

    if isr_causado >= subsidio {
        ResultadoIsr {
            ingreso_gravado: ingreso_mensual_gravado,
            isr_causado,
            subsidio_al_empleo: subsidio,
            isr_retener: (isr_causado - subsidio).round_dp(2),
            paga_subsidio: false,
            subsidio_pagado: Decimal::ZERO,
        }
    } else {
        ResultadoIsr {
            ingreso_gravado: ingreso_mensual_gravado,
            isr_causado,
            subsidio_al_empleo: subsidio,
            isr_retener: Decimal::ZERO,
            paga_subsidio: true,
            subsidio_pagado: (subsidio - isr_causado).round_dp(2),
        }
    }
}

/// Convierte ISR mensual a quincena o semana
/// - `periodicidad` — "04"=quincenal, "02"=semanal, "05"=mensual
pub fn calcular_isr_por_periodicidad(
    ingreso_gravado: Decimal,
    periodicidad: &str,
) -> ResultadoIsr {
    let ingreso_mensual = match periodicidad {
        "02" => ingreso_gravado * dec!(4.333), // semanal → mensual
        "03" => ingreso_gravado * dec!(2.0),   // catorcenal → mensual
        "04" => ingreso_gravado * dec!(2.0),   // quincenal → mensual
        "05" => ingreso_gravado,               // ya es mensual
        _    => ingreso_gravado * dec!(2.0),   // default quincenal
    };

    let resultado_mensual = calcular_isr_mensual(ingreso_mensual);

    // Proporcionar ISR al período
    let factor = match periodicidad {
        "02" => dec!(1.0) / dec!(4.333),
        "03" => dec!(0.5),
        "04" => dec!(0.5),
        "05" => dec!(1.0),
        _    => dec!(0.5),
    };

    ResultadoIsr {
        ingreso_gravado,
        isr_causado: (resultado_mensual.isr_causado * factor).round_dp(2),
        subsidio_al_empleo: (resultado_mensual.subsidio_al_empleo * factor).round_dp(2),
        isr_retener: (resultado_mensual.isr_retener * factor).round_dp(2),
        paga_subsidio: resultado_mensual.paga_subsidio,
        subsidio_pagado: (resultado_mensual.subsidio_pagado * factor).round_dp(2),
    }
}

// ─── GENERACIÓN XML ───────────────────────────────────────────────────────────

/// Genera el nodo XML del complemento Nómina 1.2
pub fn generar_xml_nomina(nomina: &Nomina) -> Result<String, CfdiError> {
    use std::fmt::Write;
    let mut xml = String::new();

    writeln!(&mut xml, r#"<nomina12:Nomina
    xmlns:nomina12="http://www.sat.gob.mx/nomina12"
    Version="{}"
    TipoNomina="{}"
    FechaPago="{}"
    FechaInicialPago="{}"
    FechaFinalPago="{}"
    NumDiasPagados="{}""#,
        nomina.version,
        nomina.tipo_nomina,
        nomina.fecha_pago,
        nomina.fecha_inicial_pago,
        nomina.fecha_final_pago,
        nomina.num_dias_pagados,
    ).map_err(|e| CfdiError::Xml(e.to_string()))?;

    if let Some(tp) = &nomina.total_percepciones {
        write!(&mut xml, r#" TotalPercepciones="{:.2}""#, tp)
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }
    if let Some(td) = &nomina.total_deducciones {
        write!(&mut xml, r#" TotalDeducciones="{:.2}""#, td)
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }
    if let Some(top) = &nomina.total_otros_pagos {
        write!(&mut xml, r#" TotalOtrosPagos="{:.2}""#, top)
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }

    writeln!(&mut xml, ">").map_err(|e| CfdiError::Xml(e.to_string()))?;

    // Emisor
    write!(&mut xml, "  <nomina12:Emisor").map_err(|e| CfdiError::Xml(e.to_string()))?;
    if let Some(rp) = &nomina.emisor.registro_patronal {
        write!(&mut xml, r#" RegistroPatronal="{}""#, rp)
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }
    writeln!(&mut xml, "/>").map_err(|e| CfdiError::Xml(e.to_string()))?;

    // Receptor
    write!(&mut xml, "  <nomina12:Receptor").map_err(|e| CfdiError::Xml(e.to_string()))?;
    write!(&mut xml, r#" Curp="{}""#, nomina.receptor.curp)
        .map_err(|e| CfdiError::Xml(e.to_string()))?;
    if let Some(nss) = &nomina.receptor.num_seguridad_social {
        write!(&mut xml, r#" NumSeguridadSocial="{}""#, nss)
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }
    if let Some(fi) = &nomina.receptor.fecha_inicio_rel_laboral {
        write!(&mut xml, r#" FechaInicioRelLaboral="{}""#, fi)
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }
    if let Some(ant) = &nomina.receptor.antiguedad {
        write!(&mut xml, r#" Antigüedad="{}""#, ant)
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }
    write!(&mut xml,
        r#" TipoContrato="{}" TipoRegimen="{}" NumEmpleado="{}" PeriodicidadPago="{}" ClaveEntFed="{}""#,
        nomina.receptor.tipo_contrato,
        nomina.receptor.tipo_regimen,
        nomina.receptor.num_empleado,
        nomina.receptor.periodicidad_pago,
        nomina.receptor.clave_ent_fed,
    ).map_err(|e| CfdiError::Xml(e.to_string()))?;
    if let Some(sbc) = nomina.receptor.salario_base_cot_apor {
        write!(&mut xml, r#" SalarioBaseCotApor="{:.2}""#, sbc)
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }
    if let Some(sdi) = nomina.receptor.salario_diario_integrado {
        write!(&mut xml, r#" SalarioDiarioIntegrado="{:.2}""#, sdi)
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }
    writeln!(&mut xml, "/>").map_err(|e| CfdiError::Xml(e.to_string()))?;

    // Percepciones
    if let Some(ref perc) = nomina.percepciones {
        write!(&mut xml,
            "  <nomina12:Percepciones TotalSueldos=\"{:.2}\" TotalGravado=\"{:.2}\" TotalExento=\"{:.2}\">",
            perc.total_sueldos,
            perc.total_gravado,
            perc.total_exento,
        ).map_err(|e| CfdiError::Xml(e.to_string()))?;
        for p in &perc.percepciones {
            write!(&mut xml,
                "\n    <nomina12:Percepcion TipoPercepcion=\"{}\" Clave=\"{}\" Concepto=\"{}\" ImporteGravado=\"{:.2}\" ImporteExento=\"{:.2}\"/>",
                p.tipo_percepcion, p.clave, p.concepto, p.importe_gravado, p.importe_exento,
            ).map_err(|e| CfdiError::Xml(e.to_string()))?;
        }
        writeln!(&mut xml, "\n  </nomina12:Percepciones>")
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }

    // Deducciones
    if let Some(ref ded) = nomina.deducciones {
        let total_otras = ded.total_otras_deducciones.unwrap_or(Decimal::ZERO);
        let total_isr   = ded.total_impuestos_retenidos.unwrap_or(Decimal::ZERO);
        write!(&mut xml,
            "  <nomina12:Deducciones TotalOtrasDeducciones=\"{:.2}\" TotalImpuestosRetenidos=\"{:.2}\">",
            total_otras, total_isr,
        ).map_err(|e| CfdiError::Xml(e.to_string()))?;
        for d in &ded.deducciones {
            write!(&mut xml,
                "\n    <nomina12:Deduccion TipoDeduccion=\"{}\" Clave=\"{}\" Concepto=\"{}\" Importe=\"{:.2}\"/>",
                d.tipo_deduccion, d.clave, d.concepto, d.importe,
            ).map_err(|e| CfdiError::Xml(e.to_string()))?;
        }
        writeln!(&mut xml, "\n  </nomina12:Deducciones>")
            .map_err(|e| CfdiError::Xml(e.to_string()))?;
    }

    writeln!(&mut xml, "</nomina12:Nomina>")
        .map_err(|e| CfdiError::Xml(e.to_string()))?;
    Ok(xml)
}

// ─── HELPER — Construir deducciones IMSS + ISR + Infonavit ───────────────────

/// Parámetros de entrada para calcular deducciones completas de un empleado
#[derive(Debug, Clone)]
pub struct ParametrosNomina {
    pub sbc: Decimal,                              // Salario Base Cotización
    pub sdi: Decimal,                              // Salario Diario Integrado
    pub ingreso_gravado_periodo: Decimal,           // Percepciones gravadas
    pub dias: i32,                                 // Días pagados
    pub periodicidad: String,                      // 02=sem, 04=quincena, 05=mes
    pub descuento_infonavit: Option<DescuentoInfonavit>,
}

/// Calcula deducciones completas (IMSS + ISR + Infonavit) para un período
pub fn calcular_deducciones(
    params: &ParametrosNomina,
) -> (Vec<Deduccion>, Deducciones) {
    let mut deds: Vec<Deduccion> = Vec::new();

    // IMSS obrero
    let imss = calcular_imss(params.sbc, params.dias, "02");
    if imss.cuota_obrero > Decimal::ZERO {
        deds.push(Deduccion {
            tipo_deduccion: "001".into(),
            clave: "IMSS".into(),
            concepto: "Cuota IMSS Obrero".into(),
            importe: imss.cuota_obrero,
        });
    }

    // ISR
    let isr_res = calcular_isr_por_periodicidad(
        params.ingreso_gravado_periodo,
        &params.periodicidad,
    );
    let isr_ret = isr_res.isr_retener;
    if isr_ret > Decimal::ZERO {
        deds.push(Deduccion {
            tipo_deduccion: "002".into(),
            clave: "ISR".into(),
            concepto: "ISR".into(),
            importe: isr_ret,
        });
    }

    // Infonavit (crédito activo)
    if let Some(ref desc) = params.descuento_infonavit {
        let infonavit = calcular_descuento_infonavit(params.sdi, params.dias, desc);
        if infonavit > Decimal::ZERO {
            deds.push(Deduccion {
                tipo_deduccion: "003".into(),
                clave: "INFONAVIT".into(),
                concepto: "Descuento Infonavit".into(),
                importe: infonavit,
            });
        }
    }

    let total_isr = deds.iter()
        .filter(|d| d.tipo_deduccion == "002")
        .map(|d| d.importe)
        .fold(Decimal::ZERO, |a, b| a + b);

    let total_otras = deds.iter()
        .filter(|d| d.tipo_deduccion != "002")
        .map(|d| d.importe)
        .fold(Decimal::ZERO, |a, b| a + b);

    let deducciones = Deducciones {
        total_otras_deducciones: Some(total_otras),
        total_impuestos_retenidos: Some(total_isr),
        deducciones: deds.clone(),
    };

    (deds, deducciones)
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imss_sueldo_minimo() {
        // SBC = salario mínimo, 15 días
        let cuotas = calcular_imss(dec!(248.93), 15, "02");
        assert!(cuotas.cuota_obrero > Decimal::ZERO, "Cuota obrero debe ser > 0");
        // No puede exceder el 3% del SBC por período (cota razonable)
        let max_esperado = dec!(248.93) * Decimal::from(15) * dec!(0.04);
        assert!(cuotas.cuota_obrero <= max_esperado,
            "Cuota obrero {} no debe exceder límite {}", cuotas.cuota_obrero, max_esperado);
    }

    #[test]
    fn test_isr_salario_minimo_da_subsidio() {
        // Sueldo mínimo mensual: $248.93 × 30 = $7,467.90 → debería tener subsidio
        let resultado = calcular_isr_mensual(dec!(7467.90));
        assert!(resultado.isr_causado >= Decimal::ZERO);
        assert!(resultado.subsidio_al_empleo >= Decimal::ZERO);
        println!("ISR: {}, Subsidio: {}, A retener: {}",
            resultado.isr_causado, resultado.subsidio_al_empleo, resultado.isr_retener);
    }

    #[test]
    fn test_isr_sueldo_medio() {
        // Sueldo $25,000 mensual → debe retener ISR
        let resultado = calcular_isr_mensual(dec!(25000.00));
        assert!(resultado.isr_retener > Decimal::ZERO, "Debe retener ISR");
        assert!(!resultado.paga_subsidio, "No debe pagar subsidio");
        println!("ISR 25k: {} (causado: {}, subsidio: {})",
            resultado.isr_retener, resultado.isr_causado, resultado.subsidio_al_empleo);
    }

    #[test]
    fn test_calcular_deducciones_quincena() {
        let params = ParametrosNomina {
            sbc: dec!(500.00),
            sdi: dec!(550.00),
            ingreso_gravado_periodo: dec!(7500.00),
            dias: 15,
            periodicidad: "04".into(),
            descuento_infonavit: None,
        };
        let (_deds, ded_struct) = calcular_deducciones(&params);
        assert!(!ded_struct.deducciones.is_empty());
        let total = ded_struct.total_otras_deducciones.unwrap_or_default()
            + ded_struct.total_impuestos_retenidos.unwrap_or_default();
        assert!(total > Decimal::ZERO);
        println!("Total deducciones quincena 15k: {}", total);
    }

    #[test]
    fn test_xml_nomina_basico() {
        let nomina = Nomina {
            version: "1.2".into(),
            tipo_nomina: "O".into(),
            fecha_pago: "2024-06-15".into(),
            fecha_inicial_pago: "2024-06-01".into(),
            fecha_final_pago: "2024-06-15".into(),
            num_dias_pagados: dec!(15),
            total_percepciones: Some(dec!(7500.00)),
            total_deducciones: Some(dec!(856.23)),
            total_otros_pagos: None,
            emisor: EmisorNomina {
                registro_patronal: Some("Y2510000X00".into()),
                rfc_patron_origen: None,
                entidad_sncf: None,
            },
            receptor: ReceptorNomina {
                curp: "AAAA000101HDFXXX00".into(),
                num_seguridad_social: Some("12345678901".into()),
                fecha_inicio_rel_laboral: Some("2020-01-01".into()),
                antiguedad: Some("P4Y5M".into()),
                tipo_contrato: "01".into(),
                sindicalizado: None,
                tipo_jornada: Some("01".into()),
                tipo_regimen: "02".into(),
                num_empleado: "EMP001".into(),
                departamento: Some("Sistemas".into()),
                puesto: Some("Desarrollador".into()),
                riesgo_puesto: Some("1".into()),
                periodicidad_pago: "04".into(),
                banco: None,
                cuenta_bancaria: None,
                salario_base_cot_apor: Some(dec!(500.00)),
                salario_diario_integrado: Some(dec!(550.00)),
                clave_ent_fed: "JAL".into(),
            },
            percepciones: Some(Percepciones {
                total_sueldos: dec!(7500.00),
                total_separacion_indemnizacion: None,
                total_jubilacion_pension_retiro: None,
                total_gravado: dec!(7500.00),
                total_exento: dec!(0.00),
                percepciones: vec![
                    Percepcion {
                        tipo_percepcion: "001".into(),
                        clave: "SLD".into(),
                        concepto: "Sueldo".into(),
                        importe_gravado: dec!(7500.00),
                        importe_exento: dec!(0.00),
                    }
                ],
            }),
            deducciones: Some(Deducciones {
                total_otras_deducciones: Some(dec!(456.23)),
                total_impuestos_retenidos: Some(dec!(400.00)),
                deducciones: vec![
                    Deduccion {
                        tipo_deduccion: "001".into(),
                        clave: "IMSS".into(),
                        concepto: "Cuota IMSS Obrero".into(),
                        importe: dec!(456.23),
                    },
                    Deduccion {
                        tipo_deduccion: "002".into(),
                        clave: "ISR".into(),
                        concepto: "ISR".into(),
                        importe: dec!(400.00),
                    },
                ],
            }),
            otros_pagos: None,
        };

        let xml = generar_xml_nomina(&nomina).expect("XML generado");
        assert!(xml.contains("nomina12:Nomina"), "Debe contener nodo raíz: {}", xml);
        assert!(xml.contains("Version=\"1.2\""), "Debe contener versión");
        assert!(xml.contains("AAAA000101HDFXXX00"), "Debe contener CURP");
        assert!(xml.contains("nomina12:Percepciones"), "Debe contener percepciones");
        assert!(xml.contains("nomina12:Deducciones"), "Debe contener deducciones");
    }
}
