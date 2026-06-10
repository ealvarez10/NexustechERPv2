//! Modelos Rust del schema PostgreSQL de NexusTech ERP
//!
//! Convenciones:
//! - Nombres de struct = PascalCase del nombre de tabla
//! - Nombres de campo = snake_case idéntico al nombre de columna
//! - Option<T> para columnas IS NULLABLE = YES
//! - Compatible con SQLx (derive FromRow)

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════
//  RES_PARTNER — Contactos / Clientes / Proveedores
//  Tabla: res_partner (77 columnas)
// ═══════════════════════════════════════════════════════════════
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ResPartner {
    pub id: i32,
    pub company_id: Option<i32>,
    pub create_date: Option<NaiveDateTime>,
    pub name: Option<String>,
    pub parent_id: Option<i32>,
    pub user_id: Option<i32>,
    pub state_id: Option<i32>,
    pub country_id: Option<i32>,
    pub industry_id: Option<i32>,
    pub color: Option<i32>,
    pub commercial_partner_id: Option<i32>,
    pub create_uid: Option<i32>,
    pub write_uid: Option<i32>,
    pub complete_name: Option<String>,
    pub ref_: Option<String>, // ref es keyword Rust, mapeado con #[sqlx(rename="ref")]
    pub lang: Option<String>,
    pub tz: Option<String>,
    pub vat: Option<String>,                  // RFC en México
    pub company_registry: Option<String>,
    pub website: Option<String>,
    pub function: Option<String>,
    pub type_: Option<String>,                // type es keyword Rust
    pub street: Option<String>,
    pub street2: Option<String>,
    pub zip: Option<String>,
    pub city: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub commercial_company_name: Option<String>,
    pub company_name: Option<String>,
    pub comment: Option<String>,
    pub partner_latitude: Option<Decimal>,
    pub partner_longitude: Option<Decimal>,
    pub active: Option<bool>,
    pub employee: Option<bool>,
    pub is_company: Option<bool>,
    pub partner_share: Option<bool>,
    pub write_date: Option<NaiveDateTime>,
    pub email_normalized: Option<String>,
    pub phone_sanitized: Option<String>,
    pub supplier_rank: Option<i32>,
    pub customer_rank: Option<i32>,
    pub autopost_bills: String,
    pub website_id: Option<i32>,
    pub is_published: Option<bool>,
    pub message_bounce: Option<i32>,
    // Campos JSONB de propiedades por empresa
    // Se mantienen como serde_json::Value para compatibilidad total
    pub properties: Option<serde_json::Value>,
    pub specific_property_product_pricelist: Option<serde_json::Value>,
    pub property_account_payable_id: Option<serde_json::Value>,
    pub property_account_receivable_id: Option<serde_json::Value>,
    pub property_payment_term_id: Option<serde_json::Value>,
    pub property_supplier_payment_term_id: Option<serde_json::Value>,
    pub credit_limit: Option<serde_json::Value>,
    pub trust: Option<serde_json::Value>,
    pub followup_next_action_date: Option<serde_json::Value>,
    pub followup_responsible_id: Option<serde_json::Value>,
    pub property_delivery_carrier_id: Option<serde_json::Value>,
}

/// Vista simplificada para listados (evita cargar los 77 campos)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ResPartnerSummary {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub vat: Option<String>,
    pub is_company: Option<bool>,
    pub customer_rank: Option<i32>,
    pub supplier_rank: Option<i32>,
    pub active: Option<bool>,
    pub city: Option<String>,
    pub country_id: Option<i32>,
}

