//! Validador de CURP mexicana
//!
//! El CURP (Clave Única de Registro de Población) tiene 18 caracteres.
//! Formato: LLLL AAMMDD H/M NNNCCC DD
//!
//!   L = Letra de nombre/apellidos (4)
//!   AAMMDD = Fecha nacimiento (6)
//!   H/M = Sexo: H=Hombre, M=Mujer
//!   NN = Estado (2 letras)
//!   CCC = Consonantes internas de los tres nombres (3)
//!   DD = Dígito verificador + siglo (1 + 1)
//!
//! Esta librería NO EXISTE en el ecosistema Rust.

use crate::error::CfdiError;

/// Catálogo de claves de estados para CURP
static ESTADOS_CURP: &[(&str, &str)] = &[
    ("AS", "Aguascalientes"),
    ("BC", "Baja California"),
    ("BS", "Baja California Sur"),
    ("CC", "Campeche"),
    ("CL", "Coahuila"),
    ("CM", "Colima"),
    ("CS", "Chiapas"),
    ("CH", "Chihuahua"),
    ("DF", "Ciudad de México"),
    ("DG", "Durango"),
    ("GT", "Guanajuato"),
    ("GR", "Guerrero"),
    ("HG", "Hidalgo"),
    ("JC", "Jalisco"),
    ("MC", "Estado de México"),
    ("MN", "Michoacán"),
    ("MS", "Morelos"),
    ("NT", "Nayarit"),
    ("NL", "Nuevo León"),
    ("OC", "Oaxaca"),
    ("PL", "Puebla"),
    ("QT", "Querétaro"),
    ("QR", "Quintana Roo"),
    ("SP", "San Luis Potosí"),
    ("SL", "Sinaloa"),
    ("SR", "Sonora"),
    ("TC", "Tabasco"),
    ("TS", "Tamaulipas"),
    ("TL", "Tlaxcala"),
    ("VZ", "Veracruz"),
    ("YN", "Yucatán"),
    ("ZS", "Zacatecas"),
    ("NE", "Nacido en el extranjero"),
];

/// CURP validada y parseada
#[derive(Debug, Clone)]
pub struct Curp {
    pub valor: String,
    pub sexo: Sexo,
    pub estado: &'static str,
    pub fecha_nacimiento: String, // YYMMDD
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sexo {
    Hombre,
    Mujer,
}

impl Curp {
    /// Parsear y validar una CURP
    pub fn parse(curp: &str) -> Result<Self, CfdiError> {
        let curp = curp.trim().to_uppercase();
        validar_curp(&curp)?;

        let sexo = match curp.chars().nth(10) {
            Some('H') => Sexo::Hombre,
            Some('M') => Sexo::Mujer,
            _ => return Err(CfdiError::RfcInvalido("Sexo CURP inválido".into())),
        };

        let estado_clave: String = curp.chars().skip(11).take(2).collect();
        let estado = ESTADOS_CURP.iter()
            .find(|(c, _)| *c == estado_clave)
            .map(|(_, n)| *n)
            .ok_or_else(|| CfdiError::RfcInvalido(format!("Estado CURP inválido: {}", estado_clave)))?;

        let fecha: String = curp.chars().skip(4).take(6).collect();

        Ok(Curp { valor: curp, sexo, estado, fecha_nacimiento: fecha })
    }

    pub fn es_hombre(&self) -> bool { self.sexo == Sexo::Hombre }
    pub fn es_mujer(&self) -> bool  { self.sexo == Sexo::Mujer }
}

impl std::fmt::Display for Curp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.valor)
    }
}

