// Generado por odoo2rs — vista field de account.tax (account_tax_fiscal_position_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "account_tax_fiscal_position_view_tree",
  "name": "account.fiscal.position.tax.list",
  "model": "account.tax",
  "type": "field",
  "arch": {
    "tag": "field",
    "attrs": {
      "name": "display_name",
      "position": "before"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "type_tax_use"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "type_tax_use"
    }
  ]
}

export function renderAccountTaxField(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
