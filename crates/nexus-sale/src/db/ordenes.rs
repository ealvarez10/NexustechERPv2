//! Órdenes de venta — acceso a sale_order

use sqlx::PgPool;
use serde::Serialize;
use rust_decimal::Decimal;
use crate::error::SaleError;

// ─── Structs para cotizaciones ────────────────────────────────────────────────

/// Orden de venta con nombre de partner (para detalle)
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct OrdenDetalle {
    pub id:              i32,
    pub name:            Option<String>,
    pub state:           Option<String>,
    pub partner_id:      Option<i32>,
    pub partner_name:    Option<String>,
    pub amount_total:    Option<Decimal>,
    pub amount_untaxed:  Option<Decimal>,
    pub amount_tax:      Option<Decimal>,
    pub date_order:      Option<String>,
    pub validity_date:   Option<chrono::NaiveDate>,
    pub note:            Option<String>,
    pub client_order_ref: Option<String>,
    pub invoice_status:  Option<String>,
}

/// Datos para crear una nueva cotización
#[derive(Debug, serde::Deserialize)]
pub struct NuevaOrden {
    pub partner_id:          i32,
    pub partner_invoice_id:  i32,
    pub partner_shipping_id: i32,
    pub note:                Option<String>,
    pub client_order_ref:    Option<String>,
    pub validity_date:       Option<chrono::NaiveDate>,
}

/// KPIs de cotizaciones (draft/sent)
#[derive(Debug, Serialize)]
pub struct KpisCotizaciones {
    pub total_borradores: i64,
    pub importe_total:    Decimal,
    pub vencidas:         i64,
}

// ─── Funciones de cotizaciones ────────────────────────────────────────────────

/// Lista cotizaciones paginadas (state IN 'draft','sent')
pub async fn listar_cotizaciones(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<OrdenDetalle>, SaleError> {
    let offset = (pagina - 1).max(0) * por_pagina;
    let rows = sqlx::query_as::<_, OrdenDetalle>(
        "SELECT s.id, s.name, s.state, s.partner_id,
                p.name AS partner_name,
                s.amount_total, s.amount_untaxed, s.amount_tax,
                s.date_order::text AS date_order,
                s.validity_date,
                s.note, s.client_order_ref, s.invoice_status
         FROM sale_order s
         LEFT JOIN res_partner p ON p.id = s.partner_id
         WHERE s.company_id = $1 AND s.state IN ('draft','sent')
         ORDER BY s.id DESC
         LIMIT $2 OFFSET $3"
    )
    .bind(company_id)
    .bind(por_pagina)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Cuenta cotizaciones (draft/sent)
pub async fn contar_cotizaciones(pool: &PgPool, company_id: i32) -> Result<i64, SaleError> {
    let row = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM sale_order WHERE company_id = $1 AND state IN ('draft','sent')"
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

/// Obtiene una orden por ID con nombre del partner (JOIN)
pub async fn por_id(pool: &PgPool, id: i32, company_id: i32) -> Result<OrdenDetalle, SaleError> {
    let row = sqlx::query_as::<_, OrdenDetalle>(
        "SELECT s.id, s.name, s.state, s.partner_id,
                p.name AS partner_name,
                s.amount_total, s.amount_untaxed, s.amount_tax,
                s.date_order::text AS date_order,
                s.validity_date,
                s.note, s.client_order_ref, s.invoice_status
         FROM sale_order s
         LEFT JOIN res_partner p ON p.id = s.partner_id
         WHERE s.id = $1 AND s.company_id = $2"
    )
    .bind(id)
    .bind(company_id)
    .fetch_optional(pool)
    .await?
    .ok_or(SaleError::NoEncontrada(id))?;
    Ok(row)
}

/// Crea una nueva cotización con state = 'draft'
pub async fn crear(pool: &PgPool, nueva: &NuevaOrden, company_id: i32) -> Result<i32, SaleError> {
    let id = sqlx::query_scalar::<_, i32>(
        "INSERT INTO sale_order (
            company_id, partner_id, partner_invoice_id, partner_shipping_id,
            note, client_order_ref, validity_date,
            state, date_order, name,
            amount_total, amount_untaxed, amount_tax
         ) VALUES (
            $1, $2, $3, $4,
            $5, $6, $7,
            'draft', NOW(), 'COTI-' || nextval('sale_order_id_seq'),
            0, 0, 0
         ) RETURNING id"
    )
    .bind(company_id)
    .bind(nueva.partner_id)
    .bind(nueva.partner_invoice_id)
    .bind(nueva.partner_shipping_id)
    .bind(&nueva.note)
    .bind(&nueva.client_order_ref)
    .bind(nueva.validity_date)
    .fetch_one(pool)
    .await?;
    Ok(id)
}

/// Confirma una cotización (draft → sale)
pub async fn confirmar(pool: &PgPool, id: i32, company_id: i32) -> Result<(), SaleError> {
    let rows = sqlx::query(
        "UPDATE sale_order SET state = 'sale', date_order = NOW()
         WHERE id = $1 AND company_id = $2 AND state IN ('draft','sent')"
    )
    .bind(id)
    .bind(company_id)
    .execute(pool)
    .await?;
    if rows.rows_affected() == 0 {
        return Err(SaleError::NoEncontrada(id));
    }
    Ok(())
}

/// Cancela una cotización
pub async fn cancelar(pool: &PgPool, id: i32, company_id: i32) -> Result<(), SaleError> {
    let rows = sqlx::query(
        "UPDATE sale_order SET state = 'cancel'
         WHERE id = $1 AND company_id = $2 AND state NOT IN ('done','cancel')"
    )
    .bind(id)
    .bind(company_id)
    .execute(pool)
    .await?;
    if rows.rows_affected() == 0 {
        return Err(SaleError::NoEncontrada(id));
    }
    Ok(())
}

/// Actualiza nota y/o referencia del cliente
pub async fn actualizar(
    pool: &PgPool,
    id: i32,
    nota: Option<String>,
    ref_cliente: Option<String>,
    company_id: i32,
) -> Result<(), SaleError> {
    let rows = sqlx::query(
        "UPDATE sale_order
         SET note = COALESCE($3, note),
             client_order_ref = COALESCE($4, client_order_ref)
         WHERE id = $1 AND company_id = $2"
    )
    .bind(id)
    .bind(company_id)
    .bind(&nota)
    .bind(&ref_cliente)
    .execute(pool)
    .await?;
    if rows.rows_affected() == 0 {
        return Err(SaleError::NoEncontrada(id));
    }
    Ok(())
}

/// KPIs de cotizaciones
pub async fn kpis_cotizaciones(pool: &PgPool, company_id: i32) -> Result<KpisCotizaciones, SaleError> {
    #[derive(sqlx::FromRow)]
    struct Row {
        total_borradores: Option<i64>,
        importe_total: Option<Decimal>,
        vencidas: Option<i64>,
    }
    let row = sqlx::query_as::<_, Row>(
        "SELECT
            COUNT(*) AS total_borradores,
            COALESCE(SUM(amount_total), 0) AS importe_total,
            COUNT(*) FILTER (WHERE validity_date < CURRENT_DATE AND validity_date IS NOT NULL) AS vencidas
         FROM sale_order
         WHERE company_id = $1 AND state IN ('draft','sent')"
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;
    Ok(KpisCotizaciones {
        total_borradores: row.total_borradores.unwrap_or(0),
        importe_total:    row.importe_total.unwrap_or(Decimal::ZERO),
        vencidas:         row.vencidas.unwrap_or(0),
    })
}

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
