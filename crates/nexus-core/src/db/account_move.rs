//! CRUD para account_move — Facturas, notas de crédito, pagos

use sqlx::PgPool;
use crate::models::AccountMove;
use crate::error::CoreError;
use rust_decimal::Decimal;

// ─── SELECT compartido — SOLO columnas que existen en el schema real ──────────

const SELECT_COLS: &str = r#"
    id, journal_id, company_id, partner_id, commercial_partner_id,
    currency_id, sequence_number, create_uid, write_uid,
    move_type, state, payment_state, auto_post,
    invoice_origin, payment_reference, name,
    date, invoice_date, invoice_date_due,
    create_date, write_date,
    amount_untaxed, amount_tax, amount_total, amount_residual,
    amount_total_signed, amount_residual_signed,
    invoice_currency_rate,
    always_tax_exigible, checked
"#;

// ─── Funciones públicas ───────────────────────────────────────────────────────

/// Lista facturas de cliente (move_type = out_invoice)
pub async fn listar_facturas(pool: &PgPool, company_id: i32, pagina: i64, por_pagina: i64) -> Result<Vec<AccountMove>, CoreError> {
    listar_por_tipo(pool, company_id, "out_invoice", pagina, por_pagina).await
}

/// Lista notas de crédito (move_type = out_refund)
pub async fn listar_notas_credito(pool: &PgPool, company_id: i32, pagina: i64, por_pagina: i64) -> Result<Vec<AccountMove>, CoreError> {
    listar_por_tipo(pool, company_id, "out_refund", pagina, por_pagina).await
}

/// Lista facturas de proveedor (move_type = in_invoice)
pub async fn listar_facturas_proveedor(pool: &PgPool, company_id: i32, pagina: i64, por_pagina: i64) -> Result<Vec<AccountMove>, CoreError> {
    listar_por_tipo(pool, company_id, "in_invoice", pagina, por_pagina).await
}

async fn listar_por_tipo(
    pool: &PgPool,
    company_id: i32,
    move_type: &str,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<AccountMove>, CoreError> {
    let offset = (pagina - 1).max(0) * por_pagina;
    let q = format!(
        "SELECT {SELECT_COLS}
         FROM account_move
         WHERE company_id = $1 AND move_type = $2 AND state != 'cancel'
         ORDER BY COALESCE(invoice_date, date) DESC NULLS LAST, id DESC
         LIMIT $3 OFFSET $4"
    );
    let rows = sqlx::query_as::<_, AccountMove>(&q)
        .bind(company_id)
        .bind(move_type)
        .bind(por_pagina)
        .bind(offset)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

/// Obtiene una factura por ID
pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<AccountMove, CoreError> {
    let q = format!("SELECT {SELECT_COLS} FROM account_move WHERE id = $1");
    let move_ = sqlx::query_as::<_, AccountMove>(&q)
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| CoreError::not_found("Factura", id))?;
    Ok(move_)
}

/// Total de facturas por empresa
pub async fn contar(pool: &PgPool, company_id: i32, move_type: &str) -> Result<i64, CoreError> {
    let n: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM account_move WHERE company_id = $1 AND move_type = $2 AND state != 'cancel'"
    )
    .bind(company_id)
    .bind(move_type)
    .fetch_one(pool)
    .await?;
    Ok(n.0)
}

