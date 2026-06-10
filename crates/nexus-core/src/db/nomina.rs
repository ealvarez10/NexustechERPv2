//! Nómina — Empleados y recibos de nómina

use sqlx::PgPool;
use serde::Serialize;
use crate::error::CoreError;

// ─── Structs ─────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Empleado {
    pub id: i32,
    pub name: String,
    pub job_title: Option<String>,
    pub department_id: Option<i32>,
    pub active: Option<bool>,
    pub company_id: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct KpisNomina {
    pub total_empleados: i64,
    pub activos: i64,
    pub inactivos: i64,
}

// ─── Funciones públicas ───────────────────────────────────────────────────────

/// Lista empleados de una empresa (paginado)
pub async fn listar_empleados(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<Empleado>, CoreError> {
    let offset = (pagina - 1).max(0) * por_pagina;
    let rows = sqlx::query_as::<_, Empleado>(
        r#"SELECT id, name, job_title, department_id, active, company_id
           FROM hr_employee
           WHERE company_id = $1
           ORDER BY name ASC
           LIMIT $2 OFFSET $3"#,
    )
    .bind(company_id)
    .bind(por_pagina)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Total de empleados de una empresa
pub async fn contar_empleados(pool: &PgPool, company_id: i32) -> Result<i64, CoreError> {
    let n: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM hr_employee WHERE company_id = $1",
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;
    Ok(n.0)
}

/// KPIs de nómina para una empresa
pub async fn kpis(pool: &PgPool, company_id: i32) -> Result<KpisNomina, CoreError> {
    let row: (i64, i64, i64) = sqlx::query_as(
        r#"SELECT
            COUNT(*)                                    AS total_empleados,
            COUNT(*) FILTER (WHERE active = true)       AS activos,
            COUNT(*) FILTER (WHERE active = false)      AS inactivos
           FROM hr_employee
           WHERE company_id = $1"#,
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;

    Ok(KpisNomina {
        total_empleados: row.0,
        activos: row.1,
        inactivos: row.2,
    })
}
