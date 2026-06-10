//! Catálogos SAT para CFDI 4.0
//!
//! Todos los catálogos están embebidos en el binario (include_str! + phf::Map).
//! No requieren archivos externos ni conexión de red.
//!
//! Catálogos incluidos (fuente: SAT sat.gob.mx/cs/buzon/validacion/catalogo):
//!   c_FormaPago        — Formas de pago (01=Efectivo, 03=Transferencia, etc.)
//!   c_MetodoPago       — PUE (Una sola exhibición), PPD (Parcialidades)
//!   c_UsoCFDI          — Uso del CFDI (G01=Adquisición mercancias, S01=Sin efectos, etc.)
//!   c_RegimenFiscal    — Régimen fiscal SAT (601=General, 612=Personas físicas, etc.)
//!   c_TipoDeComprobante— I=Ingreso, E=Egreso, T=Traslado, N=Nómina, P=Pago
//!   c_Exportacion      — 01=No aplica, 02=Definitiva, 03=Temporal, 04=Definitiva con clave
//!   c_Impuesto         — 001=ISR, 002=IVA, 003=IEPS
//!   c_TipoFactor       — Tasa, Cuota, Exento
//!   c_ObjetoImp        — 01=No objeto, 02=Sí objeto, 03=Sí objeto no obligado
//!   c_ClaveUnidad      — H87=Pieza, E48=Servicio, ACT=Actividad, etc.
//!   c_MotivoCancelacion— 01-04 motivos de cancelación SAT
//!   c_TipoRelacion     — 01-09 tipos de relación entre CFDIs
//!   c_Pais             — Catálogo de países ISO
//!   c_Periodicidad     — 01=Diario...05=Bimestral (Info Global)

pub mod forma_pago;
pub mod metodo_pago;
pub mod uso_cfdi;
pub mod regimen_fiscal;
pub mod tipo_comprobante;
pub mod exportacion;
pub mod impuesto;
pub mod tipo_factor;
pub mod objeto_imp;
pub mod clave_unidad;
pub mod motivo_cancelacion;
pub mod tipo_relacion;
pub mod pais;
pub mod periodicidad;

/// Descripción de un valor de catálogo SAT
#[derive(Debug, Clone, PartialEq)]
pub struct Catalogo {
    pub clave: &'static str,
    pub descripcion: &'static str,
}

/// Error de catálogo
#[derive(Debug)]
pub struct ClaveNoEncontrada(pub String);

impl std::fmt::Display for ClaveNoEncontrada {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Clave '{}' no encontrada en catálogo SAT", self.0)
    }
}
