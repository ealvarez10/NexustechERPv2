//! Handler de Contabilidad — Asientos (account_move) completo
//! GET  /account-moves               — lista paginada de todos los tipos
//! GET  /account-moves/kpis          — KPIs resumen
//! GET  /account-moves/{id}          — obtener asiento con JOIN a partner/journal/currency
//! GET  /account-moves/{id}/lineas   — líneas del asiento con JOIN a account_account
//! POST /account-moves               — crear nuevo asiento en borrador
//! PUT  /account-moves/{id}/confirmar — draft → posted
//! PUT  /account-moves/{id}/borrador  — posted → draft
//! PUT  /account-moves/{id}/cancelar  — → cancel

use axum::{
    extract::{Path, Query, State, Extension},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use crate::state::AppState;
use crate::api;
use crate::middleware::JwtClaims;

// ─── DTOs ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AccountMoveRow {
    pub id: i32,
    pub name: Option<String>,
    pub move_type: Option<String>,
    pub state: Option<String>,
    pub payment_state: Option<String>,
    pub date: Option<NaiveDate>,
    pub invoice_date: Option<NaiveDate>,
    pub invoice_date_due: Option<NaiveDate>,
    pub partner_id: Option<i32>,
    pub partner_name: Option<String>,
    pub journal_id: i32,
    pub journal_name: Option<String>,
    pub currency_id: Option<i32>,
    pub currency_name: Option<String>,
    pub ref_: Option<String>,
    pub invoice_origin: Option<String>,
    pub payment_reference: Option<String>,
    pub amount_untaxed: Option<Decimal>,
    pub amount_tax: Option<Decimal>,
    pub amount_total: Option<Decimal>,
    pub amount_residual: Option<Decimal>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AccountMoveLineRow {
    pub id: i32,
    pub move_id: i32,
    pub account_code: Option<String>,
    pub account_name: Option<String>,
    pub name: Option<String>,
    pub partner_id: Option<i32>,
    pub partner_name: Option<String>,
    pub debit: Option<Decimal>,
    pub credit: Option<Decimal>,
    pub balance: Option<Decimal>,
    pub quantity: Option<Decimal>,
    pub price_unit: Option<Decimal>,
    pub price_subtotal: Option<Decimal>,
}

#[derive(Debug, Serialize)]
pub struct AccountKpis {
    pub total_asientos: i64,
    pub publicados: i64,
    pub borradores: i64,
    pub cancelados: i64,
    pub total_cobrar: Decimal,
    pub total_pagar: Decimal,
}

// SELECT base con JOINs
const SEL: &str = r#"
    SELECT
        m.id, m.name, m.move_type, m.state, m.payment_state,
        m.date, m.invoice_date, m.invoice_date_due,
        m.partner_id,
        p.name AS partner_name,
        m.journal_id,
        COALESCE(j.name->>'es_MX', j.name->>'en_US', j.name::text) AS journal_name,
        m.currency_id, c.name AS currency_name,
        m.ref AS ref_, m.invoice_origin, m.payment_reference,
        m.amount_untaxed, m.amount_tax, m.amount_total, m.amount_residual
    FROM account_move m
    LEFT JOIN res_partner p     ON p.id = m.partner_id
    LEFT JOIN account_journal j ON j.id = m.journal_id
    LEFT JOIN res_currency c    ON c.id = m.currency_id
"#;

// ─── Filtros ──────────────────────────────────────────────────────────────────

#[derive(Deserialize, Default)]
pub struct AccountMoveFilter {
    pub pagina:    Option<i64>,
    pub por_pagina: Option<i64>,
    pub move_type:  Option<String>,
    pub state:      Option<String>,
    pub q:          Option<String>,
}

// ─── GET /account-moves ───────────────────────────────────────────────────────

pub async fn listar(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Query(p): Query<AccountMoveFilter>,
) -> impl IntoResponse {
    let pagina     = p.pagina.unwrap_or(1).max(1);
    let por_pagina = p.por_pagina.unwrap_or(80).min(200);
    let offset     = (pagina - 1) * por_pagina;
    let company_id = claims.0.company_id;

    // Construimos la query con parámetros opcionales
    // SQLx no soporta SQL dinámico de forma nativa, así que ramificamos los casos comunes
    let result = match (&p.move_type, &p.state, &p.q) {
        // Sin filtros adicionales
        (None, None, None) => sqlx::query_as::<_, AccountMoveRow>(
            &format!("{SEL} WHERE m.company_id = $1 ORDER BY COALESCE(m.invoice_date,m.date) DESC NULLS LAST, m.id DESC LIMIT $2 OFFSET $3")
        ).bind(company_id).bind(por_pagina).bind(offset).fetch_all(&state.db).await,

        // Solo tipo
        (Some(mt), None, None) => sqlx::query_as::<_, AccountMoveRow>(
            &format!("{SEL} WHERE m.company_id=$1 AND m.move_type=$2 ORDER BY COALESCE(m.invoice_date,m.date) DESC NULLS LAST, m.id DESC LIMIT $3 OFFSET $4")
        ).bind(company_id).bind(mt).bind(por_pagina).bind(offset).fetch_all(&state.db).await,

        // Solo estado
        (None, Some(st), None) => sqlx::query_as::<_, AccountMoveRow>(
            &format!("{SEL} WHERE m.company_id=$1 AND m.state=$2 ORDER BY COALESCE(m.invoice_date,m.date) DESC NULLS LAST, m.id DESC LIMIT $3 OFFSET $4")
        ).bind(company_id).bind(st).bind(por_pagina).bind(offset).fetch_all(&state.db).await,

        // Solo búsqueda
        (None, None, Some(q)) => sqlx::query_as::<_, AccountMoveRow>(
            &format!("{SEL} WHERE m.company_id=$1 AND (m.name ILIKE $2 OR p.name ILIKE $2 OR m.ref ILIKE $2) ORDER BY COALESCE(m.invoice_date,m.date) DESC NULLS LAST, m.id DESC LIMIT $3 OFFSET $4")
        ).bind(company_id).bind(format!("%{q}%")).bind(por_pagina).bind(offset).fetch_all(&state.db).await,

        // Tipo + estado
        (Some(mt), Some(st), None) => sqlx::query_as::<_, AccountMoveRow>(
            &format!("{SEL} WHERE m.company_id=$1 AND m.move_type=$2 AND m.state=$3 ORDER BY COALESCE(m.invoice_date,m.date) DESC NULLS LAST, m.id DESC LIMIT $4 OFFSET $5")
        ).bind(company_id).bind(mt).bind(st).bind(por_pagina).bind(offset).fetch_all(&state.db).await,

        // Tipo + búsqueda
        (Some(mt), None, Some(q)) => sqlx::query_as::<_, AccountMoveRow>(
            &format!("{SEL} WHERE m.company_id=$1 AND m.move_type=$2 AND (m.name ILIKE $3 OR p.name ILIKE $3 OR m.ref ILIKE $3) ORDER BY COALESCE(m.invoice_date,m.date) DESC NULLS LAST, m.id DESC LIMIT $4 OFFSET $5")
        ).bind(company_id).bind(mt).bind(format!("%{q}%")).bind(por_pagina).bind(offset).fetch_all(&state.db).await,

        // Estado + búsqueda
        (None, Some(st), Some(q)) => sqlx::query_as::<_, AccountMoveRow>(
            &format!("{SEL} WHERE m.company_id=$1 AND m.state=$2 AND (m.name ILIKE $3 OR p.name ILIKE $3 OR m.ref ILIKE $3) ORDER BY COALESCE(m.invoice_date,m.date) DESC NULLS LAST, m.id DESC LIMIT $4 OFFSET $5")
        ).bind(company_id).bind(st).bind(format!("%{q}%")).bind(por_pagina).bind(offset).fetch_all(&state.db).await,

        // Todo
        (Some(mt), Some(st), Some(q)) => sqlx::query_as::<_, AccountMoveRow>(
            &format!("{SEL} WHERE m.company_id=$1 AND m.move_type=$2 AND m.state=$3 AND (m.name ILIKE $4 OR p.name ILIKE $4 OR m.ref ILIKE $4) ORDER BY COALESCE(m.invoice_date,m.date) DESC NULLS LAST, m.id DESC LIMIT $5 OFFSET $6")
        ).bind(company_id).bind(mt).bind(st).bind(format!("%{q}%")).bind(por_pagina).bind(offset).fetch_all(&state.db).await,
    };

    match result {
        Ok(rows) => {
            let total = rows.len() as i64;
            api::ok(serde_json::json!({ "data": rows, "total": total, "pagina": pagina })).into_response()
        },
        Err(e) => api::error(axum::http::StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()).into_response(),
    }
}

// ─── GET /account-moves/{id} ──────────────────────────────────────────────────

pub async fn obtener(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, AccountMoveRow>(
        &format!("{SEL} WHERE m.id = $1")
    ).bind(id).fetch_optional(&state.db).await;

    match result {
        Ok(Some(row)) => api::ok(row).into_response(),
        Ok(None)      => api::error(axum::http::StatusCode::NOT_FOUND, "Asiento no encontrado").into_response(),
        Err(e)        => api::error(axum::http::StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()).into_response(),
    }
}

// ─── GET /account-moves/{id}/lineas ──────────────────────────────────────────

pub async fn lineas(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, AccountMoveLineRow>(r#"
        SELECT
            l.id, l.move_id,
            COALESCE(aa.code_store->>'es_MX', aa.code_store->>'en_US', aa.code_store::text) AS account_code,
            CASE WHEN aa.name IS NULL THEN NULL
                 WHEN jsonb_typeof(aa.name) = 'string' THEN aa.name #>> '{}'
                 ELSE COALESCE(aa.name->>'es_MX', aa.name->>'en_US', aa.name::text)
            END AS account_name,
            l.name,
            l.partner_id, p.name AS partner_name,
            l.debit, l.credit, l.balance,
            l.quantity, l.price_unit, l.price_subtotal
        FROM account_move_line l
        LEFT JOIN account_account aa ON aa.id = l.account_id
        LEFT JOIN res_partner p      ON p.id  = l.partner_id
        WHERE l.move_id = $1
        ORDER BY l.id ASC"#)
        .bind(id)
        .fetch_all(&state.db)
        .await;

    match result {
        Ok(rows) => api::ok(rows).into_response(),
        Err(e)   => api::error(axum::http::StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()).into_response(),
    }
}

// ─── GET /account-moves/kpis ──────────────────────────────────────────────────

pub async fn kpis(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
) -> impl IntoResponse {
    #[derive(sqlx::FromRow)]
    struct Row {
        total_asientos: i64,
        publicados: i64,
        borradores: i64,
        cancelados: i64,
        total_cobrar: Option<Decimal>,
        total_pagar:  Option<Decimal>,
    }
    let result = sqlx::query_as::<_, Row>(r#"
        SELECT
            COUNT(*) AS total_asientos,
            COUNT(*) FILTER (WHERE state = 'posted') AS publicados,
            COUNT(*) FILTER (WHERE state = 'draft')  AS borradores,
            COUNT(*) FILTER (WHERE state = 'cancel') AS cancelados,
            SUM(amount_residual) FILTER (WHERE move_type IN ('out_invoice','out_refund') AND state='posted' AND amount_residual > 0) AS total_cobrar,
            SUM(amount_residual) FILTER (WHERE move_type IN ('in_invoice','in_refund') AND state='posted' AND amount_residual > 0)  AS total_pagar
        FROM account_move WHERE company_id = $1"#)
        .bind(claims.0.company_id)
        .fetch_one(&state.db)
        .await;

    match result {
        Ok(r) => api::ok(AccountKpis {
            total_asientos: r.total_asientos,
            publicados:     r.publicados,
            borradores:     r.borradores,
            cancelados:     r.cancelados,
            total_cobrar:   r.total_cobrar.unwrap_or(Decimal::ZERO),
            total_pagar:    r.total_pagar.unwrap_or(Decimal::ZERO),
        }).into_response(),
        Err(e) => api::error(axum::http::StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()).into_response(),
    }
}

// ─── PUT /account-moves/{id}/confirmar ───────────────────────────────────────

pub async fn confirmar(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let r = sqlx::query_scalar::<_, i32>(
        "UPDATE account_move SET state='posted' WHERE id=$1 AND state='draft' RETURNING id"
    ).bind(id).fetch_optional(&state.db).await;
    match r {
        Ok(Some(_)) => api::ok(serde_json::json!({"ok":true,"state":"posted"})).into_response(),
        Ok(None)    => api::error(axum::http::StatusCode::CONFLICT, "El asiento no está en borrador").into_response(),
        Err(e)      => api::error(axum::http::StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()).into_response(),
    }
}

// ─── PUT /account-moves/{id}/borrador ────────────────────────────────────────

pub async fn borrador(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let r = sqlx::query_scalar::<_, i32>(
        "UPDATE account_move SET state='draft' WHERE id=$1 AND state='posted' AND payment_state NOT IN ('paid','reversed') RETURNING id"
    ).bind(id).fetch_optional(&state.db).await;
    match r {
        Ok(Some(_)) => api::ok(serde_json::json!({"ok":true,"state":"draft"})).into_response(),
        Ok(None)    => api::error(axum::http::StatusCode::CONFLICT, "No se puede restablecer a borrador").into_response(),
        Err(e)      => api::error(axum::http::StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()).into_response(),
    }
}

// ─── PUT /account-moves/{id}/cancelar ────────────────────────────────────────

pub async fn cancelar(
    State(state): State<AppState>,
    Extension(_claims): Extension<JwtClaims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let r = sqlx::query_scalar::<_, i32>(
        "UPDATE account_move SET state='cancel' WHERE id=$1 AND state != 'cancel' AND payment_state NOT IN ('paid','reversed') RETURNING id"
    ).bind(id).fetch_optional(&state.db).await;
    match r {
        Ok(Some(_)) => api::ok(serde_json::json!({"ok":true})).into_response(),
        Ok(None)    => api::error(axum::http::StatusCode::CONFLICT, "No se puede cancelar").into_response(),
        Err(e)      => api::error(axum::http::StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()).into_response(),
    }
}

// ─── POST /account-moves — crear borrador ─────────────────────────────────────

#[derive(Deserialize)]
pub struct CrearAsientoBody {
    pub move_type:  Option<String>,
    pub partner_id: Option<i32>,
    pub journal_id: Option<i32>,
    pub currency_id: Option<i32>,
    pub ref_: Option<String>,
}

pub async fn crear(
    State(state): State<AppState>,
    Extension(claims): Extension<JwtClaims>,
    Json(body): Json<CrearAsientoBody>,
) -> impl IntoResponse {
    let move_type   = body.move_type.as_deref().unwrap_or("entry");
    let company_id  = claims.0.company_id;

    // Obtener journal por defecto según tipo
    let journal_id = match body.journal_id {
        Some(j) => j,
        None => {
            let jtype = match move_type {
                "out_invoice" | "out_refund" => "sale",
                "in_invoice"  | "in_refund"  => "purchase",
                _ => "general",
            };
            sqlx::query_scalar::<_, i32>(
                "SELECT id FROM account_journal WHERE type=$1 AND company_id=$2 ORDER BY id ASC LIMIT 1"
            ).bind(jtype).bind(company_id).fetch_optional(&state.db).await
             .unwrap_or(None).unwrap_or(1)
        }
    };
    let currency_id = body.currency_id.unwrap_or_else(|| {
        // Moneda de la empresa
        1 // fallback MXN
    });

    let r = sqlx::query_scalar::<_, i32>(r#"
        INSERT INTO account_move
            (journal_id, company_id, partner_id, move_type, state, date,
             currency_id, amount_untaxed, amount_tax, amount_total, amount_residual,
             amount_total_signed, amount_residual_signed, invoice_currency_rate,
             always_tax_exigible, checked, auto_post, ref)
        VALUES ($1, $2, $3, $4, 'draft', CURRENT_DATE,
                $5, 0, 0, 0, 0, 0, 0, 1, false, false, 'no', $6)
        RETURNING id"#)
        .bind(journal_id)
        .bind(company_id)
        .bind(body.partner_id)
        .bind(move_type)
        .bind(currency_id)
        .bind(&body.ref_)
        .fetch_one(&state.db)
        .await;

    match r {
        Ok(id) => api::ok(serde_json::json!({"id": id, "ok": true})).into_response(),
        Err(e) => api::error(axum::http::StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()).into_response(),
    }
}
