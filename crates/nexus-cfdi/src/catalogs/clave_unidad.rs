//! c_ClaveUnidad — Claves de unidad SAT (UNECE Rev 21)
//! Las 40 más utilizadas en México. El catálogo completo tiene ~2800 claves.
use super::{Catalogo, ClaveNoEncontrada};
pub static CLAVES_UNIDAD: &[Catalogo] = &[
    // Servicios
    Catalogo { clave: "E48", descripcion: "Unidad de servicio" },
    Catalogo { clave: "ACT", descripcion: "Actividad" },
    Catalogo { clave: "A9",  descripcion: "Tarifa" },
    Catalogo { clave: "MON", descripcion: "Mes" },
    Catalogo { clave: "HUR", descripcion: "Hora" },
    Catalogo { clave: "DAY", descripcion: "Día" },
    // Piezas/unidades
    Catalogo { clave: "H87", descripcion: "Pieza" },
    Catalogo { clave: "EA",  descripcion: "Elemento" },
    Catalogo { clave: "KT",  descripcion: "Kit" },
    Catalogo { clave: "SET", descripcion: "Conjunto" },
    Catalogo { clave: "PR",  descripcion: "Par" },
    Catalogo { clave: "DZN", descripcion: "Docena" },
    // Peso
    Catalogo { clave: "KGM", descripcion: "Kilogramo" },
    Catalogo { clave: "GRM", descripcion: "Gramo" },
    Catalogo { clave: "TNE", descripcion: "Tonelada métrica" },
    Catalogo { clave: "LBR", descripcion: "Libra" },
    Catalogo { clave: "ONZ", descripcion: "Onza" },
    // Volumen/Líquido
    Catalogo { clave: "LTR", descripcion: "Litro" },
    Catalogo { clave: "MLT", descripcion: "Mililitro" },
    Catalogo { clave: "BLL", descripcion: "Barril" },
    Catalogo { clave: "GLI", descripcion: "Galón imperial" },
    // Longitud/Área
    Catalogo { clave: "MTR", descripcion: "Metro" },
    Catalogo { clave: "CMT", descripcion: "Centímetro" },
    Catalogo { clave: "MMT", descripcion: "Milímetro" },
    Catalogo { clave: "MTK", descripcion: "Metro cuadrado" },
    Catalogo { clave: "MTQ", descripcion: "Metro cúbico" },
    Catalogo { clave: "FOT", descripcion: "Pie" },
    // Cajas/Empaques
    Catalogo { clave: "XBX", descripcion: "Caja" },
    Catalogo { clave: "XPK", descripcion: "Paquete" },
    Catalogo { clave: "XBA", descripcion: "Fardo" },
    Catalogo { clave: "XBG", descripcion: "Bolsa" },
    Catalogo { clave: "XBT", descripcion: "Atado" },
    Catalogo { clave: "XCA", descripcion: "Lata" },
    Catalogo { clave: "XCR", descripcion: "Cajón" },
    // Otros
    Catalogo { clave: "E51", descripcion: "Trabajo" },
    Catalogo { clave: "MWH", descripcion: "Megawatt hora" },
    Catalogo { clave: "KWH", descripcion: "Kilowatt hora" },
    Catalogo { clave: "GJ",  descripcion: "Gigajoule" },
    Catalogo { clave: "BB",  descripcion: "Carga de basura" },
    Catalogo { clave: "XST", descripcion: "Hoja" },
];
pub fn buscar(clave: &str) -> Result<&'static Catalogo, ClaveNoEncontrada> {
    CLAVES_UNIDAD.iter().find(|c| c.clave == clave)
        .ok_or_else(|| ClaveNoEncontrada(format!("c_ClaveUnidad: {}", clave)))
}
pub fn es_valida(clave: &str) -> bool {
    // Siempre válida si pasa — el catálogo completo tiene 2800+ claves
    // Para validación estricta usar la DB SAT completa
    buscar(clave).is_ok()
}
