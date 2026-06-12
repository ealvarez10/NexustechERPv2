//! El visitor del AST de Python: clases `models.Model` → `ModelIr`.
//!
//! Extrae el subconjunto declarativo de un archivo de modelos de Odoo:
//! `_name`/`_inherit`/`_order`/…, las declaraciones `fields.*(...)` y las
//! firmas de métodos con sus decoradores (`@api.depends` se cablea al
//! campo compute correspondiente). Lo no-literal (defaults callables,
//! selections dinámicas, `**kwargs`) se reporta como aviso y sigue — la
//! filosofía FASE 2: extraer el 100 % de lo declarativo, nunca abortar
//! por lo imperativo.

use anyhow::{anyhow, Result};
use rustpython_parser::ast::{self, Expr, Ranged, Stmt};
use rustpython_parser::Parse;

use crate::ir::{FieldIr, MethodIr, ModelIr};
use crate::py::lit::{dotted, lit_bool, lit_json, lit_pairs, lit_str, lit_str_or_list};

/// Resultado de extraer un archivo: fragmentos de modelo + avisos no fatales.
#[derive(Debug, Default)]
pub struct Extraction {
    pub models: Vec<ModelIr>,
    pub warnings: Vec<String>,
}

/// Índice de líneas para convertir offsets del AST a líneas 1-based.
struct LineIndex {
    starts: Vec<usize>,
}

impl LineIndex {
    fn new(src: &str) -> Self {
        let mut starts = vec![0];
        for (i, b) in src.bytes().enumerate() {
            if b == b'\n' {
                starts.push(i + 1);
            }
        }
        LineIndex { starts }
    }

    fn line_of(&self, offset: usize) -> usize {
        self.starts.partition_point(|&s| s <= offset)
    }
}

/// Punto de entrada: parsea `source` y extrae todos los modelos Odoo.
pub fn extract_models(source: &str, path: &str, module: Option<&str>) -> Result<Extraction> {
    let suite = ast::Suite::parse(source, path)
        .map_err(|e| anyhow!("{path}: error de sintaxis Python: {e}"))?;
    let lines = LineIndex::new(source);
    let mut ex = Extraction::default();
    for stmt in &suite {
        if let Stmt::ClassDef(cls) = stmt {
            extract_class(cls, module, path, &lines, &mut ex);
        }
    }
    Ok(ex)
}

/// ¿La clase hereda de `models.Model` / `TransientModel` / `AbstractModel`?
fn is_models_base(expr: &Expr) -> bool {
    matches!(
        dotted(expr).as_deref(),
        Some("models.Model")
            | Some("models.TransientModel")
            | Some("models.AbstractModel")
            | Some("odoo.models.Model")
            | Some("Model")
            | Some("TransientModel")
            | Some("AbstractModel")
    )
}