// ═══════════════════════════════════════════════════════════════
//  SALE_ORDER — Órdenes de venta / Cotizaciones
//  Tabla: sale_order (52 columnas)
// ═══════════════════════════════════════════════════════════════
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SaleOrder {
    pub id: i32,
    pub company_id: i32,
    pub partner_id: i32,
    pub partner_invoice_id: i32,
    pub partner_shipping_id: i32,
    pub campaign_id: Option<i32>,
    pub source_id: Option<i32>,
    pub medium_id: Option<i32>,
    pub journal_id: Option<i32>,
    pub fiscal_position_id: Option<i32>,
    pub payment_term_id: Option<i32>,
    pub pricelist_id: Option<i32>,
    pub currency_id: Option<i32>,
    pub user_id: Option<i32>,
    pub team_id: Option<i32>,
    pub create_uid: Option<i32>,
    pub write_uid: Option<i32>,
    pub carrier_id: Option<i32>,
    pub website_id: Option<i32>,
    pub sale_order_template_id: Option<i32>,
    // Campos de texto
    pub name: String,
    pub state: Option<String>,           // draft, sent, sale, done, cancel
    pub invoice_status: Option<String>,  // nothing, to_invoice, invoiced
    pub client_order_ref: Option<String>,
    pub origin: Option<String>,
    pub reference: Option<String>,
    pub signed_by: Option<String>,
    pub access_token: Option<String>,
    pub delivery_message: Option<String>,
    pub shop_warning: Option<String>,
    pub note: Option<String>,
    // Fechas
    pub date_order: NaiveDateTime,
    pub validity_date: Option<NaiveDate>,
    pub commitment_date: Option<NaiveDateTime>,
    pub signed_on: Option<NaiveDateTime>,
    pub create_date: Option<NaiveDateTime>,
    pub write_date: Option<NaiveDateTime>,
    // Montos
    pub currency_rate: Option<Decimal>,
    pub amount_untaxed: Option<Decimal>,
    pub amount_tax: Option<Decimal>,
    pub amount_total: Option<Decimal>,
    pub prepayment_percent: Option<f64>,
    pub shipping_weight: Option<f64>,
    // Booleanos
    pub locked: Option<bool>,
    pub require_signature: Option<bool>,
    pub require_payment: Option<bool>,
    pub recompute_delivery_price: Option<bool>,
    pub cart_recovery_email_sent: Option<bool>,
    // JSONB
    pub pickup_location_data: Option<serde_json::Value>,
    pub customizable_pdf_form_fields: Option<serde_json::Value>,
}

// ═══════════════════════════════════════════════════════════════
//  SALE_ORDER_LINE — Líneas de orden de venta
//  Tabla: sale_order_line (46 columnas)
// ═══════════════════════════════════════════════════════════════
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SaleOrderLine {
    pub id: i32,
    pub order_id: i32,
    pub sequence: Option<i32>,
    pub company_id: Option<i32>,
    pub currency_id: Option<i32>,
    pub order_partner_id: Option<i32>,
    pub salesman_id: Option<i32>,
    pub product_id: Option<i32>,
    pub product_uom_id: Option<i32>,
    pub linked_line_id: Option<i32>,
    pub combo_item_id: Option<i32>,
    pub create_uid: Option<i32>,
    pub write_uid: Option<i32>,
    pub name: String,
    pub state: Option<String>,
    pub display_type: Option<String>,
    pub qty_delivered_method: Option<String>,
    pub invoice_status: Option<String>,
    pub shop_warning: Option<String>,
    // Cantidades y precios
    pub product_uom_qty: Decimal,
    pub price_unit: Decimal,
    pub discount: Option<Decimal>,
    pub price_subtotal: Option<Decimal>,
    pub price_total: Option<Decimal>,
    pub price_reduce_taxexcl: Option<Decimal>,
    pub price_reduce_taxinc: Option<Decimal>,
    pub qty_delivered: Option<Decimal>,
    pub qty_invoiced: Option<Decimal>,
    pub qty_to_invoice: Option<Decimal>,
    pub untaxed_amount_invoiced: Option<Decimal>,
    pub untaxed_amount_to_invoice: Option<Decimal>,
    pub technical_price_unit: Option<f64>,
    pub price_tax: Option<f64>,
    pub customer_lead: f64,
    // Booleanos
    pub is_downpayment: Option<bool>,
    pub is_expense: Option<bool>,
    pub is_delivery: Option<bool>,
    pub is_optional: Option<bool>,
    pub collapse_prices: Option<bool>,
    pub collapse_composition: Option<bool>,
    // Fechas
    pub create_date: Option<NaiveDateTime>,
    pub write_date: Option<NaiveDateTime>,
    // JSONB
    pub analytic_distribution: Option<serde_json::Value>,
    pub extra_tax_data: Option<serde_json::Value>,
}

