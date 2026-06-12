//! FASE 1 (frontend XML): roxmltree → `ViewIr` / `ActionIr` / `MenuIr`.
//!
//! Lee los XML de vistas de un addon (`<odoo>`/`<openerp>` con `<record
//! model="ir.ui.view">`, `act_window`, `<menuitem>`) y produce el bundle
//! que alimenta el codegen JS (descriptores para `form_view.js` /
//! `kanban_view.js` del frontend existente).

use std::collections::BTreeMap;

use anyhow::{anyhow, Result};
use roxmltree::Node;

use crate::ir::{ActionIr, MenuIr, ViewBundle, ViewButton, ViewField, ViewIr, ViewNode};

#[derive(Debug, Default)]
pub struct XmlExtraction {
    pub bundle: ViewBundle,
    pub warnings: Vec<String>,
}

pub fn extract_views(xml_src: &str, path: &str) -> Result<XmlExtraction> {
    let doc = roxmltree::Document::parse(xml_src)
        .map_err(|e| anyhow!("{path}: XML inválido: {e}"))?;
    let mut ex = XmlExtraction::default();

    for node in doc.descendants().filter(|n| n.is_element()) {
        match node.tag_name().name() {
            "record" => match node.attribute("model") {
                Some("ir.ui.view") => {
                    if let Some(v) = view_record(node, path, &mut ex.warnings) {
                        ex.bundle.views.push(v);
                    }
                }
                Some("ir.actions.act_window") => {
                    ex.bundle.actions.push(action_record(node));
                }
                _ => {}
            },
            "menuitem" => ex.bundle.menus.push(MenuIr {
                xml_id: node.attribute("id").map(str::to_string),
                name: node.attribute("name").map(str::to_string),
                parent: node.attribute("parent").map(str::to_string),
                action: node.attribute("action").map(str::to_string),
                sequence: node.attribute("sequence").map(str::to_string),
            }),
            _ => {}
        }
    }
    Ok(ex)
}

/// Texto del `<field name="X">` hijo de un `<record>`.
fn record_field<'a>(record: Node<'a, 'a>, name: &str) -> Option<Node<'a, 'a>> {
    record
        .children()
        .filter(|c| c.is_element() && c.tag_name().name() == "field")
        .find(|c| c.attribute("name") == Some(name))
}

fn view_record(record: Node, path: &str, warnings: &mut Vec<String>) -> Option<ViewIr> {
    let xml_id = record.attribute("id").map(str::to_string);
    let name = record_field(record, "name").and_then(|n| n.text()).map(str::to_string);
    let model = record_field(record, "model").and_then(|n| n.text()).map(str::to_string);

    let Some(arch_field) = record_field(record, "arch") else {
        warnings.push(format!(
            "{path}: vista {:?} sin <field name=\"arch\"> — omitida",
            xml_id.as_deref().unwrap_or("?")
        ));
        return None;
    };
    let Some(root) = arch_field.children().find(|c| c.is_element()) else {
        warnings.push(format!(
            "{path}: vista {:?} con arch vacío — omitida",
            xml_id.as_deref().unwrap_or("?")
        ));
        return None;
    };

    // Odoo ≥17 usa <list>; el frontend de NexusTech habla «tree».
    let raw_type = root.tag_name().name();
    let view_type = if raw_type == "list" { "tree" } else { raw_type }.to_string();

    let arch = view_node(root);
    let mut fields = Vec::new();
    let mut buttons = Vec::new();
    flatten(root, &mut fields, &mut buttons);

    Some(ViewIr {
        xml_id,
        name,
        model,
        view_type,
        arch,
        fields,
        buttons,
    })
}

fn action_record(record: Node) -> ActionIr {
    let text = |n: &str| record_field(record, n).and_then(|f| f.text()).map(str::to_string);
    ActionIr {
        xml_id: record.attribute("id").map(str::to_string),
        name: text("name"),
        res_model: text("res_model"),
        view_mode: text("view_mode"),
    }
}

/// Copia recursiva del arch como árbol serializable.
fn view_node(n: Node) -> ViewNode {
    let attrs: BTreeMap<String, String> = n
        .attributes()
        .map(|a| (a.name().to_string(), a.value().to_string()))
        .collect();
    let children = n.children().filter(|c| c.is_element()).map(view_node).collect();
    let text: String = n
        .children()
        .filter(|c| c.is_text())
        .filter_map(|t| t.text())
        .collect::<String>()
        .trim()
        .to_string();
    ViewNode {
        tag: n.tag_name().name().to_string(),
        attrs,
        children,
        text: (!text.is_empty()).then_some(text),
    }
}

/// Aplana todos los `<field>`/`<button>` del arch (incluye sub-vistas
/// embebidas de one2many — suficiente para el scaffolding FASE 2).
fn flatten(n: Node, fields: &mut Vec<ViewField>, buttons: &mut Vec<ViewButton>) {
    for c in n.children().filter(|c| c.is_element()) {
        match c.tag_name().name() {
            "field" => {
                if let Some(name) = c.attribute("name") {
                    let mut attrs = BTreeMap::new();
                    for a in c.attributes() {
                        if !matches!(a.name(), "name" | "string" | "widget") {
                            attrs.insert(a.name().to_string(), a.value().to_string());
                        }
                    }
                    fields.push(ViewField {
                        name: name.to_string(),
                        string: c.attribute("string").map(str::to_string),
                        widget: c.attribute("widget").map(str::to_string),
                        attrs,
                    });
                }
            }
            "button" => buttons.push(ViewButton {
                name: c.attribute("name").map(str::to_string),
                string: c.attribute("string").map(str::to_string),
                btype: c.attribute("type").map(str::to_string),
                class: c.attribute("class").map(str::to_string),
                states: c.attribute("states").map(str::to_string),
            }),
            _ => {}
        }
        flatten(c, fields, buttons);
    }
}
