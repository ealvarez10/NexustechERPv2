//! Modelos Rust del schema PostgreSQL de NexusTech ERP
//!
//! Convenciones:
//! - Nombres de struct = PascalCase del nombre de tabla
//! - Nombres de campo = snake_case idéntico al nombre de columna
//! - Option<T> para columnas IS NULLABLE = YES
//! - Compatible con SQLx (derive FromRow)

use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ═══════════════════════════════════════════════════════════════
//  RES_PARTNER — Contactos / Clientes / Proveedores
//  Tabla: res_partner (77 columnas)
// ═══════════════════════════════════════════════════════════════
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ResPartner {
    pub id:                       i32,
    pub company_id:               Option<i32>,
    pub parent_id:                Option<i32>,
    pub user_id:                  Option<i32>,
    pub country_id:               Option<i32>,
    pub commercial_partner_id:    Option<i32>,
    pub create_uid:               Option<i32>,
    pub write_uid:                Option<i32>,
    pub name:                     Option<String>,
    pub complete_name:            Option<String>,
    pub lang:                     Option<String>,
    pub vat:                      Option<String>,
    pub website:                  Option<String>,
    pub street:                   Option<String>,
    pub city:                     Option<String>,
    pub email:                    Option<String>,
    pub phone:                    Option<String>,
    pub commercial_company_name:  Option<String>,
    pub company_name:             Option<String>,
    pub active:                   Option<bool>,
    pub is_company:               Option<bool>,
    pub write_date:               Option<NaiveDateTime>,
    pub create_date:              Option<NaiveDateTime>,
    pub email_normalized:         Option<String>,
    pub phone_sanitized:          Option<String>,
    pub supplier_rank:            Option<i32>,
    pub customer_rank:            Option<i32>,
    pub autopost_bills:           String,
    pub group_rfq:                bool,
    pub group_on:                 bool,
    // JSONB opcionales
    pub property_supplier_payment_term_id: Option<serde_json::Value>,
}

/// Vista de contacto — campos suficientes para listados Y formularios
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ResPartnerSummary {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub vat: Option<String>,
    pub is_company: Option<bool>,
    pub company_name: Option<String>,
    pub customer_rank: Option<i32>,
    pub supplier_rank: Option<i32>,
    pub active: Option<bool>,
    pub city: Option<String>,
    pub zip: Option<String>,
    pub street: Option<String>,
    pub street2: Option<String>,
    pub website: Option<String>,
    pub country_id: Option<i32>,
    pub user_id: Option<i32>,
    // property_payment_term_id es JSONB en la DB (almacena [id, name] de Odoo)
    pub property_payment_term_id: Option<serde_json::Value>,
}

// ═══════════════════════════════════════════════════════════════
//  SALE_ORDER — Órdenes de venta / Cotizaciones
//  Tabla: sale_order (52 columnas)
// ═══════════════════════════════════════════════════════════════
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SaleOrder {
    pub id:                     i32,
    pub company_id:             i32,
    pub partner_id:             Option<i32>,
    pub partner_invoice_id:     Option<i32>,
    pub partner_shipping_id:    Option<i32>,
    pub campaign_id:            Option<i32>,
    pub source_id:              Option<i32>,
    pub medium_id:              Option<i32>,
    pub journal_id:             Option<i32>,
    pub fiscal_position_id:     Option<i32>,
    pub payment_term_id:        Option<i32>,
    pub pricelist_id:           Option<i32>,
    pub currency_id:            Option<i32>,
    pub user_id:                Option<i32>,
    pub team_id:                Option<i32>,
    pub create_uid:             Option<i32>,
    pub write_uid:              Option<i32>,
    pub sale_order_template_id: Option<i32>,
    // Texto
    pub name:               Option<String>,
    pub state:              Option<String>,
    pub invoice_status:     Option<String>,
    pub client_order_ref:   Option<String>,
    pub origin:             Option<String>,
    pub note:               Option<String>,
    pub picking_policy:     Option<String>,
    // Fechas
    pub date_order:     Option<NaiveDateTime>,
    pub validity_date:  Option<NaiveDate>,
    pub create_date:    Option<NaiveDateTime>,
    pub write_date:     Option<NaiveDateTime>,
    // Montos
    pub currency_rate:      Option<Decimal>,
    pub amount_untaxed:     Option<Decimal>,
    pub amount_tax:         Option<Decimal>,
    pub amount_total:       Option<Decimal>,
    pub prepayment_percent: Option<f64>,
    // Booleanos
    pub locked:             Option<bool>,
    pub require_signature:  Option<bool>,
    pub require_payment:    Option<bool>,
    // JSONB
    pub customizable_pdf_form_fields: Option<serde_json::Value>,
    // JOIN virtual (no existe en tabla, se rellena con SQL AS)
    pub partner_name: Option<String>,
}

