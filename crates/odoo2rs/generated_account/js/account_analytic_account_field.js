// Generado por odoo2rs — vista field de account.analytic.account (account_analytic_account_view_list_inherit).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "account_analytic_account_view_list_inherit",
  "name": "account.analytic.account.list.inherit",
  "model": "account.analytic.account",
  "type": "field",
  "arch": {
    "tag": "field",
    "attrs": {
      "name": "debit",
      "position": "attributes"
    },
    "children": [
      {
        "tag": "attribute",
        "attrs": {
          "name": "column_invisible"
        },
        "text": "False"
      },
      {
        "tag": "attribute",
        "attrs": {
          "name": "groups"
        },
        "text": "account.group_account_readonly"
      }
    ]
  },
  "fields": []
}

export function renderAccountAnalyticAccountField(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
