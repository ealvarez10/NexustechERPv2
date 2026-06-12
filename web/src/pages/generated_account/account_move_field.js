// Generado por odoo2rs — vista field de account.move (view_account_bill_filter).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_bill_filter",
  "name": "account.invoice.select",
  "model": "account.move",
  "type": "field",
  "arch": {
    "tag": "field",
    "attrs": {
      "name": "name",
      "position": "attributes"
    },
    "children": [
      {
        "tag": "attribute",
        "attrs": {
          "name": "string"
        },
        "text": "Bill"
      }
    ]
  },
  "fields": []
}

export function renderAccountMoveField(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
