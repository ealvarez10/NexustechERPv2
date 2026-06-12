//! CRUD para sale_order y sale_order_line — Órdenes de venta
//! Réplica completa del módulo Sales de Odoo 19

use sqlx::PgPool;
use crate::error::CoreError;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;

// ─── Structs ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct SaleOrder {
    pub id:                     i32,
    pub company_id:             i32,
    pub partner_id:             Option<i32>,
    pub partner_invoice_id:     Option<i32>,
    pub partner_shipping_id:    Option<i32>,
    pub payment_term_id:        Option<i32>,
    pub pricelist_id:           Option<i32>,
    pub currency_id:            Option<i32>,
    pub user_id:                Option<i32>,
    pub team_id:                Option<i32>,
    pub name:                   Option<String>,
    pub state:                  Option<String>,
    pub invoice_status:         Option<String>,
    pub client_order_ref:       Option<String>,
    pub origin:                 Option<String>,
    pub note:                   Option<String>,
    pub validity_date:          Option<String>,
    pub commitment_date:        Option<String>,
    pub date_order:             Option<String>,
    pub amount_untaxed:         Option<Decimal>,
    pub amount_tax:             Option<Decimal>,
    pub amount_total:           Option<Decimal>,
    pub currency_rate:          Option<Decimal>,
    pub locked:                 Option<bool>,
    pub require_signature:      Option<bool>,
    pub require_payment:        Option<bool>,
    // JOINs
    pub partner_name:           Option<String>,
    pub partner_invoice_name:   Option<String>,
    pub partner_shipping_name:  Option<String>,
    pub payment_term_name:      Option<String>,
    pub currency_name:          Option<String>,
    pub user_name:              Option<String>,
    pub team_name:              Option<String>,
    // Contadores
    pub count_facturas:         Option<i64>,
    pub count_lineas:           Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct SaleOrderLine {
    pub id:                 i32,
    pub order_id:           Option<i32>,
    pub product_id:         Option<i32>,
    pub name:               Option<String>,
    pub product_name:       Option<String>,
    pub product_uom_qty:    Option<Decimal>,
    pub price_unit:         Option<Decimal>,
    pub discount:           Option<Decimal>,
    pub price_subtotal:     Option<Decimal>,
    pub price_total:        Option<Decimal>,
    pub qty_delivered:      Option<Decimal>,
    pub qty_invoiced:       Option<Decimal>,
    pub qty_to_invoice:     Option<Decimal>,
    pub invoice_status:     Option<String>,
    pub state:              Option<String>,
    pub display_type:       Option<String>,
    pub sequence:           Option<i32>,
    pub cost:               Option<Decimal>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SaleKpis {
    pub total_facturado:       Decimal,
    pub total_por_facturar:    Decimal,
    pub ordenes_confirmadas:   i64,
    pub ordenes_este_mes:      i64,
    pub total_ventas_mes:      Decimal,
}

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct ConfirmarResult {
    pub id:    i32,
    pub state: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct CrearResult {
    pub id:   i32,
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct ActualizarOrden {
    pub partner_id:          Option<i32>,
    pub partner_invoice_id:  Option<i32>,
    pub partner_shipping_id: Option<i32>,
    pub payment_term_id:     Option<i32>,
    pub user_id:             Option<i32>,
    pub team_id:             Option<i32>,
    pub client_order_ref:    Option<String>,
    pub origin:              Option<String>,
    pub note:                Option<String>,
    pub validity_date:       Option<String>,
    pub commitment_date:     Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct NuevaLinea {
    pub product_id:      Option<i32>,
    pub name:            Option<String>,
    pub display_type:    Option<String>,   // 'line_section' | null
    pub product_uom_qty: Decimal,
    pub price_unit:      Decimal,
    pub discount:        Option<Decimal>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ActualizarLinea {
    pub name:            Option<String>,
    pub product_uom_qty: Option<Decimal>,
    pub price_unit:      Option<Decimal>,
    pub discount:        Option<Decimal>,
}

// ─── SQL Columns ──────────────────────────────────────────────────────────────

const SELECT_COLS: &str = r#"
    so.id, so.company_id, so.partner_id,
    so.partner_invoice_id, so.partner_shipping_id,
    so.payment_term_id, so.pricelist_id, so.currency_id,
    so.user_id, so.team_id,
    so.name, so.state, so.invoice_status,
    so.client_order_ref, so.origin, so.note,
    so.validity_date::text AS validity_date,
    so.commitment_date::text AS commitment_date,
    so.date_order::text AS date_order,
    so.amount_untaxed, so.amount_tax, so.amount_total,
    so.currency_rate, so.locked, so.require_signature, so.require_payment,
    rp.name AS partner_name,
    rpi.name AS partner_invoice_name,
    rps.name AS partner_shipping_name,
    apt.name AS payment_term_name,
    rc.name AS currency_name,
    ru.login AS user_name,
    ct.name AS team_name,
    (SELECT COUNT(*) FROM account_move am
     JOIN sale_order_line_invoice_rel slir ON slir.invoice_line_id = am.id
     JOIN sale_order_line sol ON sol.id = slir.order_line_id
     WHERE sol.order_id = so.id AND am.move_type IN ('out_invoice','out_refund'))::bigint AS count_facturas,
    (SELECT COUNT(*) FROM sale_order_line sol2 WHERE sol2.order_id = so.id AND sol2.display_type IS NULL)::bigint AS count_lineas
"#;

const FROM_JOIN: &str = r#"
    FROM sale_order so
    LEFT JOIN res_partner rp  ON rp.id  = so.partner_id
    LEFT JOIN res_partner rpi ON rpi.id = so.partner_invoice_id
    LEFT JOIN res_partner rps ON rps.id = so.partner_shipping_id
    LEFT JOIN account_payment_term apt ON apt.id = so.payment_term_id
    LEFT JOIN res_currency rc ON rc.id = so.currency_id
    LEFT JOIN res_users ru ON ru.id = so.user_id
    LEFT JOIN crm_team ct ON ct.id = so.team_id
"#;

// ─── Consultas ────────────────────────────────────────────────────────────────

pub async fn listar(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
    estado: Option<&str>,
    buscar: Option<&str>,
    invoice_status: Option<&str>,
) -> Result<Vec<SaleOrder>, CoreError> {
    let offset = (pagina - 1).max(0) * por_pagina;
    let mut conds = vec!["so.company_id = $1".to_string()];
    let mut idx = 2i32;

    if let Some(_est) = estado {
        conds.push(format!("so.state = ${idx}"));
        idx += 1;
    } else {
        conds.push("so.state != 'cancel'".to_string());
    }

    if let Some(_inv) = invoice_status {
        conds.push(format!("so.invoice_status = ${idx}"));
        idx += 1;
    }

    if let Some(_b) = buscar {
        conds.push(format!(
            "(LOWER(so.name) LIKE ${idx} OR LOWER(COALESCE(rp.name,'')) LIKE ${idx} OR LOWER(COALESCE(so.client_order_ref,'')) LIKE ${idx})"
        ));
        idx += 1;
    }

    let q = format!(
        "SELECT {SELECT_COLS} {FROM_JOIN}
         WHERE {}
         ORDER BY so.date_order DESC NULLS LAST
         LIMIT ${idx} OFFSET ${}",
        conds.join(" AND "),
        idx + 1
    );

    let mut qb = sqlx::query_as::<_, SaleOrder>(&q).bind(company_id);
    if let Some(est) = estado { qb = qb.bind(est); }
    if let Some(inv) = invoice_status { qb = qb.bind(inv.to_string()); }
    if let Some(b) = buscar { qb = qb.bind(format!("%{}%", b.to_lowercase())); }
    qb = qb.bind(por_pagina).bind(offset);
    let rows = qb.fetch_all(pool).await?;
    Ok(rows)
}

pub async fn contar(
    pool: &PgPool,
    company_id: i32,
    estado: Option<&str>,
    buscar: Option<&str>,
    invoice_status: Option<&str>,
) -> Result<i64, CoreError> {
    let mut conds = vec!["so.company_id = $1".to_string()];
    let mut idx = 2i32;

    if estado.is_some() {
        conds.push(format!("so.state = ${idx}"));
        idx += 1;
    } else {
        conds.push("so.state != 'cancel'".to_string());
    }
    if invoice_status.is_some() {
        conds.push(format!("so.invoice_status = ${idx}"));
        idx += 1;
    }
    if buscar.is_some() {
        conds.push(format!(
            "(LOWER(so.name) LIKE ${idx} OR LOWER(COALESCE(rp.name,'')) LIKE ${idx} OR LOWER(COALESCE(so.client_order_ref,'')) LIKE ${idx})"
        ));
    }

    let q = format!(
        "SELECT COUNT(*) FROM sale_order so
         LEFT JOIN res_partner rp ON rp.id = so.partner_id
         WHERE {}",
        conds.join(" AND ")
    );
    let mut qb = sqlx::query_as::<_, (i64,)>(&q).bind(company_id);
    if let Some(est) = estado { qb = qb.bind(est); }
    if let Some(inv) = invoice_status { qb = qb.bind(inv.to_string()); }
    if let Some(b) = buscar { qb = qb.bind(format!("%{}%", b.to_lowercase())); }
    let (n,) = qb.fetch_one(pool).await?;
    Ok(n)
}

pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<SaleOrder, CoreError> {
    let q = format!("SELECT {SELECT_COLS} {FROM_JOIN} WHERE so.id = $1");
    sqlx::query_as::<_, SaleOrder>(&q)
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| CoreError::not_found("Orden de venta", id))
}

pub async fn obtener_lineas(pool: &PgPool, order_id: i32) -> Result<Vec<SaleOrderLine>, CoreError> {
    let lineas = sqlx::query_as::<_, SaleOrderLine>(
        r#"SELECT
            sol.id, sol.order_id, sol.product_id,
            sol.name,
            COALESCE(pt.name->>'es_MX', pt.name->>'en_US', pt.name::text) AS product_name,
            sol.product_uom_qty, sol.price_unit, sol.discount,
            sol.price_subtotal, sol.price_total,
            sol.qty_delivered, sol.qty_invoiced, sol.qty_to_invoice,
            sol.invoice_status, sol.state, sol.display_type, sol.sequence,
            pp.standard_price AS cost
           FROM sale_order_line sol
           LEFT JOIN product_product pp ON pp.id = sol.product_id
           LEFT JOIN product_template pt ON pt.id = pp.product_tmpl_id
           WHERE sol.order_id = $1
           ORDER BY sol.sequence ASC NULLS LAST, sol.id ASC"#,
    )
    .bind(order_id)
    .fetch_all(pool)
    .await?;
    Ok(lineas)
}

pub async fn kpis(pool: &PgPool, company_id: i32) -> Result<SaleKpis, CoreError> {
    let row: (Option<Decimal>, Option<Decimal>, i64, i64, Option<Decimal>) = sqlx::query_as(
        r#"SELECT
            SUM(CASE WHEN invoice_status = 'invoiced'   THEN amount_total ELSE 0 END),
            SUM(CASE WHEN invoice_status = 'to_invoice' THEN amount_total ELSE 0 END),
            COUNT(*) FILTER (WHERE state = 'sale'),
            COUNT(*) FILTER (WHERE date_order >= date_trunc('month', NOW())),
            SUM(CASE WHEN date_order >= date_trunc('month', NOW()) THEN amount_total ELSE 0 END)
           FROM sale_order
           WHERE company_id = $1 AND state NOT IN ('cancel', 'draft')"#,
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;

    Ok(SaleKpis {
        total_facturado:      row.0.unwrap_or(Decimal::ZERO),
        total_por_facturar:   row.1.unwrap_or(Decimal::ZERO),
        ordenes_confirmadas:  row.2,
        ordenes_este_mes:     row.3,
        total_ventas_mes:     row.4.unwrap_or(Decimal::ZERO),
    })
}

pub async fn confirmar(pool: &PgPool, id: i32) -> Result<Option<ConfirmarResult>, CoreError> {
    let row = sqlx::query_as::<_, ConfirmarResult>(
        "UPDATE sale_order SET state='sale', date_order=COALESCE(date_order, NOW())
         WHERE id=$1 AND state IN ('draft','sent') RETURNING id, state",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    if let Some(ref r) = row {
        // Crear stock.picking (orden de entrega) como lo hace Odoo
        let order = sqlx::query_as::<_, (Option<String>, Option<i32>, Option<i32>)>(
            "SELECT name, partner_id, company_id FROM sale_order WHERE id=$1"
        )
        .bind(r.id)
        .fetch_optional(pool)
        .await?;

        if let Some((order_name, partner_id, company_id)) = order {
            let origin = order_name.clone().unwrap_or_default();
            let picking_name = format!("PICK/{}", origin.clone().trim_start_matches('S'));

            // Verificar si ya existe un picking para esta venta
            let existing: Option<(i32,)> = sqlx::query_as(
                "SELECT id FROM stock_picking WHERE sale_id=$1 LIMIT 1"
            )
            .bind(r.id)
            .fetch_optional(pool)
            .await?;

            if existing.is_none() {
                // Crear el picking
                let picking: (i32,) = sqlx::query_as(
                    "INSERT INTO stock_picking (name, company_id, partner_id, sale_id, origin, state, scheduled_date)
                     VALUES ($1, $2, $3, $4, $5, 'ready', NOW() + INTERVAL '3 days')
                     RETURNING id"
                )
                .bind(&picking_name)
                .bind(company_id.unwrap_or(1))
                .bind(partner_id)
                .bind(r.id)
                .bind(&origin)
                .fetch_one(pool)
                .await?;

                let picking_id = picking.0;

                // Crear stock.move por cada línea de producto
                let lineas = sqlx::query_as::<_, (i32, Option<i32>, Option<String>, Option<Decimal>)>(
                    "SELECT id, product_id, name, product_uom_qty FROM sale_order_line
                     WHERE order_id=$1 AND (display_type IS NULL OR display_type='product')"
                )
                .bind(r.id)
                .fetch_all(pool)
                .await?;

                for (line_id, product_id, name, qty) in lineas {
                    if product_id.is_none() { continue; }
                    let _ = sqlx::query(
                        "INSERT INTO stock_move (picking_id, product_id, sale_line_id, name, state, product_uom_qty)
                         VALUES ($1, $2, $3, $4, 'ready', $5)"
                    )
                    .bind(picking_id)
                    .bind(product_id)
                    .bind(line_id)
                    .bind(name.unwrap_or_else(|| format!("Producto #{}", product_id.unwrap_or(0))))
                    .bind(qty.unwrap_or(Decimal::ONE))
                    .execute(pool)
                    .await;
                }
            }
        }
    }

    Ok(row)
}

pub async fn cancelar(pool: &PgPool, id: i32) -> Result<Option<i32>, CoreError> {
    let row: Option<(i32,)> = sqlx::query_as(
        "UPDATE sale_order SET state='cancel' WHERE id=$1 AND state NOT IN ('done','cancel') RETURNING id",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0))
}

pub async fn enviar(pool: &PgPool, id: i32) -> Result<Option<ConfirmarResult>, CoreError> {
    let row = sqlx::query_as::<_, ConfirmarResult>(
        "UPDATE sale_order SET state='sent' WHERE id=$1 AND state='draft' RETURNING id, state",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// Restaurar a borrador (desde cancelado o enviado), desbloqueando la orden
pub async fn restaurar_borrador(pool: &PgPool, id: i32) -> Result<Option<i32>, CoreError> {
    let row: Option<(i32,)> = sqlx::query_as(
        "UPDATE sale_order SET state='draft', locked=false
         WHERE id=$1 AND state IN ('cancel','sent') RETURNING id",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0))
}

pub async fn bloquear(pool: &PgPool, id: i32, locked: bool) -> Result<(), CoreError> {
    sqlx::query("UPDATE sale_order SET locked=$1 WHERE id=$2")
        .bind(locked).bind(id)
        .execute(pool).await?;
    Ok(())
}

pub async fn crear(
    pool: &PgPool,
    company_id: i32,
    partner_id: i32,
    partner_invoice_id: Option<i32>,
    partner_shipping_id: Option<i32>,
    nota: &str,
    client_order_ref: Option<&str>,
    validity_days: Option<i32>,
) -> Result<CrearResult, CoreError> {
    // 0 días = sin expiración (validity_date NULL); None = default 30 días
    let validity_expr = match validity_days {
        Some(0)    => "NULL".to_string(),
        Some(_)    => "NOW() + ($7 || ' days')::interval".to_string(),
        None       => "NOW() + INTERVAL '30 days'".to_string(),
    };

    let sql = format!(
        r#"INSERT INTO sale_order
            (name, company_id, partner_id, partner_invoice_id, partner_shipping_id,
             state, date_order, note, client_order_ref, validity_date,
             amount_untaxed, amount_tax, amount_total, currency_id)
           VALUES (
             'S' || LPAD(nextval('sale_order_id_seq')::text, 5, '0'),
             $1, $2, COALESCE($3, $2), COALESCE($4, $2),
             'draft', NOW(), $5, $6, {validity_expr},
             0, 0, 0,
             (SELECT id FROM res_currency WHERE name='MXN' LIMIT 1)
           )
           RETURNING id, name"#,
    );

    let mut q = sqlx::query_as::<_, CrearResult>(&sql)
        .bind(company_id)
        .bind(partner_id)
        .bind(partner_invoice_id)
        .bind(partner_shipping_id)
        .bind(nota)
        .bind(client_order_ref);
    if let Some(days) = validity_days { if days > 0 { q = q.bind(days); } }
    let row = q.fetch_one(pool).await?;
    Ok(row)
}

pub async fn actualizar(
    pool: &PgPool,
    id: i32,
    data: &ActualizarOrden,
) -> Result<(), CoreError> {
    sqlx::query(
        r#"UPDATE sale_order SET
            partner_id          = COALESCE($2,  partner_id),
            partner_invoice_id  = COALESCE($3,  partner_invoice_id),
            partner_shipping_id = COALESCE($4,  partner_shipping_id),
            payment_term_id     = COALESCE($5,  payment_term_id),
            user_id             = COALESCE($6,  user_id),
            team_id             = COALESCE($7,  team_id),
            client_order_ref    = COALESCE($8,  client_order_ref),
            origin              = COALESCE($9,  origin),
            note                = COALESCE($10, note),
            validity_date       = COALESCE($11::date, validity_date),
            commitment_date     = COALESCE($12::timestamp, commitment_date)
           WHERE id = $1 AND state NOT IN ('done', 'cancel')"#,
    )
    .bind(id)
    .bind(data.partner_id)
    .bind(data.partner_invoice_id)
    .bind(data.partner_shipping_id)
    .bind(data.payment_term_id)
    .bind(data.user_id)
    .bind(data.team_id)
    .bind(&data.client_order_ref)
    .bind(&data.origin)
    .bind(&data.note)
    .bind(&data.validity_date)
    .bind(&data.commitment_date)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn agregar_linea(
    pool: &PgPool,
    order_id: i32,
    data: &NuevaLinea,
) -> Result<i32, CoreError> {
    // Obtener nombre del producto si no se proporciona
    let name = if let Some(n) = &data.name {
        n.clone()
    } else if let Some(pid) = data.product_id {
        let row: Option<(String,)> = sqlx::query_as(
            "SELECT COALESCE(pt.name->>'es_MX', pt.name->>'en_US', pt.name::text) FROM product_product pp JOIN product_template pt ON pt.id=pp.product_tmpl_id WHERE pp.id=$1 LIMIT 1"
        ).bind(pid).fetch_optional(pool).await?;
        row.map(|r| r.0).unwrap_or_else(|| format!("Producto #{pid}"))
    } else {
        "Línea".to_string()
    };

    let discount = data.discount.unwrap_or(Decimal::ZERO);
    let subtotal = data.product_uom_qty * data.price_unit * (Decimal::ONE - discount / Decimal::from(100));
    let tax_amount = subtotal * Decimal::new(16, 2); // 16% IVA
    let total = subtotal + tax_amount;

    // Obtener max sequence
    let (seq,): (Option<i32>,) = sqlx::query_as(
        "SELECT MAX(sequence) FROM sale_order_line WHERE order_id=$1"
    ).bind(order_id).fetch_one(pool).await?;
    let next_seq = seq.unwrap_or(0) + 10;

    let (new_id,): (i32,) = sqlx::query_as(
        r#"INSERT INTO sale_order_line
            (order_id, product_id, name, display_type, product_uom_qty, price_unit, discount,
             price_subtotal, price_total, state, customer_lead, sequence)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9,
             (SELECT state FROM sale_order WHERE id=$1 LIMIT 1),
             0, $10)
           RETURNING id"#,
    )
    .bind(order_id)
    .bind(data.product_id)
    .bind(&name)
    .bind(&data.display_type)
    .bind(data.product_uom_qty)
    .bind(data.price_unit)
    .bind(discount)
    .bind(subtotal)
    .bind(total)
    .bind(next_seq)
    .fetch_one(pool)
    .await?;

    // Recalcular totales de la orden
    _recalcular_totales(pool, order_id).await?;
    Ok(new_id)
}

pub async fn actualizar_linea(
    pool: &PgPool,
    order_id: i32,
    linea_id: i32,
    data: &ActualizarLinea,
) -> Result<(), CoreError> {
    sqlx::query(
        r#"UPDATE sale_order_line SET
            name            = COALESCE($3, name),
            product_uom_qty = COALESCE($4, product_uom_qty),
            price_unit      = COALESCE($5, price_unit),
            discount        = COALESCE($6, discount),
            price_subtotal  = CASE WHEN $4 IS NOT NULL OR $5 IS NOT NULL OR $6 IS NOT NULL THEN
                COALESCE($4, product_uom_qty) * COALESCE($5, price_unit) *
                (1 - COALESCE($6, discount, 0) / 100)
            ELSE price_subtotal END,
            price_total = CASE WHEN $4 IS NOT NULL OR $5 IS NOT NULL OR $6 IS NOT NULL THEN
                COALESCE($4, product_uom_qty) * COALESCE($5, price_unit) *
                (1 - COALESCE($6, discount, 0) / 100) * 1.16
            ELSE price_total END
           WHERE id=$2 AND order_id=$1"#,
    )
    .bind(order_id)
    .bind(linea_id)
    .bind(&data.name)
    .bind(data.product_uom_qty)
    .bind(data.price_unit)
    .bind(data.discount)
    .execute(pool)
    .await?;

    _recalcular_totales(pool, order_id).await?;
    Ok(())
}

pub async fn eliminar_linea(pool: &PgPool, order_id: i32, linea_id: i32) -> Result<(), CoreError> {
    sqlx::query("DELETE FROM sale_order_line WHERE id=$1 AND order_id=$2")
        .bind(linea_id)
        .bind(order_id)
        .execute(pool)
        .await?;
    _recalcular_totales(pool, order_id).await?;
    Ok(())
}

async fn _recalcular_totales(pool: &PgPool, order_id: i32) -> Result<(), CoreError> {
    sqlx::query(
        r#"UPDATE sale_order SET
            amount_untaxed = (SELECT COALESCE(SUM(price_subtotal), 0) FROM sale_order_line WHERE order_id=$1 AND display_type IS NULL),
            amount_tax     = (SELECT COALESCE(SUM(price_total - price_subtotal), 0) FROM sale_order_line WHERE order_id=$1 AND display_type IS NULL),
            amount_total   = (SELECT COALESCE(SUM(price_total), 0) FROM sale_order_line WHERE order_id=$1 AND display_type IS NULL)
           WHERE id = $1"#,
    )
    .bind(order_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn buscar_clientes(pool: &PgPool, q: &str, limit: i64) -> Result<Vec<(i32, String, Option<String>)>, CoreError> {
    let pat = format!("%{}%", q.to_lowercase());
    let rows: Vec<(i32, String, Option<String>)> = sqlx::query_as(
        "SELECT id, name, email FROM res_partner
         WHERE active=true AND (customer_rank > 0 OR is_company=true)
           AND (LOWER(name) LIKE $1 OR LOWER(email) LIKE $1)
         ORDER BY customer_rank DESC, name ASC
         LIMIT $2"
    )
    .bind(&pat)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn buscar_productos(pool: &PgPool, q: &str, limit: i64) -> Result<Vec<ProductoResumen>, CoreError> {
    let pat = format!("%{}%", q.to_lowercase());
    let rows = sqlx::query_as::<_, ProductoResumen>(
        r#"SELECT pp.id,
                  COALESCE(pt.name->>'es_MX', pt.name->>'en_US', pt.name::text) AS name,
                  pt.list_price AS precio, pt.default_code AS codigo
           FROM product_product pp
           JOIN product_template pt ON pt.id = pp.product_tmpl_id
           WHERE pt.active=true AND (pt.sale_ok=true OR pt.sale_ok IS NULL)
             AND (LOWER(pt.name::text) LIKE $1 OR LOWER(COALESCE(pt.default_code,'')) LIKE $1)
           ORDER BY pt.name::text ASC
           LIMIT $2"#,
    )
    .bind(&pat)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct ProductoResumen {
    pub id:     i32,
    pub name:   Option<String>,
    pub precio: Option<Decimal>,
    pub codigo: Option<String>,
}

// ─── FLUJO VENTAS → FACTURACIÓN ───────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct FacturaResumen {
    pub id:             i32,
    pub name:           Option<String>,
    pub state:          Option<String>,
    pub payment_state:  Option<String>,
    pub amount_total:   Option<Decimal>,
    pub invoice_date:   Option<chrono::NaiveDate>,
}

/// Obtener facturas vinculadas a una orden de venta (vía sale_order_line_invoice_rel)
pub async fn facturas_de_venta(pool: &PgPool, order_id: i32) -> Result<Vec<FacturaResumen>, CoreError> {
    let rows = sqlx::query_as::<_, FacturaResumen>(
        r#"SELECT DISTINCT am.id, am.name, am.state, am.payment_state, am.amount_total, am.invoice_date
           FROM account_move am
           JOIN account_move_line aml ON aml.move_id = am.id
           JOIN sale_order_line_invoice_rel rel ON rel.invoice_line_id = aml.id
           JOIN sale_order_line sol ON sol.id = rel.order_line_id
           WHERE sol.order_id = $1
             AND am.move_type = 'out_invoice'
             AND am.state != 'cancel'
           ORDER BY am.id ASC"#
    )
    .bind(order_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CrearFacturaResult {
    pub factura_id:   i32,
    pub factura_name: Option<String>,
    pub order_name:   String,
}

/// Crear factura de cliente a partir de una orden de venta confirmada.
/// Crea account_move (draft) + account_move_lines desde sale_order_lines.
/// Vincula vía sale_order_line_invoice_rel.
pub async fn crear_factura_desde_venta(
    pool: &PgPool,
    order_id: i32,
    company_id: i32,
    advance_payment_method: &str,
    amount_pct: Option<f64>,
    fixed_amount: Option<f64>,
) -> Result<CrearFacturaResult, CoreError> {
    // 1. Leer la orden
    let order = obtener_por_id(pool, order_id).await?;
    let state_str = order.state.as_deref().unwrap_or("");
    if state_str != "sale" && state_str != "done" {
        return Err(CoreError::Validation("Solo se pueden facturar pedidos confirmados".into()));
    }

    let lineas = obtener_lineas(pool, order_id).await?;
    let lineas: Vec<_> = lineas.into_iter()
        .filter(|l| l.display_type.as_deref() != Some("line_section"))
        .collect();
    if lineas.is_empty() {
        return Err(CoreError::Validation("El pedido no tiene líneas de producto para facturar".into()));
    }

    // 2. Obtener journal de ventas (tipo='sale') o el primero disponible
    let journal_id: i32 = sqlx::query_as::<_, (i32,)>(
        "SELECT id FROM account_journal WHERE company_id=$1 AND type='sale' ORDER BY id LIMIT 1"
    )
    .bind(company_id)
    .fetch_optional(pool)
    .await?
    .map(|r| r.0)
    .unwrap_or(1);

    let currency_id: i32 = order.currency_id.unwrap_or(33); // 33 = MXN
    let partner_id  = order.partner_id.unwrap_or(1);
    let origin_name = order.name.clone().unwrap_or_default();

    // 3. Calcular totales según el método de pago
    let subtotal_full: f64 = lineas.iter()
        .map(|l| l.price_subtotal.as_ref().and_then(|d| d.to_f64()).unwrap_or(0.0))
        .sum();
    let total_full = subtotal_full * 1.16;

    let (subtotal, total) = match advance_payment_method {
        "percentage" => {
            let pct = amount_pct.unwrap_or(30.0).min(100.0).max(0.0) / 100.0;
            let s = subtotal_full * pct;
            (s, s * 1.16)
        }
        "fixed" => {
            let f = fixed_amount.unwrap_or(0.0).min(total_full).max(0.0);
            let s = f / 1.16;
            (s, f)
        }
        _ => (subtotal_full, total_full), // "delivered" — factura regular
    };
    let iva = total - subtotal;

    // 4. Cuenta contable de ingresos
    let account_id: i32 = sqlx::query_as::<_, (i32,)>(
        "SELECT id FROM account_account WHERE account_type='income' ORDER BY id LIMIT 1"
    )
    .fetch_optional(pool)
    .await?
    .map(|r| r.0)
    .unwrap_or(1);

    // 5. Crear account_move (factura borrador)
    let move_row = sqlx::query_as::<_, (i32, Option<String>)>(
        r#"INSERT INTO account_move
            (journal_id, company_id, partner_id, commercial_partner_id, partner_shipping_id,
             move_type, state, date, invoice_date,
             currency_id, invoice_origin,
             amount_untaxed, amount_tax, amount_total, amount_residual,
             amount_total_signed, amount_residual_signed,
             invoice_currency_rate, always_tax_exigible, checked, auto_post)
           VALUES ($1, $2, $3, $3, $3,
                   'out_invoice', 'draft', CURRENT_DATE, CURRENT_DATE,
                   $4, $5,
                   $6, $7, $8, $8,
                   $8, $8,
                   1, false, false, 'no')
           RETURNING id, name"#
    )
    .bind(journal_id)
    .bind(company_id)
    .bind(partner_id)
    .bind(currency_id)
    .bind(&origin_name)
    .bind(Decimal::from_f64(subtotal).unwrap_or(Decimal::ZERO))
    .bind(Decimal::from_f64(iva).unwrap_or(Decimal::ZERO))
    .bind(Decimal::from_f64(total).unwrap_or(Decimal::ZERO))
    .fetch_one(pool)
    .await?;

    let factura_id   = move_row.0;
    let factura_name = move_row.1;

    // 6. Crear líneas de factura
    // Para anticipo % o fijo, creamos 1 línea de anticipo en vez de todas las líneas
    if advance_payment_method == "percentage" || advance_payment_method == "fixed" {
        let desc = match advance_payment_method {
            "percentage" => format!("Anticipo {}% — {}", amount_pct.unwrap_or(30.0) as i64, origin_name),
            _            => format!("Anticipo monto fijo — {}", origin_name),
        };
        sqlx::query(
            r#"INSERT INTO account_move_line
                (move_id, account_id, name, quantity, price_unit, discount,
                 price_subtotal, price_total,
                 display_type, company_id, currency_id)
               VALUES ($1, $2, $3, 1, $4, 0, $4, $5, 'product', $6, $7)"#
        )
        .bind(factura_id)
        .bind(account_id)
        .bind(&desc)
        .bind(Decimal::from_f64(subtotal).unwrap_or(Decimal::ZERO))
        .bind(Decimal::from_f64(total).unwrap_or(Decimal::ZERO))
        .bind(company_id)
        .bind(currency_id)
        .execute(pool)
        .await?;
    } else {
        // Factura regular: una línea por cada producto
        for linea in &lineas {
            let qty    = linea.product_uom_qty.as_ref().and_then(|d| d.to_f64()).unwrap_or(1.0);
            let price  = linea.price_unit.as_ref().and_then(|d| d.to_f64()).unwrap_or(0.0);
            let disc   = linea.discount.as_ref().and_then(|d| d.to_f64()).unwrap_or(0.0);
            let sub_l  = qty * price * (1.0 - disc / 100.0);
            let tot_l  = sub_l * 1.16;
            let nombre = linea.product_name.clone()
                .or_else(|| linea.name.clone())
                .unwrap_or_else(|| format!("Producto #{}", linea.product_id.unwrap_or(0)));

            let aml: (i32,) = sqlx::query_as(
                r#"INSERT INTO account_move_line
                    (move_id, product_id, account_id, name, quantity, price_unit, discount,
                     price_subtotal, price_total,
                     display_type, company_id, currency_id)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'product', $10, $11)
                   RETURNING id"#
            )
            .bind(factura_id)
            .bind(linea.product_id)
            .bind(account_id)
            .bind(&nombre)
            .bind(Decimal::from_f64(qty).unwrap_or(Decimal::ONE))
            .bind(Decimal::from_f64(price).unwrap_or(Decimal::ZERO))
            .bind(Decimal::from_f64(disc).unwrap_or(Decimal::ZERO))
            .bind(Decimal::from_f64(sub_l).unwrap_or(Decimal::ZERO))
            .bind(Decimal::from_f64(tot_l).unwrap_or(Decimal::ZERO))
            .bind(company_id)
            .bind(currency_id)
            .fetch_one(pool)
            .await?;

            // Vincular en sale_order_line_invoice_rel
            let _ = sqlx::query(
                "INSERT INTO sale_order_line_invoice_rel (order_line_id, invoice_line_id) VALUES ($1,$2) ON CONFLICT DO NOTHING"
            )
            .bind(linea.id)
            .bind(aml.0)
            .execute(pool)
            .await;
        }
    }

    // 7. Actualizar invoice_status en sale_order
    // Para anticipo: queda en "to_invoice" (falta por facturar el resto)
    // Para regular: queda en "invoiced"
    let new_inv_status = if advance_payment_method == "delivered" { "invoiced" } else { "to_invoice" };
    let _ = sqlx::query(
        "UPDATE sale_order SET invoice_status=$2 WHERE id=$1"
    )
    .bind(order_id)
    .bind(new_inv_status)
    .execute(pool)
    .await;

    Ok(CrearFacturaResult {
        factura_id,
        factura_name,
        order_name: origin_name,
    })
}

// ─── FLUJO VENTAS → ALMACÉN (Entrega) ────────────────────────────────────────

/// Información de entrega virtual para una venta confirmada.
/// Dado que no existe tabla stock_picking, devolvemos los datos del pedido
/// con el estado de entrega calculado desde las líneas.
#[derive(Debug, Clone, serde::Serialize)]
pub struct EntregaInfo {
    pub order_id:     i32,
    pub order_name:   String,
    pub partner_name: Option<String>,
    pub state:        String,        // "pendiente" | "parcial" | "entregado"
    pub lineas:       Vec<EntregaLinea>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct EntregaLinea {
    pub product_id:       Option<i32>,
    pub product_name:     Option<String>,
    pub qty_pedida:       f64,
    pub qty_entregada:    f64,
    pub qty_pendiente:    f64,
    pub unidad:           String,
}

pub async fn entrega_de_venta(pool: &PgPool, order_id: i32) -> Result<EntregaInfo, CoreError> {
    let order = obtener_por_id(pool, order_id).await?;
    let lineas = obtener_lineas(pool, order_id).await?;

    let lineas_entrega: Vec<EntregaLinea> = lineas.iter()
        .filter(|l| l.display_type.as_deref() != Some("line_section"))
        .map(|l| {
            let pedida    = l.product_uom_qty.as_ref().and_then(|d| d.to_f64()).unwrap_or(0.0);
            let entregada = l.qty_delivered.as_ref().and_then(|d| d.to_f64()).unwrap_or(0.0);
            EntregaLinea {
                product_id:    l.product_id,
                product_name:  l.product_name.clone().or_else(|| l.name.clone()),
                qty_pedida:    pedida,
                qty_entregada: entregada,
                qty_pendiente: (pedida - entregada).max(0.0),
                unidad:        "Unidad(es)".into(),
            }
        })
        .collect();

    let state = if lineas_entrega.is_empty() {
        "pendiente".into()
    } else if lineas_entrega.iter().all(|l| l.qty_pendiente == 0.0) {
        "entregado".into()
    } else if lineas_entrega.iter().any(|l| l.qty_entregada > 0.0) {
        "parcial".into()
    } else {
        "pendiente".into()
    };

    Ok(EntregaInfo {
        order_id,
        order_name:   order.name.clone().unwrap_or_default(),
        partner_name: order.partner_name,
        state,
        lineas:       lineas_entrega,
    })
}

/// Validar entrega (marcar líneas como entregadas)
pub async fn validar_entrega(pool: &PgPool, order_id: i32, lineas: Vec<(i32, f64)>) -> Result<(), CoreError> {
    for (line_id, qty) in lineas {
        sqlx::query(
            "UPDATE sale_order_line SET qty_delivered=$1 WHERE id=$2 AND order_id=$3"
        )
        .bind(Decimal::from_f64(qty).unwrap_or(Decimal::ZERO))
        .bind(line_id)
        .bind(order_id)
        .execute(pool)
        .await?;
    }
    // Recalcular invoice_status
    let _ = sqlx::query(
        r#"UPDATE sale_order SET
            invoice_status = CASE
                WHEN EXISTS (
                    SELECT 1 FROM sale_order_line
                    WHERE order_id=$1 AND qty_delivered > 0 AND invoice_status='to_invoice'
                ) THEN 'to_invoice'
                ELSE invoice_status
            END
           WHERE id=$1"#
    )
    .bind(order_id)
    .execute(pool)
    .await;
    Ok(())
}

/// Obtener el picking (entrega) vinculado a esta venta
/// Retorna (picking_id, picking_name, state, count_moves)
pub async fn get_picking_for_order(pool: &PgPool, order_id: i32)
    -> Result<Option<(i32, String, String, i64)>, CoreError>
{
    let row: Option<(i32, String, String, i64)> = sqlx::query_as(
        "SELECT sp.id, sp.name, sp.state, COUNT(sm.id)
         FROM stock_picking sp
         LEFT JOIN stock_move sm ON sm.picking_id = sp.id
         WHERE sp.sale_id = $1
         GROUP BY sp.id, sp.name, sp.state
         ORDER BY sp.id
         LIMIT 1"
    )
    .bind(order_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

// ─── Pickings ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct PickingRow {
    pub id:             i32,
    pub name:           String,
    pub origin:         Option<String>,
    pub state:          String,
    pub scheduled_date: Option<String>,
    pub date_done:      Option<String>,
    pub partner_name:   Option<String>,
    pub sale_id:        Option<i32>,
    pub count_moves:    Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct PickingMoveRow {
    pub id:              i32,
    pub picking_id:      i32,
    pub product_id:      Option<i32>,
    pub product_name:    Option<String>,
    pub name:            String,
    pub state:           String,
    pub product_uom_qty: Option<Decimal>,
    pub quantity_done:   Option<Decimal>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PickingDetail {
    pub picking: PickingRow,
    pub moves:   Vec<PickingMoveRow>,
}

pub async fn listar_pickings(
    pool: &PgPool,
    company_id: i32,
    sale_id: Option<i32>,
    state: Option<&str>,
) -> Result<Vec<PickingRow>, CoreError> {
    let rows = sqlx::query_as::<_, PickingRow>(
        r#"SELECT
            sp.id, sp.name, sp.origin, sp.state,
            TO_CHAR(sp.scheduled_date, 'YYYY-MM-DD') AS scheduled_date,
            TO_CHAR(sp.date_done,      'YYYY-MM-DD') AS date_done,
            rp.name AS partner_name,
            sp.sale_id,
            COUNT(sm.id) AS count_moves
           FROM stock_picking sp
           LEFT JOIN res_partner rp ON rp.id = sp.partner_id
           LEFT JOIN stock_move  sm ON sm.picking_id = sp.id
           WHERE sp.company_id = $1
             AND ($2::INTEGER IS NULL OR sp.sale_id = $2)
             AND ($3::TEXT    IS NULL OR sp.state   = $3)
           GROUP BY sp.id, sp.name, sp.origin, sp.state, sp.scheduled_date,
                    sp.date_done, rp.name, sp.sale_id
           ORDER BY sp.id DESC"#
    )
    .bind(company_id)
    .bind(sale_id)
    .bind(state)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn obtener_picking(pool: &PgPool, picking_id: i32) -> Result<Option<PickingDetail>, CoreError> {
    let picking = sqlx::query_as::<_, PickingRow>(
        r#"SELECT
            sp.id, sp.name, sp.origin, sp.state,
            TO_CHAR(sp.scheduled_date, 'YYYY-MM-DD') AS scheduled_date,
            TO_CHAR(sp.date_done,      'YYYY-MM-DD') AS date_done,
            rp.name AS partner_name,
            sp.sale_id,
            COUNT(sm.id) AS count_moves
           FROM stock_picking sp
           LEFT JOIN res_partner rp ON rp.id = sp.partner_id
           LEFT JOIN stock_move  sm ON sm.picking_id = sp.id
           WHERE sp.id = $1
           GROUP BY sp.id, sp.name, sp.origin, sp.state, sp.scheduled_date,
                    sp.date_done, rp.name, sp.sale_id"#
    )
    .bind(picking_id)
    .fetch_optional(pool)
    .await?;

    if let Some(p) = picking {
        let moves = sqlx::query_as::<_, PickingMoveRow>(
            r#"SELECT sm.id, sm.picking_id, sm.product_id,
                      COALESCE(pt.name->>'es_MX', pt.name->>'en_US', sm.name) AS product_name,
                      sm.name, sm.state, sm.product_uom_qty, sm.quantity_done
               FROM stock_move sm
               LEFT JOIN product_product pp ON pp.id = sm.product_id
               LEFT JOIN product_template pt ON pt.id = pp.product_tmpl_id
               WHERE sm.picking_id = $1
               ORDER BY sm.id"#
        )
        .bind(picking_id)
        .fetch_all(pool)
        .await?;
        Ok(Some(PickingDetail { picking: p, moves }))
    } else {
        Ok(None)
    }
}

pub async fn validar_picking(
    pool: &PgPool,
    picking_id: i32,
    _company_id: i32,
    moves: Vec<(i32, f64)>,
) -> Result<(), CoreError> {
    for (move_id, qty_done) in moves {
        let qty = Decimal::from_f64(qty_done).unwrap_or(Decimal::ZERO);
        sqlx::query(
            "UPDATE stock_move SET quantity_done=$1, state='done' WHERE id=$2 AND picking_id=$3"
        )
        .bind(qty)
        .bind(move_id)
        .bind(picking_id)
        .execute(pool)
        .await?;
    }
    // Cerrar picking si todos done
    let pending: Option<(i64,)> = sqlx::query_as(
        "SELECT COUNT(*) FROM stock_move WHERE picking_id=$1 AND state != 'done'"
    )
    .bind(picking_id)
    .fetch_optional(pool)
    .await?;
    if pending.map(|r| r.0).unwrap_or(0) == 0 {
        sqlx::query("UPDATE stock_picking SET state='done', date_done=NOW() WHERE id=$1")
            .bind(picking_id)
            .execute(pool)
            .await?;
        // Sincronizar qty_delivered en sale_order_line
        let _ = sqlx::query(
            r#"UPDATE sale_order_line sol
               SET qty_delivered = COALESCE((
                   SELECT sm.quantity_done FROM stock_move sm WHERE sm.sale_line_id = sol.id AND sm.picking_id=$1 LIMIT 1
               ), sol.qty_delivered)
               FROM stock_picking sp
               WHERE sp.id = $1 AND sp.sale_id IS NOT NULL AND sol.order_id = sp.sale_id"#
        )
        .bind(picking_id)
        .execute(pool)
        .await;
        // invoice_status → to_invoice
        let sale: Option<(i32,)> = sqlx::query_as(
            "SELECT sale_id FROM stock_picking WHERE id=$1"
        )
        .bind(picking_id)
        .fetch_optional(pool)
        .await?;
        if let Some((sid,)) = sale {
            let _ = sqlx::query(
                "UPDATE sale_order SET invoice_status='to_invoice' WHERE id=$1 AND state='sale'"
            )
            .bind(sid)
            .execute(pool)
            .await;
        }
    }
    Ok(())
}

// ─── DUPLICAR ORDEN ───────────────────────────────────────────────────────────

/// Duplica una orden de venta: crea nueva con estado draft copiando todos los campos y líneas.
/// Equivale al botón "Duplicate" de Odoo.
pub async fn duplicar(pool: &PgPool, order_id: i32, company_id: i32) -> Result<CrearResult, CoreError> {
    // 1. Leer la orden original
    let original = obtener_por_id(pool, order_id).await?;

    // 2. Crear la nueva orden copiando datos
    let nueva: CrearResult = sqlx::query_as(
        r#"INSERT INTO sale_order
            (name, company_id, partner_id, partner_invoice_id, partner_shipping_id,
             payment_term_id, pricelist_id, currency_id, user_id, team_id,
             state, date_order, note, client_order_ref, origin,
             validity_date, commitment_date,
             amount_untaxed, amount_tax, amount_total)
           VALUES (
             'S' || LPAD(nextval('sale_order_id_seq')::text, 5, '0'),
             $1, $2, $3, $4,
             $5, $6, $7, $8, $9,
             'draft', NOW(), $10, $11, $12,
             $13, $14,
             $15, $16, $17
           )
           RETURNING id, name"#,
    )
    .bind(company_id)
    .bind(original.partner_id)
    .bind(original.partner_invoice_id)
    .bind(original.partner_shipping_id)
    .bind(original.payment_term_id)
    .bind(original.pricelist_id)
    .bind(original.currency_id)
    .bind(original.user_id)
    .bind(original.team_id)
    .bind(&original.note)
    .bind(&original.client_order_ref)
    .bind(&original.origin)
    .bind(&original.validity_date)
    .bind(&original.commitment_date)
    .bind(original.amount_untaxed.unwrap_or_default())
    .bind(original.amount_tax.unwrap_or_default())
    .bind(original.amount_total.unwrap_or_default())
    .fetch_one(pool)
    .await?;

    let new_id = nueva.id;

    // 3. Copiar líneas de la orden original
    let lineas = obtener_lineas(pool, order_id).await?;
    for (i, l) in lineas.iter().enumerate() {
        let _ = sqlx::query(
            r#"INSERT INTO sale_order_line
                (order_id, product_id, name, display_type,
                 product_uom_qty, price_unit, discount,
                 price_subtotal, price_total, state, customer_lead, sequence)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'draft', 0, $10)"#,
        )
        .bind(new_id)
        .bind(l.product_id)
        .bind(&l.name)
        .bind(&l.display_type)
        .bind(l.product_uom_qty.unwrap_or_default())
        .bind(l.price_unit.unwrap_or_default())
        .bind(l.discount.unwrap_or_default())
        .bind(l.price_subtotal.unwrap_or_default())
        .bind(l.price_total.unwrap_or_default())
        .bind((i as i32 + 1) * 10)
        .execute(pool)
        .await;
    }

    Ok(nueva)
}
