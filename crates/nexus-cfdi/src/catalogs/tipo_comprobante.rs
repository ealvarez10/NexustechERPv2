//! c_TipoDeComprobante
use super::{Catalogo, ClaveNoEncontrada};
pub static TIPOS_COMPROBANTE: &[Catalogo] = &[
    Catalogo { clave: "I", descripcion: "Ingreso" },
    Catalogo { clave: "E", descripcion: "Egreso" },
    Catalogo { clave: "T", descripcion: "Traslado" },
    Catalogo { clave: "N", descripcion: "Nómina" },
    Catalogo { clave: "P", descripcion: "Pago" },
];
pub fn buscar(clave: &str) -> Result<&'static Catalogo, ClaveNoEncontrada> {
    TIPOS_COMPROBANTE.iter().find(|c| c.clave == clave)
        .ok_or_else(|| ClaveNoEncontrada(format!("c_TipoDeComprobante: {}", clave)))
}
pub fn es_valido(clave: &str) -> bool { buscar(clave).is_ok() }
