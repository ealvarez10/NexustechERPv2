//! Ajustes de inventario (inventory adjustments)
//! Permite corregir diferencias entre conteo físico y sistema.

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

// ────────────────────────────────────────────────────────────────────────────
// Tipos de dominio
// ────────────────────────────────────────────────────────────────────────────

/// Estado del ciclo de vida de un ajuste
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EstadoAjuste {
    Borrador,
    Validado,
    Cancelado,
}

/// Ajuste de inventario (en memoria; la persistencia se realizará
/// mediante movimientos de stock marcados como `is_inventory = true`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AjusteInventario {
    pub id: u64,
    pub product_id: i32,
    pub location_id: i32,
    pub cantidad_sistema: Decimal,
    pub cantidad_real: Decimal,
    /// real − sistema
    pub diferencia: Decimal,
    pub motivo: String,
    pub fecha: NaiveDate,
    pub estado: EstadoAjuste,
}

impl AjusteInventario {
    /// Crea un nuevo ajuste calculando la diferencia automáticamente.
    pub fn nuevo(
        id: u64,
        product_id: i32,
        location_id: i32,
        cantidad_sistema: Decimal,
        cantidad_real: Decimal,
        motivo: impl Into<String>,
    ) -> Self {
        let diferencia = cantidad_real - cantidad_sistema;
        Self {
            id,
            product_id,
            location_id,
            cantidad_sistema,
            cantidad_real,
            diferencia,
            motivo: motivo.into(),
            fecha: chrono::Local::now().date_naive(),
            estado: EstadoAjuste::Borrador,
        }
    }

    /// Recalcula y devuelve la diferencia (real − sistema).
    pub fn diferencia(&self) -> Decimal {
        self.cantidad_real - self.cantidad_sistema
    }

    /// `true` si hay menos stock del esperado.
    pub fn es_faltante(&self) -> bool {
        self.diferencia() < Decimal::ZERO
    }

    /// `true` si hay más stock del esperado.
    pub fn es_sobrante(&self) -> bool {
        self.diferencia() > Decimal::ZERO
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Validación
// ────────────────────────────────────────────────────────────────────────────

/// Verifica que el ajuste sea coherente antes de procesarlo.
///
/// Reglas:
/// - `product_id` debe ser positivo.
/// - `location_id` debe ser positivo.
/// - Las cantidades no pueden ser negativas.
/// - El motivo no puede estar vacío.
/// - Solo se puede validar un ajuste en estado `Borrador`.
pub fn validar_ajuste(ajuste: &AjusteInventario) -> Result<(), String> {
    if ajuste.product_id <= 0 {
        return Err(format!(
            "product_id inválido: {}",
            ajuste.product_id
        ));
    }
    if ajuste.location_id <= 0 {
        return Err(format!(
            "location_id inválido: {}",
            ajuste.location_id
        ));
    }
    if ajuste.cantidad_sistema < Decimal::ZERO {
        return Err(format!(
            "cantidad_sistema no puede ser negativa: {}",
            ajuste.cantidad_sistema
        ));
    }
    if ajuste.cantidad_real < Decimal::ZERO {
        return Err(format!(
            "cantidad_real no puede ser negativa: {}",
            ajuste.cantidad_real
        ));
    }
    if ajuste.motivo.trim().is_empty() {
        return Err("el motivo del ajuste no puede estar vacío".into());
    }
    if ajuste.estado != EstadoAjuste::Borrador {
        return Err(format!(
            "solo se pueden validar ajustes en estado Borrador (estado actual: {:?})",
            ajuste.estado
        ));
    }
    Ok(())
}

// ────────────────────────────────────────────────────────────────────────────
// Tests
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn d(s: &str) -> Decimal {
        Decimal::from_str(s).expect("literal decimal válido")
    }

    fn ajuste_base() -> AjusteInventario {
        AjusteInventario::nuevo(1, 42, 7, d("100.00"), d("85.50"), "Conteo físico mensual")
    }

    // ── Test 1: diferencia y clasificación de faltante ──────────────────────
    #[test]
    fn test_faltante_correcto() {
        let ajuste = ajuste_base();
        assert!(ajuste.es_faltante(), "85.50 − 100.00 debe ser faltante");
        assert!(!ajuste.es_sobrante());
        assert_eq!(ajuste.diferencia(), d("-14.50"));
    }

    // ── Test 2: sobrante ─────────────────────────────────────────────────────
    #[test]
    fn test_sobrante_correcto() {
        let ajuste = AjusteInventario::nuevo(
            2,
            10,
            5,
            d("50.00"),
            d("63.75"),
            "Reconteo tras recepción",
        );
        assert!(ajuste.es_sobrante());
        assert!(!ajuste.es_faltante());
        assert_eq!(ajuste.diferencia(), d("13.75"));
    }

    // ── Test 3: sin diferencia ───────────────────────────────────────────────
    #[test]
    fn test_sin_diferencia() {
        let ajuste =
            AjusteInventario::nuevo(3, 7, 3, d("200.00"), d("200.00"), "Auditoría de cierre");
        assert!(!ajuste.es_faltante());
        assert!(!ajuste.es_sobrante());
        assert_eq!(ajuste.diferencia(), Decimal::ZERO);
    }

    // ── Test 4: validación con motivo vacío ──────────────────────────────────
    #[test]
    fn test_validacion_motivo_vacio() {
        let mut ajuste = ajuste_base();
        ajuste.motivo = "   ".into();
        assert!(validar_ajuste(&ajuste).is_err());
    }

    // ── Test 5: validación con product_id inválido ───────────────────────────
    #[test]
    fn test_validacion_product_id_invalido() {
        let mut ajuste = ajuste_base();
        ajuste.product_id = -1;
        let err = validar_ajuste(&ajuste).unwrap_err();
        assert!(err.contains("product_id"), "mensaje: {err}");
    }

    // ── Test 6: validación de ajuste ya validado ─────────────────────────────
    #[test]
    fn test_validacion_estado_no_borrador() {
        let mut ajuste = ajuste_base();
        ajuste.estado = EstadoAjuste::Validado;
        assert!(validar_ajuste(&ajuste).is_err());
    }

    // ── Test 7: ajuste válido pasa validación ────────────────────────────────
    #[test]
    fn test_ajuste_valido() {
        let ajuste = ajuste_base();
        assert!(validar_ajuste(&ajuste).is_ok());
    }
}
