// Generado por odoo2rs — vista tree de account.cash.rounding (rounding_tree_view).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "rounding_tree_view",
  "name": "account.cash.rounding.list",
  "model": "account.cash.rounding",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Rounding List"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "name"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "rounding"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "rounding_method"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "name"
    },
    {
      "name": "rounding"
    },
    {
      "name": "rounding_method"
    }
  ]
}

export function renderAccountCashRoundingTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
