//! CRUD para product_template y product_product — Productos

use sqlx::PgPool;
use crate::models::{ProductTemplate, ProductProduct, ProductSummary};
use crate::error::CoreError;

/// Lista productos activos publicados (paginado)
pub async fn listar(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<ProductSummary>, CoreError> {
    let offset = (pagina - 1) * por_pagina;
    let rows = sqlx::query_as::<_, ProductSummary>(
        r#"
        SELECT id, default_code, list_price, active, is_published,
               categ_id, x_mercadily_brand_name, x_mercadily_stock
        FROM product_template
        WHERE (company_id = $1 OR company_id IS NULL)
          AND active = true
        ORDER BY id DESC
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

/// Obtiene un producto completo por ID
pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<ProductTemplate, CoreError> {
    let prod = sqlx::query_as::<_, ProductTemplate>(
        r#"
        SELECT id, sequence, categ_id, uom_id, company_id, color,
               create_uid, write_uid, website_id,
               website_size_x, website_size_y, website_ribbon_id, website_sequence,
               base_unit_id,
               "type" as type_, service_tracking, default_code, service_type,
               expense_policy, invoice_policy, variants_default_code,
               website_meta_og_img, sale_line_warn_msg,
               list_price, volume, weight, compare_list_price,
               sale_ok, purchase_ok, active, can_image_1024_be_zoomed,
               has_configurable_attributes, is_favorite, is_published, is_seo_optimized,
               publish_date, create_date, write_date,
               rating_last_value, base_unit_count,
               name, description, description_purchase, description_sale,
               website_meta_title, website_meta_description, website_meta_keywords,
               seo_name, website_description, description_ecommerce,
               product_properties, property_account_income_id, property_account_expense_id,
               x_mercadily_config_id, x_mercadily_min_purchase_qty, x_mercadily_stock,
               x_mercadily_id, x_mercadily_external_id, x_mercadily_slug,
               x_mercadily_full_name, x_mercadily_status, x_mercadily_condition,
               x_mercadily_sat_code, x_mercadily_brand_id, x_mercadily_brand_name,
               x_mercadily_brand_slug, x_mercadily_category_id, x_mercadily_category_name,
               x_mercadily_category_slug, x_mercadily_mpn, x_mercadily_ean,
               x_mercadily_warranty, x_mercadily_sync_status,
               x_mercadily_main_image_url, x_mercadily_main_image_alt, x_mercadily_video_url,
               x_mercadily_meta_title, x_mercadily_meta_keywords, x_mercadily_search_keywords,
               x_mercadily_description, x_mercadily_meta_description, x_mercadily_sync_notes,
               x_mercadily_price, x_mercadily_compare_at_price, x_mercadily_cost_price,
               x_mercadily_weight, x_mercadily_width, x_mercadily_height, x_mercadily_depth,
               x_mercadily_is_physical, x_mercadily_is_free_shipping, x_mercadily_is_featured,
               x_mercadily_is_visible, x_mercadily_updated_at, x_mercadily_synced_at
        FROM product_template
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| CoreError::not_found("Producto", id))?;

    Ok(prod)
}

/// Busca productos por nombre, código, marca o EAN
pub async fn buscar(
    pool: &PgPool,
    company_id: i32,
    termino: &str,
    limite: i64,
) -> Result<Vec<ProductSummary>, CoreError> {
    let patron = format!("%{}%", termino.to_lowercase());
    let rows = sqlx::query_as::<_, ProductSummary>(
        r#"
        SELECT id, default_code, list_price, active, is_published,
               categ_id, x_mercadily_brand_name, x_mercadily_stock
        FROM product_template
        WHERE (company_id = $1 OR company_id IS NULL)
          AND active = true
          AND (
              LOWER(default_code) LIKE $2
              OR LOWER(x_mercadily_brand_name) LIKE $2
              OR LOWER(x_mercadily_ean) LIKE $2
          )
        ORDER BY id DESC
        LIMIT $3
        "#,
    )
    .bind(company_id)
    .bind(&patron)
    .bind(limite)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Obtiene todas las variantes de un producto template
pub async fn obtener_variantes(pool: &PgPool, tmpl_id: i32) -> Result<Vec<ProductProduct>, CoreError> {
    let vars = sqlx::query_as::<_, ProductProduct>(
        r#"
        SELECT id, product_tmpl_id, create_uid, write_uid,
               default_code, barcode, combination_indices,
               active, create_date, write_date, product_properties
        FROM product_product
        WHERE product_tmpl_id = $1 AND active = true
        ORDER BY id ASC
        "#,
    )
    .bind(tmpl_id)
    .fetch_all(pool)
    .await?;

    Ok(vars)
}

/// Total de productos activos
pub async fn contar(pool: &PgPool, company_id: i32) -> Result<i64, CoreError> {
    let n: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM product_template WHERE (company_id = $1 OR company_id IS NULL) AND active = true"
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;
    Ok(n.0)
}

/// Lista productos con stock bajo (x_mercadily_stock <= umbral)
pub async fn con_stock_bajo(
    pool: &PgPool,
    company_id: i32,
    umbral: i32,
) -> Result<Vec<ProductSummary>, CoreError> {
    let rows = sqlx::query_as::<_, ProductSummary>(
        r#"
        SELECT id, default_code, list_price, active, is_published,
               categ_id, x_mercadily_brand_name, x_mercadily_stock
        FROM product_template
        WHERE (company_id = $1 OR company_id IS NULL)
          AND active = true
          AND x_mercadily_stock IS NOT NULL
          AND x_mercadily_stock <= $2
        ORDER BY x_mercadily_stock ASC
        LIMIT 100
        "#,
    )
    .bind(company_id)
    .bind(umbral)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
