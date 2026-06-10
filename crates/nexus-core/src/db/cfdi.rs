//! CRUD para cfdi_timbrados — Comprobantes Fiscales Digitales timbrados

use sqlx::PgPool;
use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;
use crate::error::CoreError;

// ─── Structs ─────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CfdiTimbrado {
    pub id: i32,
    pub uuid: String,
    pub folio: Option<String>,
    pub serie: Option<String>,
    pub fecha_emision: Option<String>,
    pub rfc_emisor: String,
    pub rfc_receptor: String,
    pub nombre_emisor: Option<String>,
    pub nombre_receptor: Option<String>,
    pub total: Option<Decimal>,
    pub tipo_cfdi: Option<String>,
    pub estado: Option<String>,
    pub fecha_timbrado: Option<String>,
    pub account_move_id: Option<i32>,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NuevoCfdi {
    pub uuid: String,
    pub folio: Option<String>,
    pub serie: Option<String>,
    pub fecha_emision: Option<String>,
    pub rfc_emisor: String,
    pub rfc_receptor: String,
    pub nombre_emisor: Option<String>,
    pub nombre_receptor: Option<String>,
    pub total: Option<Decimal>,
    pub tipo_cfdi: Option<String>,
    pub xml_timbrado: Option<String>,
    pub fecha_timbrado: Option<String>,
    pub account_move_id: Option<i32>,
}

const SELECT_COLS: &str = r#"
    id, uuid, folio, serie, fecha_emision,
    rfc_emisor, rfc_receptor, nombre_emisor, nombre_receptor,
    total, tipo_cfdi, estado, fecha_timbrado, account_move_id,
    created_at::text AS created_at
"#;

// ─── Funciones públicas ───────────────────────────────────────────────────────

/// Inserta un CFDI timbrado y devuelve el registro creado
pub async fn insertar(pool: &PgPool, cfdi: &NuevoCfdi) -> Result<CfdiTimbrado, CoreError> {
    let q = format!(
        r#"INSERT INTO cfdi_timbrados
               (uuid, folio, serie, fecha_emision,
                rfc_emisor, rfc_receptor, nombre_emisor, nombre_receptor,
                total, tipo_cfdi, xml_timbrado, fecha_timbrado, account_move_id)
           VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)
           ON CONFLICT (uuid) DO UPDATE
               SET fecha_timbrado = EXCLUDED.fecha_timbrado,
                   updated_at = NOW()
           RETURNING {SELECT_COLS}"#
    );
    let row = sqlx::query_as::<_, CfdiTimbrado>(&q)
        .bind(&cfdi.uuid)
        .bind(&cfdi.folio)
        .bind(&cfdi.serie)
        .bind(&cfdi.fecha_emision)
        .bind(&cfdi.rfc_emisor)
        .bind(&cfdi.rfc_receptor)
        .bind(&cfdi.nombre_emisor)
        .bind(&cfdi.nombre_receptor)
        .bind(&cfdi.total)
        .bind(&cfdi.tipo_cfdi)
        .bind(&cfdi.xml_timbrado)
        .bind(&cfdi.fecha_timbrado)
        .bind(&cfdi.account_move_id)
        .fetch_one(pool)
        .await?;
    Ok(row)
}

