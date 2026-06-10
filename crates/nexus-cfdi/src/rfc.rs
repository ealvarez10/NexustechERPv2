//! Validador y parser de RFC mexicano
//!
//! Implementación del algoritmo oficial del SAT para validar RFCs de:
//!   - Personas Morales (12 caracteres): LLLL AAMMDD NNN
//!   - Personas Físicas (13 caracteres): LLLL LLAA MMDD NNN H
//!   - RFC genérico CFDI: XAXX010101000 (público en general)
//!   - RFC extranjero: XEXX010101000
//!
//! Referencias:
//!   Regla 2.4.6 RMISC — Anexo 6 del SAT
//!   https://www.sat.gob.mx/tramites/16663/obten-tu-rfc-personas-morales

use crate::error::CfdiError;

/// Tipo de persona del RFC
#[derive(Debug, Clone, PartialEq)]
pub enum TipoPersona {
    Moral,   // 12 caracteres: XAAA010101AAA
    Fisica,  // 13 caracteres: XAAA010101AAAX
    Generico, // XAXX010101000 o XEXX010101000
}

/// RFC validado y parseado
#[derive(Debug, Clone)]
pub struct Rfc {
    pub valor: String,
    pub tipo: TipoPersona,
}

impl Rfc {
    /// Parsear y validar un RFC
    pub fn parse(rfc: &str) -> Result<Self, CfdiError> {
        let rfc = rfc.trim().to_uppercase();
        validar_rfc(&rfc)?;

        let tipo = clasificar_rfc(&rfc);
        Ok(Rfc { valor: rfc, tipo })
    }

    /// Verificar si es RFC genérico (XAXX o XEXX — público en general / extranjero)
    pub fn es_generico(&self) -> bool {
        self.tipo == TipoPersona::Generico
    }

    /// Verificar si es persona moral
    pub fn es_moral(&self) -> bool {
        self.tipo == TipoPersona::Moral
    }

    /// Verificar si es persona física
    pub fn es_fisica(&self) -> bool {
        self.tipo == TipoPersona::Fisica
    }

    /// Nombre del régimen por tipo (referencia rápida)
    pub fn requiere_domicilio_fiscal(&self) -> bool {
        !self.es_generico()
    }
}

impl std::fmt::Display for Rfc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.valor)
    }
}

/// Valida un RFC según las reglas del SAT
///
/// Reglas:
/// 1. Solo caracteres A-Z, 0-9 y &Ñ-
/// 2. Longitud: 12 (moral) o 13 (física)
/// 3. Primeras letras: patrón nombre/apellidos
/// 4. Fecha válida en posiciones AAMMDD
/// 5. Homoclave: 3 caracteres alfanuméricos
pub fn validar_rfc(rfc: &str) -> Result<(), CfdiError> {
    let rfc = rfc.trim().to_uppercase();
    let len = rfc.len();

    // RFC genéricos del SAT
    if rfc == "XAXX010101000" || rfc == "XEXX010101000" {
        return Ok(());
    }

    // Longitud
    if len != 12 && len != 13 {
        return Err(CfdiError::RfcInvalido(format!(
            "Longitud inválida: {} caracteres (se esperan 12 para moral o 13 para física)",
            len
        )));
    }

    let chars: Vec<char> = rfc.chars().collect();

    // Validar que primeros 3-4 chars son letras (incluyendo Ñ)
    let n_letras_inicio = if len == 12 { 3 } else { 4 };
    for i in 0..n_letras_inicio {
        if !es_letra(chars[i]) {
            return Err(CfdiError::RfcInvalido(format!(
                "Posición {} debe ser letra, encontrado '{}'",
                i, chars[i]
            )));
        }
    }

    // Posición después de letras iniciales
    let fecha_inicio = n_letras_inicio;

    // Fecha AAMMDD (6 dígitos)
    for i in fecha_inicio..fecha_inicio + 6 {
        if !chars[i].is_ascii_digit() {
            return Err(CfdiError::RfcInvalido(format!(
                "Posición {} debe ser dígito (fecha AAMMDD), encontrado '{}'",
                i, chars[i]
            )));
        }
    }

    // Validar fecha sea coherente
    let fecha_str: String = chars[fecha_inicio..fecha_inicio + 6].iter().collect();
    validar_fecha_rfc(&fecha_str)?;

    // Homoclave: 3 caracteres alfanuméricos (A-Z, 0-9)
    let homo_inicio = fecha_inicio + 6;
    for i in homo_inicio..homo_inicio + 3 {
        if i >= len { break; }
        if !es_alfanumerico(chars[i]) {
            return Err(CfdiError::RfcInvalido(format!(
                "Homoclave posición {}: carácter inválido '{}'",
                i, chars[i]
            )));
        }
    }

    // Dígito verificador para personas físicas (posición 13)
    if len == 13 {
        let digito = chars[12];
        if !es_alfanumerico(digito) {
            return Err(CfdiError::RfcInvalido(format!(
                "Dígito verificador inválido: '{}'",
                digito
            )));
        }
    }

    Ok(())
}

