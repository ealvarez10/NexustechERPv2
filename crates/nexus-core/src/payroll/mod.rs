//! Motor de cálculo de nómina mexicana
//!
//! Implementa:
//! - ISR mensual 2024 con tablas SAT (Art. 152 LISR)
//! - IMSS cuotas obreras y patronales (2024)
//! - Cálculo de percepciones / deducciones
//! - Subsidio al empleo mensual 2024

use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

// ─── Tablas ISR 2024 (Tablas de retención mensual Art. 152 LISR) ────────────

#[derive(Debug, Clone)]
struct TramoIsr {
    limite_inferior: Decimal,
    cuota_fija: Decimal,
    tasa_excedente: Decimal,
}

fn tabla_isr_2024_mensual() -> Vec<TramoIsr> {
    vec![
        TramoIsr { limite_inferior: Decimal::from_str("0.01").unwrap(),       cuota_fija: Decimal::ZERO,                          tasa_excedente: Decimal::from_str("0.0192").unwrap() },
        TramoIsr { limite_inferior: Decimal::from_str("746.05").unwrap(),     cuota_fija: Decimal::from_str("14.32").unwrap(),     tasa_excedente: Decimal::from_str("0.0640").unwrap() },
        TramoIsr { limite_inferior: Decimal::from_str("6332.06").unwrap(),    cuota_fija: Decimal::from_str("371.83").unwrap(),    tasa_excedente: Decimal::from_str("0.1088").unwrap() },
        TramoIsr { limite_inferior: Decimal::from_str("11128.01").unwrap(),   cuota_fija: Decimal::from_str("893.63").unwrap(),    tasa_excedente: Decimal::from_str("0.1600").unwrap() },
        TramoIsr { limite_inferior: Decimal::from_str("12935.83").unwrap(),   cuota_fija: Decimal::from_str("1182.88").unwrap(),   tasa_excedente: Decimal::from_str("0.1792").unwrap() },
        TramoIsr { limite_inferior: Decimal::from_str("15487.72").unwrap(),   cuota_fija: Decimal::from_str("1640.18").unwrap(),   tasa_excedente: Decimal::from_str("0.2136").unwrap() },
        TramoIsr { limite_inferior: Decimal::from_str("30992.01").unwrap(),   cuota_fija: Decimal::from_str("4960.37").unwrap(),   tasa_excedente: Decimal::from_str("0.2352").unwrap() },
        TramoIsr { limite_inferior: Decimal::from_str("46487.05").unwrap(),   cuota_fija: Decimal::from_str("8605.22").unwrap(),   tasa_excedente: Decimal::from_str("0.3000").unwrap() },
        TramoIsr { limite_inferior: Decimal::from_str("93474.91").unwrap(),   cuota_fija: Decimal::from_str("22665.17").unwrap(),  tasa_excedente: Decimal::from_str("0.3200").unwrap() },
        TramoIsr { limite_inferior: Decimal::from_str("124949.06").unwrap(),  cuota_fija: Decimal::from_str("32691.18").unwrap(),  tasa_excedente: Decimal::from_str("0.3400").unwrap() },
        TramoIsr { limite_inferior: Decimal::from_str("374847.21").unwrap(),  cuota_fija: Decimal::from_str("117912.32").unwrap(), tasa_excedente: Decimal::from_str("0.3500").unwrap() },
    ]
}

/// Tabla de Subsidio al Empleo mensual 2024
fn tabla_subsidio_2024() -> Vec<(Decimal, Decimal)> {
    // (límite_superior, subsidio_mensual)
    vec![
        (Decimal::from_str("1768.96").unwrap(),  Decimal::from_str("407.02").unwrap()),
        (Decimal::from_str("2653.38").unwrap(),  Decimal::from_str("406.83").unwrap()),
        (Decimal::from_str("3472.84").unwrap(),  Decimal::from_str("406.62").unwrap()),
        (Decimal::from_str("3537.87").unwrap(),  Decimal::from_str("392.77").unwrap()),
        (Decimal::from_str("4446.15").unwrap(),  Decimal::from_str("382.46").unwrap()),
        (Decimal::from_str("4717.18").unwrap(),  Decimal::from_str("354.23").unwrap()),
        (Decimal::from_str("5335.42").unwrap(),  Decimal::from_str("324.87").unwrap()),
        (Decimal::from_str("6224.67").unwrap(),  Decimal::from_str("294.63").unwrap()),
        (Decimal::from_str("7113.90").unwrap(),  Decimal::from_str("253.54").unwrap()),
        (Decimal::from_str("7382.33").unwrap(),  Decimal::from_str("217.61").unwrap()),
        (Decimal::from_str("99999999.99").unwrap(), Decimal::ZERO),
    ]
}

// ─── Tasas IMSS 2024 ─────────────────────────────────────────────────────────

const UMA_DIARIA_2024: f64 = 108.57;        // Valor UMA diaria 2024 (DOF)
const UMA_MENSUAL_2024: f64 = 3299.12;      // UMA mensual (25 días hábiles aprox)

