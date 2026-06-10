//! Configuración de NexusTech ERP v2
//!
//! Carga variables de entorno desde `.env` (desarrollo) o del entorno del sistema (producción).
//! Un solo punto de verdad para toda la configuración del servidor.

use dotenvy::dotenv;
use std::env;
use crate::error::CoreError;

/// Configuración completa de la aplicación
#[derive(Debug, Clone)]
pub struct Config {
    // ─── Base de datos ─────────────────────────────────────────
    /// URL PostgreSQL principal: postgres://user:pass@host:5432/dbname
    pub database_url: String,
    /// Tamaño máximo del pool de conexiones
    pub database_pool_max: u32,
    /// Timeout de conexión en segundos
    pub database_connect_timeout_secs: u64,

    // ─── Servidor HTTP ─────────────────────────────────────────
    /// Dirección de escucha: 0.0.0.0:8080
    pub server_host: String,
    /// Puerto HTTP
    pub server_port: u16,
    /// Entorno: "development" | "production" | "staging"
    pub environment: Environment,

    // ─── Auth / JWT ────────────────────────────────────────────
    /// Secreto para firmar tokens JWT (mín. 32 caracteres)
    pub jwt_secret: String,
    /// Expiración del access token en segundos (default: 3600 = 1h)
    pub jwt_access_expires_secs: u64,
    /// Expiración del refresh token en segundos (default: 604800 = 7d)
    pub jwt_refresh_expires_secs: u64,

    // ─── Redis / Caché ─────────────────────────────────────────
    /// URL Redis: redis://127.0.0.1:6379
    pub redis_url: String,

    // ─── NexusSearch ───────────────────────────────────────────
    /// URL de NexusSearch: http://127.0.0.1:7700
    pub search_url: String,
    /// Master key de NexusSearch
    pub search_key: String,

    // ─── Email ─────────────────────────────────────────────────
    pub smtp_host: Option<String>,
    pub smtp_port: Option<u16>,
    pub smtp_user: Option<String>,
    pub smtp_pass: Option<String>,
    pub smtp_from: Option<String>,

    // ─── CFDI / PAC ────────────────────────────────────────────
    /// PAC activo: "facturama" | "sw_sapien"
    pub cfdi_pac: Option<String>,
    /// Token/credenciales del PAC (formato depende del PAC)
    pub cfdi_pac_token: Option<String>,
    /// Modo prueba del PAC
    pub cfdi_pac_sandbox: bool,

    // ─── Empresa por defecto ────────────────────────────────────
    /// ID de empresa en la DB (default: 1)
    pub default_company_id: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl Environment {
    pub fn es_produccion(&self) -> bool {
        *self == Environment::Production
    }
    pub fn es_desarrollo(&self) -> bool {
        *self == Environment::Development
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Development => write!(f, "development"),
            Environment::Staging     => write!(f, "staging"),
            Environment::Production  => write!(f, "production"),
        }
    }
}

