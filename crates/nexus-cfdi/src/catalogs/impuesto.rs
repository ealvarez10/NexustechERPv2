//! c_Impuesto — ISR, IVA, IEPS
use super::{Catalogo, ClaveNoEncontrada};
pub static IMPUESTOS: &[Catalogo] = &[
    Catalogo { clave: "001", descripcion: "ISR" },
    Catalogo { clave: "002", descripcion: "IVA" },
    Catalogo { clave: "003", descripcion: "IEPS" },
];
pub fn buscar(clave: &str) -> Result<&'static Catalogo, ClaveNoEncontrada> {
    IMPUESTOS.iter().find(|c| c.clave == clave)
        .ok_or_else(|| ClaveNoEncontrada(format!("c_Impuesto: {}", clave)))
}
pub fn es_valido(clave: &str) -> bool { buscar(clave).is_ok() }
/// Tasa estándar de IVA
pub const IVA_TASA: &str = "0.160000";
/// Tasa IVA tasa cero
pub const IVA_TASA_CERO: &str = "0.000000";
