// Generado por odoo2rs — vista search de account.fiscal.position (view_account_position_filter).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_position_filter",
  "name": "account.fiscal.position.filter",
  "model": "account.fiscal.position",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Search Fiscal Positions"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "name",
          "string": "Fiscal Position"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('active', '=', False)]",
          "name": "active",
          "string": "Archived"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('is_domestic', '=', True)]",
          "name": "domestic",
          "string": "Domestic"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "name",
      "string": "Fiscal Position"
    }
  ]
}

export function renderAccountFiscalPositionSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
