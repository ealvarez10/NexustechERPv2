//! Complemento Carta Porte 3.1
//! XSD: http://www.sat.gob.mx/CartaPorte31
//!
//! Requerido para transporte de mercancías por carretera (autotransporte federal).
//! Obligatorio desde 2022 para facturar fletes en México.
//!
//! TODO: Implementación completa pendiente
//! Prioridad: Alta — muchos clientes de transporte lo requieren

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartaPorte {
    pub version: String,            // "3.1"
    pub transp_internac: String,    // "Sí" o "No"
    pub entrada_salida_merc: Option<String>, // "Entrada" o "Salida"
    pub pais_origen_destino: Option<String>,
    pub via_entrada_salida: Option<String>,
    pub total_dist_rec: Option<f64>, // km totales
    pub ubicaciones: Vec<Ubicacion>,
    pub mercancias: Mercancias,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ubicacion {
    pub tipo_ubic: String, // "Origen" o "Destino"
    pub id_ubicacion: String,
    pub rfc_remitente_dest: Option<String>,
    pub nombre_remitente_dest: Option<String>,
    pub fecha_hora_salida_llegada: Option<String>,
    pub domicilio: Option<Domicilio>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domicilio {
    pub calle: Option<String>,
    pub num_exterior: Option<String>,
    pub colonia: Option<String>,
    pub municipio: Option<String>,
    pub estado: String,
    pub pais: String,
    pub codigo_postal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mercancias {
    pub peso_bruto_total: f64,
    pub unidad_peso: String,    // "KGM"
    pub num_total_mercancias: u32,
    pub cargo_por_tasacion: Option<f64>,
    pub mercancia: Vec<Mercancia>,
    pub autotransporte: Option<Autotransporte>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mercancia {
    pub bienes_transp: String,      // clave SAT
    pub claveunidad: String,
    pub descripcion: String,
    pub cantidad: f64,
    pub peso_en_kg: f64,
    pub valor_mercancia: Option<f64>,
    pub moneda: Option<String>,
    pub fraccion_arancelaria: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Autotransporte {
    pub perm_sct: String,       // tipo permiso SCT
    pub num_perm_sct: String,
    pub config_vehicular: String,
    pub placa_vm: String,
    pub anio_modelo_vm: u32,
}

// TODO: generar_xml() — pendiente
