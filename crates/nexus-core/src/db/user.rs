//! CRUD para res_users — Usuarios del sistema
//!
//! Autenticación compatible con el formato de contraseñas del schema NexusTech ERP.
//! Soporta: PBKDF2-SHA512 (formato legacy) y Argon2id (nuevos usuarios).

use sqlx::PgPool;
use crate::models::ResUsers;
use crate::auth::{hashear_password, DatosUsuario};
use crate::error::CoreError;

/// Busca un usuario activo por login
pub async fn obtener_por_login(pool: &PgPool, login: &str) -> Result<ResUsers, CoreError> {
    let user = sqlx::query_as::<_, ResUsers>(
        r#"
        SELECT id, company_id, partner_id, create_uid, write_uid,
               login, password, active, share,
               create_date, write_date,
               action_id, totp_secret, notification_type
        FROM res_users
        WHERE "login" = $1 AND active = true
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
               action_id, totp_secret, notification_type
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

/// Autentica usuario — busca por login y verifica contraseña
pub async fn autenticar(
    pool: &PgPool,
    login_input: &str,
    password: &str,
) -> Result<DatosUsuario, CoreError> {
    // Query simple: buscar usuario por login
    // Evitamos alias de tabla para compatibilidad con prepared statements
    let row: Option<(i32, i32, i32, Option<String>)> = sqlx::query_as(
        "SELECT id, company_id, partner_id, password FROM res_users WHERE active = true AND login = $1 LIMIT 1"
    )
    .bind(login_input)
    .fetch_optional(pool)
    .await?;

    let (user_id, company_id, partner_id, hash_opt) = row
        .ok_or_else(|| CoreError::Auth("Credenciales incorrectas".into()))?;

    let hash = hash_opt.as_deref().unwrap_or("");
    if hash.is_empty() {
        return Err(CoreError::Auth("Credenciales incorrectas".into()));
    }

    let valido = verificar_password_compatible(password, hash)?;
    if !valido {
        return Err(CoreError::Auth("Credenciales incorrectas".into()));
    }

    // Obtener email del partner asociado
    let email: Option<String> = sqlx::query_scalar(
        "SELECT email FROM res_partner WHERE id = $1"
    )
    .bind(partner_id)
    .fetch_optional(pool)
    .await?
    .flatten();

    Ok(DatosUsuario {
        user_id,
        company_id,
        email: email.unwrap_or_else(|| login_input.to_string()),
        roles: vec!["nexus.user".into()],
    })
}

/// Verifica contraseña soportando PBKDF2 (legacy) y Argon2id
fn verificar_password_compatible(password: &str, hash: &str) -> Result<bool, CoreError> {
    // Argon2id: $argon2id$...
    if hash.starts_with("$argon2") {
        use argon2::{Argon2, password_hash::{PasswordHash, PasswordVerifier}};
        let parsed = PasswordHash::new(hash)
            .map_err(|_| CoreError::Auth("Hash de contraseña inválido".into()))?;
        return Ok(Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok());
    }

    // PBKDF2-SHA512: $pbkdf2-sha512$iteraciones$salt$hash
    if hash.starts_with("$pbkdf2-sha512$") {
        return verificar_pbkdf2_sha512(password, hash);
    }

    // PBKDF2-SHA256: $pbkdf2-sha256$...
    if hash.starts_with("$pbkdf2-sha256$") {
        return verificar_pbkdf2_sha256(password, hash);
    }

    // Hash no reconocido
    Err(CoreError::Auth("Formato de contraseña no soportado".into()))
}

/// Verifica PBKDF2-SHA512 en formato Passlib $pbkdf2-sha512$iters$salt$hash
fn verificar_pbkdf2_sha512(password: &str, hash: &str) -> Result<bool, CoreError> {
    use ring::pbkdf2;

    // Formato: $pbkdf2-sha512$600000$<salt_ab64>$<hash_ab64>
    let partes: Vec<&str> = hash.split('$').collect();
    if partes.len() < 5 {
        return Err(CoreError::Auth("Formato PBKDF2 inválido".into()));
    }

    let iteraciones: u32 = partes[2].parse()
        .map_err(|_| CoreError::Auth("Iteraciones PBKDF2 inválidas".into()))?;
    let salt  = base64_passlib_decode(partes[3])?;
    let hash_bytes = base64_passlib_decode(partes[4])?;

    let iters = std::num::NonZeroU32::new(iteraciones)
        .ok_or_else(|| CoreError::Auth("Iteraciones PBKDF2 inválidas".into()))?;

    // ring::pbkdf2::verify verifica derivando el mismo número de bytes que hash_bytes.len()
    let resultado = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        iters,
        &salt,
        password.as_bytes(),
        &hash_bytes,
    );
    Ok(resultado.is_ok())
}

/// Verifica PBKDF2-SHA256 en formato Passlib
fn verificar_pbkdf2_sha256(password: &str, hash: &str) -> Result<bool, CoreError> {
    use ring::pbkdf2;

    let partes: Vec<&str> = hash.split('$').collect();
    if partes.len() < 5 {
        return Err(CoreError::Auth("Formato PBKDF2 inválido".into()));
    }

    let iteraciones: u32 = partes[2].parse()
        .map_err(|_| CoreError::Auth("Iteraciones PBKDF2 inválidas".into()))?;
    let salt_b64 = partes[3];
    let hash_b64 = partes[4];

    let salt = base64_passlib_decode(salt_b64)?;
    let hash_bytes = base64_passlib_decode(hash_b64)?;

    let alg = pbkdf2::PBKDF2_HMAC_SHA256;
    let iters = std::num::NonZeroU32::new(iteraciones)
        .ok_or_else(|| CoreError::Auth("Iteraciones PBKDF2 inválidas".into()))?;

    let resultado = pbkdf2::verify(alg, iters, &salt, password.as_bytes(), &hash_bytes);
    Ok(resultado.is_ok())
}

/// Decodifica base64 en formato Passlib (usa '.' en vez de '+', sin padding)
fn base64_passlib_decode(s: &str) -> Result<Vec<u8>, CoreError> {
    use base64::Engine;
    // Passlib usa alphabet modificado: '.' en lugar de '+', '/' normal, sin padding
    let corregido = s.replace('.', "+");
    // Añadir padding si falta
    let padding = match corregido.len() % 4 {
        2 => "==",
        3 => "=",
        _ => "",
    };
    let con_padding = format!("{}{}", corregido, padding);
    base64::engine::general_purpose::STANDARD
        .decode(&con_padding)
        .map_err(|e| CoreError::Auth(format!("Error decodificando base64 PBKDF2: {}", e)))
}

/// Cambia la contraseña de un usuario (guarda como Argon2id)
pub async fn cambiar_password(
    pool: &PgPool,
    user_id: i32,
    nueva_password: &str,
) -> Result<(), CoreError> {
    if nueva_password.len() < 8 {
        return Err(CoreError::Validation(
            "La contraseña debe tener al menos 8 caracteres".into()
        ));
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
pub async fn listar_activos(
    pool: &PgPool,
    company_id: i32,
) -> Result<Vec<(i32, String, bool)>, CoreError> {
    let rows: Vec<(i32, String, bool)> = sqlx::query_as(
        r#"
        SELECT id, login, COALESCE(share, false)
        FROM res_users
        WHERE company_id = $1 AND active = true
        ORDER BY login ASC
        "#,
    )
    .bind(company_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    const HASH_ADMIN: &str = "$pbkdf2-sha512$600000$917L2XsPgdBaSwlh7N0b4w$SBFV9mksmJsRkWK9QOKtHksmPEACVwxJWQg.KTXyAh6EKAeF1cbi3Zois8mXS1zRo9JNvGUXxihAqyu3RuJvlg";

    #[test]
    fn test_pbkdf2_ab64_decode() {
        let salt_raw = "917L2XsPgdBaSwlh7N0b4w";
        let decoded = base64_passlib_decode(salt_raw).unwrap();
        // Python: f75ecbd97b0f81d05a4b0961ecdd1be3
        assert_eq!(decoded[0], 0xf7, "Primer byte del salt debe ser 0xf7");
        println!("Salt OK: {:?}", &decoded[..4]);
    }

    #[test]
    fn test_pbkdf2_sha512_admin() {
        let resultado = verificar_pbkdf2_sha512("admin", HASH_ADMIN).unwrap();
        assert!(resultado, "La contraseña 'admin' debe verificarse correctamente");
    }

    #[test]
    fn test_pbkdf2_sha512_wrong_pass() {
        let resultado = verificar_pbkdf2_sha512("wrong", HASH_ADMIN).unwrap();
        assert!(!resultado, "Contraseña incorrecta debe retornar false");
    }

    #[test]
    fn test_verificar_password_compatible_pbkdf2() {
        let resultado = verificar_password_compatible("admin", HASH_ADMIN).unwrap();
        assert!(resultado, "Compatibilidad PBKDF2 debe funcionar");
    }
}
