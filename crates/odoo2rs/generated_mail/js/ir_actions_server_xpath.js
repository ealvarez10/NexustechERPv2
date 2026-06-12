// Generado por odoo2rs — vista xpath de ir.actions.server (view_server_action_form_template).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_server_action_form_template",
  "name": "ir.actions.server.form",
  "model": "ir.actions.server",
  "type": "xpath",
  "arch": {
    "tag": "xpath",
    "attrs": {
      "expr": "//form",
      "position": "inside"
    },
    "children": [
      {
        "tag": "chatter"
      }
    ]
  },
  "fields": []
}

export function renderIrActionsServerXpath(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
