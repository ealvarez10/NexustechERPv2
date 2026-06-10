//! Órdenes del punto de venta — pos_order

use sqlx::PgPool;
use serde::Serialize;
use rust_decimal::Decimal;
use crate::error::PosError;

/// Orden de venta en POS
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct OrdenPos {
    pub id:           i32,
    pub name:         Option<String>,
    pub state:        Option<String>,
    pub amount_total: Option<Decimal>,
    pub amount_tax:   Option<Decimal>,
    pub partner_id:   Option<i32>,
    pub session_id:   Option<i32>,
    pub company_id:   i32,
}

/// KPIs del POS
#[derive(Debug, Clone, Serialize)]
pub struct KpisPos {
    pub ventas_hoy:     i64,
    pub importe_hoy:    Decimal,
    pub ventas_mes:     i64,
    pub importe_mes:    Decimal,
    pub ticket_promedio: Decimal,
}

/// Lista órdenes POS paginadas
pub async fn listar(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<OrdenPos>, PosError> {
    let offset = (pagina - 1).max(0) * por_pagina;
    let rows = sqlx::query_as::<_, OrdenPos>(
        "SELECT id, name, state, amount_total, amount_tax, partner_id, session_id, company_id
         FROM pos_order
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

/// KPIs del día y del mes
pub async fn kpis(pool: &PgPool, company_id: i32) -> Result<KpisPos, PosError> {
    #[derive(sqlx::FromRow)]
    struct Row {
        ventas_hoy:     Option<i64>,
        importe_hoy:    Option<Decimal>,
        ventas_mes:     Option<i64>,
        importe_mes:    Option<Decimal>,
        ticket_promedio: Option<Decimal>,
    }

    let row = sqlx::query_as::<_, Row>(
        "SELECT
            COUNT(*) FILTER (WHERE date_trunc('day', create_date) = date_trunc('day', NOW())) AS ventas_hoy,
            COALESCE(SUM(amount_total) FILTER (WHERE date_trunc('day', create_date) = date_trunc('day', NOW())), 0) AS importe_hoy,
            COUNT(*) FILTER (WHERE date_trunc('month', create_date) = date_trunc('month', NOW())) AS ventas_mes,
            COALESCE(SUM(amount_total) FILTER (WHERE date_trunc('month', create_date) = date_trunc('month', NOW())), 0) AS importe_mes,
            COALESCE(AVG(amount_total), 0) AS ticket_promedio
         FROM pos_order
         WHERE company_id = $1 AND state IN ('done', 'paid', 'invoiced')"
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;

    Ok(KpisPos {
        ventas_hoy:     row.ventas_hoy.unwrap_or(0),
        importe_hoy:    row.importe_hoy.unwrap_or(Decimal::ZERO),
        ventas_mes:     row.ventas_mes.unwrap_or(0),
        importe_mes:    row.importe_mes.unwrap_or(Decimal::ZERO),
        ticket_promedio: row.ticket_promedio.unwrap_or(Decimal::ZERO),
    })
}

/// Órdenes del día actual
pub async fn ordenes_hoy(pool: &PgPool, company_id: i32) -> Result<Vec<OrdenPos>, PosError> {
    let rows = sqlx::query_as::<_, OrdenPos>(
        "SELECT id, name, state, amount_total, amount_tax, partner_id, session_id, company_id
         FROM pos_order
         WHERE company_id = $1
           AND date_trunc('day', create_date) = date_trunc('day', NOW())
         ORDER BY id DESC"
    )
    .bind(company_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
