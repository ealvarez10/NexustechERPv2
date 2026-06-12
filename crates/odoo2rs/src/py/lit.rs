//! Evaluación de literales del AST de Python — el subconjunto «constante»
//! que el transpilador puede materializar en el IR sin ejecutar nada.
//! Todo lo que no sea literal (lambdas, refs, f-strings, comprehensions)
//! devuelve `None` y el llamador decide si es un aviso o un descarte.

use rustpython_parser::ast::{Constant, Expr};

/// Literal escalar → JSON (`str`, `bool`, `int`, `float`, `None`,
/// y `-n` vía UnaryOp).
pub fn lit_json(expr: &Expr) -> Option<serde_json::Value> {
    match expr {
        Expr::Constant(c) => match &c.value {
            Constant::Str(s) => Some(serde_json::Value::String(s.clone())),
            Constant::Bool(b) => Some(serde_json::Value::Bool(*b)),
            Constant::None => Some(serde_json::Value::Null),
            Constant::Int(i) => {
                let s = i.to_string();
                s.parse::<i64>().ok().map(serde_json::Value::from)
            }
            Constant::Float(f) => serde_json::Number::from_f64(*f).map(serde_json::Value::Number),
            _ => None,
        },
        Expr::UnaryOp(u) if matches!(u.op, rustpython_parser::ast::UnaryOp::USub) => {
            match lit_json(&u.operand)? {
                serde_json::Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        Some(serde_json::Value::from(-i))
                    } else {
                        n.as_f64()
                            .and_then(serde_json::Number::from_f64)
                            .map(serde_json::Value::Number)
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

pub fn lit_str(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Constant(c) => match &c.value {
            Constant::Str(s) => Some(s.clone()),
            _ => None,
        },
        _ => None,
    }
}

pub fn lit_bool(expr: &Expr) -> Option<bool> {
    match expr {
        Expr::Constant(c) => match &c.value {
            Constant::Bool(b) => Some(*b),
            _ => None,
        },
        _ => None,
    }
}

/// `'a'` o `['a', 'b']` → vec de strings (forma de `_inherit`).
pub fn lit_str_or_list(expr: &Expr) -> Vec<String> {
    if let Some(s) = lit_str(expr) {
        return vec![s];
    }
    lit_str_list(expr)
}

/// `['a', 'b']` / `('a', 'b')` → vec de strings.
pub fn lit_str_list(expr: &Expr) -> Vec<String> {
    let elts = match expr {
        Expr::List(l) => &l.elts,
        Expr::Tuple(t) => &t.elts,
        _ => return Vec::new(),
    };
    elts.iter().filter_map(lit_str).collect()
}

/// `[('draft', 'Borrador'), ...]` → pares (valor, etiqueta) de Selection.
/// `None` si la expresión no es una lista de 2-tuplas literales (p. ej.
/// selection dinámica vía método).
pub fn lit_pairs(expr: &Expr) -> Option<Vec<(String, String)>> {
    let elts = match expr {
        Expr::List(l) => &l.elts,
        Expr::Tuple(t) => &t.elts,
        _ => return None,
    };
    let mut out = Vec::with_capacity(elts.len());
    for e in elts {
        let pair = match e {
            Expr::Tuple(t) => &t.elts,
            Expr::List(l) => &l.elts,
            _ => return None,
        };
        if pair.len() != 2 {
            return None;
        }
        out.push((lit_str(&pair[0])?, lit_str(&pair[1])?));
    }
    Some(out)
}

/// Nombre punteado de una expresión: `api.depends` → `"api.depends"`.
pub fn dotted(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Name(n) => Some(n.id.to_string()),
        Expr::Attribute(a) => Some(format!("{}.{}", dotted(&a.value)?, a.attr.as_str())),
        _ => None,
    }
}
