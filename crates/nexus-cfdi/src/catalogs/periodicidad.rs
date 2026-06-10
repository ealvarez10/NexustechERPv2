//! c_Periodicidad — Para CFDI globales (InformacionGlobal)
use super::{Catalogo, ClaveNoEncontrada};
pub static PERIODICIDADES: &[Catalogo] = &[
    Catalogo { clave: "01", descripcion: "Diario" },
    Catalogo { clave: "02", descripcion: "Semanal" },
    Catalogo { clave: "03", descripcion: "Quincenal" },
    Catalogo { clave: "04", descripcion: "Mensual" },
    Catalogo { clave: "05", descripcion: "Bimestral" },
];
pub fn buscar(clave: &str) -> Result<&'static Catalogo, ClaveNoEncontrada> {
    PERIODICIDADES.iter().find(|c| c.clave == clave)
        .ok_or_else(|| ClaveNoEncontrada(format!("c_Periodicidad: {}", clave)))
}
pub fn es_valida(clave: &str) -> bool { buscar(clave).is_ok() }
