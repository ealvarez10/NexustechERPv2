//! JWT — Autenticación de NexusTech ERP v2
//!
//! Genera y valida tokens JWT para la API REST.
//! Access token: vida corta (1h por defecto)
//! Refresh token: vida larga (7d por defecto)
//!
//! Claims personalizados incluyen: user_id, company_id, roles, email

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use crate::error::CoreError;

/// Algoritmo de firma
const ALGORITMO: Algorithm = Algorithm::HS256;

/// Claims del access token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject — user_id como string
    pub sub: String,
    /// Issued at (Unix timestamp)
    pub iat: i64,
    /// Expiration (Unix timestamp)
    pub exp: i64,
    /// Tipo: "access" o "refresh"
    pub tipo: String,
    /// ID del usuario
    pub user_id: i32,
    /// ID de la empresa activa
    pub company_id: i32,
    /// Email del usuario
    pub email: String,
    /// Roles del usuario (grupos de permisos)
    pub roles: Vec<String>,
}

/// Par de tokens retornado al iniciar sesión
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    /// Segundos hasta expiración del access token
    pub expires_in: u64,
}

/// Datos necesarios para crear tokens
#[derive(Debug, Clone)]
pub struct DatosUsuario {
    pub user_id: i32,
    pub company_id: i32,
    pub email: String,
    pub roles: Vec<String>,
}

/// Genera un par de tokens (access + refresh) para un usuario autenticado
pub fn generar_tokens(
    datos: &DatosUsuario,
    jwt_secret: &str,
    access_expires_secs: u64,
    refresh_expires_secs: u64,
) -> Result<TokenPair, CoreError> {
    let ahora = Utc::now().timestamp();
    let key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let header = Header::new(ALGORITMO);

    // Access token
    let access_claims = Claims {
        sub: datos.user_id.to_string(),
        iat: ahora,
        exp: ahora + access_expires_secs as i64,
        tipo: "access".into(),
        user_id: datos.user_id,
        company_id: datos.company_id,
        email: datos.email.clone(),
        roles: datos.roles.clone(),
    };
    let access_token = encode(&header, &access_claims, &key)
        .map_err(|e| CoreError::Auth(format!("Error generando access token: {}", e)))?;

    // Refresh token (menos información, vida más larga)
    let refresh_claims = Claims {
        sub: datos.user_id.to_string(),
        iat: ahora,
        exp: ahora + refresh_expires_secs as i64,
        tipo: "refresh".into(),
        user_id: datos.user_id,
        company_id: datos.company_id,
        email: datos.email.clone(),
        roles: vec![],  // refresh tokens no llevan roles
    };
    let refresh_token = encode(&header, &refresh_claims, &key)
        .map_err(|e| CoreError::Auth(format!("Error generando refresh token: {}", e)))?;

    Ok(TokenPair {
        access_token,
        refresh_token,
        token_type: "Bearer".into(),
        expires_in: access_expires_secs,
    })
}

/// Valida y decodifica un access token
pub fn validar_access_token(token: &str, jwt_secret: &str) -> Result<Claims, CoreError> {
    let claims = decodificar(token, jwt_secret)?;
    if claims.tipo != "access" {
        return Err(CoreError::TokenInvalido);
    }
    Ok(claims)
}

/// Valida y decodifica un refresh token
pub fn validar_refresh_token(token: &str, jwt_secret: &str) -> Result<Claims, CoreError> {
    let claims = decodificar(token, jwt_secret)?;
    if claims.tipo != "refresh" {
        return Err(CoreError::TokenInvalido);
    }
    Ok(claims)
}

/// Renueva el access token desde un refresh token válido
pub fn renovar_access_token(
    refresh_token: &str,
    jwt_secret: &str,
    access_expires_secs: u64,
) -> Result<String, CoreError> {
    let claims = validar_refresh_token(refresh_token, jwt_secret)?;
    let ahora = Utc::now().timestamp();
    let key = EncodingKey::from_secret(jwt_secret.as_bytes());

    let new_claims = Claims {
        sub: claims.user_id.to_string(),
        iat: ahora,
        exp: ahora + access_expires_secs as i64,
        tipo: "access".into(),
        user_id: claims.user_id,
        company_id: claims.company_id,
        email: claims.email,
        roles: claims.roles,
    };

    encode(&Header::new(ALGORITMO), &new_claims, &key)
        .map_err(|e| CoreError::Auth(format!("Error renovando token: {}", e)))
}

/// Verifica si un usuario tiene un rol específico
pub fn tiene_rol(claims: &Claims, rol: &str) -> bool {
    claims.roles.iter().any(|r| r == rol)
}

