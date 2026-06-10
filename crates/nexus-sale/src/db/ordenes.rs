//! Órdenes de venta — acceso a sale_order

use sqlx::PgPool;
use serde::Serialize;
use rust_decimal::Decimal;
use crate::error::SaleError;

/// Orden de venta
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct OrdenVenta {
    pub id:             i32,
    pub name:           Option<String>,
    pub state:          Option<String>,
    pub amount_total:   Option<Decimal>,
    pub amount_untaxed: Option<Decimal>,
    pub amount_tax:     Option<Decimal>,
    pub partner_id:     Option<i32>,
    pub company_id:     i32,
}

/// KPIs de ventas
#[derive(Debug, Clone, Serialize)]
pub struct KpisVentas {
    pub total_ordenes:    i64,
    pub ordenes_mes:      i64,
    pub importe_mes:      Decimal,
    pub importe_total:    Decimal,
    pub ticket_promedio:  Decimal,
    pub clientes_activos: i64,
}

/// Lista órdenes de venta paginadas
pub async fn listar(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<OrdenVenta>, SaleError> {
    let offset = (pagina - 1).max(0) * por_pagina;
    let rows = sqlx::query_as::<_, OrdenVenta>(
        "SELECT id, name, state, amount_total, amount_untaxed, amount_tax, partner_id, company_id
         FROM sale_order
         WHERE company_id = $1
         ORDER BY id DESC
         LIMIT $2 OFFSET $3"
    )
    .bind(company_id)
    .bind(por_pagina)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// KPIs de ventas
pub async fn kpis(
    pool: &PgPool,
    company_id: i32,
) -> Result<KpisVentas, SaleError> {
    let row = sqlx::query!(
        r#"SELECT
            COUNT(*) AS total_ordenes,
            COUNT(*) FILTER (WHERE date_order >= date_trunc('month', NOW())) AS ordenes_mes,
            COALESCE(SUM(amount_total) FILTER (WHERE date_order >= date_trunc('month', NOW())), 0) AS importe_mes,
            COALESCE(SUM(amount_total), 0) AS importe_total,
            COALESCE(AVG(amount_total), 0) AS ticket_promedio,
            COUNT(DISTINCT partner_id) FILTER (WHERE date_order >= date_trunc('month', NOW())) AS clientes_activos
           FROM sale_order
           WHERE company_id = $1 AND state IN ('sale', 'done')"#,
        company_id
    )
    .fetch_one(pool)
    .await?;

    Ok(KpisVentas {
        total_ordenes:    row.total_ordenes.unwrap_or(0),
        ordenes_mes:      row.ordenes_mes.unwrap_or(0),
        importe_mes:      row.importe_mes.unwrap_or(Decimal::ZERO),
        importe_total:    row.importe_total.unwrap_or(Decimal::ZERO),
        ticket_promedio:  row.ticket_promedio.unwrap_or(Decimal::ZERO),
        clientes_activos: row.clientes_activos.unwrap_or(0),
    })
}

/// Obtiene una orden por ID
pub async fn obtener(pool: &PgPool, order_id: i32) -> Result<OrdenVenta, SaleError> {
    let row = sqlx::query_as::<_, OrdenVenta>(
        "SELECT id, name, state, amount_total, amount_untaxed, amount_tax, partner_id, company_id
         FROM sale_order WHERE id = $1"
    )
    .bind(order_id)
    .fetch_optional(pool)
    .await?
    .ok_or(SaleError::NoEncontrada(order_id))?;
    Ok(row)
}

/// Top clientes por importe total
pub async fn top_clientes(
    pool: &PgPool,
    company_id: i32,
    limite: i64,
) -> Result<Vec<(Option<String>, Decimal)>, SaleError> {
    #[derive(sqlx::FromRow)]
    struct Row { partner_name: Option<String>, total: Option<Decimal> }
    let rows = sqlx::query_as::<_, Row>(
        "SELECT p.name AS partner_name, SUM(s.amount_total) AS total
         FROM sale_order s
         LEFT JOIN res_partner p ON p.id = s.partner_id
         WHERE s.company_id = $1 AND s.state IN ('sale','done')
         GROUP BY p.name
         ORDER BY total DESC
         LIMIT $2"
    )
    .bind(company_id)
    .bind(limite)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|r| (r.partner_name, r.total.unwrap_or(Decimal::ZERO))).collect())
}
