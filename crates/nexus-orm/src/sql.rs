//! Puente OVal ↔ Postgres: binding tipado de parámetros y decodificación
//! de filas a valores dinámicos, guiados por los metadatos de campo.
//!
//! El tipo de la **columna** (no el del valor) manda: un `OVal::Int(100)`
//! contra una columna Monetary se enlaza como `numeric`, contra un
//! many2one como `int4`. Así el SQL generado nunca pelea con los tipos
//! reales del esquema Odoo existente.

use chrono::{NaiveDate, NaiveDateTime};
use sqlx::postgres::{PgArguments, PgRow};
use sqlx::query::Query;
use sqlx::{Postgres, Row};

use crate::error::{OError, OResult};
use crate::fields::{FieldDef, FieldType};
use crate::registry::Registry;
use crate::value::OVal;

pub type PgQuery<'q> = Query<'q, Postgres, PgArguments>;

/// Enlaza un `OVal` a la consulta según el tipo de la columna destino.
/// `OVal::Null` no se enlaza nunca: el generador emite `IS NULL` /
/// literal `NULL` en el SQL.
pub fn bind_typed<'q>(q: PgQuery<'q>, ftype: &FieldType, v: &OVal) -> OResult<PgQuery<'q>> {
    if matches!(v, OVal::Null) {
        return Err(OError::Internal(
            "bind_typed recibió Null; los NULL se emiten como literal SQL".into(),
        ));
    }
    Ok(match ftype {
        FieldType::Boolean => q.bind(v.is_truthy()),
        FieldType::Integer | FieldType::Many2one { .. } => {
            let i = v.as_int()?;
            let i32v: i32 = i.try_into().map_err(|_| OError::Internal(
                format!("valor {i} fuera de rango para columna int4"),
            ))?;
            q.bind(i32v)
        }
        FieldType::Float => q.bind(v.as_float()?),
        FieldType::Monetary => q.bind(v.as_decimal()?),
        FieldType::Char | FieldType::Text | FieldType::Html | FieldType::Selection => {
            q.bind(v.as_str()?.to_string())
        }
        FieldType::Date => match v {
            OVal::Date(d) => q.bind(*d),
            OVal::Str(s) => q.bind(
                NaiveDate::parse_from_str(s, "%Y-%m-%d")
                    .map_err(|e| OError::Internal(format!("fecha inválida '{s}': {e}")))?,
            ),
            other => return Err(OError::Type { expected: "Date", got: other.type_name() }),
        },
        FieldType::Datetime => match v {
            OVal::DateTime(dt) => q.bind(*dt),
            OVal::Str(s) => q.bind(
                NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                    .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S"))
                    .map_err(|e| OError::Internal(format!("datetime inválido '{s}': {e}")))?,
            ),
            other => return Err(OError::Type { expected: "DateTime", got: other.type_name() }),
        },
        FieldType::Json => q.bind(v.to_json()),
        FieldType::Binary => {
            return Err(OError::Internal(
                "campos Binary aún no soportados en SQL (v0)".into(),
            ))
        }
        FieldType::One2many { .. } | FieldType::Many2many { .. } => {
            return Err(OError::Internal(format!(
                "los campos x2many no son columnas enlazables (tipo {ftype:?})"
            )))
        }
    })
}

/// Decodifica la columna `idx` de una fila a `OVal` según el campo.
pub fn decode_field(
    row: &PgRow,
    idx: usize,
    fdef: &FieldDef,
    registry: &Registry,
) -> OResult<OVal> {
    let v = match &fdef.ftype {
        FieldType::Boolean => row
            .try_get::<Option<bool>, _>(idx)?
            .map(OVal::Bool)
            .unwrap_or(OVal::Null),
        FieldType::Integer => decode_int(row, idx)?,
        FieldType::Float => row
            .try_get::<Option<f64>, _>(idx)?
            .map(OVal::Float)
            .unwrap_or(OVal::Null),
        FieldType::Monetary => row
            .try_get::<Option<rust_decimal::Decimal>, _>(idx)?
            .map(OVal::Decimal)
            .unwrap_or(OVal::Null),
        FieldType::Char | FieldType::Text | FieldType::Html | FieldType::Selection => row
            .try_get::<Option<String>, _>(idx)?
            .map(|s| OVal::Str(s.into()))
            .unwrap_or(OVal::Null),
        FieldType::Date => row
            .try_get::<Option<NaiveDate>, _>(idx)?
            .map(OVal::Date)
            .unwrap_or(OVal::Null),
        FieldType::Datetime => row
            .try_get::<Option<NaiveDateTime>, _>(idx)?
            .map(OVal::DateTime)
            .unwrap_or(OVal::Null),
        FieldType::Json => row
            .try_get::<Option<serde_json::Value>, _>(idx)?
            .map(OVal::Json)
            .unwrap_or(OVal::Null),
        FieldType::Many2one { comodel } => match decode_int(row, idx)? {
            OVal::Int(id) => match registry.model_id(comodel) {
                Ok(mid) => OVal::Ref(mid, id),
                // Comodelo aún no migrado al Registry: degrada a Int.
                Err(_) => OVal::Int(id),
            },
            other => other, // Null
        },
        FieldType::Binary => OVal::Null, // v0: no se materializan binarios
        FieldType::One2many { .. } | FieldType::Many2many { .. } => {
            return Err(OError::Internal(format!(
                "decode_field no aplica a x2many ('{}')",
                fdef.name
            )))
        }
    };
    Ok(v)
}

/// int4 con degradación a int8 (algunos ids/contadores son bigint).
fn decode_int(row: &PgRow, idx: usize) -> OResult<OVal> {
    match row.try_get::<Option<i32>, _>(idx) {
        Ok(v) => Ok(v.map(|i| OVal::Int(i as i64)).unwrap_or(OVal::Null)),
        Err(_) => Ok(row
            .try_get::<Option<i64>, _>(idx)?
            .map(OVal::Int)
            .unwrap_or(OVal::Null)),
    }
}
