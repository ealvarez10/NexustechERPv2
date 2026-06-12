// Generado por odoo2rs — vista search de account.cash.rounding (rounding_search_view).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "rounding_search_view",
  "name": "account.cash.rounding.search",
  "model": "account.cash.rounding",
  "type": "search",
  "arch": {
    "tag": "search",
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "name"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "name"
    }
  ]
}

export function renderAccountCashRoundingSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
