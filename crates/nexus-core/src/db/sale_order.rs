//! CRUD para sale_order y sale_order_line — Órdenes de venta

use sqlx::PgPool;
use crate::models::{SaleOrder, SaleOrderLine};
use crate::error::CoreError;
use rust_decimal::Decimal;
use chrono::NaiveDate;

// ─── Sale Order ──────────────────────────────────────────────────────────────

/// Lista órdenes de venta de una empresa (paginado)
pub async fn listar(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<SaleOrder>, CoreError> {
    let offset = (pagina - 1) * por_pagina;
    let rows = sqlx::query_as::<_, SaleOrder>(
        r#"
        SELECT id, company_id, partner_id, partner_invoice_id, partner_shipping_id,
               campaign_id, source_id, medium_id, journal_id, fiscal_position_id,
               payment_term_id, pricelist_id, currency_id, user_id, team_id,
               create_uid, write_uid, carrier_id, website_id, sale_order_template_id,
               name, state, invoice_status, client_order_ref, origin, reference,
               signed_by, access_token, delivery_message, shop_warning, note,
               date_order, validity_date, commitment_date, signed_on,
               create_date, write_date, currency_rate,
               amount_untaxed, amount_tax, amount_total,
               prepayment_percent, shipping_weight,
               locked, require_signature, require_payment,
               recompute_delivery_price, cart_recovery_email_sent,
               pickup_location_data, customizable_pdf_form_fields
        FROM sale_order
        WHERE company_id = $1
          AND state NOT IN ('cancel')
        ORDER BY date_order DESC
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

/// Obtiene una orden de venta por ID
pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<SaleOrder, CoreError> {
    let orden = sqlx::query_as::<_, SaleOrder>(
        r#"
        SELECT id, company_id, partner_id, partner_invoice_id, partner_shipping_id,
               campaign_id, source_id, medium_id, journal_id, fiscal_position_id,
               payment_term_id, pricelist_id, currency_id, user_id, team_id,
               create_uid, write_uid, carrier_id, website_id, sale_order_template_id,
               name, state, invoice_status, client_order_ref, origin, reference,
               signed_by, access_token, delivery_message, shop_warning, note,
               date_order, validity_date, commitment_date, signed_on,
               create_date, write_date, currency_rate,
               amount_untaxed, amount_tax, amount_total,
               prepayment_percent, shipping_weight,
               locked, require_signature, require_payment,
               recompute_delivery_price, cart_recovery_email_sent,
               pickup_location_data, customizable_pdf_form_fields
        FROM sale_order
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| CoreError::not_found("Orden de venta", id))?;

    Ok(orden)
}

/// Obtiene una orden por nombre (folio: S00001)
pub async fn obtener_por_nombre(pool: &PgPool, nombre: &str, company_id: i32) -> Result<SaleOrder, CoreError> {
    let orden = sqlx::query_as::<_, SaleOrder>(
        r#"
        SELECT id, company_id, partner_id, partner_invoice_id, partner_shipping_id,
               campaign_id, source_id, medium_id, journal_id, fiscal_position_id,
               payment_term_id, pricelist_id, currency_id, user_id, team_id,
               create_uid, write_uid, carrier_id, website_id, sale_order_template_id,
               name, state, invoice_status, client_order_ref, origin, reference,
               signed_by, access_token, delivery_message, shop_warning, note,
               date_order, validity_date, commitment_date, signed_on,
               create_date, write_date, currency_rate,
               amount_untaxed, amount_tax, amount_total,
               prepayment_percent, shipping_weight,
               locked, require_signature, require_payment,
               recompute_delivery_price, cart_recovery_email_sent,
               pickup_location_data, customizable_pdf_form_fields
        FROM sale_order
        WHERE name = $1 AND company_id = $2
        LIMIT 1
        "#,
    )
    .bind(nombre)
    .bind(company_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| CoreError::NotFound(format!("Orden '{}' no encontrada", nombre)))?;

    Ok(orden)
}

/// Lista órdenes por estado: draft | sent | sale | done | cancel
pub async fn listar_por_estado(
    pool: &PgPool,
    company_id: i32,
    estado: &str,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<SaleOrder>, CoreError> {
    let offset = (pagina - 1) * por_pagina;
    let rows = sqlx::query_as::<_, SaleOrder>(
        r#"
        SELECT id, company_id, partner_id, partner_invoice_id, partner_shipping_id,
               campaign_id, source_id, medium_id, journal_id, fiscal_position_id,
               payment_term_id, pricelist_id, currency_id, user_id, team_id,
               create_uid, write_uid, carrier_id, website_id, sale_order_template_id,
               name, state, invoice_status, client_order_ref, origin, reference,
               signed_by, access_token, delivery_message, shop_warning, note,
               date_order, validity_date, commitment_date, signed_on,
               create_date, write_date, currency_rate,
               amount_untaxed, amount_tax, amount_total,
               prepayment_percent, shipping_weight,
               locked, require_signature, require_payment,
               recompute_delivery_price, cart_recovery_email_sent,
               pickup_location_data, customizable_pdf_form_fields
        FROM sale_order
        WHERE company_id = $1 AND state = $2
        ORDER BY date_order DESC
        LIMIT $3 OFFSET $4
        "#,
    )
    .bind(company_id)
    .bind(estado)
    .bind(por_pagina)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Total de órdenes por empresa (para paginación)
pub async fn contar(pool: &PgPool, company_id: i32) -> Result<i64, CoreError> {
    let n: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sale_order WHERE company_id = $1 AND state != 'cancel'"
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;
    Ok(n.0)
}

/// KPIs de ventas: total facturado, total pendiente, ordenes del mes
pub async fn kpis(pool: &PgPool, company_id: i32) -> Result<SaleKpis, CoreError> {
    let row: (Option<Decimal>, Option<Decimal>, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            SUM(CASE WHEN invoice_status = 'invoiced' THEN amount_total ELSE 0 END),
            SUM(CASE WHEN invoice_status = 'to_invoice' THEN amount_total ELSE 0 END),
            COUNT(*) FILTER (WHERE state = 'sale'),
            COUNT(*) FILTER (WHERE date_order >= date_trunc('month', NOW()))
        FROM sale_order
        WHERE company_id = $1 AND state NOT IN ('cancel', 'draft')
        "#,
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;

    Ok(SaleKpis {
        total_facturado: row.0.unwrap_or(Decimal::ZERO),
        total_por_facturar: row.1.unwrap_or(Decimal::ZERO),
        ordenes_confirmadas: row.2,
        ordenes_este_mes: row.3,
    })
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SaleKpis {
    pub total_facturado: Decimal,
    pub total_por_facturar: Decimal,
    pub ordenes_confirmadas: i64,
    pub ordenes_este_mes: i64,
}

// ─── Sale Order Line ─────────────────────────────────────────────────────────

/// Obtiene las líneas de una orden de venta
pub async fn obtener_lineas(pool: &PgPool, order_id: i32) -> Result<Vec<SaleOrderLine>, CoreError> {
    let lineas = sqlx::query_as::<_, SaleOrderLine>(
        r#"
        SELECT id, order_id, sequence, company_id, currency_id, order_partner_id,
               salesman_id, product_id, product_uom_id, linked_line_id, combo_item_id,
               create_uid, write_uid, name, state, display_type,
               qty_delivered_method, invoice_status, shop_warning,
               product_uom_qty, price_unit, discount,
               price_subtotal, price_total, price_reduce_taxexcl, price_reduce_taxinc,
               qty_delivered, qty_invoiced, qty_to_invoice,
               untaxed_amount_invoiced, untaxed_amount_to_invoice,
               technical_price_unit, price_tax, customer_lead,
               is_downpayment, is_expense, is_delivery, is_optional,
               collapse_prices, collapse_composition,
               create_date, write_date,
               analytic_distribution, extra_tax_data
        FROM sale_order_line
        WHERE order_id = $1
          AND display_type IS NULL
        ORDER BY sequence ASC, id ASC
        "#,
    )
    .bind(order_id)
    .fetch_all(pool)
    .await?;

    Ok(lineas)
}
