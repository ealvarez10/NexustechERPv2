//! Movimientos de stock (entradas, salidas, transferencias)

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error::InventoryError;

/// Movimiento de inventario mapeado desde stock_move.
/// Columnas verificadas en el schema real de la base de datos.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct MovimientoStock {
    pub id: i32,
    pub company_id: i32,
    pub product_id: i32,
    pub product_uom: i32,
    pub location_id: i32,
    pub location_dest_id: i32,
    pub picking_id: Option<i32>,
    pub picking_type_id: Option<i32>,
    pub warehouse_id: Option<i32>,
    pub state: Option<String>,
    pub origin: Option<String>,
    pub reference: Option<String>,
    pub product_uom_qty: Decimal,
    pub quantity: Option<Decimal>,
    pub product_qty: Option<Decimal>,
    pub is_inventory: Option<bool>,
}

/// Tipo semántico de movimiento derivado del contexto
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TipoMovimiento {
    Entrada,  // recepción de compra
    Salida,   // entrega de venta
    Interno,  // transferencia entre ubicaciones
    Ajuste,   // ajuste de inventario
}

/// KPIs globales de movimientos
#[derive(Debug, Serialize)]
pub struct KpisMovimientos {
    pub total_movimientos: i64,
    pub pendientes: i64,
    pub completados: i64,
    pub en_proceso: i64,
}

/// Fila auxiliar para contar estados
#[derive(sqlx::FromRow)]
struct ConteoEstado {
    estado: Option<String>,
    total: Option<i64>,
}

// ────────────────────────────────────────────────────────────────────────────
// Funciones públicas
// ────────────────────────────────────────────────────────────────────────────

/// Lista movimientos recientes paginados para una empresa
pub async fn listar_movimientos(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<MovimientoStock>, InventoryError> {
    let offset = pagina.saturating_sub(1) * por_pagina;
    let movimientos = sqlx::query_as::<_, MovimientoStock>(
        r#"
        SELECT
            id,
            company_id,
            product_id,
            product_uom,
            location_id,
            location_dest_id,
            picking_id,
            picking_type_id,
            warehouse_id,
            state,
            origin,
            reference,
            product_uom_qty,
            quantity,
            product_qty,
            is_inventory
        FROM stock_move
        WHERE company_id = $1
        ORDER BY id DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(company_id)
    .bind(por_pagina)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    tracing::debug!(
        company_id,
        pagina,
        por_pagina,
        count = movimientos.len(),
        "movimientos listados"
    );

    Ok(movimientos)
}

/// Calcula KPIs de movimientos para una empresa
pub async fn kpis(pool: &PgPool, company_id: i32) -> Result<KpisMovimientos, InventoryError> {
    let conteos = sqlx::query_as::<_, ConteoEstado>(
        r#"
        SELECT state AS estado, COUNT(*) AS total
        FROM stock_move
        WHERE company_id = $1
        GROUP BY state
        "#,
    )
    .bind(company_id)
    .fetch_all(pool)
    .await?;

    let mut total_movimientos: i64 = 0;
    let mut pendientes: i64 = 0;
    let mut completados: i64 = 0;
    let mut en_proceso: i64 = 0;

    for fila in &conteos {
        let n = fila.total.unwrap_or(0);
        total_movimientos += n;
        match fila.estado.as_deref() {
            Some("draft") | Some("waiting") | Some("confirmed") => pendientes += n,
            Some("done") => completados += n,
            Some("assigned") | Some("partially_available") => en_proceso += n,
            _ => {}
        }
    }

    Ok(KpisMovimientos {
        total_movimientos,
        pendientes,
        completados,
        en_proceso,
    })
}

/// Historial de movimientos de un producto (todas las empresas)
pub async fn historial_producto(
    pool: &PgPool,
    product_id: i32,
) -> Result<Vec<MovimientoStock>, InventoryError> {
    let movimientos = sqlx::query_as::<_, MovimientoStock>(
        r#"
        SELECT
            id,
            company_id,
            product_id,
            product_uom,
            location_id,
            location_dest_id,
            picking_id,
            picking_type_id,
            warehouse_id,
            state,
            origin,
            reference,
            product_uom_qty,
            quantity,
            product_qty,
            is_inventory
        FROM stock_move
        WHERE product_id = $1
        ORDER BY id DESC
        "#,
    )
    .bind(product_id)
    .fetch_all(pool)
    .await?;

    tracing::debug!(
        product_id,
        count = movimientos.len(),
        "historial de producto obtenido"
    );

    Ok(movimientos)
}
