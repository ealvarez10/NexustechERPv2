//! c_ObjetoImp — Objeto del impuesto en concepto
use super::{Catalogo, ClaveNoEncontrada};
pub static OBJETO_IMP: &[Catalogo] = &[
    Catalogo { clave: "01", descripcion: "No objeto de impuesto" },
    Catalogo { clave: "02", descripcion: "Sí objeto de impuesto" },
    Catalogo { clave: "03", descripcion: "Sí objeto del impuesto y no obligado al desglose" },
    Catalogo { clave: "04", descripcion: "Sí objeto del impuesto y no causa impuesto" },
    Catalogo { clave: "05", descripcion: "Sí objeto del impuesto, IVA crédito IEPS" },
];
pub fn buscar(clave: &str) -> Result<&'static Catalogo, ClaveNoEncontrada> {
    OBJETO_IMP.iter().find(|c| c.clave == clave)
        .ok_or_else(|| ClaveNoEncontrada(format!("c_ObjetoImp: {}", clave)))
}
pub fn es_valido(clave: &str) -> bool { buscar(clave).is_ok() }