/// Valida el formato de una CURP
pub fn validar_curp(curp: &str) -> Result<(), CfdiError> {
    let curp = curp.trim().to_uppercase();

    if curp.len() != 18 {
        return Err(CfdiError::RfcInvalido(format!(
            "CURP debe tener 18 caracteres, tiene {}", curp.len()
        )));
    }

    let chars: Vec<char> = curp.chars().collect();

    // Posiciones 0-3: 4 letras (nombre y apellidos)
    for i in 0..4 {
        if !es_letra_curp(chars[i]) {
            return Err(CfdiError::RfcInvalido(format!(
                "CURP posición {}: debe ser letra, encontrado '{}'", i, chars[i]
            )));
        }
    }

    // Posición 1: primera vocal interna del primer apellido (puede ser vocal)
    // (ya validada como letra arriba)

    // Posiciones 4-9: fecha de nacimiento AAMMDD
    for i in 4..10 {
        if !chars[i].is_ascii_digit() {
            return Err(CfdiError::RfcInvalido(format!(
                "CURP posición {}: fecha debe ser dígito, encontrado '{}'", i, chars[i]
            )));
        }
    }

    // Validar fecha coherente
    let mes: u32 = format!("{}{}", chars[6], chars[7]).parse().unwrap_or(0);
    let dia: u32 = format!("{}{}", chars[8], chars[9]).parse().unwrap_or(0);
    if mes < 1 || mes > 12 { return Err(CfdiError::RfcInvalido(format!("CURP mes inválido: {:02}", mes))); }
    if dia < 1 || dia > 31 { return Err(CfdiError::RfcInvalido(format!("CURP día inválido: {:02}", dia))); }

    // Posición 10: H o M (sexo)
    if chars[10] != 'H' && chars[10] != 'M' {
        return Err(CfdiError::RfcInvalido(format!("CURP sexo inválido '{}': debe ser H o M", chars[10])));
    }

    // Posiciones 11-12: estado (2 letras)
    for i in 11..13 {
        if !chars[i].is_ascii_uppercase() {
            return Err(CfdiError::RfcInvalido(format!(
                "CURP posición {}: estado debe ser letra mayúscula", i
            )));
        }
    }
    let estado: String = chars[11..13].iter().collect();
    if !ESTADOS_CURP.iter().any(|(c, _)| *c == estado) {
        return Err(CfdiError::RfcInvalido(format!("CURP estado inválido: {}", estado)));
    }

    // Posiciones 13-15: 3 consonantes internas
    for i in 13..16 {
        if !es_consonante_o_x(chars[i]) {
            return Err(CfdiError::RfcInvalido(format!(
                "CURP posición {}: debe ser consonante, encontrado '{}'", i, chars[i]
            )));
        }
    }

    // Posiciones 16-17: dígito verificador + siglo
    if !chars[16].is_ascii_alphanumeric() {
        return Err(CfdiError::RfcInvalido("CURP posición 16: dígito verificador inválido".into()));
    }
    if !chars[17].is_ascii_digit() {
        return Err(CfdiError::RfcInvalido("CURP posición 17: dígito verificador inválido".into()));
    }

    Ok(())
}

fn es_letra_curp(c: char) -> bool {
    c.is_ascii_uppercase() || c == 'Ñ'
}

fn es_consonante_o_x(c: char) -> bool {
    matches!(c, 'B'|'C'|'D'|'F'|'G'|'H'|'J'|'K'|'L'|'M'|'N'|'Ñ'|'P'|'Q'|'R'|'S'|'T'|'V'|'W'|'X'|'Y'|'Z')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curp_valida() {
        // CURPs de ejemplo (no reales, solo formato)
        assert!(validar_curp("GOCA800510HDFRNS09").is_ok(), "CURP hombre válida");
        assert!(validar_curp("ROML850320MDFMRZ02").is_ok(), "CURP mujer válida");
    }

    #[test]
    fn test_curp_invalida_longitud() {
        assert!(validar_curp("GOCA800510").is_err(), "CURP corta debe fallar");
        assert!(validar_curp("GOCA800510HDFRNS099999").is_err(), "CURP larga debe fallar");
    }

    #[test]
    fn test_curp_sexo_invalido() {
        assert!(validar_curp("GOCA800510XDFRNS09").is_err(), "Sexo X inválido");
    }

    #[test]
    fn test_curp_parse() {
        let curp = Curp::parse("GOCA800510HDFRNS09").unwrap();
        assert!(curp.es_hombre());
        assert!(!curp.es_mujer());
        assert_eq!(curp.fecha_nacimiento, "800510");
    }

    #[test]
    fn test_curp_estado_invalido() {
        assert!(validar_curp("GOCA800510HXXRNS09").is_err(), "Estado XX inválido");
    }
}
