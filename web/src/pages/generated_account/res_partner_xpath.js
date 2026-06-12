// Generado por odoo2rs — vista xpath de res.partner (res_partner_view_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "res_partner_view_search",
  "name": "res.partner.search.inherit",
  "model": "res.partner",
  "type": "xpath",
  "arch": {
    "tag": "xpath",
    "attrs": {
      "expr": "//filter[@name='inactive']",
      "position": "before"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "invisible": "1",
          "name": "fiscal_country_codes"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('customer_rank','>', 0)]",
          "name": "customer",
          "string": "Customer Invoices"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('supplier_rank','>', 0)]",
          "name": "supplier",
          "string": "Vendor Bills"
        }
      },
      {
        "tag": "separator"
      }
    ]
  },
  "fields": [
    {
      "name": "fiscal_country_codes",
      "attrs": {
        "invisible": "1"
      }
    }
  ]
}

export function renderResPartnerXpath(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
