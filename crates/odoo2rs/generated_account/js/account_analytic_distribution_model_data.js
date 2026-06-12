// Generado por odoo2rs — vista data de account.analytic.distribution.model (account_analytic_distribution_model_form_inherit).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "account_analytic_distribution_model_form_inherit",
  "name": "account.analytic.distribution.model.inherit.form",
  "model": "account.analytic.distribution.model",
  "type": "data",
  "arch": {
    "tag": "data",
    "children": [
      {
        "tag": "xpath",
        "attrs": {
          "expr": "//field[@name='partner_category_id']",
          "position": "after"
        },
        "children": [
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "prefix_placeholder"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "name": "account_prefix",
              "options": "{'placeholder_field': 'prefix_placeholder'}",
              "string": "Accounts Prefixes"
            }
          }
        ]
      },
      {
        "tag": "xpath",
        "attrs": {
          "expr": "//field[@name='company_id']",
          "position": "before"
        },
        "children": [
          {
            "tag": "field",
            "attrs": {
              "name": "product_id"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "name": "product_categ_id"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "prefix_placeholder",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "account_prefix",
      "string": "Accounts Prefixes",
      "attrs": {
        "options": "{'placeholder_field': 'prefix_placeholder'}"
      }
    },
    {
      "name": "product_id"
    },
    {
      "name": "product_categ_id"
    }
  ]
}

export function renderAccountAnalyticDistributionModelData(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
