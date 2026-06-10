//! CRUD para stock — Inventario físico
//!
//! Basado en el schema real de stock_quant, stock_location y product_template.

use sqlx::PgPool;
use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;
use crate::error::CoreError;

// ─── Structs ─────────────────────────────────────────────────────────────────

/// KPIs de inventario
#[derive(Debug, Serialize, Deserialize)]
pub struct KpisInventario {
    pub total_productos_con_stock: i64,
    pub total_sin_stock: i64,
    pub valor_inventario: Decimal,
    pub alertas_stock_bajo: i64,
}

/// Stock por producto (vista resumida)
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct StockProducto {
    pub product_id: i32,
    pub product_name: Option<String>,
    pub cantidad_disponible: Decimal,
    pub cantidad_reservada: Decimal,
    pub unidad: Option<String>,
    pub ubicacion: Option<String>,
}

// ─── Consultas ────────────────────────────────────────────────────────────────

/// Lista stock disponible para una empresa (paginado)
pub async fn listar_stock(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<StockProducto>, CoreError> {
    let offset = (pagina - 1) * por_pagina;

    let rows = sqlx::query_as::<_, StockProducto>(
        r#"
        SELECT
            sq.product_id,
            pt.name AS product_name,
            COALESCE(SUM(sq.quantity), 0)          AS cantidad_disponible,
            COALESCE(SUM(sq.reserved_quantity), 0) AS cantidad_reservada,
            uu.name                                AS unidad,
            sl.complete_name                       AS ubicacion
        FROM stock_quant sq
        JOIN product_product pp ON pp.id = sq.product_id
        JOIN product_template pt ON pt.id = pp.product_tmpl_id
        LEFT JOIN uom_uom uu ON uu.id = pt.uom_id
        LEFT JOIN stock_location sl ON sl.id = sq.location_id
        WHERE sq.company_id = $1
          AND sl.usage = 'internal'
        GROUP BY sq.product_id, pt.name, uu.name, sl.complete_name
        ORDER BY pt.name ASC
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

/// Total de registros de stock para paginación
pub async fn contar_stock(pool: &PgPool, company_id: i32) -> Result<i64, CoreError> {
    let row: (i64,) = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT COUNT(DISTINCT sq.product_id)
        FROM stock_quant sq
        JOIN stock_location sl ON sl.id = sq.location_id
        WHERE sq.company_id = $1
          AND sl.usage = 'internal'
        "#,
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;

    Ok(row.0)
}

/// Obtiene KPIs del inventario para una empresa
pub async fn kpis(pool: &PgPool, company_id: i32) -> Result<KpisInventario, CoreError> {
    // Productos con stock positivo
    let con_stock: (i64,) = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT COUNT(DISTINCT sq.product_id)
        FROM stock_quant sq
        JOIN stock_location sl ON sl.id = sq.location_id
        WHERE sq.company_id = $1
          AND sl.usage = 'internal'
          AND sq.quantity > 0
        "#,
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;

    // Productos sin stock (quantity <= 0)
    let sin_stock: (i64,) = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT COUNT(DISTINCT sq.product_id)
        FROM stock_quant sq
        JOIN stock_location sl ON sl.id = sq.location_id
        WHERE sq.company_id = $1
          AND sl.usage = 'internal'
          AND sq.quantity <= 0
        "#,
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;

    // Valor del inventario (qty * standard_price)
    let valor: (Option<Decimal>,) = sqlx::query_as::<_, (Option<Decimal>,)>(
        r#"
        SELECT SUM(sq.quantity * pp_cost.standard_price)
        FROM stock_quant sq
        JOIN stock_location sl ON sl.id = sq.location_id
        JOIN product_product pp ON pp.id = sq.product_id
        JOIN product_product pp_cost ON pp_cost.id = sq.product_id
        WHERE sq.company_id = $1
          AND sl.usage = 'internal'
          AND sq.quantity > 0
        "#,
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;

    // Alertas: productos con quantity < 5 (umbral conservador sin tabla reorderpoint)
    let alertas: (i64,) = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT COUNT(DISTINCT sq.product_id)
        FROM stock_quant sq
        JOIN stock_location sl ON sl.id = sq.location_id
        WHERE sq.company_id = $1
          AND sl.usage = 'internal'
          AND sq.quantity > 0
          AND sq.quantity < 5
        "#,
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;

    Ok(KpisInventario {
        total_productos_con_stock: con_stock.0,
        total_sin_stock: sin_stock.0,
        valor_inventario: valor.0.unwrap_or(Decimal::ZERO),
        alertas_stock_bajo: alertas.0,
    })
}

/// Busca stock de un producto específico (todas las ubicaciones)
pub async fn stock_por_producto(
    pool: &PgPool,
    product_id: i32,
) -> Result<Vec<StockProducto>, CoreError> {
    let rows = sqlx::query_as::<_, StockProducto>(
        r#"
        SELECT
            sq.product_id,
            pt.name AS product_name,
            COALESCE(sq.quantity, 0)          AS cantidad_disponible,
            COALESCE(sq.reserved_quantity, 0) AS cantidad_reservada,
            uu.name                           AS unidad,
            sl.complete_name                  AS ubicacion
        FROM stock_quant sq
        JOIN product_product pp ON pp.id = sq.product_id
        JOIN product_template pt ON pt.id = pp.product_tmpl_id
        LEFT JOIN uom_uom uu ON uu.id = pt.uom_id
        LEFT JOIN stock_location sl ON sl.id = sq.location_id
        WHERE sq.product_id = $1
          AND sl.usage = 'internal'
        ORDER BY sl.complete_name ASC
        "#,
    )
    .bind(product_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Productos con stock bajo (quantity entre 0 y 5)
pub async fn productos_stock_bajo(
    pool: &PgPool,
    company_id: i32,
    limite: i32,
) -> Result<Vec<StockProducto>, CoreError> {
    let rows = sqlx::query_as::<_, StockProducto>(
        r#"
        SELECT
            sq.product_id,
            pt.name AS product_name,
            COALESCE(SUM(sq.quantity), 0)          AS cantidad_disponible,
            COALESCE(SUM(sq.reserved_quantity), 0) AS cantidad_reservada,
            uu.name                                AS unidad,
            sl.complete_name                       AS ubicacion
        FROM stock_quant sq
        JOIN product_product pp ON pp.id = sq.product_id
        JOIN product_template pt ON pt.id = pp.product_tmpl_id
        LEFT JOIN uom_uom uu ON uu.id = pt.uom_id
        LEFT JOIN stock_location sl ON sl.id = sq.location_id
        WHERE sq.company_id = $1
          AND sl.usage = 'internal'
        GROUP BY sq.product_id, pt.name, uu.name, sl.complete_name
        HAVING COALESCE(SUM(sq.quantity), 0) > 0
           AND COALESCE(SUM(sq.quantity), 0) < 5
        ORDER BY cantidad_disponible ASC
        LIMIT $2
        "#,
    )
    .bind(company_id)
    .bind(limite)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
