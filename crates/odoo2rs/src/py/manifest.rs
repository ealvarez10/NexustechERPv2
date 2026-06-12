//! Parseo de `__manifest__.py` — un archivo cuyo cuerpo es un dict literal.

use anyhow::{anyhow, Result};
use rustpython_parser::ast::{self, Expr, Stmt};
use rustpython_parser::Parse;

use crate::ir::ManifestIr;
use crate::py::lit::{lit_str, lit_str_list};

pub fn parse_manifest(source: &str, path: &str) -> Result<ManifestIr> {
    let suite = ast::Suite::parse(source, path)
        .map_err(|e| anyhow!("{path}: error de sintaxis Python: {e}"))?;

    let dict = suite
        .iter()
        .find_map(|s| match s {
            Stmt::Expr(e) => match e.value.as_ref() {
                Expr::Dict(d) => Some(d),
                _ => None,
            },
            _ => None,
        })
        .ok_or_else(|| anyhow!("{path}: no contiene un dict literal de manifiesto"))?;

    let mut m = ManifestIr::default();
    for (k, v) in dict.keys.iter().zip(dict.values.iter()) {
        let Some(key) = k.as_ref().and_then(lit_str) else {
            continue; // claves no-literales (**spread) no aplican a manifiestos
        };
        match key.as_str() {
            "name" => m.name = lit_str(v),
            "version" => m.version = lit_str(v),
            "depends" => m.depends = lit_str_list(v),
            "data" => m.data = lit_str_list(v),
            _ => {}
        }
    }
    Ok(m)
}
