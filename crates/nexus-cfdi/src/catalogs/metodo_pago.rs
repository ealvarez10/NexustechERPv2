//! c_MetodoPago — PUE o PPD
use super::{Catalogo, ClaveNoEncontrada};
pub static METODOS_PAGO: &[Catalogo] = &[
    Catalogo { clave: "PUE", descripcion: "Pago en una sola exhibición" },
    Catalogo { clave: "PPD", descripcion: "Pago en parcialidades o diferido" },
];
pub fn buscar(clave: &str) -> Result<&'static Catalogo, ClaveNoEncontrada> {
    METODOS_PAGO.iter().find(|c| c.clave == clave)
        .ok_or_else(|| ClaveNoEncontrada(format!("c_MetodoPago: {}", clave)))
}
pub fn es_valido(clave: &str) -> bool { buscar(clave).is_ok() }