/// KPIs de facturación
pub async fn kpis_facturacion(pool: &PgPool, company_id: i32) -> Result<FacturacionKpis, CoreError> {
    #[derive(sqlx::FromRow)]
    struct Row {
        total_cobrado:     Option<Decimal>,
        total_por_cobrar:  Option<Decimal>,
        total_vencido:     Option<Decimal>,
        facturas_emitidas: i64,
        total_facturas:    i64,
        facturas_vencidas: i64,
    }

    let row = sqlx::query_as::<_, Row>(
        r#"SELECT
            SUM(amount_total)    FILTER (WHERE payment_state = 'paid')                                                   AS total_cobrado,
            SUM(amount_residual) FILTER (WHERE payment_state IN ('not_paid', 'in_payment', 'partial'))                   AS total_por_cobrar,
            SUM(amount_residual) FILTER (WHERE payment_state IN ('not_paid','partial') AND invoice_date_due < CURRENT_DATE) AS total_vencido,
            COUNT(*)             FILTER (WHERE state = 'posted')                                                          AS facturas_emitidas,
            COUNT(*)                                                                                                       AS total_facturas,
            COUNT(*)             FILTER (WHERE invoice_date_due < CURRENT_DATE AND payment_state NOT IN ('paid','reversed')) AS facturas_vencidas
           FROM account_move
           WHERE company_id = $1 AND move_type = 'out_invoice' AND state != 'cancel'"#,
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;

    Ok(FacturacionKpis {
        total_cobrado:      row.total_cobrado.unwrap_or(Decimal::ZERO),
        total_por_cobrar:   row.total_por_cobrar.unwrap_or(Decimal::ZERO),
        total_vencido:      row.total_vencido.unwrap_or(Decimal::ZERO),
        facturas_emitidas:  row.facturas_emitidas,
        total_facturas:     row.total_facturas,
        monto_total:        row.total_cobrado.unwrap_or(Decimal::ZERO) + row.total_por_cobrar.unwrap_or(Decimal::ZERO),
        facturas_vencidas:  row.facturas_vencidas,
    })
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FacturacionKpis {
    pub total_cobrado:    Decimal,
    pub total_por_cobrar: Decimal,
    pub total_vencido:    Decimal,
    pub monto_total:      Decimal,
    pub facturas_emitidas: i64,
    pub total_facturas:   i64,
    pub facturas_vencidas: i64,
}

/// Facturas pendientes por cobrar
pub async fn listar_por_cobrar(pool: &PgPool, company_id: i32, pagina: i64, por_pagina: i64) -> Result<Vec<AccountMove>, CoreError> {
    let offset = (pagina - 1).max(0) * por_pagina;
    let q = format!(
        "SELECT {SELECT_COLS}
         FROM account_move
         WHERE company_id = $1
           AND move_type = 'out_invoice'
           AND state = 'posted'
           AND amount_residual > 0
         ORDER BY COALESCE(invoice_date_due, date) ASC NULLS LAST
         LIMIT $2 OFFSET $3"
    );
    let rows = sqlx::query_as::<_, AccountMove>(&q)
        .bind(company_id)
        .bind(por_pagina)
        .bind(offset)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

// ─── Account Move Line ────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct AccountMoveLine {
    pub id: i32,
    pub move_id: i32,
    pub product_id: Option<i32>,
    pub name: Option<String>,
    pub quantity: Option<Decimal>,
    pub price_unit: Option<Decimal>,
    pub price_subtotal: Option<Decimal>,
    pub price_total: Option<Decimal>,
    pub discount: Option<Decimal>,
}

/// Obtiene líneas de producto de una factura
pub async fn obtener_lineas(pool: &PgPool, move_id: i32) -> Result<Vec<AccountMoveLine>, CoreError> {
    let lineas = sqlx::query_as::<_, AccountMoveLine>(
        r#"SELECT id, move_id, product_id, name, quantity,
                  price_unit, price_subtotal, price_total, discount
           FROM account_move_line
           WHERE move_id = $1
             AND display_type IN ('product', 'consu')
           ORDER BY id ASC"#,
    )
    .bind(move_id)
    .fetch_all(pool)
    .await?;
    Ok(lineas)
}

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct FacturaCambioResult {
    pub id: i32,
}

/// Confirmar factura (draft → posted)
pub async fn confirmar(pool: &PgPool, id: i32) -> Result<Option<FacturaCambioResult>, CoreError> {
    let row = sqlx::query_as::<_, FacturaCambioResult>(
        "UPDATE account_move SET state='posted' WHERE id=$1 AND state='draft' RETURNING id"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// Registrar pago en factura (posted → paid)
pub async fn registrar_pago(pool: &PgPool, id: i32) -> Result<Option<FacturaCambioResult>, CoreError> {
    let row = sqlx::query_as::<_, FacturaCambioResult>(
        "UPDATE account_move SET payment_state='paid', amount_residual=0, amount_residual_signed=0 WHERE id=$1 AND state='posted' RETURNING id"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// Cancelar factura
pub async fn cancelar(pool: &PgPool, id: i32) -> Result<Option<FacturaCambioResult>, CoreError> {
    let row = sqlx::query_as::<_, FacturaCambioResult>(
        "UPDATE account_move SET state='cancel' WHERE id=$1 AND state NOT IN ('cancel') AND payment_state NOT IN ('paid','reversed') RETURNING id"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct FacturaCrearResult {
    pub id: i32,
    pub name: Option<String>,
}

/// Crear nueva factura de cliente
pub async fn crear(
    pool: &PgPool,
    company_id: i32,
    partner_id: i32,
    journal_id: i32,
    currency_id: i32,
) -> Result<FacturaCrearResult, CoreError> {
    let row = sqlx::query_as::<_, FacturaCrearResult>(
        r#"INSERT INTO account_move
            (journal_id, company_id, partner_id, move_type, state, date,
             currency_id, amount_untaxed, amount_tax, amount_total, amount_residual,
             amount_total_signed, amount_residual_signed, invoice_currency_rate,
             always_tax_exigible, checked, auto_post)
           VALUES ($1, $2, $3, 'out_invoice', 'draft', CURRENT_DATE,
                   $4, 0, 0, 0, 0, 0, 0, 1,
                   false, false, 'no')
           RETURNING id, name"#,
    )
    .bind(journal_id)
    .bind(company_id)
    .bind(partner_id)
    .bind(currency_id)
    .fetch_one(pool)
    .await?;
    Ok(row)
}
