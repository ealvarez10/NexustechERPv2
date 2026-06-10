//! c_Exportacion — Tipo de exportación
use super::{Catalogo, ClaveNoEncontrada};
pub static EXPORTACIONES: &[Catalogo] = &[
    Catalogo { clave: "01", descripcion: "No aplica" },
    Catalogo { clave: "02", descripcion: "Definitiva con clave de pedimento A1" },
    Catalogo { clave: "03", descripcion: "Temporal" },
    Catalogo { clave: "04", descripcion: "Definitiva con clave de pedimento diferente a A1 o cuando no se tiene número de pedimento" },
];
pub fn buscar(clave: &str) -> Result<&'static Catalogo, ClaveNoEncontrada> {
    EXPORTACIONES.iter().find(|c| c.clave == clave)
        .ok_or_else(|| ClaveNoEncontrada(format!("c_Exportacion: {}", clave)))
}
pub fn es_valida(clave: &str) -> bool { buscar(clave).is_ok() }
