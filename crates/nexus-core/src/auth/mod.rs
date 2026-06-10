//! Autenticación y autorización de NexusTech ERP v2

pub mod jwt;
pub mod password;

pub use jwt::{Claims, DatosUsuario, TokenPair, roles};
pub use jwt::{generar_tokens, validar_access_token, validar_refresh_token, renovar_access_token, tiene_rol};
pub use password::{hashear_password, verificar_password};
