//! CRUD para stock — Inventario físico
//!
//! Basado en el schema real de stock_quant, stock_location y product_template.

use sqlx::PgPool;
use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;
use crate::error::CoreError;

// ─── Structs ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct AjusteStock {
    pub product_id: i32,
    pub cantidad: Decimal,
}

/// KPIs de inventario
#[derive(Debug, Serialize, Deserialize)]
pub struct KpisInventario {
    pub total_productos_con_stock: i64,
    pub total_sin_stock:           i64,
    pub valor_inventario:          Decimal,
    pub alertas_stock_bajo:        i64,
}

/// Stock por producto (vista resumida)
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct StockProducto {
    pub product_id:          i32,
    /// Nombre extraído del JSONB (es_MX → en_US → id)
    pub product_name:        Option<String>,
    pub cantidad_disponible: Decimal,
    pub cantidad_reservada:  Decimal,
    pub ubicacion:           Option<String>,
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Extrae el nombre español de un JSONB multilang, o inglés como fallback.
/// Se usa directamente en SQL con COALESCE.
const NOMBRE_EXPR: &str = r#"
    COALESCE(
        pt.name->>'es_MX',
        pt.name->>'en_US',
        pt.name::text
    )
"#;

// stock_quant.company_id puede ser NULL — filtramos por location interna
const WHERE_STOCK: &str = r#"
    sl.usage = 'internal'
    AND sq.quantity > 0
"#;

// ─── Consultas ───────────────────────────────────────────────────────────────

/// Lista stock disponible (paginado)
pub async fn listar_stock(
    pool: &PgPool,
    _company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<StockProducto>, CoreError> {
    let offset = (pagina - 1).max(0) * por_pagina;

    let rows = sqlx::query_as::<_, StockProducto>(&format!(
        r#"SELECT
            sq.product_id,
            {NOMBRE_EXPR}                                AS product_name,
            COALESCE(SUM(sq.quantity), 0)               AS cantidad_disponible,
            COALESCE(SUM(sq.reserved_quantity), 0)      AS cantidad_reservada,
            sl.name                             AS ubicacion
           FROM stock_quant sq
           JOIN product_product pp  ON pp.id = sq.product_id
           JOIN product_template pt ON pt.id = pp.product_tmpl_id
           JOIN stock_location   sl ON sl.id = sq.location_id
           WHERE {WHERE_STOCK}
           GROUP BY sq.product_id, pt.name, sl.name
           ORDER BY product_name ASC
           LIMIT $1 OFFSET $2"#
    ))
    .bind(por_pagina)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Total de líneas de stock para paginación
pub async fn contar_stock(pool: &PgPool, _company_id: i32) -> Result<i64, CoreError> {
    let row: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(DISTINCT sq.product_id)
           FROM stock_quant sq
           JOIN stock_location sl ON sl.id = sq.location_id
           WHERE sl.usage = 'internal' AND sq.quantity > 0"#,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

/// Stock de un producto específico (por variante product_product.id)
pub async fn stock_por_producto(
    pool: &PgPool,
    product_id: i32,
) -> Result<Vec<StockProducto>, CoreError> {
    let rows = sqlx::query_as::<_, StockProducto>(&format!(
        r#"SELECT
            sq.product_id,
            {NOMBRE_EXPR}                                AS product_name,
            COALESCE(SUM(sq.quantity), 0)               AS cantidad_disponible,
            COALESCE(SUM(sq.reserved_quantity), 0)      AS cantidad_reservada,
            sl.name                             AS ubicacion
           FROM stock_quant sq
           JOIN product_product pp  ON pp.id = sq.product_id
           JOIN product_template pt ON pt.id = pp.product_tmpl_id
           JOIN stock_location   sl ON sl.id = sq.location_id
           WHERE sq.product_id = $1
           GROUP BY sq.product_id, pt.name, sl.name"#
    ))
    .bind(product_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Productos con stock bajo (cantidad disponible < umbral)
pub async fn productos_stock_bajo(
    pool: &PgPool,
    _company_id: i32,
    umbral: i32,
) -> Result<Vec<StockProducto>, CoreError> {
    let rows = sqlx::query_as::<_, StockProducto>(&format!(
        r#"SELECT
            sq.product_id,
            {NOMBRE_EXPR}                                AS product_name,
            COALESCE(SUM(sq.quantity), 0)               AS cantidad_disponible,
            COALESCE(SUM(sq.reserved_quantity), 0)      AS cantidad_reservada,
            sl.name                             AS ubicacion
           FROM stock_quant sq
           JOIN product_product pp  ON pp.id = sq.product_id
           JOIN product_template pt ON pt.id = pp.product_tmpl_id
           JOIN stock_location   sl ON sl.id = sq.location_id
           WHERE {WHERE_STOCK}
           GROUP BY sq.product_id, pt.name, sl.name
           HAVING SUM(sq.quantity) < $1
           ORDER BY cantidad_disponible ASC
           LIMIT 50"#
    ))
    .bind(umbral)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// KPIs de inventario
pub async fn kpis(pool: &PgPool, _company_id: i32) -> Result<KpisInventario, CoreError> {
    #[derive(sqlx::FromRow)]
    struct Row {
        con_stock:       i64,
        sin_stock:       i64,
        valor:           Option<Decimal>,
        stock_bajo:      i64,
    }

    let row = sqlx::query_as::<_, Row>(
        r#"WITH stock_agg AS (
            SELECT
                sq.product_id,
                SUM(sq.quantity)           AS qty,
                MAX(pt.list_price::numeric) AS precio
            FROM stock_quant sq
            JOIN product_product pp  ON pp.id = sq.product_id
            JOIN product_template pt ON pt.id = pp.product_tmpl_id
            JOIN stock_location   sl ON sl.id = sq.location_id
            WHERE sl.usage = 'internal'
            GROUP BY sq.product_id
           )
           SELECT
               COUNT(*) FILTER (WHERE qty > 0)             AS con_stock,
               COUNT(*) FILTER (WHERE qty <= 0)             AS sin_stock,
               SUM(qty::numeric * precio::numeric)          AS valor,
               COUNT(*) FILTER (WHERE qty > 0 AND qty < 10) AS stock_bajo
           FROM stock_agg"#,
    )
    .fetch_one(pool)
    .await?;

    Ok(KpisInventario {
        total_productos_con_stock: row.con_stock,
        total_sin_stock:           row.sin_stock,
        valor_inventario:          row.valor.unwrap_or(Decimal::ZERO),
        alertas_stock_bajo:        row.stock_bajo,
    })
}

/// Realiza un ajuste de inventario manual
pub async fn ajustar(pool: &PgPool, company_id: i32, datos: AjusteStock) -> Result<(), CoreError> {
    let mut tx = pool.begin().await?;

    let row = sqlx::query("SELECT id, quantity FROM stock_quant WHERE product_id = $1 LIMIT 1")
        .bind(datos.product_id)
        .fetch_optional(&mut *tx)
        .await?;

    if let Some(r) = row {
        let q: Decimal = sqlx::Row::try_get(&r, "quantity").unwrap_or(Decimal::ZERO);
        let new_q = q + datos.cantidad;
        let id: i32 = sqlx::Row::get(&r, "id");
        sqlx::query("UPDATE stock_quant SET quantity = $1 WHERE id = $2")
            .bind(new_q)
            .bind(id)
            .execute(&mut *tx)
            .await?;
    } else {
        sqlx::query(
            "INSERT INTO stock_quant (product_id, location_id, company_id, quantity, reserved_quantity) VALUES ($1, 1, $2, $3, 0)"
        )
        .bind(datos.product_id)
        .bind(company_id)
        .bind(datos.cantidad)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}
