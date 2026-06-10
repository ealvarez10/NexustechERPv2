//! c_TipoRelacion — Tipo de relación entre CFDIs
use super::{Catalogo, ClaveNoEncontrada};
pub static TIPOS_RELACION: &[Catalogo] = &[
    Catalogo { clave: "01", descripcion: "Nota de crédito de los documentos relacionados" },
    Catalogo { clave: "02", descripcion: "Nota de débito de los documentos relacionados" },
    Catalogo { clave: "03", descripcion: "Devolución de mercancía sobre facturas o traslados previos" },
    Catalogo { clave: "04", descripcion: "Sustitución de los CFDI previos" },
    Catalogo { clave: "05", descripcion: "Traslados de mercancias facturados previamente" },
    Catalogo { clave: "06", descripcion: "Factura generada por los traslados previos" },
    Catalogo { clave: "07", descripcion: "CFDI por aplicación de anticipos" },
    Catalogo { clave: "08", descripcion: "Factura generada por pagos en parcialidades" },
    Catalogo { clave: "09", descripcion: "Factura generada por pagos diferidos" },
];
pub fn buscar(clave: &str) -> Result<&'static Catalogo, ClaveNoEncontrada> {
    TIPOS_RELACION.iter().find(|c| c.clave == clave)
        .ok_or_else(|| ClaveNoEncontrada(format!("c_TipoRelacion: {}", clave)))
}
pub fn es_valido(clave: &str) -> bool { buscar(clave).is_ok() }
