//! Líneas de orden de venta — sale_order_line

use sqlx::PgPool;
use serde::Serialize;
use rust_decimal::Decimal;
use crate::error::SaleError;

/// Datos para agregar una línea a una orden
#[derive(Debug, serde::Deserialize)]
pub struct NuevaLinea {
    pub product_id:      Option<i32>,
    pub name:            Option<String>,   // Si no viene, se toma del producto
    pub display_type:    Option<String>,   // 'line_section' | null
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

/// Líneas de una orden (solo accesibles si pertenecen a la misma empresa)
pub async fn por_orden(pool: &PgPool, order_id: i32, company_id: i32) -> Result<Vec<LineaVenta>, SaleError> {
    // Verifica que la orden pertenezca a la empresa del usuario
    let ok: Option<i32> = sqlx::query_scalar(
        "SELECT id FROM sale_order WHERE id = $1 AND company_id = $2"
    )
    .bind(order_id)
    .bind(company_id)
    .fetch_optional(pool)
    .await?;

    if ok.is_none() {
        return Err(SaleError::NoEncontrada(order_id));
    }

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

/// Agrega una línea a una orden (solo si state = 'draft' o 'sent' y misma empresa)
pub async fn agregar(pool: &PgPool, linea: &NuevaLinea, order_id: i32, company_id: i32) -> Result<i32, SaleError> {
    let descuento = linea.discount.unwrap_or(Decimal::ZERO);
    let factor = Decimal::ONE - descuento / Decimal::ONE_HUNDRED;
    let subtotal = linea.product_uom_qty * linea.price_unit * factor;
    let price_total = subtotal;

    // Verificar que la orden está en estado editable y pertenece a la empresa
    let state: Option<String> = sqlx::query_scalar(
        "SELECT state FROM sale_order WHERE id = $1 AND company_id = $2"
    )
    .bind(order_id)
    .bind(company_id)
    .fetch_optional(pool)
    .await?;

    match state.as_deref() {
        Some("draft") | Some("sent") => {},
        _ => return Err(SaleError::NoEncontrada(order_id)),
    }

    // Resolver el nombre: usar el enviado, o buscar nombre del producto, o fallback
    let nombre: String = if let Some(ref n) = linea.name {
        n.clone()
    } else if let Some(pid) = linea.product_id {
        sqlx::query_scalar::<_, String>(
            "SELECT COALESCE(name, 'Producto') FROM product_template WHERE id = $1"
        )
        .bind(pid)
        .fetch_optional(pool)
        .await?
        .unwrap_or_else(|| "Producto".to_string())
    } else {
        "Línea".to_string()
    };

    let id = sqlx::query_scalar::<_, i32>(
        "INSERT INTO sale_order_line
            (order_id, product_id, name, display_type, product_uom_qty, price_unit, price_subtotal, price_total, discount)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         RETURNING id"
    )
    .bind(order_id)
    .bind(linea.product_id)
    .bind(&nombre)
    .bind(&linea.display_type)
    .bind(linea.product_uom_qty)
    .bind(linea.price_unit)
    .bind(subtotal)
    .bind(price_total)
    .bind(linea.discount)
    .fetch_one(pool)
    .await?;

    // Recalcular totales de la orden tras insertar la línea
    _recalcular_totales(pool, order_id).await?;

    Ok(id)
}

/// Elimina una línea de una orden (con validación de empresa)
pub async fn eliminar(pool: &PgPool, linea_id: i32, order_id: i32, company_id: i32) -> Result<(), SaleError> {
    // Verifica que la orden pertenezca a la empresa
    let ok: Option<i32> = sqlx::query_scalar(
        "SELECT id FROM sale_order WHERE id = $1 AND company_id = $2"
    )
    .bind(order_id)
    .bind(company_id)
    .fetch_optional(pool)
    .await?;

    if ok.is_none() {
        return Err(SaleError::NoEncontrada(order_id));
    }

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

    // Recalcular totales de la orden tras eliminar la línea
    _recalcular_totales(pool, order_id).await?;

    Ok(())
}

/// Recalcula los totales del encabezado de la orden a partir de sus líneas
async fn _recalcular_totales(pool: &PgPool, order_id: i32) -> Result<(), SaleError> {
    sqlx::query(
        r#"UPDATE sale_order SET
            amount_untaxed = (
                SELECT COALESCE(SUM(price_subtotal), 0)
                FROM sale_order_line
                WHERE order_id = $1 AND (display_type IS NULL OR display_type = '')
            ),
            amount_tax = (
                SELECT COALESCE(SUM(price_total - price_subtotal), 0)
                FROM sale_order_line
                WHERE order_id = $1 AND (display_type IS NULL OR display_type = '')
            ),
            amount_total = (
                SELECT COALESCE(SUM(price_total), 0)
                FROM sale_order_line
                WHERE order_id = $1 AND (display_type IS NULL OR display_type = '')
            )
           WHERE id = $1"#,
    )
    .bind(order_id)
    .execute(pool)
    .await?;
    Ok(())
}
