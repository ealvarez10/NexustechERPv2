//! `OVal` — el valor dinámico del kernel (§3.1 del reporte odoo2rs).
//!
//! La fidelidad semántica con Odoo exige renunciar al tipado estático
//! *dentro* del kernel: un campo leído de un recordset puede ser texto,
//! número, fecha, referencia a otro modelo o `False`. `OVal` es ese
//! universo cerrado de valores, con la semántica «falsy» de Python
//! (`False`, `0`, `""`, recordset vacío) reproducida explícitamente.

use std::cmp::Ordering;
use std::fmt;

use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use smol_str::SmolStr;

use crate::error::{OError, OResult};

/// Identificador denso de un modelo dentro del `Registry`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModelId(pub u32);

impl ModelId {
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

/// Identificador de registro (cubre `int4` e `int8` del esquema Odoo).
pub type RecordId = i64;

/// Valor dinámico de un campo — el equivalente al «cualquier cosa» de Python
/// dentro del ORM de Odoo, pero como enum cerrado y seguro.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum OVal {
    /// El `False` de Odoo en campos char/relacionales/numéricos vacíos.
    #[default]
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    /// Campos Monetary / numeric — nunca float, para no divergir en centavos.
    Decimal(Decimal),
    Str(SmolStr),
    Date(NaiveDate),
    DateTime(NaiveDateTime),
    /// many2one: referencia a un registro de otro modelo.
    Ref(ModelId, RecordId),
    /// one2many / many2many: conjunto de referencias.
    RefSet(ModelId, Vec<RecordId>),
    /// jsonb (p. ej. nombres i18n de `product_template.name`).
    Json(serde_json::Value),
}

impl OVal {
    /// Semántica de verdad de Python/Odoo: `False`, `0`, `0.0`, `""`,
    /// recordset vacío y `None` son falsos; todo lo demás, verdadero.
    pub fn is_truthy(&self) -> bool {
        match self {
            OVal::Null => false,
            OVal::Bool(b) => *b,
            OVal::Int(i) => *i != 0,
            OVal::Float(f) => *f != 0.0,
            OVal::Decimal(d) => !d.is_zero(),
            OVal::Str(s) => !s.is_empty(),
            OVal::Date(_) | OVal::DateTime(_) | OVal::Ref(_, _) => true,
            OVal::RefSet(_, ids) => !ids.is_empty(),
            OVal::Json(v) => match v {
                serde_json::Value::Null => false,
                serde_json::Value::Bool(b) => *b,
                _ => true,
            },
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            OVal::Null => "Null",
            OVal::Bool(_) => "Bool",
            OVal::Int(_) => "Int",
            OVal::Float(_) => "Float",
            OVal::Decimal(_) => "Decimal",
            OVal::Str(_) => "Str",
            OVal::Date(_) => "Date",
            OVal::DateTime(_) => "DateTime",
            OVal::Ref(_, _) => "Ref",
            OVal::RefSet(_, _) => "RefSet",
            OVal::Json(_) => "Json",
        }
    }

    fn type_err(&self, expected: &'static str) -> OError {
        OError::Type {
            expected,
            got: self.type_name(),
        }
    }

    /// Coerción a texto. `Null` → `""` (el `False` de un Char vacío).
    pub fn as_str(&self) -> OResult<SmolStr> {
        match self {
            OVal::Str(s) => Ok(s.clone()),
            OVal::Null => Ok(SmolStr::default()),
            _ => Err(self.type_err("Str")),
        }
    }

    /// Coerción a entero. `Null` → `0`; `Ref` → su id (como hace Odoo con `.id`).
    pub fn as_int(&self) -> OResult<i64> {
        match self {
            OVal::Int(i) => Ok(*i),
            OVal::Bool(b) => Ok(*b as i64),
            OVal::Ref(_, id) => Ok(*id),
            OVal::Null => Ok(0),
            _ => Err(self.type_err("Int")),
        }
    }

    /// Coerción a flotante. `Null` → `0.0`.
    pub fn as_float(&self) -> OResult<f64> {
        match self {
            OVal::Float(f) => Ok(*f),
            OVal::Int(i) => Ok(*i as f64),
            OVal::Decimal(d) => d.to_f64().ok_or_else(|| self.type_err("Float")),
            OVal::Null => Ok(0.0),
            _ => Err(self.type_err("Float")),
        }
    }

    /// Coerción a Decimal (Monetary). `Null` → `0`.
    pub fn as_decimal(&self) -> OResult<Decimal> {
        match self {
            OVal::Decimal(d) => Ok(*d),
            OVal::Int(i) => Ok(Decimal::from(*i)),
            OVal::Float(f) => Decimal::from_f64(*f).ok_or_else(|| self.type_err("Decimal")),
            OVal::Null => Ok(Decimal::ZERO),
            _ => Err(self.type_err("Decimal")),
        }
    }

    pub fn as_bool(&self) -> bool {
        self.is_truthy()
    }

    /// `Ref` → `(modelo, id)`.
    pub fn as_ref(&self) -> OResult<(ModelId, RecordId)> {
        match self {
            OVal::Ref(m, id) => Ok((*m, *id)),
            _ => Err(self.type_err("Ref")),
        }
    }

