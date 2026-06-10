//! Conciliación bancaria — matching de movimientos contables con extracto bancario
//!
//! Permite emparejar automáticamente:
//! - Pagos recibidos (cobros de facturas) con movimientos del extracto
//! - Transferencias SPEI salientes con su confirmación bancaria
//! - Devoluciones y rechazos

use rust_decimal::Decimal;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Movimiento del extracto bancario (importado del banco)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovimientoBancario {
    pub id: u64,
    pub fecha: NaiveDate,
    pub descripcion: String,
    pub referencia: Option<String>,
    pub monto: Decimal,
    pub saldo: Decimal,
    pub tipo: TipoMovimiento,
    pub conciliado: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TipoMovimiento {
    Cargo,
    Abono,
}

/// Resultado de conciliación
#[derive(Debug, Serialize)]
pub struct ResultadoConciliacion {
    pub total_movimientos: usize,
    pub conciliados: usize,
    pub pendientes: usize,
    pub diferencia_saldo: Decimal,
    pub movimientos_sin_match: Vec<MovimientoBancario>,
}

/// Intenta conciliar automáticamente movimientos con pagos del ERP
pub fn conciliar_automatico(
    movimientos: &mut Vec<MovimientoBancario>,
    pagos_erp: &[(u64, Decimal, NaiveDate)], // (id_pago, monto, fecha)
) -> ResultadoConciliacion {
    let mut conciliados = 0usize;

    for mov in movimientos.iter_mut() {
        if mov.conciliado {
            conciliados += 1;
            continue;
        }

        // Buscar un pago del ERP que coincida en monto y fecha ±3 días
        let encontrado = pagos_erp.iter().any(|(_, monto, fecha)| {
            let diff = if mov.fecha > *fecha {
                (mov.fecha - *fecha).num_days()
            } else {
                (*fecha - mov.fecha).num_days()
            };
            *monto == mov.monto && diff <= 3
        });

        if encontrado {
            mov.conciliado = true;
            conciliados += 1;
        }
    }

    let total = movimientos.len();
    let pendientes = total - conciliados;
    let sin_match: Vec<MovimientoBancario> = movimientos.iter()
        .filter(|m| !m.conciliado)
        .cloned()
        .collect();

    let diferencia = sin_match.iter()
        .fold(Decimal::ZERO, |acc, m| {
            if m.tipo == TipoMovimiento::Abono {
                acc + m.monto
            } else {
                acc - m.monto
            }
        });

    ResultadoConciliacion {
        total_movimientos: total,
        conciliados,
        pendientes,
        diferencia_saldo: diferencia,
        movimientos_sin_match: sin_match,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_conciliacion_perfecta() {
        let fecha = NaiveDate::from_ymd_opt(2024, 6, 10).unwrap();
        let mut movs = vec![
            MovimientoBancario {
                id: 1,
                fecha,
                descripcion: "SPEI ENTRANTE".into(),
                referencia: Some("RASTxyz".into()),
                monto: dec!(5000.00),
                saldo: dec!(15000.00),
                tipo: TipoMovimiento::Abono,
                conciliado: false,
            }
        ];

        let pagos = vec![(1u64, dec!(5000.00), fecha)];
        let res = conciliar_automatico(&mut movs, &pagos);

        assert_eq!(res.conciliados, 1);
        assert_eq!(res.pendientes, 0);
        assert_eq!(res.diferencia_saldo, Decimal::ZERO);
    }

    #[test]
    fn test_conciliacion_sin_match() {
        let fecha = NaiveDate::from_ymd_opt(2024, 6, 10).unwrap();
        let mut movs = vec![
            MovimientoBancario {
                id: 1,
                fecha,
                descripcion: "Cargo desconocido".into(),
                referencia: None,
                monto: dec!(1234.56),
                saldo: dec!(10000.00),
                tipo: TipoMovimiento::Cargo,
                conciliado: false,
            }
        ];

        let pagos: Vec<(u64, Decimal, NaiveDate)> = vec![];
        let res = conciliar_automatico(&mut movs, &pagos);

        assert_eq!(res.conciliados, 0);
        assert_eq!(res.pendientes, 1);
        assert_eq!(res.movimientos_sin_match.len(), 1);
    }
}