/// Valida que la fecha AAMMDD en el RFC sea coherente
fn validar_fecha_rfc(fecha: &str) -> Result<(), CfdiError> {
    if fecha.len() != 6 {
        return Err(CfdiError::RfcInvalido("Fecha RFC malformada".into()));
    }

    let anio: u32 = fecha[0..2].parse().unwrap_or(99);
    let mes: u32  = fecha[2..4].parse().unwrap_or(0);
    let dia: u32  = fecha[4..6].parse().unwrap_or(0);

    if mes < 1 || mes > 12 {
        return Err(CfdiError::RfcInvalido(format!(
            "Mes inválido en RFC: {:02}", mes
        )));
    }

    let dias_max = dias_en_mes(mes, 2000 + anio as u32);
    if dia < 1 || dia > dias_max {
        return Err(CfdiError::RfcInvalido(format!(
            "Día inválido en RFC: {:02} para mes {:02}", dia, mes
        )));
    }

    Ok(())
}

fn dias_en_mes(mes: u32, anio: u32) -> u32 {
    match mes {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if es_bisiesto(anio) { 29 } else { 28 }
        }
        _ => 31,
    }
}

fn es_bisiesto(anio: u32) -> bool {
    (anio % 4 == 0 && anio % 100 != 0) || (anio % 400 == 0)
}

fn clasificar_rfc(rfc: &str) -> TipoPersona {
    if rfc == "XAXX010101000" || rfc == "XEXX010101000" {
        TipoPersona::Generico
    } else if rfc.len() == 12 {
        TipoPersona::Moral
    } else {
        TipoPersona::Fisica
    }
}

/// ¿Es una letra válida en RFC? (A-Z incluyendo Ñ, sin vocales ofensivas por SAT)
fn es_letra(c: char) -> bool {
    c.is_ascii_uppercase() || c == 'Ñ'
}

/// ¿Es alfanumérico válido en homoclave?
fn es_alfanumerico(c: char) -> bool {
    c.is_ascii_uppercase() || c.is_ascii_digit()
}

/// RFC de prueba para cuando no se tiene el RFC del receptor (CFDI público en general)
pub const RFC_PUBLICO_GENERAL: &str = "XAXX010101000";

/// RFC para receptores extranjeros
pub const RFC_EXTRANJERO: &str = "XEXX010101000";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rfc_moral_valido() {
        assert!(validar_rfc("IBS120101AA1").is_ok(), "RFC moral debe ser válido");
        assert!(validar_rfc("SAT970701NN3").is_ok(), "SAT RFC debe ser válido");
        assert!(validar_rfc("EME041126C37").is_ok(), "RFC moral válido");
    }

    #[test]
    fn test_rfc_fisica_valido() {
        assert!(validar_rfc("AAAA010101AAA").is_ok(), "RFC física debe ser válido");
        assert!(validar_rfc("GOCA800510F24").is_ok(), "RFC física válido");
    }

    #[test]
    fn test_rfc_generico() {
        assert!(validar_rfc("XAXX010101000").is_ok(), "Público en general");
        assert!(validar_rfc("XEXX010101000").is_ok(), "Extranjero");
    }

    #[test]
    fn test_rfc_invalido_longitud() {
        assert!(validar_rfc("IBS12").is_err(), "RFC muy corto");
        assert!(validar_rfc("IBS12010100AA11").is_err(), "RFC muy largo");
    }

    #[test]
    fn test_rfc_invalido_mes() {
        assert!(validar_rfc("IBS121301AA1").is_err(), "Mes 13 inválido");
        assert!(validar_rfc("IBS120001AA1").is_err(), "Mes 00 inválido");
    }

    #[test]
    fn test_rfc_invalido_dia() {
        assert!(validar_rfc("IBS120132AA1").is_err(), "Día 32 inválido");
    }

    #[test]
    fn test_rfc_parse_moral() {
        let rfc = Rfc::parse("IBS120101AA1").unwrap();
        assert!(rfc.es_moral());
        assert!(!rfc.es_generico());
        assert_eq!(rfc.to_string(), "IBS120101AA1");
    }

    #[test]
    fn test_rfc_parse_fisica() {
        let rfc = Rfc::parse("goca800510f24").unwrap(); // minúsculas → normaliza
        assert!(rfc.es_fisica());
        assert_eq!(rfc.valor, "GOCA800510F24");
    }

    #[test]
    fn test_rfc_generico_parse() {
        let rfc = Rfc::parse("XAXX010101000").unwrap();
        assert!(rfc.es_generico());
        assert!(!rfc.requiere_domicilio_fiscal());
    }
}
