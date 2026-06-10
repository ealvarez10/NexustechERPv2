//! c_TipoFactor — Tasa, Cuota, Exento
use super::{Catalogo, ClaveNoEncontrada};
pub static TIPOS_FACTOR: &[Catalogo] = &[
    Catalogo { clave: "Tasa",   descripcion: "Tasa — porcentaje aplicado a la base" },
    Catalogo { clave: "Cuota",  descripcion: "Cuota — importe fijo por unidad" },
    Catalogo { clave: "Exento", descripcion: "Exento — no aplica factor" },
];
pub fn buscar(clave: &str) -> Result<&'static Catalogo, ClaveNoEncontrada> {
    TIPOS_FACTOR.iter().find(|c| c.clave == clave)
        .ok_or_else(|| ClaveNoEncontrada(format!("c_TipoFactor: {}", clave)))
}
pub fn es_valido(clave: &str) -> bool { buscar(clave).is_ok() }