// ═══════════════════════════════════════════════════════════════
//  SALE_ORDER_LINE — Líneas de orden de venta
//  Tabla: sale_order_line (46 columnas)
// ═══════════════════════════════════════════════════════════════
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SaleOrderLine {
    pub id:              i32,
    pub order_id:        Option<i32>,
    pub sequence:        Option<i32>,
    pub company_id:      Option<i32>,
    pub currency_id:     Option<i32>,
    pub product_id:      Option<i32>,
    pub create_uid:      Option<i32>,
    pub write_uid:       Option<i32>,
    // Texto
    pub name:            Option<String>,
    pub state:           Option<String>,
    pub display_type:    Option<String>,
    pub invoice_status:  Option<String>,
    // Cantidades y precios
    pub product_uom_qty: Option<Decimal>,
    pub price_unit:      Option<Decimal>,
    pub discount:        Option<Decimal>,
    pub price_subtotal:  Option<Decimal>,
    pub price_total:     Option<Decimal>,
    pub qty_delivered:   Option<Decimal>,
    pub qty_invoiced:    Option<Decimal>,
    pub price_tax:       Option<f64>,
    pub customer_lead:   Option<f64>,
    // Booleanos
    pub is_downpayment:  Option<bool>,
    pub is_expense:      Option<bool>,
    pub is_delivery:     Option<bool>,
    // Fechas
    pub create_date:     Option<NaiveDateTime>,
    pub write_date:      Option<NaiveDateTime>,
}

// ═══════════════════════════════════════════════════════════════
//  PRODUCT_TEMPLATE — Plantilla de productos
//  Tabla: product_template (95 columnas — incluye campos x_mercadily_*)
// ═══════════════════════════════════════════════════════════════
/// Plantilla de producto — solo columnas que existen en esta instancia del schema
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductTemplate {
    pub id:                  i32,
    pub sequence:            Option<i32>,
    pub categ_id:            Option<i32>,
    pub uom_id:              i32,
    pub company_id:          Option<i32>,
    pub color:               Option<i32>,
    pub create_uid:          Option<i32>,
    pub write_uid:           Option<i32>,
    // Texto
    pub type_:               Option<String>,   // consu | service | product
    pub service_tracking:    Option<String>,
    pub default_code:        Option<String>,
    // Montos
    pub list_price:          Option<Decimal>,
    pub volume:              Option<Decimal>,
    pub weight:              Option<Decimal>,
    // Booleanos
    pub sale_ok:             Option<bool>,
    pub purchase_ok:         Option<bool>,
    pub active:              Option<bool>,
    pub is_favorite:         Option<bool>,
    // Fechas
    pub create_date:         Option<NaiveDateTime>,
    pub write_date:          Option<NaiveDateTime>,
    // JSONB multiidioma — extraído como texto por SQL
    pub name:                Option<String>,
    pub description:         Option<String>,
    pub description_sale:    Option<String>,
    pub product_properties:  Option<serde_json::Value>,
    pub property_account_income_id: Option<serde_json::Value>,
}

/// Vista de producto para listados — solo columnas reales del schema
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductSummary {
    pub id:           i32,
    pub default_code: Option<String>,
    pub list_price:   Option<Decimal>,
    pub active:       Option<bool>,
    pub categ_id:     Option<i32>,
    /// Nombre extraído del JSONB multiidioma (es_MX o en_US)
    pub name:         Option<String>,
    pub type_:        Option<String>,
    /// Nombre de categoría (JOIN)
    pub categ_name:   Option<String>,
}

// ═══════════════════════════════════════════════════════════════
//  ACCOUNT_MOVE — Facturas / Notas de crédito / Pólizas
//  Tabla: account_move (88 columnas)
// ═══════════════════════════════════════════════════════════════
/// Movimiento contable / factura — columnas reales verificadas en el schema
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AccountMove {
    pub id:                       i32,
    pub journal_id:               i32,
    pub company_id:               Option<i32>,
    pub partner_id:               Option<i32>,
    pub commercial_partner_id:    Option<i32>,
    pub currency_id:              Option<i32>,
    pub sequence_number:          Option<i32>,
    pub create_uid:               Option<i32>,
    pub write_uid:                Option<i32>,
    // Tipo / estado
    pub move_type:      Option<String>,
    pub state:          Option<String>,
    pub payment_state:  Option<String>,
    pub auto_post:      Option<String>,
    // Texto
    pub name:               Option<String>,
    pub invoice_origin:     Option<String>,
    pub payment_reference:  Option<String>,
    // Fechas
    pub date:              Option<NaiveDate>,
    pub invoice_date:      Option<NaiveDate>,
    pub invoice_date_due:  Option<NaiveDate>,
    pub create_date:       Option<NaiveDateTime>,
    pub write_date:        Option<NaiveDateTime>,
    // Montos
    pub amount_untaxed:         Option<Decimal>,
    pub amount_tax:             Option<Decimal>,
    pub amount_total:           Option<Decimal>,
    pub amount_residual:        Option<Decimal>,
    pub amount_total_signed:    Option<Decimal>,
    pub amount_residual_signed: Option<Decimal>,
    /// Tasa de cambio — nombre real en la tabla es `invoice_currency_rate`
    pub invoice_currency_rate:  Option<rust_decimal::Decimal>,
    // Booleanos
    pub always_tax_exigible: Option<bool>,
    pub checked:             Option<bool>,
}

