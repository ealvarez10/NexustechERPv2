//! Dominios de Odoo → SQL parametrizado (§3.3 del reporte).
//!
//! Un dominio `[('state','!=','cancel'), '|', ('a','=',1), ('b','>',2)]`
//! es notación prefija con AND implícito. Este módulo lo parsea desde su
//! forma JSON (como lo emite el transpilador o lo manda la API estilo
//! `call_kw`) y lo compila a una cláusula `WHERE` con placeholders `$n`.
//!
//! El `listar()` manual de `nexus-core/src/db/sale_order.rs` (Vec de
//! condiciones + índices incrementales) es exactamente el output de este
//! compilador, escrito a mano — aquí se mecaniza.

use std::collections::VecDeque;

use crate::error::{OError, OResult};
use crate::fields::FieldType;
use crate::model::ModelDef;
use crate::value::OVal;

/// Operadores de término de dominio.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainOp {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    /// `like` de Odoo: envuelve el patrón en `%...%`.
    Like,
    ILike,
    NotLike,
    NotILike,
    /// `=like` / `=ilike`: patrón crudo, sin envolver.
    EqLike,
    EqILike,
    In,
    NotIn,
}

impl DomainOp {
    pub fn parse(s: &str) -> OResult<Self> {
        Ok(match s {
            "=" | "==" => DomainOp::Eq,
            "!=" | "<>" => DomainOp::Ne,
            ">" => DomainOp::Gt,
            ">=" => DomainOp::Ge,
            "<" => DomainOp::Lt,
            "<=" => DomainOp::Le,
            "like" => DomainOp::Like,
            "ilike" => DomainOp::ILike,
            "not like" => DomainOp::NotLike,
            "not ilike" => DomainOp::NotILike,
            "=like" => DomainOp::EqLike,
            "=ilike" => DomainOp::EqILike,
            "in" => DomainOp::In,
            "not in" => DomainOp::NotIn,
            other => return Err(OError::Domain(format!("operador desconocido: '{other}'"))),
        })
    }

    fn sql(self) -> &'static str {
        match self {
            DomainOp::Eq => "=",
            DomainOp::Ne => "<>",
            DomainOp::Gt => ">",
            DomainOp::Ge => ">=",
            DomainOp::Lt => "<",
            DomainOp::Le => "<=",
            DomainOp::Like | DomainOp::EqLike => "LIKE",
            DomainOp::ILike | DomainOp::EqILike => "ILIKE",
            DomainOp::NotLike => "NOT LIKE",
            DomainOp::NotILike => "NOT ILIKE",
            DomainOp::In => "IN",
            DomainOp::NotIn => "NOT IN",
        }
    }
}

/// Valor del lado derecho de un término.
#[derive(Debug, Clone, PartialEq)]
pub enum LeafValue {
    One(OVal),
    Many(Vec<OVal>),
}

/// AST normalizado de un dominio.
#[derive(Debug, Clone, PartialEq)]
pub enum Domain {
    Leaf {
        field: String,
        op: DomainOp,
        value: LeafValue,
    },
    And(Vec<Domain>),
    Or(Vec<Domain>),
    Not(Box<Domain>),
}

/// Resultado de compilar un dominio: SQL + parámetros tipados a enlazar.
#[derive(Debug)]
pub struct SqlWhere {
    pub sql: String,
    /// `(tipo de columna, valor)` por cada `$n`, en orden.
    pub params: Vec<(FieldType, OVal)>,
}

enum Tok {
    And,
    Or,
    Not,
    Leaf(Domain),
}

impl Domain {
    /// Dominio vacío (siempre verdadero), como `[]` en Odoo.
    pub fn all() -> Domain {
        Domain::And(Vec::new())
    }

    pub fn leaf(field: &str, op: &str, value: impl Into<OVal>) -> OResult<Domain> {
        Ok(Domain::Leaf {
            field: field.to_string(),
            op: DomainOp::parse(op)?,
            value: LeafValue::One(value.into()),
        })
    }

    pub fn leaf_in(field: &str, op: &str, values: Vec<OVal>) -> OResult<Domain> {
        Ok(Domain::Leaf {
            field: field.to_string(),
            op: DomainOp::parse(op)?,
            value: LeafValue::Many(values),
        })
    }

    /// Parsea la representación JSON de un dominio Odoo:
    /// `[["state","=","sale"],"|",["a","!=",null],["b",">",100]]`
    pub fn parse_json(src: &str) -> OResult<Domain> {
        let v: serde_json::Value = serde_json::from_str(src)
            .map_err(|e| OError::Domain(format!("JSON inválido: {e}")))?;
        Self::from_json(&v)
    }

    pub fn from_json(v: &serde_json::Value) -> OResult<Domain> {
        let arr = v
            .as_array()
            .ok_or_else(|| OError::Domain("un dominio debe ser una lista".into()))?;

        let mut toks: VecDeque<Tok> = VecDeque::with_capacity(arr.len());
        for item in arr {
            toks.push_back(Self::parse_token(item)?);
        }

        // Notación prefija con AND implícito entre términos sobrantes.
        let mut parts = Vec::new();
        while !toks.is_empty() {
            parts.push(Self::consume(&mut toks)?);
        }
        Ok(match parts.len() {
            0 => Domain::all(),
            1 => parts.pop().unwrap(),
            _ => Domain::And(parts),
        })
    }