// ═══════════════════════════════════════════════════════════════
//  PRODUCT_TEMPLATE — Plantilla de productos
//  Tabla: product_template (95 columnas — incluye campos x_mercadily_*)
// ═══════════════════════════════════════════════════════════════
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductTemplate {
    pub id: i32,
    pub sequence: Option<i32>,
    pub categ_id: Option<i32>,
    pub uom_id: i32,
    pub company_id: Option<i32>,
    pub color: Option<i32>,
    pub create_uid: Option<i32>,
    pub write_uid: Option<i32>,
    pub website_id: Option<i32>,
    pub website_size_x: Option<i32>,
    pub website_size_y: Option<i32>,
    pub website_ribbon_id: Option<i32>,
    pub website_sequence: Option<i32>,
    pub base_unit_id: Option<i32>,
    pub type_: Option<String>,
    pub service_tracking: Option<String>,
    pub default_code: Option<String>,
    pub service_type: Option<String>,
    pub expense_policy: Option<String>,
    pub invoice_policy: Option<String>,
    pub variants_default_code: Option<String>,
    pub website_meta_og_img: Option<String>,
    pub sale_line_warn_msg: Option<String>,
    pub list_price: Option<Decimal>,
    pub volume: Option<Decimal>,
    pub weight: Option<Decimal>,
    pub compare_list_price: Option<Decimal>,
    pub sale_ok: Option<bool>,
    pub purchase_ok: Option<bool>,
    pub active: Option<bool>,
    pub can_image_1024_be_zoomed: Option<bool>,
    pub has_configurable_attributes: Option<bool>,
    pub is_favorite: Option<bool>,
    pub is_published: Option<bool>,
    pub is_seo_optimized: Option<bool>,
    pub publish_date: Option<NaiveDateTime>,
    pub create_date: Option<NaiveDateTime>,
    pub write_date: Option<NaiveDateTime>,
    pub rating_last_value: Option<f64>,
    pub base_unit_count: f64,
    // JSONB (campos multiidioma)
    pub name: serde_json::Value,
    pub description: Option<serde_json::Value>,
    pub description_purchase: Option<serde_json::Value>,
    pub description_sale: Option<serde_json::Value>,
    pub website_meta_title: Option<serde_json::Value>,
    pub website_meta_description: Option<serde_json::Value>,
    pub website_meta_keywords: Option<serde_json::Value>,
    pub seo_name: Option<serde_json::Value>,
    pub website_description: Option<serde_json::Value>,
    pub description_ecommerce: Option<serde_json::Value>,
    pub product_properties: Option<serde_json::Value>,
    pub property_account_income_id: Option<serde_json::Value>,
    pub property_account_expense_id: Option<serde_json::Value>,
    // Campos custom x_mercadily_* (NexusTech específicos)
    pub x_mercadily_config_id: Option<i32>,
    pub x_mercadily_min_purchase_qty: Option<i32>,
    pub x_mercadily_stock: Option<i32>,
    pub x_mercadily_id: Option<String>,
    pub x_mercadily_external_id: Option<String>,
    pub x_mercadily_slug: Option<String>,
    pub x_mercadily_full_name: Option<String>,
    pub x_mercadily_status: Option<String>,
    pub x_mercadily_condition: Option<String>,
    pub x_mercadily_sat_code: Option<String>,
    pub x_mercadily_brand_id: Option<String>,
    pub x_mercadily_brand_name: Option<String>,
    pub x_mercadily_brand_slug: Option<String>,
    pub x_mercadily_category_id: Option<String>,
    pub x_mercadily_category_name: Option<String>,
    pub x_mercadily_category_slug: Option<String>,
    pub x_mercadily_mpn: Option<String>,
    pub x_mercadily_ean: Option<String>,
    pub x_mercadily_warranty: Option<String>,
    pub x_mercadily_sync_status: Option<String>,
    pub x_mercadily_main_image_url: Option<String>,
    pub x_mercadily_main_image_alt: Option<String>,
    pub x_mercadily_video_url: Option<String>,
    pub x_mercadily_meta_title: Option<String>,
    pub x_mercadily_meta_keywords: Option<String>,
    pub x_mercadily_search_keywords: Option<String>,
    pub x_mercadily_description: Option<String>,
    pub x_mercadily_meta_description: Option<String>,
    pub x_mercadily_sync_notes: Option<String>,
    pub x_mercadily_price: Option<Decimal>,
    pub x_mercadily_compare_at_price: Option<Decimal>,
    pub x_mercadily_cost_price: Option<Decimal>,
    pub x_mercadily_weight: Option<Decimal>,
    pub x_mercadily_width: Option<Decimal>,
    pub x_mercadily_height: Option<Decimal>,
    pub x_mercadily_depth: Option<Decimal>,
    pub x_mercadily_is_physical: Option<bool>,
    pub x_mercadily_is_free_shipping: Option<bool>,
    pub x_mercadily_is_featured: Option<bool>,
    pub x_mercadily_is_visible: Option<bool>,
    pub x_mercadily_updated_at: Option<NaiveDateTime>,
    pub x_mercadily_synced_at: Option<NaiveDateTime>,
}

