//! Compras — Órdenes de compra (purchase_order)

use sqlx::PgPool;
use serde::Serialize;
use rust_decimal::Decimal;
use crate::error::CoreError;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PurchaseOrderLine {
    pub id: i32,
    pub order_id: i32,
    pub product_id: Option<i32>,
    pub name: String,
    pub product_qty: Decimal,
    pub price_unit: Decimal,
    pub discount: Option<Decimal>,
    pub price_subtotal: Option<Decimal>,
    pub price_total: Option<Decimal>,
    pub qty_received: Option<Decimal>,
    pub qty_invoiced: Option<Decimal>,
}

// ─── Structs ─────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PurchaseOrder {
    pub id: i32,
    pub company_id: i32,
    pub partner_id: i32,
    pub name: String,
    pub state: Option<String>,
    pub invoice_status: Option<String>,
    pub date_order: Option<String>,
    pub amount_untaxed: Option<Decimal>,
    pub amount_tax: Option<Decimal>,
    pub amount_total: Option<Decimal>,
    pub partner_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct KpisCompras {
    pub total_ordenes: i64,
    pub confirmadas: i64,
    pub en_borrador: i64,
    pub completadas: i64,
    pub canceladas: i64,
    pub monto_total: Decimal,
    pub monto_este_mes: Decimal,
}

const SELECT_COLS: &str = r#"
    po.id, po.company_id, po.partner_id, po.name,
    po.state, po.invoice_status,
    po.date_order::text AS date_order,
    po.amount_untaxed, po.amount_tax, po.amount_total,
    rp.name AS partner_name
"#;

const FROM_JOIN: &str = r#"
    FROM purchase_order po
    LEFT JOIN res_partner rp ON rp.id = po.partner_id
"#;

// ─── Funciones públicas ───────────────────────────────────────────────────────

/// Lista órdenes de compra de una empresa (paginado)
pub async fn listar(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<PurchaseOrder>, CoreError> {
    let offset = (pagina - 1).max(0) * por_pagina;
    let q = format!(
        "SELECT {SELECT_COLS} {FROM_JOIN}
         WHERE po.company_id = $1
         ORDER BY po.date_order DESC NULLS LAST
         LIMIT $2 OFFSET $3"
    );
    let rows = sqlx::query_as::<_, PurchaseOrder>(&q)
        .bind(company_id)
        .bind(por_pagina)
        .bind(offset)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

/// Total de órdenes de compra de una empresa
pub async fn contar(pool: &PgPool, company_id: i32) -> Result<i64, CoreError> {
    let n: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM purchase_order WHERE company_id = $1",
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;
    Ok(n.0)
}

/// KPIs de compras para una empresa
pub async fn kpis(pool: &PgPool, company_id: i32) -> Result<KpisCompras, CoreError> {
    let row: (i64, i64, i64, i64, i64, Option<Decimal>, Option<Decimal>) = sqlx::query_as(
        r#"SELECT
            COUNT(*)                                                           AS total_ordenes,
            COUNT(*) FILTER (WHERE state = 'purchase')                         AS confirmadas,
            COUNT(*) FILTER (WHERE state = 'draft')                            AS en_borrador,
            COUNT(*) FILTER (WHERE state = 'done')                             AS completadas,
            COUNT(*) FILTER (WHERE state = 'cancel')                           AS canceladas,
            SUM(amount_total) FILTER (WHERE state NOT IN ('cancel', 'draft'))  AS monto_total,
            SUM(amount_total) FILTER (
                WHERE state NOT IN ('cancel', 'draft')
                  AND date_order >= date_trunc('month', NOW())
            )                                                                  AS monto_este_mes
           FROM purchase_order
           WHERE company_id = $1"#,
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;

    Ok(KpisCompras {
        total_ordenes: row.0,
        confirmadas: row.1,
        en_borrador: row.2,
        completadas: row.3,
        canceladas: row.4,
        monto_total: row.5.unwrap_or(Decimal::ZERO),
        monto_este_mes: row.6.unwrap_or(Decimal::ZERO),
    })
}

/// Obtiene una orden de compra por ID
pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<PurchaseOrder, CoreError> {
    let q = format!("SELECT {SELECT_COLS} {FROM_JOIN} WHERE po.id = $1");
    let orden = sqlx::query_as::<_, PurchaseOrder>(&q)
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| CoreError::not_found("Orden de compra", id))?;
    Ok(orden)
}

/// Obtiene las líneas de una orden de compra
pub async fn obtener_lineas(pool: &PgPool, order_id: i32) -> Result<Vec<PurchaseOrderLine>, CoreError> {
    let lineas = sqlx::query_as::<_, PurchaseOrderLine>(
        r#"SELECT id, order_id, product_id, name, product_qty, price_unit,
                  discount, price_subtotal, price_total, qty_received, qty_invoiced
           FROM purchase_order_line
           WHERE order_id = $1
             AND (display_type IS NULL OR display_type = '')
           ORDER BY sequence ASC NULLS LAST, id ASC"#,
    )
    .bind(order_id)
    .fetch_all(pool)
    .await?;
    Ok(lineas)
}
