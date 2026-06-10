//! CRUD para res_users — Usuarios del sistema
//!
//! Incluye la lógica de autenticación: buscar por login,
//! verificar contraseña Argon2id, retornar datos para el JWT.

use sqlx::PgPool;
use crate::models::ResUsers;
use crate::auth::{hashear_password, verificar_password, DatosUsuario};
use crate::error::CoreError;

/// Busca un usuario por su email/login
pub async fn obtener_por_login(pool: &PgPool, login: &str) -> Result<ResUsers, CoreError> {
    let user = sqlx::query_as::<_, ResUsers>(
        r#"
        SELECT id, company_id, partner_id, create_uid, write_uid,
               login, password, active, share,
               create_date, write_date,
               groups_id, action_id, totp_secret
        FROM res_users
        WHERE login = $1 AND active = true
        LIMIT 1
        "#,
    )
    .bind(login)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| CoreError::Auth("Credenciales incorrectas".into()))?;

    Ok(user)
}

/// Busca un usuario por ID
pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<ResUsers, CoreError> {
    let user = sqlx::query_as::<_, ResUsers>(
        r#"
        SELECT id, company_id, partner_id, create_uid, write_uid,
               login, password, active, share,
               create_date, write_date,
               groups_id, action_id, totp_secret
        FROM res_users
        WHERE id = $1
        LIMIT 1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| CoreError::not_found("Usuario", id))?;

    Ok(user)
}

/// Intenta autenticar un usuario — devuelve DatosUsuario para generar JWT
///
/// Verifica:
/// 1. El usuario existe y está activo
/// 2. La contraseña (Argon2id)
pub async fn autenticar(
    pool: &PgPool,
    login: &str,
    password: &str,
) -> Result<DatosUsuario, CoreError> {
    let user = obtener_por_login(pool, login).await?;

    let hash = user.password.as_deref().unwrap_or("");
    if hash.is_empty() {
        return Err(CoreError::Auth("Credenciales incorrectas".into()));
    }

    let valido = verificar_password(password, hash)?;
    if !valido {
        return Err(CoreError::Auth("Credenciales incorrectas".into()));
    }

    // Obtener email del partner del usuario
    let email: String = sqlx::query_scalar(
        "SELECT COALESCE(email, login) FROM res_partner WHERE id = (SELECT partner_id FROM res_users WHERE id = $1)"
    )
    .bind(user.id)
    .fetch_optional(pool)
    .await?
    .unwrap_or_else(|| user.login.clone());

    Ok(DatosUsuario {
        user_id: user.id,
        company_id: user.company_id.unwrap_or(1),
        email,
        roles: vec!["nexus.user".into()],  // TODO: cargar grupos reales de res_groups
    })
}

/// Actualiza el hash de contraseña de un usuario
pub async fn cambiar_password(
    pool: &PgPool,
    user_id: i32,
    nueva_password: &str,
) -> Result<(), CoreError> {
    if nueva_password.len() < 8 {
        return Err(CoreError::Validation("La contraseña debe tener al menos 8 caracteres".into()));
    }

    let hash = hashear_password(nueva_password)?;

    sqlx::query(
        "UPDATE res_users SET password = $2, write_date = NOW() WHERE id = $1"
    )
    .bind(user_id)
    .bind(&hash)
    .execute(pool)
    .await?;

    Ok(())
}

/// Lista usuarios activos de una empresa
pub async fn listar_activos(pool: &PgPool, company_id: i32) -> Result<Vec<(i32, String, bool)>, CoreError> {
    let rows: Vec<(i32, String, bool)> = sqlx::query_as(
        r#"
        SELECT u.id, u.login, COALESCE(u.share, false)
        FROM res_users u
        WHERE u.company_id = $1 AND u.active = true
        ORDER BY u.login ASC
        "#,
    )
    .bind(company_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