/// Vista de producto para listados (sin los 95 campos)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductSummary {
    pub id: i32,
    pub default_code: Option<String>,
    pub list_price: Option<Decimal>,
    pub active: Option<bool>,
    pub is_published: Option<bool>,
    pub categ_id: Option<i32>,
    pub x_mercadily_brand_name: Option<String>,
    pub x_mercadily_stock: Option<i32>,
}

// ═══════════════════════════════════════════════════════════════
//  ACCOUNT_MOVE — Facturas / Notas de crédito / Pólizas
//  Tabla: account_move (88 columnas)
// ═══════════════════════════════════════════════════════════════
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AccountMove {
    pub id: i32,
    pub journal_id: i32,
    pub company_id: Option<i32>,
    pub partner_id: Option<i32>,
    pub commercial_partner_id: Option<i32>,
    pub partner_shipping_id: Option<i32>,
    pub partner_bank_id: Option<i32>,
    pub fiscal_position_id: Option<i32>,
    pub invoice_payment_term_id: Option<i32>,
    pub currency_id: Option<i32>,
    pub sequence_number: Option<i32>,
    pub create_uid: Option<i32>,
    pub write_uid: Option<i32>,
    // Tipo de movimiento
    pub move_type: Option<String>,     // out_invoice, in_invoice, out_refund, in_refund, entry
    pub state: Option<String>,         // draft, posted, cancel
    pub payment_state: Option<String>, // not_paid, in_payment, paid, partial, reversed, invoicing_legacy
    pub invoice_source_email: Option<String>,
    pub invoice_origin: Option<String>,
    pub ref_: Option<String>,
    pub name: Option<String>,          // Folio / número de factura
    // Fechas
    pub invoice_date: Option<NaiveDate>,
    pub invoice_date_due: Option<NaiveDate>,
    pub date: Option<NaiveDate>,
    pub create_date: Option<NaiveDateTime>,
    pub write_date: Option<NaiveDateTime>,
    // Montos
    pub amount_untaxed: Option<Decimal>,
    pub amount_tax: Option<Decimal>,
    pub amount_total: Option<Decimal>,
    pub amount_residual: Option<Decimal>,
    pub amount_untaxed_signed: Option<Decimal>,
    pub amount_tax_signed: Option<Decimal>,
    pub amount_total_signed: Option<Decimal>,
    pub amount_residual_signed: Option<Decimal>,
    pub currency_rate: Option<f64>,
    // CFDI específicos (México)
    pub l10n_mx_edi_cfdi_uuid: Option<String>,      // UUID del timbre fiscal
    pub l10n_mx_edi_cfdi_supplier_rfc: Option<String>,
    pub l10n_mx_edi_cfdi_customer_rfc: Option<String>,
    pub l10n_mx_edi_usage: Option<String>,
    pub l10n_mx_edi_payment_method_id: Option<i32>,
    // Booleanos
    pub auto_post: Option<String>,
    pub is_storno: Option<bool>,
    pub always_tax_exigible: Option<bool>,
    // JSONB
    pub tax_totals: Option<serde_json::Value>,
    pub invoice_cash_rounding_id: Option<i32>,
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
