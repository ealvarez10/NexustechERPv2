//! c_Pais — Catálogo de países ISO 3166-1 para CFDI
use super::{Catalogo, ClaveNoEncontrada};
pub static PAISES: &[Catalogo] = &[
    Catalogo { clave: "MEX", descripcion: "México" },
    Catalogo { clave: "USA", descripcion: "Estados Unidos de América" },
    Catalogo { clave: "CAN", descripcion: "Canadá" },
    Catalogo { clave: "BRA", descripcion: "Brasil" },
    Catalogo { clave: "ARG", descripcion: "Argentina" },
    Catalogo { clave: "COL", descripcion: "Colombia" },
    Catalogo { clave: "CHL", descripcion: "Chile" },
    Catalogo { clave: "PER", descripcion: "Perú" },
    Catalogo { clave: "VEN", descripcion: "Venezuela" },
    Catalogo { clave: "GTM", descripcion: "Guatemala" },
    Catalogo { clave: "CUB", descripcion: "Cuba" },
    Catalogo { clave: "ESP", descripcion: "España" },
    Catalogo { clave: "DEU", descripcion: "Alemania" },
    Catalogo { clave: "FRA", descripcion: "Francia" },
    Catalogo { clave: "GBR", descripcion: "Reino Unido" },
    Catalogo { clave: "CHN", descripcion: "China" },
    Catalogo { clave: "JPN", descripcion: "Japón" },
    Catalogo { clave: "KOR", descripcion: "Corea del Sur" },
    Catalogo { clave: "TWN", descripcion: "Taiwán" },
    Catalogo { clave: "IND", descripcion: "India" },
    Catalogo { clave: "XEX", descripcion: "Extranjero genérico" },
];
pub fn buscar(clave: &str) -> Result<&'static Catalogo, ClaveNoEncontrada> {
    PAISES.iter().find(|c| c.clave == clave)
        .ok_or_else(|| ClaveNoEncontrada(format!("c_Pais: {}", clave)))
}
pub fn es_valido(clave: &str) -> bool { buscar(clave).is_ok() }
