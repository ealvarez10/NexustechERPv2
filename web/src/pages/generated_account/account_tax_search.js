// Generado por odoo2rs — vista search de account.tax (account_tax_view_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "account_tax_view_search",
  "name": "account.tax.search.filters",
  "model": "account.tax",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Search Taxes"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "filter_domain": "['|', '|', ('name', 'ilike', self), ('description', 'ilike', self), ('invoice_label', 'ilike', self)]",
          "name": "name",
          "string": "Tax"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_multi_company",
          "name": "company_id"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "name",
      "string": "Tax",
      "attrs": {
        "filter_domain": "['|', '|', ('name', 'ilike', self), ('description', 'ilike', self), ('invoice_label', 'ilike', self)]"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "groups": "base.group_multi_company"
      }
    }
  ]
}

export function renderAccountTaxSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