/// Cuotas IMSS porcentajes (ramas)
struct CuotasImss {
    /// Seguro de Enfermedad y Maternidad (SEM) - Prestaciones en Especie
    sem_prestaciones_especie_patron:  Decimal,
    sem_prestaciones_especie_obrero:  Decimal,
    /// SEM - Prestaciones en Dinero
    sem_prestaciones_dinero_patron:   Decimal,
    sem_prestaciones_dinero_obrero:   Decimal,
    /// Invalidez y Vida
    invalidez_vida_patron:            Decimal,
    invalidez_vida_obrero:            Decimal,
    /// Cesantía en Edad Avanzada y Vejez (CEAV)
    ceav_patron:                      Decimal,
    ceav_obrero:                      Decimal,
    /// Guarderías y Prestaciones Sociales (solo patrón)
    guarderias_patron:                Decimal,
    /// Retiro (solo patrón)
    retiro_patron:                    Decimal,
    /// Riesgo de Trabajo (varía por empresa - usamos 0.54355% media)
    riesgo_trabajo_patron:            Decimal,
    /// Cuota Fija SEM (sobre salario ≤ 3 UMA)
    cuota_fija_patron_por_trabajador: Decimal,
}

fn cuotas_imss_2024() -> CuotasImss {
    CuotasImss {
        sem_prestaciones_especie_patron:  Decimal::from_str("0.01050").unwrap(),
        sem_prestaciones_especie_obrero:  Decimal::from_str("0.00375").unwrap(),
        sem_prestaciones_dinero_patron:   Decimal::from_str("0.007").unwrap(),
        sem_prestaciones_dinero_obrero:   Decimal::from_str("0.0025").unwrap(),
        invalidez_vida_patron:            Decimal::from_str("0.01750").unwrap(),
        invalidez_vida_obrero:            Decimal::from_str("0.00625").unwrap(),
        ceav_patron:                      Decimal::from_str("0.03150").unwrap(),
        ceav_obrero:                      Decimal::from_str("0.01125").unwrap(),
        guarderias_patron:                Decimal::from_str("0.01000").unwrap(),
        retiro_patron:                    Decimal::from_str("0.02000").unwrap(),
        riesgo_trabajo_patron:            Decimal::from_str("0.0054355").unwrap(),
        // Cuota fija = 20.40% de 3 UMA mensuales por trabajador (patrón)
        cuota_fija_patron_por_trabajador: Decimal::from_str("0.2040").unwrap()
            * Decimal::from_str(&format!("{:.2}", UMA_MENSUAL_2024 * 3.0)).unwrap(),
    }
}

// ─── Input/Output ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct EntradaNomina {
    /// Salario Diario Integrado (SDI)
    pub sdi: f64,
    /// Días trabajados en el período
    pub dias_periodo: u32,
    /// Tipo de nómina: "mensual", "quincenal", "semanal"
    pub tipo: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResultadoNomina {
    // ── Percepciones ──
    pub salario_bruto:       Decimal,
    // ── Deducciones ──
    pub isr_retenido:        Decimal,
    pub subsidio_empleo:     Decimal,
    pub imss_obrero:         Decimal,
    pub total_deducciones:   Decimal,
    // ── Neto ──
    pub salario_neto:        Decimal,
    // ── Costo patrón ──
    pub imss_patron:         Decimal,
    pub cuota_fija_patron:   Decimal,
    pub costo_total_patron:  Decimal,
}

// ─── Motor principal ─────────────────────────────────────────────────────────

pub fn calcular_nomina(entrada: &EntradaNomina) -> ResultadoNomina {
    let dias = Decimal::from(entrada.dias_periodo);
    let sdi  = Decimal::from_str(&format!("{:.6}", entrada.sdi)).unwrap_or(Decimal::ZERO);

    // Salario base del período (SDI * días)
    let salario_bruto = (sdi * dias).round_dp(2);

    // ── Calcular ISR mensual equivalente ────────────────────────────────────
    // Convertimos al equivalente mensual para aplicar la tabla
    let factor_mensual: Decimal = match entrada.tipo.as_str() {
        "quincenal" => Decimal::from(2),
        "semanal"   => Decimal::from_str("4.333").unwrap(),
        _           => Decimal::ONE, // mensual
    };
    let base_mensual = (salario_bruto * factor_mensual).round_dp(2);

    let isr_mensual = calcular_isr_mensual(base_mensual);
    let subsidio    = calcular_subsidio(base_mensual);

    // ISR del período (dividir por factor)
    let isr_periodo = (isr_mensual / factor_mensual).round_dp(2);
    let subsidio_periodo = (subsidio / factor_mensual).round_dp(2);

    // ISR neto a retener (si ISR > subsidio)
    let isr_retenido = (isr_periodo - subsidio_periodo).max(Decimal::ZERO).round_dp(2);

    // ── Cuotas IMSS obrero ───────────────────────────────────────────────────
    let imss_obrero = calcular_imss_obrero(sdi, entrada.dias_periodo);

    // ── Deducciones totales ──────────────────────────────────────────────────
    let total_deducciones = (isr_retenido + imss_obrero).round_dp(2);
    let salario_neto = (salario_bruto - total_deducciones).round_dp(2);

    // ── Costo para el patrón ────────────────────────────────────────────────
    let cuotas = cuotas_imss_2024();
    let imss_patron = calcular_imss_patron(sdi, entrada.dias_periodo, &cuotas);
    let cuota_fija_patron = cuotas.cuota_fija_patron_por_trabajador
        * (dias / Decimal::from(30)).min(Decimal::ONE)  // prorrateada
        .round_dp(2);
    let costo_total_patron = (salario_bruto + imss_patron + cuota_fija_patron).round_dp(2);

    ResultadoNomina {
        salario_bruto,
        isr_retenido,
        subsidio_empleo: subsidio_periodo,
        imss_obrero,
        total_deducciones,
        salario_neto,
        imss_patron,
        cuota_fija_patron,
        costo_total_patron,
    }
}

