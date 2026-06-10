//! CRUD para res_partner — Contactos, Clientes y Proveedores
//!
//! Operaciones optimizadas para la tabla res_partner.
//! Usa query_as dinámico (no macros) para compilar sin DB disponible.
//! El cargo sqlx prepare genera el cache offline cuando la DB está disponible.

use sqlx::PgPool;
use crate::models::{ResPartner, ResPartnerSummary};
use crate::error::CoreError;

// ─── Listado y búsqueda ──────────────────────────────────────────────────────

/// Lista contactos activos (paginado)
pub async fn listar(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<ResPartnerSummary>, CoreError> {
    let offset = (pagina - 1) * por_pagina;

    let registros = sqlx::query_as::<_, ResPartnerSummary>(
        r#"
        SELECT
            id, name, email, phone, vat,
            is_company, customer_rank, supplier_rank, active, city, country_id
        FROM res_partner
        WHERE company_id = $1
          AND active = true
          AND (parent_id IS NULL OR is_company = true)
        ORDER BY name ASC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(company_id)
    .bind(por_pagina)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(registros)
}

/// Total de contactos activos para paginación
pub async fn contar(pool: &PgPool, company_id: i32) -> Result<i64, CoreError> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM res_partner WHERE company_id = $1 AND active = true",
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;

    Ok(row.0)
}

/// Busca contactos por nombre, email o RFC
pub async fn buscar(
    pool: &PgPool,
    company_id: i32,
    termino: &str,
    limite: i64,
) -> Result<Vec<ResPartnerSummary>, CoreError> {
    let patron = format!("%{}%", termino.to_lowercase());

    let registros = sqlx::query_as::<_, ResPartnerSummary>(
        r#"
        SELECT
            id, name, email, phone, vat,
            is_company, customer_rank, supplier_rank, active, city, country_id
        FROM res_partner
        WHERE company_id = $1
          AND active = true
          AND (
              LOWER(name) LIKE $2
              OR LOWER(COALESCE(email, '')) LIKE $2
              OR LOWER(COALESCE(vat, '')) LIKE $2
              OR LOWER(COALESCE(phone, '')) LIKE $2
          )
        ORDER BY customer_rank DESC, name ASC
        LIMIT $3
        "#,
    )
    .bind(company_id)
    .bind(&patron)
    .bind(limite)
    .fetch_all(pool)
    .await?;

    Ok(registros)
}

// ─── Lectura individual ──────────────────────────────────────────────────────

/// Obtiene un resumen de contacto por ID (sin cargar todos los campos JSONB)
pub async fn obtener_resumen(pool: &PgPool, id: i32) -> Result<ResPartnerSummary, CoreError> {
    let partner = sqlx::query_as::<_, ResPartnerSummary>(
        r#"
        SELECT id, name, email, phone, vat,
               is_company, customer_rank, supplier_rank, active, city, country_id
        FROM res_partner
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| CoreError::not_found("Contacto", id))?;

    Ok(partner)
}

/// Lista solo clientes activos (customer_rank > 0)
pub async fn listar_clientes(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<ResPartnerSummary>, CoreError> {
    let offset = (pagina - 1) * por_pagina;

    let registros = sqlx::query_as::<_, ResPartnerSummary>(
        r#"
        SELECT id, name, email, phone, vat,
               is_company, customer_rank, supplier_rank, active, city, country_id
        FROM res_partner
        WHERE company_id = $1 AND active = true AND customer_rank > 0
        ORDER BY customer_rank DESC, name ASC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(company_id)
    .bind(por_pagina)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(registros)
}

/// Lista solo proveedores activos (supplier_rank > 0)
pub async fn listar_proveedores(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<ResPartnerSummary>, CoreError> {
    let offset = (pagina - 1) * por_pagina;

    let registros = sqlx::query_as::<_, ResPartnerSummary>(
        r#"
        SELECT id, name, email, phone, vat,
               is_company, customer_rank, supplier_rank, active, city, country_id
        FROM res_partner
        WHERE company_id = $1 AND active = true AND supplier_rank > 0
        ORDER BY supplier_rank DESC, name ASC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(company_id)
    .bind(por_pagina)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(registros)
}

/// Busca un contacto por RFC (campo vat)
pub async fn obtener_por_rfc(
    pool: &PgPool,
    company_id: i32,
    rfc: &str,
) -> Result<Option<ResPartnerSummary>, CoreError> {
    let partner = sqlx::query_as::<_, ResPartnerSummary>(
        r#"
        SELECT id, name, email, phone, vat,
               is_company, customer_rank, supplier_rank, active, city, country_id
        FROM res_partner
        WHERE company_id = $1 AND active = true AND UPPER(vat) = UPPER($2)
        LIMIT 1
        "#,
    )
    .bind(company_id)
    .bind(rfc)
    .fetch_optional(pool)
    .await?;

    Ok(partner)
}

// ─── Escritura ───────────────────────────────────────────────────────────────

/// Datos para crear un nuevo contacto
#[derive(Debug, Clone)]
pub struct NuevoPartner {
    pub company_id: i32,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub vat: Option<String>,
    pub is_company: bool,
    pub street: Option<String>,
    pub city: Option<String>,
    pub zip: Option<String>,
    pub country_id: Option<i32>,
    pub customer_rank: i32,
    pub supplier_rank: i32,
    pub create_uid: i32,
}

/// Crea un nuevo contacto y devuelve su ID
pub async fn crear(pool: &PgPool, datos: &NuevoPartner) -> Result<i32, CoreError> {
    let row: (i32,) = sqlx::query_as(
        r#"
        INSERT INTO res_partner (
            company_id, name, email, phone, vat,
            is_company, street, city, zip, country_id,
            customer_rank, supplier_rank,
            active, autopost_bills,
            create_uid, write_uid,
            create_date, write_date
        ) VALUES (
            $1, $2, $3, $4, $5,
            $6, $7, $8, $9, $10,
            $11, $12,
            true, 'never',
            $13, $13,
            NOW(), NOW()
        )
        RETURNING id
        "#,
    )
    .bind(datos.company_id)
    .bind(&datos.name)
    .bind(&datos.email)
    .bind(&datos.phone)
    .bind(&datos.vat)
    .bind(datos.is_company)
    .bind(&datos.street)
    .bind(&datos.city)
    .bind(&datos.zip)
    .bind(datos.country_id)
    .bind(datos.customer_rank)
    .bind(datos.supplier_rank)
    .bind(datos.create_uid)
    .fetch_one(pool)
    .await?;

    Ok(row.0)
}

/// Actualiza campos básicos de un contacto
pub async fn actualizar_contacto(
    pool: &PgPool,
    id: i32,
    nombre: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    write_uid: i32,
) -> Result<(), CoreError> {
    sqlx::query(
        r#"
        UPDATE res_partner
        SET
            name       = COALESCE($2, name),
            email      = COALESCE($3, email),
            phone      = COALESCE($4, phone),
            write_uid  = $5,
            write_date = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(nombre)
    .bind(email)
    .bind(phone)
    .bind(write_uid)
    .execute(pool)
    .await?;

    Ok(())
}

/// Desactiva un contacto (soft delete — active = false)
pub async fn desactivar(pool: &PgPool, id: i32, write_uid: i32) -> Result<(), CoreError> {
    let rows = sqlx::query(
        "UPDATE res_partner SET active = false, write_uid = $2, write_date = NOW() WHERE id = $1",
    )
    .bind(id)
    .bind(write_uid)
    .execute(pool)
    .await?
    .rows_affected();

    if rows == 0 {
        return Err(CoreError::not_found("Contacto", id));
    }

    Ok(())
}
