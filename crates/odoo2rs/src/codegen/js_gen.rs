//! `ViewIr` → página JS para el frontend de NexusTech.
//!
//! El descriptor completo de la vista se incrusta como `DESCRIPTOR` (JSON)
//! y la función `render…` lo proyecta sobre los componentes existentes:
//! `renderFormPage` de `components/form_view.js` para forms, y una tabla
//! HTML estilo `o-list` para tree. Es scaffolding FASE 2: el binding de
//! datos reales (api.js) se conecta a mano o en FASE 3.

use std::fmt::Write as _;

use crate::ir::ViewIr;

/// `sale.order` → `SaleOrder`.
fn camel(s: &str) -> String {
    s.split(|c: char| !c.is_alphanumeric())
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

/// Nombre de la función `render…` generada.
pub fn render_fn_name(view: &ViewIr) -> String {
    format!(
        "render{}{}",
        camel(view.model.as_deref().unwrap_or("view")),
        camel(&view.view_type)
    )
}

/// Nombre de archivo sugerido para la página generada.
pub fn js_file_name(view: &ViewIr) -> String {
    let model = view.model.as_deref().unwrap_or("view").replace('.', "_");
    format!("{model}_{}.js", view.view_type)
}

pub fn view_js(view: &ViewIr) -> String {
    let model = view.model.as_deref().unwrap_or("?");
    let descriptor =
        serde_json::to_string_pretty(view).expect("ViewIr siempre serializa a JSON");
    let fn_name = render_fn_name(view);
    let mut out = String::new();

    let _ = writeln!(
        out,
        "// Generado por odoo2rs — vista {} de {} ({}).\n\
         // NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.\n",
        view.view_type,
        model,
        view.xml_id.as_deref().unwrap_or("sin xml_id")
    );

    match view.view_type.as_str() {
        "form" => {
            out.push_str("import { renderFormPage } from '../components/form_view.js'\n\n");
            let _ = writeln!(out, "export const DESCRIPTOR = {descriptor}\n");
            let _ = writeln!(
                out,
                "export function {fn_name}(record = {{}}) {{\n\
                 \x20 renderFormPage({{\n\
                 \x20   breadcrumb: [{{ label: '{model}' }}],\n\
                 \x20   title: record.name || record.display_name || 'Nuevo',\n\
                 \x20   currentStatus: record.state || '',\n\
                 \x20   // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo\n\
                 \x20   statusSteps: [],\n\
                 \x20   statusButtons: DESCRIPTOR.buttons?.map(b => ({{\n\
                 \x20     label: b.string || b.name,\n\
                 \x20     primary: (b.class || '').includes('btn-primary'),\n\
                 \x20     // TODO(odoo2rs): conectar a /api/v1/orm/{model}/<método> (≈ call_kw)\n\
                 \x20     onClick: `alert('TODO: ${{b.name}}')`,\n\
                 \x20   }})) || [],\n\
                 \x20   fieldGroups: [{{\n\
                 \x20     fields: DESCRIPTOR.fields.map(f => ({{\n\
                 \x20       label: f.string || f.name,\n\
                 \x20       value: record[f.name] ?? '',\n\
                 \x20     }})),\n\
                 \x20   }}],\n\
                 \x20   id: record.id || '',\n\
                 \x20 }})\n\
                 }}"
            );
        }
        "kanban" => {
            out.push_str(
                "import { kanbanViewHtml } from '../components/kanban_view.js'\n\n",
            );
            let _ = writeln!(out, "export const DESCRIPTOR = {descriptor}\n");
            let _ = writeln!(
                out,
                "export function {fn_name}(records = []) {{\n\
                 \x20 return kanbanViewHtml({{\n\
                 \x20   // TODO(odoo2rs): columnas desde la selection del campo state del modelo\n\
                 \x20   columns: [],\n\
                 \x20   records,\n\
                 \x20   stateField: 'state',\n\
                 \x20 }})\n\
                 }}"
            );
        }
        // tree y demás: tabla genérica con las columnas de la vista.
        _ => {
            let _ = writeln!(out, "export const DESCRIPTOR = {descriptor}\n");
            let _ = writeln!(
                out,
                "export function {fn_name}(records = []) {{\n\
                 \x20 const cols = DESCRIPTOR.fields\n\
                 \x20 return `\n\
                 \x20   <table class=\"o-list-table\">\n\
                 \x20     <thead><tr>${{cols.map(c => `<th>${{c.string || c.name}}</th>`).join('')}}</tr></thead>\n\
                 \x20     <tbody>\n\
                 \x20       ${{records.map(r => `<tr>${{cols.map(c => `<td>${{r[c.name] ?? ''}}</td>`).join('')}}</tr>`).join('')}}\n\
                 \x20     </tbody>\n\
                 \x20   </table>`\n\
                 }}"
            );
        }
    }
    out
}