impl Config {
    /// Carga la configuración desde variables de entorno.
    /// En desarrollo carga `.env` automáticamente.
    pub fn from_env() -> Result<Self, CoreError> {
        // Cargar .env si existe (silencioso si no existe — producción usa vars del sistema)
        let _ = dotenv();

        let environment = match env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "development".into())
            .to_lowercase()
            .as_str()
        {
            "production" => Environment::Production,
            "staging"    => Environment::Staging,
            _            => Environment::Development,
        };

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| CoreError::Config("DATABASE_URL no configurada".into()))?;

        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| CoreError::Config("JWT_SECRET no configurada".into()))?;

        if jwt_secret.len() < 32 {
            return Err(CoreError::Config(
                "JWT_SECRET debe tener al menos 32 caracteres".into()
            ));
        }

        Ok(Config {
            database_url,
            database_pool_max: env_u32("DATABASE_POOL_MAX", 10),
            database_connect_timeout_secs: env_u64("DATABASE_CONNECT_TIMEOUT_SECS", 10),

            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            server_port: env_u16("SERVER_PORT", 8080),
            environment,

            jwt_secret,
            jwt_access_expires_secs:  env_u64("JWT_ACCESS_EXPIRES_SECS",  3600),
            jwt_refresh_expires_secs: env_u64("JWT_REFRESH_EXPIRES_SECS", 604800),

            redis_url:  env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".into()),

            search_url: env::var("MEILI_URL").unwrap_or_else(|_| "http://127.0.0.1:7700".into()),
            search_key: env::var("MEILI_MASTER_KEY").unwrap_or_else(|_| "nexustech_dev_key_2026".into()),

            smtp_host: env::var("SMTP_HOST").ok(),
            smtp_port: env::var("SMTP_PORT").ok().and_then(|p| p.parse().ok()),
            smtp_user: env::var("SMTP_USER").ok(),
            smtp_pass: env::var("SMTP_PASS").ok(),
            smtp_from: env::var("SMTP_FROM").ok(),

            cfdi_pac:         env::var("CFDI_PAC").ok(),
            cfdi_pac_token:   env::var("CFDI_PAC_TOKEN").ok(),
            cfdi_pac_sandbox: env::var("CFDI_PAC_SANDBOX")
                .map(|v| v == "true" || v == "1")
                .unwrap_or(true),  // sandbox por defecto — seguridad

            default_company_id: env_i32("DEFAULT_COMPANY_ID", 1),
        })
    }

    /// Genera la dirección completa del servidor (host:port)
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }

    /// URL de la DB sin credenciales (para logs seguros)
    pub fn database_url_safe(&self) -> String {
        // Enmascara: postgres://user:PASS@host:port/db → postgres://user:***@host:port/db
        if let Some(at_pos) = self.database_url.rfind('@') {
            if let Some(colon_pos) = self.database_url[..at_pos].rfind(':') {
                let prefix = &self.database_url[..colon_pos + 1];
                let suffix = &self.database_url[at_pos..];
                return format!("{}***{}", prefix, suffix);
            }
        }
        "postgres://***".into()
    }
}

// ─── Helpers de lectura de env ──────────────────────────────────────────────

fn env_u32(key: &str, default: u32) -> u32 {
    env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn env_u64(key: &str, default: u64) -> u64 {
    env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn env_u16(key: &str, default: u16) -> u16 {
    env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn env_i32(key: &str, default: i32) -> i32 {
    env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_url_safe_enmascara_password() {
        let config = Config {
            database_url: "postgres://nexus:SuperSecreta123@127.0.0.1:5432/nexus_db".into(),
            database_pool_max: 10,
            database_connect_timeout_secs: 10,
            server_host: "0.0.0.0".into(),
            server_port: 8080,
            environment: Environment::Development,
            jwt_secret: "a".repeat(32),
            jwt_access_expires_secs: 3600,
            jwt_refresh_expires_secs: 604800,
            redis_url: "redis://127.0.0.1:6379".into(),
            search_url: "http://127.0.0.1:7700".into(),
            search_key: "key".into(),
            smtp_host: None, smtp_port: None, smtp_user: None,
            smtp_pass: None, smtp_from: None,
            cfdi_pac: None, cfdi_pac_token: None, cfdi_pac_sandbox: true,
            default_company_id: 1,
        };

        let safe = config.database_url_safe();
        assert!(!safe.contains("SuperSecreta123"), "La contraseña NO debe aparecer en logs");
        assert!(safe.contains("***"), "Debe contener ***");
        println!("URL segura: {}", safe);
    }

    #[test]
    fn test_server_addr() {
        let mut config = Config {
            database_url: "postgres://x:y@localhost/z".into(),
            database_pool_max: 10,
            database_connect_timeout_secs: 10,
            server_host: "127.0.0.1".into(),
            server_port: 9000,
            environment: Environment::Development,
            jwt_secret: "a".repeat(32),
            jwt_access_expires_secs: 3600,
            jwt_refresh_expires_secs: 604800,
            redis_url: "redis://127.0.0.1:6379".into(),
            search_url: "http://127.0.0.1:7700".into(),
            search_key: "k".into(),
            smtp_host: None, smtp_port: None, smtp_user: None,
            smtp_pass: None, smtp_from: None,
            cfdi_pac: None, cfdi_pac_token: None, cfdi_pac_sandbox: true,
            default_company_id: 1,
        };
        assert_eq!(config.server_addr(), "127.0.0.1:9000");
    }
}
