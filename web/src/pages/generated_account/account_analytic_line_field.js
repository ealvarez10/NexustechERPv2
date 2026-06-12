// Generado por odoo2rs — vista field de account.analytic.line (view_account_analytic_line_pivot).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_analytic_line_pivot",
  "name": "account.analytic.line.pivot",
  "model": "account.analytic.line",
  "type": "field",
  "arch": {
    "tag": "field",
    "attrs": {
      "name": "account_id",
      "position": "after"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "invisible": "1",
          "name": "partner_id",
          "type": "row"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "partner_id",
      "attrs": {
        "invisible": "1",
        "type": "row"
      }
    }
  ]
}

export function renderAccountAnalyticLineField(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
