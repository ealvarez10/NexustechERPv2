//! nexus-search — Integración con NexusSearch (instancia Meilisearch propia)
//!
//! Reutiliza la misma instancia NexusSearch del storefront NexusTech.
//! URL: MEILI_URL env var (default: http://127.0.0.1:7700)
//! Key: MEILI_MASTER_KEY env var
//!
//! Índices ERP:
//!   erp_products → product_template + variantes
//!   erp_partners → res_partner (clientes, proveedores, contactos)
//!   erp_orders   → sale_order (ventas)
//!   erp_leads    → crm_lead (CRM pipeline)
//!   erp_pos      → productos para autocompletado POS (<50ms)

pub mod client;
pub mod indexer;
pub mod search;
pub mod setup;
pub mod error;

pub use client::NexusSearchClient;
pub use error::SearchError;

/// Nombres de índices ERP en NexusSearch
pub mod indexes {
    pub const PRODUCTS: &str = "erp_products";
    pub const PARTNERS: &str = "erp_partners";
    pub const ORDERS:   &str = "erp_orders";
    pub const LEADS:    &str = "erp_leads";
    pub const POS:      &str = "erp_pos";
}