/// Roles estándar de NexusTech ERP
pub mod roles {
    pub const ADMIN:      &str = "nexus.admin";
    pub const VENTAS:     &str = "nexus.ventas";
    pub const COMPRAS:    &str = "nexus.compras";
    pub const INVENTARIO: &str = "nexus.inventario";
    pub const CONTABLE:   &str = "nexus.contable";
    pub const POS:        &str = "nexus.pos";
    pub const CRM:        &str = "nexus.crm";
    pub const RRHH:       &str = "nexus.rrhh";
    pub const LECTURA:    &str = "nexus.lectura";
}

fn decodificar(token: &str, jwt_secret: &str) -> Result<Claims, CoreError> {
    let key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let mut validation = Validation::new(ALGORITMO);
    validation.validate_exp = true;
    validation.leeway = 0; // Sin tolerancia — tokens expirados son rechazados inmediatamente

    decode::<Claims>(token, &key, &validation)
        .map(|data| data.claims)
        .map_err(|e| {
            use jsonwebtoken::errors::ErrorKind;
            match e.kind() {
                ErrorKind::ExpiredSignature => CoreError::TokenInvalido,
                ErrorKind::InvalidSignature => CoreError::TokenInvalido,
                _ => CoreError::Auth(format!("Token inválido: {}", e)),
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &str = "nexustech_jwt_secret_test_32chars!";

    fn datos_test() -> DatosUsuario {
        DatosUsuario {
            user_id: 42,
            company_id: 1,
            email: "admin@nexustechnologies.com.mx".into(),
            roles: vec![roles::ADMIN.into(), roles::VENTAS.into()],
        }
    }

    #[test]
    fn test_generar_y_validar_tokens() {
        let datos = datos_test();
        let par = generar_tokens(&datos, SECRET, 3600, 604800).unwrap();

        assert!(!par.access_token.is_empty());
        assert!(!par.refresh_token.is_empty());
        assert_eq!(par.token_type, "Bearer");
        assert_eq!(par.expires_in, 3600);

        let claims = validar_access_token(&par.access_token, SECRET).unwrap();
        assert_eq!(claims.user_id, 42);
        assert_eq!(claims.company_id, 1);
        assert_eq!(claims.email, "admin@nexustechnologies.com.mx");
        assert_eq!(claims.tipo, "access");
        assert!(claims.roles.contains(&roles::ADMIN.to_string()));
    }

    #[test]
    fn test_refresh_token_es_tipo_refresh() {
        let datos = datos_test();
        let par = generar_tokens(&datos, SECRET, 3600, 604800).unwrap();

        // Refresh token NO debe validar como access
        assert!(validar_access_token(&par.refresh_token, SECRET).is_err());
        // Pero sí como refresh
        let rc = validar_refresh_token(&par.refresh_token, SECRET).unwrap();
        assert_eq!(rc.tipo, "refresh");
        assert!(rc.roles.is_empty(), "Refresh tokens no deben llevar roles");
    }

    #[test]
    fn test_access_token_no_es_refresh() {
        let datos = datos_test();
        let par = generar_tokens(&datos, SECRET, 3600, 604800).unwrap();
        assert!(validar_refresh_token(&par.access_token, SECRET).is_err());
    }

    #[test]
    fn test_token_expirado() {
        // Generar claims con exp en el pasado (hace 10 segundos)
        let ahora = Utc::now().timestamp();
        let key = EncodingKey::from_secret(SECRET.as_bytes());
        let claims_pasado = Claims {
            sub: "1".into(),
            iat: ahora - 20,
            exp: ahora - 10,   // ya expiró
            tipo: "access".into(),
            user_id: 1,
            company_id: 1,
            email: "test@test.com".into(),
            roles: vec![],
        };
        let token = encode(&Header::new(ALGORITMO), &claims_pasado, &key).unwrap();
        assert!(validar_access_token(&token, SECRET).is_err(), "Token expirado debe fallar");
    }

    #[test]
    fn test_secret_incorrecto() {
        let datos = datos_test();
        let par = generar_tokens(&datos, SECRET, 3600, 604800).unwrap();
        assert!(validar_access_token(&par.access_token, "wrong_secret_wrong_secret_wrong!").is_err());
    }

    #[test]
    fn test_renovar_access_token() {
        let datos = datos_test();
        let par = generar_tokens(&datos, SECRET, 3600, 604800).unwrap();
        let nuevo = renovar_access_token(&par.refresh_token, SECRET, 3600).unwrap();
        let claims = validar_access_token(&nuevo, SECRET).unwrap();
        assert_eq!(claims.user_id, 42);
        assert_eq!(claims.tipo, "access");
    }

    #[test]
    fn test_tiene_rol() {
        let datos = datos_test();
        let par = generar_tokens(&datos, SECRET, 3600, 604800).unwrap();
        let claims = validar_access_token(&par.access_token, SECRET).unwrap();
        assert!(tiene_rol(&claims, roles::ADMIN));
        assert!(tiene_rol(&claims, roles::VENTAS));
        assert!(!tiene_rol(&claims, roles::CONTABLE));
    }
}
