//! c_UsoCFDI — Uso del CFDI (receptor)
use super::{Catalogo, ClaveNoEncontrada};
pub static USOS_CFDI: &[Catalogo] = &[
    Catalogo { clave: "G01", descripcion: "Adquisición de mercancias" },
    Catalogo { clave: "G02", descripcion: "Devoluciones, descuentos o bonificaciones" },
    Catalogo { clave: "G03", descripcion: "Gastos en general" },
    Catalogo { clave: "I01", descripcion: "Construcciones" },
    Catalogo { clave: "I02", descripcion: "Mobilario y equipo de oficina por inversiones" },
    Catalogo { clave: "I03", descripcion: "Equipo de transporte" },
    Catalogo { clave: "I04", descripcion: "Equipo de computo y accesorios" },
    Catalogo { clave: "I05", descripcion: "Dados, troqueles, moldes, matrices y herramental" },
    Catalogo { clave: "I06", descripcion: "Comunicaciones telefónicas" },
    Catalogo { clave: "I07", descripcion: "Comunicaciones satelitales" },
    Catalogo { clave: "I08", descripcion: "Otra maquinaria y equipo" },
    Catalogo { clave: "D01", descripcion: "Honorarios médicos, dentales y gastos hospitalarios" },
    Catalogo { clave: "D02", descripcion: "Gastos médicos por incapacidad o discapacidad" },
    Catalogo { clave: "D03", descripcion: "Gastos funerales" },
    Catalogo { clave: "D04", descripcion: "Donativos" },
    Catalogo { clave: "D05", descripcion: "Intereses reales efectivamente pagados por créditos hipotecarios" },
    Catalogo { clave: "D06", descripcion: "Aportaciones voluntarias al SAR" },
    Catalogo { clave: "D07", descripcion: "Primas por seguros de gastos médicos" },
    Catalogo { clave: "D08", descripcion: "Gastos de transportación escolar obligatoria" },
    Catalogo { clave: "D09", descripcion: "Depósitos en cuentas para el ahorro, primas que tengan como base planes de pensiones" },
    Catalogo { clave: "D10", descripcion: "Pagos por servicios educativos (colegiaturas)" },
    Catalogo { clave: "S01", descripcion: "Sin efectos fiscales" },
    Catalogo { clave: "CP01", descripcion: "Pagos" },
    Catalogo { clave: "CN01", descripcion: "Nómina" },
];
pub fn buscar(clave: &str) -> Result<&'static Catalogo, ClaveNoEncontrada> {
    USOS_CFDI.iter().find(|c| c.clave == clave)
        .ok_or_else(|| ClaveNoEncontrada(format!("c_UsoCFDI: {}", clave)))
}
pub fn es_valido(clave: &str) -> bool { buscar(clave).is_ok() }