fn calcular_isr_mensual(base: Decimal) -> Decimal {
    let tabla = tabla_isr_2024_mensual();
    let mut tramo_actual = &tabla[0];

    for tramo in tabla.iter().rev() {
        if base >= tramo.limite_inferior {
            tramo_actual = tramo;
            break;
        }
    }

    let excedente = (base - tramo_actual.limite_inferior).max(Decimal::ZERO);
    (tramo_actual.cuota_fija + excedente * tramo_actual.tasa_excedente).round_dp(2)
}

fn calcular_subsidio(base_mensual: Decimal) -> Decimal {
    let tabla = tabla_subsidio_2024();
    for (limite, subsidio) in &tabla {
        if base_mensual <= *limite {
            return *subsidio;
        }
    }
    Decimal::ZERO
}

fn calcular_imss_obrero(sdi: Decimal, dias: u32) -> Decimal {
    let cuotas = cuotas_imss_2024();
    let base   = sdi * Decimal::from(dias);
    let uma_m  = Decimal::from_str(&format!("{:.2}", UMA_DIARIA_2024 * dias as f64)).unwrap();

    // Cuota SEM especie obrero: solo aplica sobre excedente de 3 UMAs
    let excedente = (base - uma_m * Decimal::from(3)).max(Decimal::ZERO);
    let sem_especie_obrero    = excedente * cuotas.sem_prestaciones_especie_obrero;
    let sem_dinero_obrero     = base * cuotas.sem_prestaciones_dinero_obrero;
    let invalidez_vida_obrero = base * cuotas.invalidez_vida_obrero;
    let ceav_obrero           = base * cuotas.ceav_obrero;

    (sem_especie_obrero + sem_dinero_obrero + invalidez_vida_obrero + ceav_obrero).round_dp(2)
}

fn calcular_imss_patron(sdi: Decimal, dias: u32, cuotas: &CuotasImss) -> Decimal {
    let base = sdi * Decimal::from(dias);
    let uma  = Decimal::from_str(&format!("{:.2}", UMA_DIARIA_2024 * dias as f64)).unwrap();

    let excedente = (base - uma * Decimal::from(3)).max(Decimal::ZERO);
    let sem_especie_patron    = excedente * cuotas.sem_prestaciones_especie_patron;
    let sem_dinero_patron     = base * cuotas.sem_prestaciones_dinero_patron;
    let invalidez_vida_patron = base * cuotas.invalidez_vida_patron;
    let ceav_patron           = base * cuotas.ceav_patron;
    let guarderias            = base * cuotas.guarderias_patron;
    let retiro                = base * cuotas.retiro_patron;
    let riesgo_trabajo        = base * cuotas.riesgo_trabajo_patron;

    (sem_especie_patron + sem_dinero_patron + invalidez_vida_patron
        + ceav_patron + guarderias + retiro + riesgo_trabajo).round_dp(2)
}

// ─── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nomina_salario_minimo() {
        let entrada = EntradaNomina {
            sdi: 248.93,   // SDI al salario mínimo 2024 ($7,467.87/30d aprox)
            dias_periodo: 30,
            tipo: "mensual".to_string(),
        };
        let r = calcular_nomina(&entrada);
        println!("Bruto: {} | ISR: {} | IMSS obrero: {} | Neto: {}",
            r.salario_bruto, r.isr_retenido, r.imss_obrero, r.salario_neto);
        assert!(r.salario_neto > Decimal::ZERO);
        assert!(r.salario_neto < r.salario_bruto);
    }

    #[test]
    fn test_nomina_ejecutivo_alto() {
        let entrada = EntradaNomina {
            sdi: 2000.0,  // SDI alto
            dias_periodo: 30,
            tipo: "mensual".to_string(),
        };
        let r = calcular_nomina(&entrada);
        assert!(r.isr_retenido > Decimal::ZERO);
        assert!(r.imss_patron > r.imss_obrero);
    }
}
