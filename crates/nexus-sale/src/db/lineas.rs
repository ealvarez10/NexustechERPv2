//! Líneas de orden de venta — sale_order_line

use sqlx::PgPool;
use serde::Serialize;
use rust_decimal::Decimal;
use crate::error::SaleError;

/// Línea de orden de venta
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct LineaVenta {
    pub id:              i32,
    pub order_id:        i32,
    pub product_id:      Option<i32>,
    pub name:            Option<String>,
    pub product_uom_qty: Decimal,
    pub price_unit:      Decimal,
    pub price_subtotal:  Decimal,
    pub discount:        Option<Decimal>,
}

/// Líneas de una orden
pub async fn por_orden(pool: &PgPool, order_id: i32) -> Result<Vec<LineaVenta>, SaleError> {
    let rows = sqlx::query_as::<_, LineaVenta>(
        "SELECT id, order_id, product_id, name, product_uom_qty, price_unit, price_subtotal, discount
         FROM sale_order_line
         WHERE order_id = $1
         ORDER BY id"
    )
    .bind(order_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
