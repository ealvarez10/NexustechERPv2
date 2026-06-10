//! c_RegimenFiscal — Régimen fiscal del emisor/receptor
use super::{Catalogo, ClaveNoEncontrada};
pub static REGIMENES_FISCALES: &[Catalogo] = &[
    Catalogo { clave: "601", descripcion: "General de Ley Personas Morales" },
    Catalogo { clave: "603", descripcion: "Personas Morales con Fines no Lucrativos" },
    Catalogo { clave: "605", descripcion: "Sueldos y Salarios e Ingresos Asimilados a Salarios" },
    Catalogo { clave: "606", descripcion: "Arrendamiento" },
    Catalogo { clave: "607", descripcion: "Régimen de Enajenación o Adquisición de Bienes" },
    Catalogo { clave: "608", descripcion: "Demás ingresos" },
    Catalogo { clave: "610", descripcion: "Residentes en el Extranjero sin Establecimiento Permanente en México" },
    Catalogo { clave: "611", descripcion: "Ingresos por Dividendos (socios y accionistas)" },
    Catalogo { clave: "612", descripcion: "Personas Físicas con Actividades Empresariales y Profesionales" },
    Catalogo { clave: "614", descripcion: "Ingresos por intereses" },
    Catalogo { clave: "615", descripcion: "Régimen de los ingresos por obtención de premios" },
    Catalogo { clave: "616", descripcion: "Sin obligaciones fiscales" },
    Catalogo { clave: "620", descripcion: "Sociedades Cooperativas de Producción que optan por diferir sus ingresos" },
    Catalogo { clave: "621", descripcion: "Incorporación Fiscal" },
    Catalogo { clave: "622", descripcion: "Actividades Agrícolas, Ganaderas, Silvícolas y Pesqueras" },
    Catalogo { clave: "623", descripcion: "Opcional para Grupos de Sociedades" },
    Catalogo { clave: "624", descripcion: "Coordinados" },
    Catalogo { clave: "625", descripcion: "Régimen de las Actividades Empresariales con ingresos a través de Plataformas Tecnológicas" },
    Catalogo { clave: "626", descripcion: "Régimen Simplificado de Confianza" },
];
pub fn buscar(clave: &str) -> Result<&'static Catalogo, ClaveNoEncontrada> {
    REGIMENES_FISCALES.iter().find(|c| c.clave == clave)
        .ok_or_else(|| ClaveNoEncontrada(format!("c_RegimenFiscal: {}", clave)))
}
pub fn es_valido(clave: &str) -> bool { buscar(clave).is_ok() }
