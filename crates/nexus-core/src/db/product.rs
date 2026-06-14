//! CRUD para product_template y product_product — Productos

use sqlx::PgPool;
use crate::models::{ProductTemplate, ProductProduct, ProductSummary};
use crate::error::CoreError;
use serde::Deserialize;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Deserialize)]
pub struct NuevoProducto {
    pub name: String,
    pub type_: String, // product, service, consu
    pub list_price: Option<Decimal>,
    pub default_code: Option<String>,
    pub barcode: Option<String>,
}

// ─── SELECT compartido para ProductSummary ────────────────────────────────────
// Solo columnas que existen en el schema real de esta instancia

const SUMMARY_COLS: &str = r#"
    pt.id,
    pt.default_code,
    pt.list_price,
    pt.active,
    pt.categ_id,
    COALESCE(
        pt.name->>'es_MX',
        pt.name->>'en_US',
        pt.name->>0,
        pt.name::text,
        '(sin nombre)'
    ) AS name,
    "type" AS type_,
    pc.name AS categ_name
"#;

const SUMMARY_FROM: &str = r#"
    product_template pt
    LEFT JOIN product_category pc ON pc.id = pt.categ_id
"#;

// ─── SELECT para ProductTemplate completo ─────────────────────────────────────
const TEMPLATE_COLS: &str = r#"
    id, sequence, categ_id, uom_id, company_id, color,
    create_uid, write_uid,
    "type" AS type_, service_tracking, default_code,
    list_price, volume, weight,
    sale_ok, purchase_ok, active, is_favorite,
    create_date, write_date,
    COALESCE(
        name->>'es_MX',
        name->>'en_US',
        name->>0,
        name::text,
        '(sin nombre)'
    ) AS name,
    description::text AS description,
    description_sale::text AS description_sale,
    product_properties, property_account_income_id
"#;

// ─── Funciones públicas ───────────────────────────────────────────────────────

/// Lista productos activos (paginado)
pub async fn listar(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<ProductSummary>, CoreError> {
    let offset = (pagina - 1).max(0) * por_pagina;
    let q = format!(
        "SELECT {SUMMARY_COLS} FROM {SUMMARY_FROM}
         WHERE (pt.company_id = $1 OR pt.company_id IS NULL) AND pt.active = true
         ORDER BY pt.id DESC
         LIMIT $2 OFFSET $3"
    );
    let rows = sqlx::query_as::<_, ProductSummary>(&q)
        .bind(company_id)
        .bind(por_pagina)
        .bind(offset)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

/// Obtiene un producto completo por ID
pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<ProductTemplate, CoreError> {
    let q = format!(
        "SELECT {TEMPLATE_COLS} FROM product_template WHERE id = $1"
    );
    let prod = sqlx::query_as::<_, ProductTemplate>(&q)
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| CoreError::not_found("Producto", id))?;
    Ok(prod)
}

/// Busca productos por nombre (JSONB), código o descripción
pub async fn buscar(
    pool: &PgPool,
    company_id: i32,
    termino: &str,
    limite: i64,
) -> Result<Vec<ProductSummary>, CoreError> {
    let patron = format!("%{}%", termino.to_lowercase());
    let q = format!(
        "SELECT {SUMMARY_COLS} FROM {SUMMARY_FROM}
         WHERE (pt.company_id = $1 OR pt.company_id IS NULL)
           AND pt.active = true
           AND (
               LOWER(pt.default_code) LIKE $2
               OR LOWER(pt.name::text) LIKE $2
           )
         ORDER BY pt.id DESC
         LIMIT $3"
    );
    let rows = sqlx::query_as::<_, ProductSummary>(&q)
        .bind(company_id)
        .bind(&patron)
        .bind(limite)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

/// Total de productos activos
pub async fn contar(pool: &PgPool, company_id: i32) -> Result<i64, CoreError> {
    let n: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM product_template
         WHERE (company_id = $1 OR company_id IS NULL) AND active = true"
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;
    Ok(n.0)
}

/// Obtiene todas las variantes de un producto template
pub async fn obtener_variantes(pool: &PgPool, tmpl_id: i32) -> Result<Vec<ProductProduct>, CoreError> {
    let vars = sqlx::query_as::<_, ProductProduct>(
        r#"SELECT id, product_tmpl_id, create_uid, write_uid,
                  default_code, barcode, combination_indices,
                  active, create_date, write_date, product_properties
           FROM product_product
           WHERE product_tmpl_id = $1 AND active = true
           ORDER BY id ASC"#,
    )
    .bind(tmpl_id)
    .fetch_all(pool)
    .await?;
    Ok(vars)
}

/// Crea un nuevo producto (product_template y product_product)
pub async fn crear(pool: &PgPool, company_id: i32, datos: NuevoProducto) -> Result<ProductTemplate, CoreError> {
    let mut tx = pool.begin().await?;
    let uom_id = 1; // Unidades por defecto

    let name_json = serde_json::json!({ "es_MX": datos.name });

    let tmpl: ProductTemplate = sqlx::query_as(
        r#"INSERT INTO product_template (name, type, list_price, default_code, company_id, active, uom_id, service_tracking, create_date, write_date)
           VALUES ($1, $2, $3, $4, $5, true, $6, 'no', NOW(), NOW())
           RETURNING * "#
    )
    .bind(&name_json)
    .bind(&datos.type_)
    .bind(datos.list_price)
    .bind(&datos.default_code)
    .bind(company_id)
    .bind(uom_id)
    .fetch_one(&mut *tx)
    .await?;

    let _prod: ProductProduct = sqlx::query_as(
        r#"INSERT INTO product_product (product_tmpl_id, default_code, barcode, active, create_date, write_date)
           VALUES ($1, $2, $3, true, NOW(), NOW())
           RETURNING * "#
    )
    .bind(tmpl.id)
    .bind(&datos.default_code)
    .bind(serde_json::json!(datos.barcode))
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(tmpl)
}
