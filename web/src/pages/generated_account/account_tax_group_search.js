// Generado por odoo2rs — vista search de account.tax.group (account_tax_group_view_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "account_tax_group_view_search",
  "name": "account.tax.group.search.filters",
  "model": "account.tax.group",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Search Group"
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
          "name": "country_id"
        }
      },
      {
        "tag": "group",
        "children": [
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'country_id'}",
              "domain": "[]",
              "name": "group_by_country",
              "string": "Country"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "name"
    },
    {
      "name": "country_id"
    }
  ]
}

export function renderAccountTaxGroupSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
