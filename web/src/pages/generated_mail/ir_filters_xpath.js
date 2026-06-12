// Generado por odoo2rs — vista xpath de ir.filters (ir_filters_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "ir_filters_view_tree",
  "name": "ir.filters.view.tree.inherit",
  "model": "ir.filters",
  "type": "xpath",
  "arch": {
    "tag": "xpath",
    "attrs": {
      "expr": "//field[@name='user_ids']",
      "position": "attributes"
    },
    "children": [
      {
        "tag": "attribute",
        "attrs": {
          "name": "widget"
        },
        "text": "many2many_avatar_user"
      }
    ]
  },
  "fields": []
}

export function renderIrFiltersXpath(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
