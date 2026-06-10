//! CRUD para account_move — Facturas, notas de crédito, pagos
//!
//! Tabla principal de contabilidad. Incluye lógica CFDI para México.

use sqlx::PgPool;
use crate::models::AccountMove;
use crate::error::CoreError;
use rust_decimal::Decimal;

/// Lista facturas de una empresa (move_type = out_invoice)
pub async fn listar_facturas(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<AccountMove>, CoreError> {
    listar_por_tipo(pool, company_id, "out_invoice", pagina, por_pagina).await
}

/// Lista notas de crédito (move_type = out_refund)
pub async fn listar_notas_credito(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<AccountMove>, CoreError> {
    listar_por_tipo(pool, company_id, "out_refund", pagina, por_pagina).await
}

/// Lista facturas de proveedor (move_type = in_invoice)
pub async fn listar_facturas_proveedor(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<AccountMove>, CoreError> {
    listar_por_tipo(pool, company_id, "in_invoice", pagina, por_pagina).await
}

async fn listar_por_tipo(
    pool: &PgPool,
    company_id: i32,
    move_type: &str,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<AccountMove>, CoreError> {
    let offset = (pagina - 1) * por_pagina;
    let rows = sqlx::query_as::<_, AccountMove>(
        r#"
        SELECT id, journal_id, company_id, partner_id, commercial_partner_id,
               partner_shipping_id, partner_bank_id, fiscal_position_id,
               invoice_payment_term_id, currency_id, sequence_number,
               create_uid, write_uid,
               move_type, state, payment_state, invoice_source_email,
               invoice_origin, "ref" as ref_, name,
               invoice_date, invoice_date_due, date,
               create_date, write_date,
               amount_untaxed, amount_tax, amount_total, amount_residual,
               amount_untaxed_signed, amount_tax_signed, amount_total_signed, amount_residual_signed,
               currency_rate,
               l10n_mx_edi_cfdi_uuid, l10n_mx_edi_cfdi_supplier_rfc,
               l10n_mx_edi_cfdi_customer_rfc, l10n_mx_edi_usage,
               l10n_mx_edi_payment_method_id,
               auto_post, is_storno, always_tax_exigible,
               tax_totals, invoice_cash_rounding_id
        FROM account_move
        WHERE company_id = $1
          AND move_type = $2
          AND state != 'cancel'
        ORDER BY invoice_date DESC, id DESC
        LIMIT $3 OFFSET $4
        "#,
    )
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
    let move_ = sqlx::query_as::<_, AccountMove>(
        r#"
        SELECT id, journal_id, company_id, partner_id, commercial_partner_id,
               partner_shipping_id, partner_bank_id, fiscal_position_id,
               invoice_payment_term_id, currency_id, sequence_number,
               create_uid, write_uid,
               move_type, state, payment_state, invoice_source_email,
               invoice_origin, "ref" as ref_, name,
               invoice_date, invoice_date_due, date,
               create_date, write_date,
               amount_untaxed, amount_tax, amount_total, amount_residual,
               amount_untaxed_signed, amount_tax_signed, amount_total_signed, amount_residual_signed,
               currency_rate,
               l10n_mx_edi_cfdi_uuid, l10n_mx_edi_cfdi_supplier_rfc,
               l10n_mx_edi_cfdi_customer_rfc, l10n_mx_edi_usage,
               l10n_mx_edi_payment_method_id,
               auto_post, is_storno, always_tax_exigible,
               tax_totals, invoice_cash_rounding_id
        FROM account_move
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| CoreError::not_found("Factura", id))?;

    Ok(move_)
}

/// Busca factura por UUID del timbre fiscal (CFDI)
pub async fn obtener_por_uuid(
    pool: &PgPool,
    uuid: &str,
) -> Result<Option<AccountMove>, CoreError> {
    let move_ = sqlx::query_as::<_, AccountMove>(
        r#"
        SELECT id, journal_id, company_id, partner_id, commercial_partner_id,
               partner_shipping_id, partner_bank_id, fiscal_position_id,
               invoice_payment_term_id, currency_id, sequence_number,
               create_uid, write_uid,
               move_type, state, payment_state, invoice_source_email,
               invoice_origin, "ref" as ref_, name,
               invoice_date, invoice_date_due, date,
               create_date, write_date,
               amount_untaxed, amount_tax, amount_total, amount_residual,
               amount_untaxed_signed, amount_tax_signed, amount_total_signed, amount_residual_signed,
               currency_rate,
               l10n_mx_edi_cfdi_uuid, l10n_mx_edi_cfdi_supplier_rfc,
               l10n_mx_edi_cfdi_customer_rfc, l10n_mx_edi_usage,
               l10n_mx_edi_payment_method_id,
               auto_post, is_storno, always_tax_exigible,
               tax_totals, invoice_cash_rounding_id
        FROM account_move
        WHERE l10n_mx_edi_cfdi_uuid = $1
        LIMIT 1
        "#,
    )
    .bind(uuid)
    .fetch_optional(pool)
    .await?;

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

/// KPIs de facturación: total facturado, por cobrar, vencido
pub async fn kpis_facturacion(pool: &PgPool, company_id: i32) -> Result<FacturacionKpis, CoreError> {
    let row: (Option<Decimal>, Option<Decimal>, Option<Decimal>, i64) = sqlx::query_as(
        r#"
        SELECT
            SUM(amount_total) FILTER (WHERE payment_state = 'paid'),
            SUM(amount_residual) FILTER (WHERE payment_state IN ('not_paid', 'in_payment', 'partial')),
            SUM(amount_residual) FILTER (
                WHERE payment_state IN ('not_paid', 'partial')
                  AND invoice_date_due < CURRENT_DATE
            ),
            COUNT(*) FILTER (WHERE state = 'posted')
        FROM account_move
        WHERE company_id = $1
          AND move_type = 'out_invoice'
          AND state = 'posted'
        "#,
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;

    Ok(FacturacionKpis {
        total_cobrado: row.0.unwrap_or(Decimal::ZERO),
        total_por_cobrar: row.1.unwrap_or(Decimal::ZERO),
        total_vencido: row.2.unwrap_or(Decimal::ZERO),
        facturas_emitidas: row.3,
    })
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FacturacionKpis {
    pub total_cobrado: Decimal,
    pub total_por_cobrar: Decimal,
    pub total_vencido: Decimal,
    pub facturas_emitidas: i64,
}

/// Facturas pendientes por cobrar (por cobrar + vencidas)
pub async fn listar_por_cobrar(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<AccountMove>, CoreError> {
    let offset = (pagina - 1) * por_pagina;
    let rows = sqlx::query_as::<_, AccountMove>(
        r#"
        SELECT id, journal_id, company_id, partner_id, commercial_partner_id,
               partner_shipping_id, partner_bank_id, fiscal_position_id,
               invoice_payment_term_id, currency_id, sequence_number,
               create_uid, write_uid,
               move_type, state, payment_state, invoice_source_email,
               invoice_origin, "ref" as ref_, name,
               invoice_date, invoice_date_due, date,
               create_date, write_date,
               amount_untaxed, amount_tax, amount_total, amount_residual,
               amount_untaxed_signed, amount_tax_signed, amount_total_signed, amount_residual_signed,
               currency_rate,
               l10n_mx_edi_cfdi_uuid, l10n_mx_edi_cfdi_supplier_rfc,
               l10n_mx_edi_cfdi_customer_rfc, l10n_mx_edi_usage,
               l10n_mx_edi_payment_method_id,
               auto_post, is_storno, always_tax_exigible,
               tax_totals, invoice_cash_rounding_id
        FROM account_move
        WHERE company_id = $1
          AND move_type = 'out_invoice'
          AND state = 'posted'
          AND payment_state IN ('not_paid', 'in_payment', 'partial')
        ORDER BY invoice_date_due ASC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(company_id)
    .bind(por_pagina)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