    fn parse_token(v: &serde_json::Value) -> OResult<Tok> {
        if let Some(s) = v.as_str() {
            return Ok(match s {
                "&" => Tok::And,
                "|" => Tok::Or,
                "!" => Tok::Not,
                other => {
                    return Err(OError::Domain(format!(
                        "operador lógico desconocido: '{other}'"
                    )))
                }
            });
        }
        let term = v.as_array().ok_or_else(|| {
            OError::Domain(format!("término inválido (se esperaba lista): {v}"))
        })?;
        if term.len() != 3 {
            return Err(OError::Domain(format!(
                "un término tiene 3 elementos [campo, op, valor]: {v}"
            )));
        }
        let field = term[0]
            .as_str()
            .ok_or_else(|| OError::Domain("el campo del término debe ser texto".into()))?;
        let op = DomainOp::parse(
            term[1]
                .as_str()
                .ok_or_else(|| OError::Domain("el operador del término debe ser texto".into()))?,
        )?;
        let value = match &term[2] {
            serde_json::Value::Array(items) => {
                LeafValue::Many(items.iter().map(OVal::from_json).collect())
            }
            other => LeafValue::One(OVal::from_json(other)),
        };
        Ok(Tok::Leaf(Domain::Leaf {
            field: field.to_string(),
            op,
            value,
        }))
    }

    fn consume(toks: &mut VecDeque<Tok>) -> OResult<Domain> {
        match toks.pop_front() {
            Some(Tok::And) => Ok(Domain::And(vec![
                Self::consume(toks)?,
                Self::consume(toks)?,
            ])),
            Some(Tok::Or) => Ok(Domain::Or(vec![
                Self::consume(toks)?,
                Self::consume(toks)?,
            ])),
            Some(Tok::Not) => Ok(Domain::Not(Box::new(Self::consume(toks)?))),
            Some(Tok::Leaf(d)) => Ok(d),
            None => Err(OError::Domain(
                "dominio incompleto: operador sin operandos".into(),
            )),
        }
    }

    /// Compila a SQL contra la definición del modelo. `first_param` es el
    /// índice del primer placeholder (`1` → `$1`), para componer con otras
    /// condiciones ya existentes en la consulta.
    pub fn to_sql(&self, def: &ModelDef, first_param: usize) -> OResult<SqlWhere> {
        let mut params = Vec::new();
        let mut n = first_param;
        let sql = self.emit(def, &mut params, &mut n)?;
        Ok(SqlWhere { sql, params })
    }

    fn emit(
        &self,
        def: &ModelDef,
        params: &mut Vec<(FieldType, OVal)>,
        n: &mut usize,
    ) -> OResult<String> {
        match self {
            Domain::And(children) if children.is_empty() => Ok("true".into()),
            Domain::Or(children) if children.is_empty() => Ok("false".into()),
            Domain::And(children) => {
                let parts: Vec<String> = children
                    .iter()
                    .map(|c| c.emit(def, params, n))
                    .collect::<OResult<_>>()?;
                Ok(format!("({})", parts.join(" AND ")))
            }
            Domain::Or(children) => {
                let parts: Vec<String> = children
                    .iter()
                    .map(|c| c.emit(def, params, n))
                    .collect::<OResult<_>>()?;
                Ok(format!("({})", parts.join(" OR ")))
            }
            Domain::Not(inner) => Ok(format!("NOT ({})", inner.emit(def, params, n)?)),
            Domain::Leaf { field, op, value } => Self::emit_leaf(def, field, *op, value, params, n),
        }
    }