    /// Conversión «mejor esfuerzo» desde JSON (valores de dominios, IR, API).
    pub fn from_json(v: &serde_json::Value) -> OVal {
        match v {
            serde_json::Value::Null => OVal::Null,
            serde_json::Value::Bool(b) => OVal::Bool(*b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    OVal::Int(i)
                } else {
                    OVal::Float(n.as_f64().unwrap_or(0.0))
                }
            }
            serde_json::Value::String(s) => OVal::Str(s.into()),
            other => OVal::Json(other.clone()),
        }
    }

    /// Serialización a JSON para la API (`Ref` → id, `RefSet` → lista de ids).
    pub fn to_json(&self) -> serde_json::Value {
        use serde_json::{json, Value};
        match self {
            OVal::Null => Value::Null,
            OVal::Bool(b) => json!(b),
            OVal::Int(i) => json!(i),
            OVal::Float(f) => json!(f),
            OVal::Decimal(d) => json!(d.to_string()),
            OVal::Str(s) => json!(s.as_str()),
            OVal::Date(d) => json!(d.format("%Y-%m-%d").to_string()),
            OVal::DateTime(dt) => json!(dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            OVal::Ref(_, id) => json!(id),
            OVal::RefSet(_, ids) => json!(ids),
            OVal::Json(v) => v.clone(),
        }
    }

    /// Comparación laxa entre valores (para `sorted()` y dominios evaluados
    /// en memoria). Cruza tipos numéricos; `Null` ordena primero.
    pub fn cmp_loose(&self, other: &OVal) -> Ordering {
        use OVal::*;
        match (self, other) {
            (Null, Null) => Ordering::Equal,
            (Null, _) => Ordering::Less,
            (_, Null) => Ordering::Greater,
            (Bool(a), Bool(b)) => a.cmp(b),
            (Str(a), Str(b)) => a.cmp(b),
            (Date(a), Date(b)) => a.cmp(b),
            (DateTime(a), DateTime(b)) => a.cmp(b),
            (Ref(_, a), Ref(_, b)) => a.cmp(b),
            // Numéricos cruzados → vía Decimal cuando sea posible
            (a, b) => match (a.as_decimal(), b.as_decimal()) {
                (Ok(da), Ok(db)) => da.cmp(&db),
                _ => Ordering::Equal,
            },
        }
    }
}

impl fmt::Display for OVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OVal::Null => write!(f, "False"),
            OVal::Bool(b) => write!(f, "{}", if *b { "True" } else { "False" }),
            OVal::Int(i) => write!(f, "{i}"),
            OVal::Float(v) => write!(f, "{v}"),
            OVal::Decimal(d) => write!(f, "{d}"),
            OVal::Str(s) => write!(f, "{s}"),
            OVal::Date(d) => write!(f, "{}", d.format("%Y-%m-%d")),
            OVal::DateTime(dt) => write!(f, "{}", dt.format("%Y-%m-%d %H:%M:%S")),
            OVal::Ref(m, id) => write!(f, "<{},{}>", m.0, id),
            OVal::RefSet(m, ids) => write!(f, "<{},{:?}>", m.0, ids),
            OVal::Json(v) => write!(f, "{v}"),
        }
    }
}

// ─── From<T> ergonómicos para código generado y manual ─────────────────────

impl From<bool> for OVal {
    fn from(v: bool) -> Self {
        OVal::Bool(v)
    }
}
impl From<i32> for OVal {
    fn from(v: i32) -> Self {
        OVal::Int(v as i64)
    }
}
impl From<i64> for OVal {
    fn from(v: i64) -> Self {
        OVal::Int(v)
    }
}
impl From<f64> for OVal {
    fn from(v: f64) -> Self {
        OVal::Float(v)
    }
}
impl From<Decimal> for OVal {
    fn from(v: Decimal) -> Self {
        OVal::Decimal(v)
    }
}
impl From<&str> for OVal {
    fn from(v: &str) -> Self {
        OVal::Str(v.into())
    }
}
impl From<String> for OVal {
    fn from(v: String) -> Self {
        OVal::Str(v.into())
    }
}
impl From<SmolStr> for OVal {
    fn from(v: SmolStr) -> Self {
        OVal::Str(v)
    }
}
impl From<NaiveDate> for OVal {
    fn from(v: NaiveDate) -> Self {
        OVal::Date(v)
    }
}
impl From<NaiveDateTime> for OVal {
    fn from(v: NaiveDateTime) -> Self {
        OVal::DateTime(v)
    }
}
impl<T: Into<OVal>> From<Option<T>> for OVal {
    fn from(v: Option<T>) -> Self {
        match v {
            Some(x) => x.into(),
            None => OVal::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truthiness_estilo_python() {
        assert!(!OVal::Null.is_truthy());
        assert!(!OVal::Int(0).is_truthy());
        assert!(!OVal::Str("".into()).is_truthy());
        assert!(!OVal::RefSet(ModelId(0), vec![]).is_truthy());
        assert!(OVal::Int(-1).is_truthy());
        assert!(OVal::Str("x".into()).is_truthy());
        assert!(OVal::Ref(ModelId(0), 7).is_truthy());
    }

    #[test]
    fn coerciones_falsy() {
        assert_eq!(OVal::Null.as_int().unwrap(), 0);
        assert_eq!(OVal::Null.as_str().unwrap(), "");
        assert_eq!(OVal::Null.as_decimal().unwrap(), Decimal::ZERO);
        assert_eq!(OVal::Ref(ModelId(1), 42).as_int().unwrap(), 42);
    }

    #[test]
    fn cmp_numerico_cruzado() {
        assert_eq!(
            OVal::Int(2).cmp_loose(&OVal::Decimal(Decimal::new(15, 1))), // 2 vs 1.5
            Ordering::Greater
        );
        assert_eq!(OVal::Null.cmp_loose(&OVal::Int(0)), Ordering::Less);
    }

    #[test]
    fn json_roundtrip_basico() {
        let v = OVal::from_json(&serde_json::json!("sale"));
        assert_eq!(v, OVal::Str("sale".into()));
        assert_eq!(OVal::Int(5).to_json(), serde_json::json!(5));
        assert_eq!(OVal::Ref(ModelId(0), 9).to_json(), serde_json::json!(9));
    }
}