fn extract_class(
    cls: &ast::StmtClassDef,
    module: Option<&str>,
    path: &str,
    lines: &LineIndex,
    ex: &mut Extraction,
) {
    let odoo_base = cls.bases.iter().any(is_models_base);

    let mut name: Option<String> = None;
    let mut inherits: Vec<String> = Vec::new();
    let mut table = None;
    let mut description = None;
    let mut order = None;
    let mut rec_name = None;
    let mut fields: Vec<FieldIr> = Vec::new();
    let mut methods: Vec<MethodIr> = Vec::new();

    for stmt in &cls.body {
        match stmt {
            Stmt::Assign(a) => {
                let [Expr::Name(target)] = &a.targets[..] else {
                    continue; // asignación múltiple/destructurada: no declarativa
                };
                let key = target.id.as_str();
                match key {
                    "_name" => name = lit_str(&a.value),
                    "_inherit" => inherits = lit_str_or_list(&a.value),
                    "_inherits" => ex.warnings.push(format!(
                        "{path}: clase '{}': `_inherits` (herencia por delegación) \
                         no soportado en kernel v0 — ignorado",
                        cls.name
                    )),
                    "_table" => table = lit_str(&a.value),
                    "_description" => description = lit_str(&a.value),
                    "_order" => order = lit_str(&a.value),
                    "_rec_name" => rec_name = lit_str(&a.value),
                    _ => {
                        if let Expr::Call(call) = a.value.as_ref() {
                            if let Some(pytype) = fields_call_type(&call.func) {
                                if let Some(f) =
                                    field_ir(key, &pytype, call, &cls.name, path, ex)
                                {
                                    fields.push(f);
                                }
                            }
                        }
                    }
                }
            }
            Stmt::FunctionDef(f) => methods.push(method_ir(
                f.name.as_str(),
                &f.args,
                &f.decorator_list,
                lines.line_of(usize::from(f.range().start())),
            )),
            Stmt::AsyncFunctionDef(f) => methods.push(method_ir(
                f.name.as_str(),
                &f.args,
                &f.decorator_list,
                lines.line_of(usize::from(f.range().start())),
            )),
            _ => {}
        }
    }

    // Clase Python normal (helper, mixin sin registro): no es un modelo.
    if name.is_none() && inherits.is_empty() {
        if odoo_base {
            ex.warnings.push(format!(
                "{path}: clase '{}' hereda de models.* pero no declara _name ni \
                 _inherit; ignorada",
                cls.name
            ));
        }
        return;
    }

    // Semántica Odoo: sin `_name`, el fragmento extiende el primer `_inherit`.
    let inherit = name.is_none();
    let model = name.unwrap_or_else(|| inherits[0].clone());

    // `@api.depends` del método compute → `depends` del campo.
    for f in &mut fields {
        if let Some(m) = &f.compute {
            if f.depends.is_empty() {
                if let Some(mi) = methods.iter().find(|mi| &mi.name == m) {
                    f.depends = mi.depends.clone();
                }
            }
        }
    }

    let mut ir = ModelIr::new(&model);
    ir.module = module.map(str::to_string);
    ir.inherit = inherit;
    ir.inherits = inherits;
    ir.table = table;
    ir.description = description;
    ir.order = order;
    ir.rec_name = rec_name;
    ir.fields = fields;
    ir.methods = methods;
    ex.models.push(ir);
}

/// `fields.Char` / `odoo.fields.Char` → `Some("Char")`.
fn fields_call_type(func: &Expr) -> Option<String> {
    let Expr::Attribute(attr) = func else {
        return None;
    };
    match dotted(&attr.value).as_deref() {
        Some("fields") | Some("odoo.fields") => Some(attr.attr.to_string()),
        _ => None,
    }
}

/// Tipo Python de `fields.*` → nombre de tipo del IR (los que reconoce
/// `nexus_orm::ir::FieldIr::to_def`).
fn ir_type(pytype: &str) -> Option<&'static str> {
    Some(match pytype {
        "Boolean" => "boolean",
        "Integer" => "integer",
        "Float" => "float",
        "Monetary" => "monetary",
        "Char" => "char",
        "Text" => "text",
        "Html" => "html",
        "Selection" => "selection",
        "Date" => "date",
        "Datetime" => "datetime",
        "Binary" => "binary",
        "Image" => "binary", // aproximación: Image es Binary + resize
        "Json" => "json",
        "Many2one" => "many2one",
        "One2many" => "one2many",
        "Many2many" => "many2many",
        _ => return None,
    })
}