    fn emit_leaf(
        def: &ModelDef,
        field: &str,
        op: DomainOp,
        value: &LeafValue,
        params: &mut Vec<(FieldType, OVal)>,
        n: &mut usize,
    ) -> OResult<String> {
        if field.contains('.') {
            return Err(OError::Domain(format!(
                "rutas con punto aún no soportadas en dominios: '{field}' (v0)"
            )));
        }
        let fdef = def.field(field)?;
        if !fdef.is_column() {
            return Err(OError::Domain(format!(
                "el campo '{}.{}' no es una columna almacenada (v0 no busca sobre computados/x2many)",
                def.name, field
            )));
        }
        let col = &fdef.name;

        match (op, value) {
            // IN / NOT IN — lista expandida; lista vacía colapsa a constante.
            (DomainOp::In, LeafValue::Many(vals)) | (DomainOp::NotIn, LeafValue::Many(vals)) => {
                if vals.is_empty() {
                    return Ok(if op == DomainOp::In { "false" } else { "true" }.into());
                }
                let mut holes = Vec::with_capacity(vals.len());
                for v in vals {
                    holes.push(format!("${n}"));
                    params.push((fdef.ftype.clone(), v.clone()));
                    *n += 1;
                }
                Ok(format!("{col} {} ({})", op.sql(), holes.join(", ")))
            }
            (DomainOp::In | DomainOp::NotIn, LeafValue::One(_)) => Err(OError::Domain(format!(
                "'{field} in ...' requiere una lista de valores"
            ))),

            (_, LeafValue::Many(_)) => Err(OError::Domain(format!(
                "el operador {:?} no acepta listas (campo '{field}')",
                op
            ))),

            // NULL / False de Odoo
            (DomainOp::Eq, LeafValue::One(OVal::Null)) => Ok(format!("{col} IS NULL")),
            (DomainOp::Ne, LeafValue::One(OVal::Null)) => Ok(format!("{col} IS NOT NULL")),

            // ('active','=',False): en datos Odoo un bool NULL significa False.
            (DomainOp::Eq, LeafValue::One(OVal::Bool(false)))
                if fdef.ftype == FieldType::Boolean =>
            {
                Ok(format!("({col} IS NULL OR {col} = false)"))
            }
            (DomainOp::Ne, LeafValue::One(OVal::Bool(false)))
                if fdef.ftype == FieldType::Boolean =>
            {
                Ok(format!("{col} = true"))
            }

            // like / ilike — envolver en %...%; =like / =ilike — patrón crudo.
            (
                DomainOp::Like | DomainOp::ILike | DomainOp::NotLike | DomainOp::NotILike,
                LeafValue::One(v),
            ) => {
                let pattern = format!("%{}%", v.as_str()?);
                let sql = format!("{col} {} ${n}", op.sql());
                params.push((FieldType::Char, OVal::Str(pattern.into())));
                *n += 1;
                Ok(sql)
            }
            (DomainOp::EqLike | DomainOp::EqILike, LeafValue::One(v)) => {
                let sql = format!("{col} {} ${n}", op.sql());
                params.push((FieldType::Char, OVal::Str(v.as_str()?)));
                *n += 1;
                Ok(sql)
            }

            // Comparación simple parametrizada.
            (_, LeafValue::One(v)) => {
                let sql = format!("{col} {} ${n}", op.sql());
                params.push((fdef.ftype.clone(), v.clone()));
                *n += 1;
                Ok(sql)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::FieldDef;

    fn modelo_prueba() -> ModelDef {
        let mut def = ModelDef::new("x.order");
        def.add_field(FieldDef::selection(
            "state",
            &[("draft", "Borrador"), ("sale", "Confirmada")],
        ));
        def.add_field(FieldDef::many2one("partner_id", "res.partner"));
        def.add_field(FieldDef::monetary("amount_total"));
        def.add_field(FieldDef::boolean("active"));
        def.add_field(FieldDef::char("name"));
        def
    }

    #[test]
    fn and_implicito_con_or_prefijo() {
        let d = Domain::parse_json(
            r#"[["state","=","sale"],"|",["partner_id","!=",null],["amount_total",">",100]]"#,
        )
        .unwrap();
        let w = d.to_sql(&modelo_prueba(), 1).unwrap();
        assert_eq!(
            w.sql,
            "(state = $1 AND (partner_id IS NOT NULL OR amount_total > $2))"
        );
        assert_eq!(w.params.len(), 2);
        assert_eq!(w.params[0].1, OVal::Str("sale".into()));
        assert_eq!(w.params[1].1, OVal::Int(100));
    }

    #[test]
    fn in_expandido_y_vacio() {
        let d = Domain::parse_json(r#"[["state","in",["draft","sent"]]]"#).unwrap();
        let w = d.to_sql(&modelo_prueba(), 3).unwrap();
        assert_eq!(w.sql, "state IN ($3, $4)");

        let vacio = Domain::parse_json(r#"[["state","in",[]]]"#).unwrap();
        assert_eq!(vacio.to_sql(&modelo_prueba(), 1).unwrap().sql, "false");
    }

    #[test]
    fn bool_false_estilo_odoo() {
        let d = Domain::parse_json(r#"[["active","=",false]]"#).unwrap();
        let w = d.to_sql(&modelo_prueba(), 1).unwrap();
        assert_eq!(w.sql, "(active IS NULL OR active = false)");
        assert!(w.params.is_empty());
    }

    #[test]
    fn ilike_envuelve_patron() {
        let d = Domain::parse_json(r#"[["name","ilike","barcode"]]"#).unwrap();
        let w = d.to_sql(&modelo_prueba(), 1).unwrap();
        assert_eq!(w.sql, "name ILIKE $1");
        assert_eq!(w.params[0].1, OVal::Str("%barcode%".into()));
    }

    #[test]
    fn not_y_dominio_vacio() {
        let d = Domain::parse_json(r#"["!",["state","=","cancel"]]"#).unwrap();
        let w = d.to_sql(&modelo_prueba(), 1).unwrap();
        assert_eq!(w.sql, "NOT (state = $1)");

        assert_eq!(
            Domain::parse_json("[]").unwrap().to_sql(&modelo_prueba(), 1).unwrap().sql,
            "true"
        );
    }

    #[test]
    fn campo_desconocido_es_error() {
        let d = Domain::parse_json(r#"[["nope","=",1]]"#).unwrap();
        assert!(d.to_sql(&modelo_prueba(), 1).is_err());
    }
}
