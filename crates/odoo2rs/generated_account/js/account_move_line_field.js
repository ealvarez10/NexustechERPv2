// Generado por odoo2rs — vista field de account.move.line (view_move_line_tax_audit_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_move_line_tax_audit_tree",
  "name": "account.move.line.tax.audit.list",
  "model": "account.move.line",
  "type": "field",
  "arch": {
    "tag": "field",
    "attrs": {
      "name": "matching_number",
      "position": "replace"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "tax_line_id",
          "string": "Tax"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "tax_base_amount",
          "sum": "Total Base Amount"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "tax_line_id",
      "string": "Tax"
    },
    {
      "name": "tax_base_amount",
      "attrs": {
        "sum": "Total Base Amount"
      }
    }
  ]
}

export function renderAccountMoveLineField(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
