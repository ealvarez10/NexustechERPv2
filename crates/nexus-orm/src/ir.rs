//! OdooIR declarativo — el esqueleto de parseo y registro dinámico.
//!
//! La FASE 2 del transpilador `odoo2rs` emite la parte declarativa de un
//! módulo Odoo (campos, `_inherit`, `_order`, selections, computes) como
//! JSON. Este módulo lo parsea a `ModelIr` y lo convierte en un
//! `IrFragment` registrable en el `RegistryBuilder` — registro de modelos
//! 100 % dinámico, sin recompilar el kernel.
//!
//! Ejemplo de IR que el extractor produciría para un fragmento de modelo:
//!
//! ```json
//! {
//!   "model": "sale.order",
//!   "module": "sale",
//!   "description": "Orden de venta",
//!   "order": "date_order desc, id desc",
//!   "fields": [
//!     {"name": "name", "type": "char", "required": true},
//!     {"name": "partner_id", "type": "many2one", "comodel": "res.partner"},
//!     {"name": "state", "type": "selection",
//!      "selection": [["draft","Borrador"],["sale","Confirmada"]]},
//!     {"name": "amount_tax", "type": "monetary",
//!      "compute": "_compute_amounts", "depends": ["amount_untaxed"]}
//!   ]
//! }
//! ```

use serde::Deserialize;

use crate::error::{OError, OResult};
use crate::fields::{ComputeDef, FieldDef, FieldType};
use crate::model::{ModelDef, ModelFragment};

/// IR de un fragmento de modelo (una clase Python extraída).
#[derive(Debug, Clone, Deserialize)]
pub struct ModelIr {
    /// `_name` (o el modelo extendido si `inherit` es `true`).
    pub model: String,
    #[serde(default)]
    pub module: Option<String>,
    /// `true` ⇒ fragmento `_inherit` puro (extiende un modelo existente).
    #[serde(default)]
    pub inherit: bool,
    #[serde(default)]
    pub table: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub order: Option<String>,
    #[serde(default)]
    pub rec_name: Option<String>,
    #[serde(default)]
    pub fields: Vec<FieldIr>,
    #[serde(default)]
    pub methods: Vec<MethodOrString>,
}

/// Representa un método que puede venir como un string simple (en el shim de Python)
/// o como un objeto estructurado con "name" (en la extracción de odoo2rs).
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum MethodOrString {
    String(String),
    Struct {
        name: String,
    },
}

impl MethodOrString {
    pub fn as_str(&self) -> &str {
        match self {
            MethodOrString::String(s) => s.as_str(),
            MethodOrString::Struct { name } => name.as_str(),
        }
    }
}

/// IR de un campo (`fields.*` extraído del AST).
#[derive(Debug, Clone, Deserialize)]
pub struct FieldIr {
    pub name: String,
    #[serde(rename = "type")]
    pub ftype: String,
    #[serde(default)]
    pub comodel: Option<String>,
    /// Inverso de one2many.
    #[serde(default)]
    pub inverse: Option<String>,
    #[serde(default)]
    pub string: Option<String>,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub readonly: bool,
    /// `None` = no especificado (true para campos normales, false para
    /// computados — la misma convención de Odoo).
    #[serde(default)]
    pub store: Option<bool>,
    #[serde(default)]
    pub selection: Vec<(String, String)>,
    #[serde(default)]
    pub compute: Option<String>,
    #[serde(default)]
    pub depends: Vec<String>,
    #[serde(default)]
    pub related: Option<String>,
}

impl FieldIr {
    fn comodel_requerido(&self) -> OResult<&str> {
        self.comodel.as_deref().ok_or_else(|| {
            OError::Registry(format!(
                "el campo relacional '{}' (tipo {}) requiere 'comodel'",
                self.name, self.ftype
            ))
        })
    }

