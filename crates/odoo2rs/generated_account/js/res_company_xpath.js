// Generado por odoo2rs — vista xpath de res.company (view_company_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_company_form",
  "name": "res.company.form.inherit.account",
  "model": "res.company",
  "type": "xpath",
  "arch": {
    "tag": "xpath",
    "attrs": {
      "expr": "//field[@name='country_id']",
      "position": "after"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "invisible": "1",
          "name": "account_enabled_tax_country_ids"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "account_enabled_tax_country_ids",
      "attrs": {
        "invisible": "1"
      }
    }
  ]
}

export function renderResCompanyXpath(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
