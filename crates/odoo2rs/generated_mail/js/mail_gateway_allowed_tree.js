// Generado por odoo2rs — vista tree de mail.gateway.allowed (mail_gateway_allowed_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_gateway_allowed_view_tree",
  "name": "mail.gateway.allowed.view.list",
  "model": "mail.gateway.allowed",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "editable": "top",
      "string": "Mail Gateway Allowed"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "email"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "email"
    }
  ]
}

export function renderMailGatewayAllowedTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
