//! `ModelIr` → fragmento Rust registrable en `nexus_orm::RegistryBuilder`.
//!
//! La parte declarativa (campos, `_order`, selections, computes) se genera
//! completa; los métodos salen como stubs que devuelven
//! `OError::Internal("pendiente de transpilar…")` con la referencia al .py
//! de origen — el contrato de la FASE 3a es ir reemplazando esos stubs por
//! traducciones (o ruteándolos a nexus-pyvm en FASE 3b).

use std::fmt::Write as _;

use crate::ir::{FieldIr, ModelIr};

/// Palabras reservadas de Rust que un identificador Python podría pisar.
const RUST_KEYWORDS: &[&str] = &[
    "as", "box", "break", "const", "continue", "crate", "dyn", "else", "enum",
    "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match",
    "mod", "move", "mut", "pub", "ref", "return", "self", "static", "struct",
    "super", "trait", "true", "type", "unsafe", "use", "where", "while",
    "async", "await", "yield", "try",
];

fn rust_ident(name: &str) -> String {
    if RUST_KEYWORDS.contains(&name) {
        format!("{name}_")
    } else {
        name.to_string()
    }
}

/// `sale.order` → `SaleOrder`.
fn camel(s: &str) -> String {
    s.split(['.', '_'])
        .filter(|p| !p.is_empty())
        .map(|p| {
            let mut cs = p.chars();
            match cs.next() {
                Some(f) => f.to_uppercase().collect::<String>() + cs.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

/// Nombre del struct del fragmento generado.
pub fn struct_name(ir: &ModelIr) -> String {
    let ext = if ir.inherit { "Ext" } else { "" };
    format!("{}{}Fragment", camel(&ir.model), ext)
}

/// Literal Rust de un string (escapado vía `{:?}`).
fn s(v: &str) -> String {
    format!("{v:?}")
}

/// Expresión constructora de `FieldDef` para un campo del IR.
/// `None` ⇒ el campo no es generable (se emite un comentario TODO).
fn field_expr(f: &FieldIr) -> Option<String> {
    let name = &f.name;
    let mut e = match f.ftype.as_str() {
        "boolean" => format!("FieldDef::boolean({})", s(name)),
        "integer" => format!("FieldDef::integer({})", s(name)),
        "float" => format!("FieldDef::float({})", s(name)),
        "monetary" => format!("FieldDef::monetary({})", s(name)),
        "char" => format!("FieldDef::char({})", s(name)),
        "text" => format!("FieldDef::text({})", s(name)),
        "html" => format!("FieldDef::html({})", s(name)),
        "date" => format!("FieldDef::date({})", s(name)),
        "datetime" => format!("FieldDef::datetime({})", s(name)),
        "json" => format!("FieldDef::json({})", s(name)),
        "binary" => format!("FieldDef::new({}, FieldType::Binary)", s(name)),
        "selection" => {
            let pairs = f
                .selection
                .iter()
                .map(|(v, l)| format!("({}, {})", s(v), s(l)))
                .collect::<Vec<_>>()
                .join(", ");
            format!("FieldDef::selection({}, &[{pairs}])", s(name))
        }
        "many2one" => format!("FieldDef::many2one({}, {})", s(name), s(f.comodel.as_ref()?)),
        "one2many" => format!(
            "FieldDef::one2many({}, {}, {})",
            s(name),
            s(f.comodel.as_ref()?),
            s(f.inverse.as_ref()?)
        ),
        "many2many" => format!(
            "FieldDef::many2many({}, {})",
            s(name),
            s(f.comodel.as_ref()?)
        ),
        _ => return None,
    };

    if let Some(label) = &f.string {
        if label != name {
            write!(e, ".string({})", s(label)).unwrap();
        }
    }
    if f.required {
        e.push_str(".required()");
    }
    if f.readonly {
        e.push_str(".readonly()");
    }
    if let Some(method) = &f.compute {
        let deps = f
            .depends
            .iter()
            .map(|d| s(d))
            .collect::<Vec<_>>()
            .join(", ");
        write!(e, ".computed({}, &[{deps}])", s(method)).unwrap();
        if f.store {
            e.push_str(".stored()");
        }
    } else if !f.store && !matches!(f.ftype.as_str(), "one2many" | "many2many") {
        // store=False explícito sobre un campo normal: sin builder; se
        // resuelve mutando el struct (FieldDef.store es pub).
        return Some(format!(
            "{{ let mut f = {e}; f.store = false; f }}"
        ));
    }

    if let Some(d) = &f.default {
        let lit = match d {
            serde_json::Value::Bool(b) => Some(b.to_string()),
            serde_json::Value::String(v) => Some(s(v)),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Some(format!("{i}i64"))
                } else {
                    n.as_f64().map(|x| format!("{x}f64"))
                }
            }
            _ => None,
        };
        if let Some(lit) = lit {
            write!(e, ".default_val({lit})").unwrap();
        }
    }

    if let Some(rel) = &f.related {
        // FieldDef no tiene builder para related=; campo pub.
        return Some(format!(
            "{{ let mut f = {e}; f.related = Some({}.into()); f }}",
            s(rel)
        ));
    }
    Some(e)
}

/// Genera el módulo Rust completo de un fragmento.
pub fn fragment_rs(ir: &ModelIr, origin: &str) -> String {
    let st = struct_name(ir);
    let mut out = String::new();

    let _ = writeln!(
        out,
        "//! Generado por odoo2rs desde `{origin}` — NO EDITAR A MANO;\n\
         //! regenerar con `odoo2rs gen-rust`.\n\
         //! Modelo: `{}`{}\n",
        ir.model,
        if ir.inherit { " (fragmento _inherit)" } else { "" }
    );
    out.push_str("use nexus_orm::prelude::*;\n\n");
    let _ = writeln!(out, "pub struct {st};\n");
    let _ = writeln!(out, "#[async_trait]\nimpl ModelFragment for {st} {{");
    let _ = writeln!(out, "    fn model_name(&self) -> &str {{\n        {}\n    }}\n", s(&ir.model));
    if let Some(m) = &ir.module {
        let _ = writeln!(out, "    fn module(&self) -> &str {{\n        {}\n    }}\n", s(m));
    }
    if ir.inherit {
        out.push_str("    fn is_extension(&self) -> bool {\n        true\n    }\n\n");
    }

    // build(): atributos + campos.
    out.push_str("    fn build(&self, def: &mut ModelDef) {\n");
    if let Some(t) = &ir.table {
        let _ = writeln!(out, "        def.table = {}.into();", s(t));
    }
    if let Some(d) = &ir.description {
        let _ = writeln!(out, "        def.description = {}.into();", s(d));
    }
    if let Some(o) = &ir.order {
        let _ = writeln!(out, "        def.order = {}.into();", s(o));
    }
    if let Some(r) = &ir.rec_name {
        let _ = writeln!(out, "        def.rec_name = {}.into();", s(r));
    }
    for f in &ir.fields {
        match field_expr(f) {
            Some(e) => {
                let _ = writeln!(out, "        def.add_field({e});");
            }
            None => {
                let _ = writeln!(
                    out,
                    "        // TODO(odoo2rs): campo '{}' ({}) no generable — \
                     falta comodel/inverse o tipo sin equivalente.",
                    f.name, f.ftype
                );
            }
        }
    }
    out.push_str("    }\n");

    // Métodos: vtable + dispatch + stubs.
    if !ir.methods.is_empty() {
        let names = ir
            .methods
            .iter()
            .map(|m| s(&m.name))
            .collect::<Vec<_>>()
            .join(", ");
        let _ = writeln!(
            out,
            "\n    fn methods(&self) -> Vec<&str> {{\n        vec![{names}]\n    }}\n"
        );
        out.push_str(
            "    async fn call(\n        &self,\n        env: &Env,\n        ctx: &CallCtx,\n        rs: &Recordset,\n        args: &[OVal],\n    ) -> OResult<OVal> {\n        match ctx.method() {\n",
        );
        for m in &ir.methods {
            let _ = writeln!(
                out,
                "            {} => self.{}(env, ctx, rs, args).await,",
                s(&m.name),
                rust_ident(&m.name)
            );
        }
        out.push_str(
            "            other => Err(OError::Internal(format!(\n                \"método '{other}' no implementado en este fragmento\"\n            ))),\n        }\n    }\n",
        );
    }
    out.push_str("}\n");

    if !ir.methods.is_empty() {
        let _ = writeln!(out, "\nimpl {st} {{");
        for m in &ir.methods {
            let decos = if m.decorators.is_empty() {
                String::new()
            } else {
                format!(" Decoradores: {}.", m.decorators.join(", "))
            };
            let _ = writeln!(
                out,
                "    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original \
                 (`{origin}:{}`).{decos}",
                m.line
            );
            let _ = writeln!(
                out,
                "    async fn {}(\n        &self,\n        _env: &Env,\n        _ctx: &CallCtx,\n        _rs: &Recordset,\n        _args: &[OVal],\n    ) -> OResult<OVal> {{\n        Err(OError::Internal(\n            \"pendiente de transpilar (FASE 3): {}.{}\".into(),\n        ))\n    }}\n",
                rust_ident(&m.name),
                ir.model,
                m.name
            );
        }
        out.push_str("}\n");
    }
    out
}