// ═══════════════════════════════════════════════════════════════
//  CRM_LEAD — Leads y oportunidades
//  Tabla: crm_lead
// ═══════════════════════════════════════════════════════════════
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CrmLead {
    pub id: i32,
    pub company_id: Option<i32>,
    pub partner_id: Option<i32>,
    pub user_id: Option<i32>,
    pub team_id: Option<i32>,
    pub stage_id: Option<i32>,
    pub campaign_id: Option<i32>,
    pub source_id: Option<i32>,
    pub medium_id: Option<i32>,
    pub create_uid: Option<i32>,
    pub write_uid: Option<i32>,
    pub referred: Option<String>,
    pub type_: Option<String>,        // lead, opportunity
    pub name: String,
    pub email_from: Option<String>,
    pub phone: Option<String>,
    pub mobile: Option<String>,
    pub contact_name: Option<String>,
    pub partner_name: Option<String>,
    pub street: Option<String>,
    pub street2: Option<String>,
    pub city: Option<String>,
    pub zip: Option<String>,
    pub website: Option<String>,
    pub description: Option<String>,
    pub active: Option<bool>,
    pub probability: Option<f64>,
    pub automated_probability: Option<f64>,
    pub expected_revenue: Option<Decimal>,
    pub prorated_revenue: Option<Decimal>,
    pub recurring_revenue: Option<Decimal>,
    pub recurring_plan: Option<i32>,
    pub recurring_revenue_monthly: Option<Decimal>,
    pub priority: Option<String>,
    pub date_deadline: Option<NaiveDate>,
    pub date_open: Option<NaiveDateTime>,
    pub date_closed: Option<NaiveDateTime>,
    pub date_last_stage_update: Option<NaiveDateTime>,
    pub date_conversion: Option<NaiveDateTime>,
    pub create_date: Option<NaiveDateTime>,
    pub write_date: Option<NaiveDateTime>,
    pub lang_id: Option<i32>,
    pub country_id: Option<i32>,
    pub state_id: Option<i32>,
    pub color: Option<i32>,
    pub message_bounce: Option<i32>,
}

// ═══════════════════════════════════════════════════════════════
//  RES_USERS — Usuarios del sistema
// ═══════════════════════════════════════════════════════════════
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ResUsers {
    pub id: i32,
    pub company_id: i32,
    pub partner_id: i32,
    pub create_uid: Option<i32>,
    pub write_uid: Option<i32>,
    pub login: String,
    pub password: Option<String>,
    pub active: Option<bool>,
    pub share: Option<bool>,
    pub create_date: Option<NaiveDateTime>,
    pub write_date: Option<NaiveDateTime>,
    pub action_id: Option<i32>,
    pub totp_secret: Option<String>,
    pub notification_type: Option<String>,
}

// ═══════════════════════════════════════════════════════════════
//  RES_COMPANY — Empresas
// ═══════════════════════════════════════════════════════════════
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ResCompany {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub partner_id: Option<i32>,
    pub currency_id: Option<i32>,
    pub country_id: Option<i32>,
    pub state_id: Option<i32>,
    pub create_uid: Option<i32>,
    pub write_uid: Option<i32>,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub vat: Option<String>,          // RFC
    pub street: Option<String>,
    pub street2: Option<String>,
    pub zip: Option<String>,
    pub city: Option<String>,
    pub active: Option<bool>,
    pub create_date: Option<NaiveDateTime>,
    pub write_date: Option<NaiveDateTime>,
    // CFDI México
    pub l10n_mx_edi_pac: Option<String>,
    pub l10n_mx_edi_pac_test_env: Option<bool>,
    pub l10n_mx_edi_certificate_ids: Option<serde_json::Value>,
}

// ═══════════════════════════════════════════════════════════════
//  PRODUCT_PRODUCT — Variantes de producto
// ═══════════════════════════════════════════════════════════════
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductProduct {
    pub id: i32,
    pub product_tmpl_id: i32,
    pub create_uid: Option<i32>,
    pub write_uid: Option<i32>,
    pub default_code: Option<String>,
    pub barcode: Option<serde_json::Value>,
    pub combination_indices: Option<String>,
    pub active: Option<bool>,
    pub create_date: Option<NaiveDateTime>,
    pub write_date: Option<NaiveDateTime>,
    pub product_properties: Option<serde_json::Value>,
}
