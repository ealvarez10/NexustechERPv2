// Generado por odoo2rs — vista search de mail.gateway.allowed (mail_gateway_allowed_view_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_gateway_allowed_view_search",
  "name": "mail.gateway.allowed.view.search",
  "model": "mail.gateway.allowed",
  "type": "search",
  "arch": {
    "tag": "search",
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

export function renderMailGatewayAllowedSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
