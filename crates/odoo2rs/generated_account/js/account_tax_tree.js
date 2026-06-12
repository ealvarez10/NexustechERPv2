// Generado por odoo2rs — vista tree de account.tax (account_tax_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "account_tax_view_tree",
  "name": "account.invoice.line.tax.search",
  "model": "account.tax",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Account Tax"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "display_name",
          "string": "Name"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "tax_scope"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "description"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "display_name",
      "string": "Name"
    },
    {
      "name": "tax_scope"
    },
    {
      "name": "description"
    }
  ]
}

export function renderAccountTaxTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
