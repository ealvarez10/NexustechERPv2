//! Hash y verificación de contraseñas con Argon2id
//!
//! Argon2id es el estándar recomendado por OWASP para hashing de contraseñas.
//! Compatible con los hashes existentes en la base de datos.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::error::CoreError;

/// Genera un hash Argon2id de la contraseña
pub fn hashear_password(password: &str) -> Result<String, CoreError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| CoreError::Internal(format!("Error al hashear contraseña: {}", e)))
}

/// Verifica si una contraseña coincide con su hash
pub fn verificar_password(password: &str, hash: &str) -> Result<bool, CoreError> {
    let parsed = PasswordHash::new(hash)
        .map_err(|e| CoreError::Auth(format!("Hash de contraseña inválido: {}", e)))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_y_verificar() {
        let pass = "NexusTech2026!Segura";
        let hash = hashear_password(pass).unwrap();

        assert!(hash.starts_with("$argon2"), "Debe ser hash argon2");
        assert!(verificar_password(pass, &hash).unwrap(), "Debe verificar correctamente");
        assert!(!verificar_password("MalPassword", &hash).unwrap(), "Password incorrecto debe fallar");
    }

    #[test]
    fn test_hashes_distintos_misma_pass() {
        let pass = "mismapassword123";
        let h1 = hashear_password(pass).unwrap();
        let h2 = hashear_password(pass).unwrap();
        // Cada hash debe ser único (salt aleatoria)
        assert_ne!(h1, h2, "Cada hash debe ser único por el salt");
        assert!(verificar_password(pass, &h1).unwrap());
        assert!(verificar_password(pass, &h2).unwrap());
    }
}
