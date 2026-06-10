//! CRUD para res_company — Empresas del sistema

use sqlx::PgPool;
use crate::models::ResCompany;
use crate::error::CoreError;

/// Obtiene una empresa por ID
pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<ResCompany, CoreError> {
    let company = sqlx::query_as::<_, ResCompany>(
        r#"
        SELECT id, parent_id, partner_id, currency_id, country_id, state_id,
               create_uid, write_uid, name, email, phone, website, vat,
               street, street2, zip, city, active, create_date, write_date,
               l10n_mx_edi_pac, l10n_mx_edi_pac_test_env, l10n_mx_edi_certificate_ids
        FROM res_company
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| CoreError::not_found("Empresa", id))?;

    Ok(company)
}

/// Lista todas las empresas activas
pub async fn listar_activas(pool: &PgPool) -> Result<Vec<ResCompany>, CoreError> {
    let companies = sqlx::query_as::<_, ResCompany>(
        r#"
        SELECT id, parent_id, partner_id, currency_id, country_id, state_id,
               create_uid, write_uid, name, email, phone, website, vat,
               street, street2, zip, city, active, create_date, write_date,
               l10n_mx_edi_pac, l10n_mx_edi_pac_test_env, l10n_mx_edi_certificate_ids
        FROM res_company
        WHERE active = true
        ORDER BY id ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(companies)
}
