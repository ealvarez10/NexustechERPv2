//! CRUD para sale_order y sale_order_line — Órdenes de venta

use sqlx::PgPool;
use crate::error::CoreError;
use rust_decimal::Decimal;

// ─── Struct simplificado (columnas reales del schema) ────────────────────────

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct SaleOrder {
    pub id:                 i32,
    pub company_id:         i32,
    pub partner_id:         Option<i32>,
    pub name:               Option<String>,
    pub state:              Option<String>,
    pub invoice_status:     Option<String>,
    /// date_order como texto — evita conflicto TIMESTAMP vs TIMESTAMPTZ
    pub date_order:         Option<String>,
    pub amount_untaxed:     Option<Decimal>,
    pub amount_tax:         Option<Decimal>,
    pub amount_total:       Option<Decimal>,
    pub currency_rate:      Option<Decimal>,
    pub picking_policy:     Option<String>,
    pub partner_name:       Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct SaleOrderLine {
    pub id:              i32,
    pub order_id:        Option<i32>,
    pub product_id:      Option<i32>,
    pub name:            Option<String>,
    pub product_uom_qty: Option<Decimal>,
    pub price_unit:      Option<Decimal>,
    pub discount:        Option<Decimal>,
    pub price_subtotal:  Option<Decimal>,
    pub price_total:     Option<Decimal>,
    pub state:           Option<String>,
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

const SELECT_COLS: &str = r#"
    so.id, so.company_id, so.partner_id,
    so.name, so.state, so.invoice_status,
    so.date_order::text AS date_order,
    so.amount_untaxed, so.amount_tax, so.amount_total,
    so.currency_rate,
    so.picking_policy,
    rp.name AS partner_name
"#;

const FROM_JOIN: &str = r#"
    FROM sale_order so
    LEFT JOIN res_partner rp ON rp.id = so.partner_id
"#;

// ─── Funciones públicas ───────────────────────────────────────────────────────

/// Lista órdenes de venta de una empresa (paginado)
pub async fn listar(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<SaleOrder>, CoreError> {
    let offset = (pagina - 1).max(0) * por_pagina;
    let q = format!(
        "SELECT {SELECT_COLS} {FROM_JOIN}
         WHERE so.company_id = $1 AND so.state NOT IN ('cancel')
         ORDER BY so.date_order DESC NULLS LAST
         LIMIT $2 OFFSET $3"
    );
    let rows = sqlx::query_as::<_, SaleOrder>(&q)
        .bind(company_id)
        .bind(por_pagina)
        .bind(offset)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

/// Obtiene una orden de venta por ID
pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<SaleOrder, CoreError> {
    let q = format!(
        "SELECT {SELECT_COLS} {FROM_JOIN} WHERE so.id = $1"
    );
    let orden = sqlx::query_as::<_, SaleOrder>(&q)
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| CoreError::not_found("Orden de venta", id))?;
    Ok(orden)
}

/// Obtiene una orden por nombre (folio)
pub async fn obtener_por_nombre(pool: &PgPool, nombre: &str, company_id: i32) -> Result<SaleOrder, CoreError> {
    let q = format!(
        "SELECT {SELECT_COLS} {FROM_JOIN}
         WHERE so.name = $1 AND so.company_id = $2 LIMIT 1"
    );
    let orden = sqlx::query_as::<_, SaleOrder>(&q)
        .bind(nombre)
        .bind(company_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| CoreError::NotFound(format!("Orden '{}' no encontrada", nombre)))?;
    Ok(orden)
}

/// Lista órdenes por estado
pub async fn listar_por_estado(
    pool: &PgPool,
    company_id: i32,
    estado: &str,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<SaleOrder>, CoreError> {
    let offset = (pagina - 1).max(0) * por_pagina;
    let q = format!(
        "SELECT {SELECT_COLS} {FROM_JOIN}
         WHERE so.company_id = $1 AND so.state = $2
         ORDER BY so.date_order DESC NULLS LAST
         LIMIT $3 OFFSET $4"
    );
    let rows = sqlx::query_as::<_, SaleOrder>(&q)
        .bind(company_id)
        .bind(estado)
        .bind(por_pagina)
        .bind(offset)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

/// Total de órdenes
pub async fn contar(pool: &PgPool, company_id: i32) -> Result<i64, CoreError> {
    let n: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sale_order WHERE company_id = $1 AND state != 'cancel'"
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;
    Ok(n.0)
}

/// KPIs de ventas
pub async fn kpis(pool: &PgPool, company_id: i32) -> Result<SaleKpis, CoreError> {
    let row: (Option<Decimal>, Option<Decimal>, i64, i64) = sqlx::query_as(
        r#"SELECT
            SUM(CASE WHEN invoice_status = 'invoiced'   THEN amount_total ELSE 0 END),
            SUM(CASE WHEN invoice_status = 'to_invoice' THEN amount_total ELSE 0 END),
            COUNT(*) FILTER (WHERE state = 'sale'),
            COUNT(*) FILTER (WHERE date_order >= date_trunc('month', NOW()))
         FROM sale_order
         WHERE company_id = $1 AND state NOT IN ('cancel', 'draft')"#,
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;

    Ok(SaleKpis {
        total_facturado:     row.0.unwrap_or(Decimal::ZERO),
        total_por_facturar:  row.1.unwrap_or(Decimal::ZERO),
        ordenes_confirmadas: row.2,
        ordenes_este_mes:    row.3,
    })
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SaleKpis {
    pub total_facturado:     Decimal,
    pub total_por_facturar:  Decimal,
    pub ordenes_confirmadas: i64,
    pub ordenes_este_mes:    i64,
}

// ─── Sale Order Line ──────────────────────────────────────────────────────────

pub async fn obtener_lineas(pool: &PgPool, order_id: i32) -> Result<Vec<SaleOrderLine>, CoreError> {
    let lineas = sqlx::query_as::<_, SaleOrderLine>(
        r#"SELECT id, order_id, product_id, name, state,
                  product_uom_qty, price_unit, discount,
                  price_subtotal, price_total
           FROM sale_order_line
           WHERE order_id = $1
             AND (display_type IS NULL OR display_type = '')
           ORDER BY sequence ASC NULLS LAST, id ASC"#,
    )
    .bind(order_id)
    .fetch_all(pool)
    .await?;
    Ok(lineas)
}

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct ConfirmarResult {
    pub id: i32,
    pub state: Option<String>,
}

/// Confirmar orden de venta (draft/sent → sale)
pub async fn confirmar(pool: &PgPool, id: i32) -> Result<Option<ConfirmarResult>, CoreError> {
    let row = sqlx::query_as::<_, ConfirmarResult>(
        "UPDATE sale_order SET state='sale', date_order=NOW() WHERE id=$1 AND state IN ('draft','sent') RETURNING id, state"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// Cancelar orden de venta
pub async fn cancelar(pool: &PgPool, id: i32) -> Result<Option<i32>, CoreError> {
    let row: Option<(i32,)> = sqlx::query_as(
        "UPDATE sale_order SET state='cancel' WHERE id=$1 AND state NOT IN ('done','cancel') RETURNING id"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0))
}

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct CrearResult {
    pub id: i32,
    pub name: String,
}

/// Crear nueva orden de venta
pub async fn crear(
    pool: &PgPool,
    company_id: i32,
    partner_id: i32,
    nota: &str,
) -> Result<CrearResult, CoreError> {
    let row = sqlx::query_as::<_, CrearResult>(
        r#"INSERT INTO sale_order
            (name, company_id, partner_id, partner_invoice_id, partner_shipping_id,
             state, date_order, note, amount_untaxed, amount_tax, amount_total)
           VALUES (
             'SO/' || nextval('sale_order_id_seq')::text,
             $1, $2, $2, $2,
             'draft', NOW(), $3, 0, 0, 0
           )
           RETURNING id, name"#,
    )
    .bind(company_id)
    .bind(partner_id)
    .bind(nota)
    .fetch_one(pool)
    .await?;
    Ok(row)
}
