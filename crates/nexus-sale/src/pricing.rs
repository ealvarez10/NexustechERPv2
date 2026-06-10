//! Motor de cálculo de precios y descuentos — lógica pura sin DB

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::error::SaleError;

/// Aplica un descuento porcentual a un precio base
///
/// # Ejemplo
/// ```
/// use nexus_sale::aplicar_descuento;
/// use rust_decimal_macros::dec;
/// let precio = aplicar_descuento(dec!(1000.00), dec!(15.0)).unwrap();
/// assert_eq!(precio, dec!(850.00));
/// ```
pub fn aplicar_descuento(precio_base: Decimal, descuento_pct: Decimal) -> Result<Decimal, SaleError> {
    if descuento_pct < Decimal::ZERO || descuento_pct > dec!(100) {
        return Err(SaleError::DescuentoInvalido);
    }
    let resultado = precio_base * (dec!(1) - descuento_pct / dec!(100));
    Ok(resultado.round_dp(6))
}

/// Calcula el subtotal de una línea (cantidad × precio con descuento)
pub fn subtotal_linea(
    cantidad: Decimal,
    precio_unitario: Decimal,
    descuento_pct: Decimal,
) -> Result<Decimal, SaleError> {
    if cantidad < Decimal::ZERO {
        return Err(SaleError::PrecioInvalido("Cantidad no puede ser negativa".into()));
    }
    let precio_desc = aplicar_descuento(precio_unitario, descuento_pct)?;
    Ok((cantidad * precio_desc).round_dp(2))
}

/// Calcula impuesto sobre un subtotal
pub fn calcular_iva(subtotal: Decimal, tasa_pct: Decimal) -> Decimal {
    (subtotal * tasa_pct / dec!(100)).round_dp(2)
}

/// Totales de una orden completa
#[derive(Debug, Clone)]
pub struct TotalesOrden {
    pub subtotal:  Decimal,
    pub impuestos: Decimal,
    pub total:     Decimal,
}

/// Calcula totales a partir de las líneas
///
/// # Argumentos
/// `lineas` — Vec de `(cantidad, precio_unitario, descuento_pct)`
/// `tasa_iva` — Tasa IVA en porcentaje (ej. `16.0` para México)
pub fn calcular_totales(
    lineas: &[(Decimal, Decimal, Decimal)],
    tasa_iva: Decimal,
) -> TotalesOrden {
    let subtotal: Decimal = lineas
        .iter()
        .map(|(qty, price, desc)| subtotal_linea(*qty, *price, *desc).unwrap_or(Decimal::ZERO))
        .sum();

    let impuestos = calcular_iva(subtotal, tasa_iva);
    TotalesOrden {
        subtotal,
        impuestos,
        total: subtotal + impuestos,
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_descuento_15_pct() {
        let precio = aplicar_descuento(dec!(1000.00), dec!(15.0)).unwrap();
        assert_eq!(precio.round_dp(2), dec!(850.00));
    }

    #[test]
    fn test_descuento_cero() {
        let precio = aplicar_descuento(dec!(500.00), dec!(0.0)).unwrap();
        assert_eq!(precio.round_dp(2), dec!(500.00));
    }

    #[test]
    fn test_descuento_100_pct() {
        let precio = aplicar_descuento(dec!(300.00), dec!(100.0)).unwrap();
        assert_eq!(precio.round_dp(2), dec!(0.00));
    }

    #[test]
    fn test_descuento_invalido() {
        assert!(aplicar_descuento(dec!(1000.00), dec!(101.0)).is_err());
        assert!(aplicar_descuento(dec!(1000.00), dec!(-5.0)).is_err());
    }

    #[test]
    fn test_subtotal_linea() {
        // 5 unidades × $200 c/u con 10% descuento = $900
        let sub = subtotal_linea(dec!(5), dec!(200.00), dec!(10.0)).unwrap();
        assert_eq!(sub, dec!(900.00));
    }

    #[test]
    fn test_calcular_iva_16_pct() {
        let iva = calcular_iva(dec!(1000.00), dec!(16.0));
        assert_eq!(iva, dec!(160.00));
    }

    #[test]
    fn test_totales_orden() {
        let lineas = vec![
            (dec!(2), dec!(500.00), dec!(0.0)),   // $1000
            (dec!(1), dec!(300.00), dec!(10.0)),  // $270 (con 10% desc)
        ];
        let t = calcular_totales(&lineas, dec!(16.0));
        assert_eq!(t.subtotal, dec!(1270.00));
        assert_eq!(t.impuestos, dec!(203.20));
        assert_eq!(t.total, dec!(1473.20));
    }

    #[test]
    fn test_cantidad_negativa() {
        assert!(subtotal_linea(dec!(-1), dec!(100.00), dec!(0.0)).is_err());
    }
}
