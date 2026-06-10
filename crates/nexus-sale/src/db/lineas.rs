//! Líneas de orden de venta — sale_order_line

use sqlx::PgPool;
use serde::Serialize;
use rust_decimal::Decimal;
use crate::error::SaleError;

/// Datos para agregar una línea a una orden
#[derive(Debug, serde::Deserialize)]
pub struct NuevaLinea {
    pub product_id:      Option<i32>,
    pub name:            String,
    pub product_uom_qty: Decimal,
    pub price_unit:      Decimal,
    pub discount:        Option<Decimal>,
}

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

/// Agrega una línea a una orden (solo si state = 'draft' o 'sent')
pub async fn agregar(pool: &PgPool, linea: &NuevaLinea, order_id: i32) -> Result<i32, SaleError> {
    let descuento = linea.discount.unwrap_or(Decimal::ZERO);
    let factor = Decimal::ONE - descuento / Decimal::ONE_HUNDRED;
    let subtotal = linea.product_uom_qty * linea.price_unit * factor;
    let price_total = subtotal; // sin IVA por ahora

    // Verificar que la orden está en estado editable
    let state: Option<String> = sqlx::query_scalar(
        "SELECT state FROM sale_order WHERE id = $1"
    )
    .bind(order_id)
    .fetch_optional(pool)
    .await?;

    match state.as_deref() {
        Some("draft") | Some("sent") => {},
        _ => return Err(SaleError::NoEncontrada(order_id)),
    }

    let id = sqlx::query_scalar::<_, i32>(
        "INSERT INTO sale_order_line
            (order_id, product_id, name, product_uom_qty, price_unit, price_subtotal, price_total, discount)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         RETURNING id"
    )
    .bind(order_id)
    .bind(linea.product_id)
    .bind(&linea.name)
    .bind(linea.product_uom_qty)
    .bind(linea.price_unit)
    .bind(subtotal)
    .bind(price_total)
    .bind(linea.discount)
    .fetch_one(pool)
    .await?;
    Ok(id)
}

/// Elimina una línea de una orden
pub async fn eliminar(pool: &PgPool, linea_id: i32, order_id: i32) -> Result<(), SaleError> {
    let rows = sqlx::query(
        "DELETE FROM sale_order_line WHERE id = $1 AND order_id = $2"
    )
    .bind(linea_id)
    .bind(order_id)
    .execute(pool)
    .await?;
    if rows.rows_affected() == 0 {
        return Err(SaleError::NoEncontrada(linea_id));
    }
    Ok(())
}