fn field_ir(
    name: &str,
    pytype: &str,
    call: &ast::ExprCall,
    class_name: &str,
    path: &str,
    ex: &mut Extraction,
) -> Option<FieldIr> {
    let Some(ftype) = ir_type(pytype) else {
        ex.warnings.push(format!(
            "{path}: {class_name}.{name}: tipo fields.{pytype} sin equivalente \
             en el kernel v0 — campo omitido"
        ));
        return None;
    };
    let mut f = FieldIr::new(name, ftype);

    // Argumentos posicionales: la convención varía por tipo de campo.
    let pos = |i: usize| call.args.get(i);
    match ftype {
        "many2one" => {
            f.comodel = pos(0).and_then(lit_str);
            f.string = pos(1).and_then(lit_str);
        }
        "one2many" => {
            f.comodel = pos(0).and_then(lit_str);
            f.inverse = pos(1).and_then(lit_str);
            f.string = pos(2).and_then(lit_str);
        }
        "many2many" => {
            f.comodel = pos(0).and_then(lit_str);
            f.relation = pos(1).and_then(lit_str);
            f.column1 = pos(2).and_then(lit_str);
            f.column2 = pos(3).and_then(lit_str);
            f.string = pos(4).and_then(lit_str);
        }
        "selection" => {
            if let Some(arg0) = pos(0) {
                match lit_pairs(arg0) {
                    Some(pairs) => f.selection = pairs,
                    None => ex.warnings.push(format!(
                        "{path}: {class_name}.{name}: selection dinámica \
                         (no-literal) — opciones vacías en el IR"
                    )),
                }
            }
            f.string = pos(1).and_then(lit_str);
        }
        _ => {
            f.string = pos(0).and_then(lit_str);
        }
    }

    // Argumentos con nombre.
    for kw in &call.keywords {
        let Some(arg) = &kw.arg else {
            ex.warnings.push(format!(
                "{path}: {class_name}.{name}: `**kwargs` en la declaración — \
                 no analizable estáticamente"
            ));
            continue;
        };
        let v = &kw.value;
        match arg.as_str() {
            "string" => f.string = lit_str(v).or(f.string),
            "comodel_name" => f.comodel = lit_str(v).or(f.comodel),
            "inverse_name" => f.inverse = lit_str(v).or(f.inverse),
            "relation" => f.relation = lit_str(v).or(f.relation),
            "column1" => f.column1 = lit_str(v).or(f.column1),
            "column2" => f.column2 = lit_str(v).or(f.column2),
            "help" => f.help = lit_str(v),
            "related" => f.related = lit_str(v),
            "compute" => match lit_str(v) {
                Some(m) => f.compute = Some(m),
                None => ex.warnings.push(format!(
                    "{path}: {class_name}.{name}: compute= no es string literal"
                )),
            },
            "required" => match lit_bool(v) {
                Some(b) => f.required = b,
                None => ex.warnings.push(format!(
                    "{path}: {class_name}.{name}: required= no-literal \
                     (¿states?) — se asume False"
                )),
            },
            "readonly" => match lit_bool(v) {
                Some(b) => f.readonly = b,
                None => ex.warnings.push(format!(
                    "{path}: {class_name}.{name}: readonly= no-literal — \
                     se asume False"
                )),
            },
            "store" => match lit_bool(v) {
                Some(b) => f.store = b,
                None => ex.warnings.push(format!(
                    "{path}: {class_name}.{name}: store= no-literal — \
                     se asume True"
                )),
            },
            "selection" => match lit_pairs(v) {
                Some(pairs) => f.selection = pairs,
                None => ex.warnings.push(format!(
                    "{path}: {class_name}.{name}: selection= dinámica — \
                     opciones vacías en el IR"
                )),
            },
            "default" => match lit_json(v) {
                Some(j) => f.default = Some(j),
                None => ex.warnings.push(format!(
                    "{path}: {class_name}.{name}: default= callable/no-literal \
                     — pendiente para FASE 3"
                )),
            },
            // Atributos reconocidos sin efecto en el IR v0.
            "index" | "copy" | "tracking" | "translate" | "digits"
            | "currency_field" | "domain" | "context" | "ondelete"
            | "group_operator" | "aggregator" | "groups" | "states"
            | "check_company" | "company_dependent" | "inverse" | "search"
            | "recursive" | "precompute" | "sanitize" | "attachment"
            | "max_width" | "max_height" | "size" | "auto_join"
            | "delegate" | "group_expand" | "default_export_compatible" => {}
            other => ex.warnings.push(format!(
                "{path}: {class_name}.{name}: atributo '{other}=' desconocido \
                 — ignorado"
            )),
        }
    }
    Some(f)
}

fn method_ir(
    name: &str,
    args: &ast::Arguments,
    decorators: &[Expr],
    line: usize,
) -> MethodIr {
    let arg_names: Vec<String> = args
        .posonlyargs
        .iter()
        .chain(args.args.iter())
        .map(|a| a.def.arg.to_string())
        .filter(|a| a != "self")
        .collect();

    let mut deco_strs = Vec::new();
    let mut depends = Vec::new();
    for d in decorators {
        match d {
            Expr::Call(c) => {
                let base = dotted(&c.func).unwrap_or_else(|| "<expr>".into());
                let str_args: Vec<String> = c.args.iter().filter_map(lit_str).collect();
                if base.ends_with("depends") {
                    depends.extend(str_args.iter().cloned());
                }
                deco_strs.push(format!(
                    "{base}({})",
                    str_args
                        .iter()
                        .map(|s| format!("'{s}'"))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            other => {
                deco_strs.push(dotted(other).unwrap_or_else(|| "<expr>".into()));
            }
        }
    }

    MethodIr {
        name: name.to_string(),
        args: arg_names,
        decorators: deco_strs,
        depends,
        line,
    }
}
