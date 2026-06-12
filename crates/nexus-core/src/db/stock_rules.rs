//! Lógica para Reglas de Reabastecimiento (Orderpoints)
//!
//! Permite evaluar el stock actual contra los umbrales configurados
//! y generar borradores de compras (Purchase Orders) automáticamente.

use sqlx::PgPool;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::error::CoreError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Orderpoint {
    pub id: i32,
    pub product_id: i32,
    pub location_id: i32,
    pub qty_min: Decimal,
    pub qty_max: Decimal,
    pub qty_multiple: Decimal,
    pub active: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NuevoOrderpoint {
    pub product_id: i32,
    pub location_id: i32,
    pub qty_min: Decimal,
    pub qty_max: Decimal,
    pub qty_multiple: Decimal,
}

/// Crear o actualizar una regla
pub async fn upsert_orderpoint(
    pool: &PgPool,
    company_id: i32,
    datos: NuevoOrderpoint,
) -> Result<i32, CoreError> {
    let row = sqlx::query!(
        r#"
        INSERT INTO stock_orderpoint (product_id, company_id, location_id, qty_min, qty_max, qty_multiple)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (product_id, location_id, company_id)
        DO UPDATE SET qty_min = EXCLUDED.qty_min, qty_max = EXCLUDED.qty_max, qty_multiple = EXCLUDED.qty_multiple
        RETURNING id
        "#,
        datos.product_id,
        company_id,
        datos.location_id,
        datos.qty_min,
        datos.qty_max,
        datos.qty_multiple
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}

/// Evalúa el stock y retorna una lista de (product_id, qty_to_order)
pub async fn evaluar_necesidades(
    pool: &PgPool,
    company_id: i32,
) -> Result<Vec<(i32, Decimal)>, CoreError> {
    // Busca orderpoints donde la suma del stock_quant es menor que qty_min
    let rows = sqlx::query!(
        r#"
        SELECT 
            op.product_id, 
            op.qty_min, 
            op.qty_max, 
            op.qty_multiple,
            COALESCE(SUM(sq.quantity), 0) AS current_stock
        FROM stock_orderpoint op
        LEFT JOIN stock_quant sq 
            ON sq.product_id = op.product_id AND sq.location_id = op.location_id
        WHERE op.company_id = $1 AND op.active = true
        GROUP BY op.product_id, op.qty_min, op.qty_max, op.qty_multiple
        HAVING COALESCE(SUM(sq.quantity), 0) < op.qty_min
        "#,
        company_id
    )
    .fetch_all(pool)
    .await?;

    let mut necesidades = Vec::new();
    for r in rows {
        let current = r.current_stock.unwrap_or(Decimal::ZERO);
        let needed = r.qty_max - current;
        
        // Redondear según qty_multiple
        let mult = r.qty_multiple;
        let final_qty = if mult > Decimal::ZERO {
            let mut qty = needed;
            let remainder = qty % mult;
            if remainder > Decimal::ZERO {
                qty += mult - remainder;
            }
            qty
        } else {
            needed
        };

        if final_qty > Decimal::ZERO {
            necesidades.push((r.product_id, final_qty));
        }
    }

    Ok(necesidades)
}

/// Dispara la generación automática de Compras basadas en las necesidades
pub async fn ejecutar_scheduler(pool: &PgPool, company_id: i32, user_id: i32) -> Result<usize, CoreError> {
    let faltantes = evaluar_necesidades(pool, company_id).await?;
    if faltantes.is_empty() {
        return Ok(0);
    }

    // Para simplificar: por cada faltante creamos una orden de compra (draft) al proveedor por defecto
    // En Odoo esto lo maneja el "Run Scheduler" agrupando por proveedor.
    let mut num_orders = 0;
    
    for (prod_id, qty) in faltantes {
        // Buscar el proveedor del producto (product_supplierinfo o asumiendo id=1 temporalmente)
        // En una implementación real se lee res_partner ligado al producto.
        let partner_id = 1; 

        // Creamos la cabecera
        let po_row = sqlx::query!(
            "INSERT INTO purchase_order (company_id, partner_id, state, amount_total, create_uid) 
             VALUES ($1, $2, 'draft', 0, $3) RETURNING id",
            company_id, partner_id, user_id
        )
        .fetch_one(pool)
        .await?;
        
        // Creamos la línea
        sqlx::query!(
            "INSERT INTO purchase_order_line (order_id, product_id, product_qty, price_unit, price_subtotal)
             VALUES ($1, $2, $3, 0, 0)",
            po_row.id, prod_id, qty
        )
        .execute(pool)
        .await?;

        num_orders += 1;
    }

    Ok(num_orders)
}
