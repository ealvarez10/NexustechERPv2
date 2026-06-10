//! c_MotivoCancelacion — Motivos de cancelación CFDI
use super::{Catalogo, ClaveNoEncontrada};
pub static MOTIVOS_CANCELACION: &[Catalogo] = &[
    Catalogo { clave: "01", descripcion: "Comprobante emitido con errores con relación" },
    Catalogo { clave: "02", descripcion: "Comprobante emitido con errores sin relación" },
    Catalogo { clave: "03", descripcion: "No se llevó a cabo la operación" },
    Catalogo { clave: "04", descripcion: "Operación nominativa relacionada en una factura global" },
];
pub fn buscar(clave: &str) -> Result<&'static Catalogo, ClaveNoEncontrada> {
    MOTIVOS_CANCELACION.iter().find(|c| c.clave == clave)
        .ok_or_else(|| ClaveNoEncontrada(format!("c_MotivoCancelacion: {}", clave)))
}
pub fn es_valido(clave: &str) -> bool { buscar(clave).is_ok() }
