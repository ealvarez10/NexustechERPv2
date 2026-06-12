//! OdooIR — el IR que emite el transpilador (FASE 2 del reporte).
//!
//! Este módulo es el lado `Serialize` del contrato cuyo lado `Deserialize`
//! vive en `nexus_orm::ir` (`ModelIr`/`FieldIr`): el JSON que se emite aquí
//! es exactamente el que `nexus_orm::ir::parse_ir` consume y registra en el
//! `RegistryBuilder`. Los campos extra (`methods`, `help`, `default`,
//! `relation`, `inherits`…) no los consume el kernel v0, pero serde los
//! ignora al deserializar — quedan disponibles para la FASE 3 (codegen de
//! cuerpos de métodos) y futuras iteraciones del kernel.
//!
//! El IR de vistas (`ViewIr`) no tiene contraparte en el kernel: alimenta
//! el codegen JS (descriptores para `form_view.js` / `kanban_view.js`).

use std::collections::BTreeMap;

use serde::Serialize;

fn is_false(b: &bool) -> bool {
    !*b
}
fn is_true(b: &bool) -> bool {
    *b
}

// ─── Modelos (contrato con nexus_orm::ir) ───────────────────────────────

/// IR de un fragmento de modelo: una clase Python `models.Model` extraída.
#[derive(Debug, Clone, Serialize)]
pub struct ModelIr {
    /// `_name` (o el modelo extendido si `inherit` es `true`).
    pub model: String,
    /// Módulo Odoo de origen (carpeta del addon).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    /// `true` ⇒ fragmento `_inherit` puro (sin `_name` propio).
    #[serde(skip_serializing_if = "is_false")]
    pub inherit: bool,
    /// Lista completa de `_inherit` (el kernel v0 solo usa el flag).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inherits: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_name: Option<String>,
    pub fields: Vec<FieldIr>,
    /// Métodos detectados (firma + decoradores). El kernel los ignora;
    /// son el insumo de la FASE 3a (codegen de stubs y traducción).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub methods: Vec<MethodIr>,
}

impl ModelIr {
    pub fn new(model: &str) -> Self {
        ModelIr {
            model: model.to_string(),
            module: None,
            inherit: false,
            inherits: Vec::new(),
            table: None,
            description: None,
            order: None,
            rec_name: None,
            fields: Vec::new(),
            methods: Vec::new(),
        }
    }
}

/// IR de un campo `fields.*`. Los nombres de tipo son los que
/// `nexus_orm::ir::FieldIr::to_def` reconoce ("char", "many2one"…).
#[derive(Debug, Clone, Serialize)]
pub struct FieldIr {
    pub name: String,
    #[serde(rename = "type")]
    pub ftype: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comodel: Option<String>,
    /// Inverso de one2many.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inverse: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub readonly: bool,
    /// `true` por defecto, como en el kernel; se omite del JSON si es true.
    #[serde(skip_serializing_if = "is_true")]
    pub store: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub selection: Vec<(String, String)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub depends: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related: Option<String>,
    // ── extras que el kernel v0 todavía no consume ──
    /// Tabla intermedia de many2many.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help: Option<String>,
    /// `default=` solo si es un literal escalar; los callables se reportan
    /// como aviso y quedan para FASE 3b.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
}

impl FieldIr {
    pub fn new(name: &str, ftype: &str) -> Self {
        FieldIr {
            name: name.to_string(),
            ftype: ftype.to_string(),
            comodel: None,
            inverse: None,
            string: None,
            required: false,
            readonly: false,
            store: true,
            selection: Vec::new(),
            compute: None,
            depends: Vec::new(),
            related: None,
            relation: None,
            column1: None,
            column2: None,
            help: None,
            default: None,
        }
    }
}

/// Firma de un método de modelo (cuerpos: FASE 3).
#[derive(Debug, Clone, Serialize)]
pub struct MethodIr {
    pub name: String,
    /// Argumentos posicionales sin `self`.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    /// Decoradores reconstruidos: `api.depends('order_line.price_subtotal')`.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub decorators: Vec<String>,
    /// Rutas de `@api.depends` (también se copian al campo compute).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub depends: Vec<String>,
    /// Línea 1-based del `def` en el .py de origen.
    pub line: usize,
}

// ─── Vistas / acciones / menús ──────────────────────────────────────────

/// Una vista `ir.ui.view` extraída de un XML de Odoo.
#[derive(Debug, Clone, Serialize)]
pub struct ViewIr {
    /// `id` del `<record>` (xml id).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// form | tree | kanban | search | calendar | … (`list` se normaliza
    /// a `tree`).
    #[serde(rename = "type")]
    pub view_type: String,
    /// El `arch` completo como árbol simplificado.
    pub arch: ViewNode,
    /// Todos los `<field>` del arch, aplanados (incluye sub-vistas o2m).
    pub fields: Vec<ViewField>,
    /// Todos los `<button>` del arch.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub buttons: Vec<ViewButton>,
}

/// Nodo XML simplificado (tag + atributos + hijos + texto).
#[derive(Debug, Clone, Serialize)]
pub struct ViewNode {
    pub tag: String,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub attrs: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<ViewNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// `<field name="..."/>` dentro de un arch. Los atributos de visibilidad
/// (`invisible`, `readonly`, `required`) pueden ser expresiones Python;
/// se conservan crudos en `attrs`.
#[derive(Debug, Clone, Serialize)]
pub struct ViewField {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget: Option<String>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub attrs: BTreeMap<String, String>,
}

/// `<button .../>` dentro de un arch.
#[derive(Debug, Clone, Serialize)]
pub struct ViewButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    /// `object` | `action`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub btype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    /// `states="draft,sent"` crudo, si existe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<String>,
}

/// `ir.actions.act_window`.
#[derive(Debug, Clone, Serialize)]
pub struct ActionIr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub res_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_mode: Option<String>,
}

/// `<menuitem/>`.
#[derive(Debug, Clone, Serialize)]
pub struct MenuIr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<String>,
}

/// Todo lo extraíble de los XML de un addon.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ViewBundle {
    pub views: Vec<ViewIr>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<ActionIr>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub menus: Vec<MenuIr>,
}

/// `__manifest__.py` de un addon.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ManifestIr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub depends: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<String>,
}