    pub fn to_def(&self) -> OResult<FieldDef> {
        let ftype = match self.ftype.as_str() {
            "boolean" | "bool" => FieldType::Boolean,
            "integer" | "int" => FieldType::Integer,
            "float" => FieldType::Float,
            "monetary" => FieldType::Monetary,
            "char" => FieldType::Char,
            "text" => FieldType::Text,
            "html" => FieldType::Html,
            "selection" => FieldType::Selection,
            "date" => FieldType::Date,
            "datetime" => FieldType::Datetime,
            "binary" => FieldType::Binary,
            "json" | "jsonb" => FieldType::Json,
            "many2one" => FieldType::Many2one {
                comodel: self.comodel_requerido()?.to_string(),
            },
            "one2many" => FieldType::One2many {
                comodel: self.comodel_requerido()?.to_string(),
                inverse: self.inverse.clone().ok_or_else(|| {
                    OError::Registry(format!(
                        "one2many '{}' requiere 'inverse'",
                        self.name
                    ))
                })?,
            },
            "many2many" => FieldType::Many2many {
                comodel: self.comodel_requerido()?.to_string(),
                relation: None,
                column1: None,
                column2: None,
            },
            other => {
                return Err(OError::Registry(format!(
                    "tipo de campo desconocido en IR: '{other}' (campo '{}')",
                    self.name
                )))
            }
        };

        let mut def = FieldDef::new(&self.name, ftype);
        if let Some(s) = &self.string {
            def.string = s.clone();
        }
        def.required = self.required;
        def.readonly = self.readonly;
        def.selection = self.selection.clone();
        def.related = self.related.clone();
        if let Some(method) = &self.compute {
            def.compute = Some(ComputeDef {
                method: method.clone(),
                depends: self.depends.clone(),
            });
        }
        // Convención Odoo: store=True implícito en campos normales,
        // store=False implícito en computados; x2many nunca es columna.
        def.store = self.store.unwrap_or(self.compute.is_none())
            && !matches!(
                def.ftype,
                FieldType::One2many { .. } | FieldType::Many2many { .. }
            );
        Ok(def)
    }
}

/// Parsea IR JSON: un objeto modelo o una lista de modelos.
pub fn parse_ir(json: &str) -> OResult<Vec<ModelIr>> {
    let v: serde_json::Value = serde_json::from_str(json)
        .map_err(|e| OError::Registry(format!("IR JSON inválido: {e}")))?;
    let irs: Vec<ModelIr> = if v.is_array() {
        serde_json::from_value(v)
            .map_err(|e| OError::Registry(format!("IR JSON inválido: {e}")))?
    } else {
        vec![serde_json::from_value(v)
            .map_err(|e| OError::Registry(format!("IR JSON inválido: {e}")))?]
    };
    Ok(irs)
}

/// Fragmento puramente declarativo construido desde IR. No aporta métodos;
/// los cuerpos llegan por otros fragmentos (codegen FASE 3a) o, en el
/// futuro, por `nexus-pyvm` (FASE 3b).
pub struct IrFragment {
    ir: ModelIr,
}

impl IrFragment {
    pub fn new(ir: ModelIr) -> Self {
        IrFragment { ir }
    }
}

impl ModelFragment for IrFragment {
    fn model_name(&self) -> &str {
        &self.ir.model
    }

    fn module(&self) -> &str {
        self.ir.module.as_deref().unwrap_or("base")
    }

    fn is_extension(&self) -> bool {
        self.ir.inherit
    }

    fn build(&self, def: &mut ModelDef) {
        if let Some(t) = &self.ir.table {
            def.table = t.clone();
        }
        if let Some(d) = &self.ir.description {
            def.description = d.clone();
        }
        if let Some(o) = &self.ir.order {
            def.order = o.clone();
        }
        if let Some(r) = &self.ir.rec_name {
            def.rec_name = r.clone();
        }
        for f in &self.ir.fields {
            match f.to_def() {
                Ok(fd) => def.add_field(fd),
                Err(e) => {
                    // build() es infalible por diseño del trait; el IR ya
                    // fue validado al parsear, esto solo cubre defectos.
                    tracing::error!(
                        modelo = %self.ir.model,
                        campo = %f.name,
                        "campo IR inválido ignorado: {e}"
                    );
                }
            }
        }
    }

    fn methods(&self) -> Vec<&str> {
        self.ir.methods.iter().map(|m| m.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_modelo_simple() {
        let irs = parse_ir(
            r#"{
                "model": "x.demo",
                "module": "demo",
                "fields": [
                    {"name": "name", "type": "char", "required": true},
                    {"name": "partner_id", "type": "many2one", "comodel": "res.partner"},
                    {"name": "total", "type": "monetary",
                     "compute": "_compute_total", "depends": ["qty"]},
                    {"name": "qty", "type": "integer"}
                ]
            }"#,
        )
        .unwrap();
        assert_eq!(irs.len(), 1);
        assert_eq!(irs[0].fields.len(), 4);

        let total = irs[0].fields[2].to_def().unwrap();
        assert!(!total.store, "compute sin store=true no es columna");
        assert_eq!(total.compute.as_ref().unwrap().method, "_compute_total");
    }

    #[test]
    fn relacional_sin_comodel_falla() {
        let irs =
            parse_ir(r#"{"model":"x.bad","fields":[{"name":"p","type":"many2one"}]}"#).unwrap();
        assert!(irs[0].fields[0].to_def().is_err());
    }

    #[test]
    fn lista_de_modelos() {
        let irs = parse_ir(r#"[{"model":"a.a"},{"model":"b.b","inherit":true}]"#).unwrap();
        assert_eq!(irs.len(), 2);
        assert!(irs[1].inherit);
    }
}