/// Lista CFDIs de un emisor (paginado)
pub async fn listar(
    pool: &PgPool,
    rfc_emisor: &str,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<CfdiTimbrado>, CoreError> {
    let offset = (pagina - 1).max(0) * por_pagina;
    let q = format!(
        "SELECT {SELECT_COLS} FROM cfdi_timbrados
         WHERE rfc_emisor = $1
         ORDER BY created_at DESC NULLS LAST
         LIMIT $2 OFFSET $3"
    );
    let rows = sqlx::query_as::<_, CfdiTimbrado>(&q)
        .bind(rfc_emisor)
        .bind(por_pagina)
        .bind(offset)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

/// Lista todos los CFDIs sin filtro de emisor (paginado)
pub async fn listar_todos(
    pool: &PgPool,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<CfdiTimbrado>, CoreError> {
    let offset = (pagina - 1).max(0) * por_pagina;
    let q = format!(
        "SELECT {SELECT_COLS} FROM cfdi_timbrados
         ORDER BY created_at DESC NULLS LAST
         LIMIT $1 OFFSET $2"
    );
    let rows = sqlx::query_as::<_, CfdiTimbrado>(&q)
        .bind(por_pagina)
        .bind(offset)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

/// Total de CFDIs de un emisor
pub async fn contar(pool: &PgPool, rfc_emisor: &str) -> Result<i64, CoreError> {
    let n: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM cfdi_timbrados WHERE rfc_emisor = $1",
    )
    .bind(rfc_emisor)
    .fetch_one(pool)
    .await?;
    Ok(n.0)
}

/// Total de todos los CFDIs
pub async fn contar_todos(pool: &PgPool) -> Result<i64, CoreError> {
    let n: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM cfdi_timbrados",
    )
    .fetch_one(pool)
    .await?;
    Ok(n.0)
}

/// Obtiene un CFDI por UUID
pub async fn por_uuid(pool: &PgPool, uuid: &str) -> Result<Option<CfdiTimbrado>, CoreError> {
    let q = format!(
        "SELECT {SELECT_COLS} FROM cfdi_timbrados WHERE uuid = $1"
    );
    let row = sqlx::query_as::<_, CfdiTimbrado>(&q)
        .bind(uuid)
        .fetch_optional(pool)
        .await?;
    Ok(row)
}

/// Cancela un CFDI actualizando su estado
pub async fn cancelar(pool: &PgPool, uuid: &str) -> Result<(), CoreError> {
    sqlx::query_as::<_, (i32,)>(
        "UPDATE cfdi_timbrados SET estado = 'cancelado', updated_at = NOW()
         WHERE uuid = $1 RETURNING id",
    )
    .bind(uuid)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| CoreError::NotFound(format!("CFDI con uuid={} no encontrado", uuid)))?;
    Ok(())
}

/// KPIs de CFDI para un emisor
pub async fn kpis(pool: &PgPool, rfc_emisor: &str) -> Result<serde_json::Value, CoreError> {
    let row: (i64, i64, i64, Option<Decimal>) = sqlx::query_as(
        r#"SELECT
            COUNT(*)                                          AS total_timbrados,
            COUNT(*) FILTER (WHERE estado = 'vigente')       AS vigentes,
            COUNT(*) FILTER (WHERE estado = 'cancelado')     AS cancelados,
            SUM(total) FILTER (WHERE estado = 'vigente')     AS monto_total
           FROM cfdi_timbrados
           WHERE rfc_emisor = $1"#,
    )
    .bind(rfc_emisor)
    .fetch_one(pool)
    .await?;

    Ok(serde_json::json!({
        "total_timbrados": row.0,
        "vigentes":        row.1,
        "cancelados":      row.2,
        "monto_total":     row.3.unwrap_or(Decimal::ZERO),
    }))
}

/// KPIs globales (sin filtro de RFC)
pub async fn kpis_globales(pool: &PgPool) -> Result<serde_json::Value, CoreError> {
    let row: (i64, i64, i64, Option<Decimal>) = sqlx::query_as(
        r#"SELECT
            COUNT(*)                                          AS total_timbrados,
            COUNT(*) FILTER (WHERE estado = 'vigente')       AS vigentes,
            COUNT(*) FILTER (WHERE estado = 'cancelado')     AS cancelados,
            SUM(total) FILTER (WHERE estado = 'vigente')     AS monto_total
           FROM cfdi_timbrados"#,
    )
    .fetch_one(pool)
    .await?;

    Ok(serde_json::json!({
        "total_timbrados": row.0,
        "vigentes":        row.1,
        "cancelados":      row.2,
        "monto_total":     row.3.unwrap_or(Decimal::ZERO),
    }))
}
