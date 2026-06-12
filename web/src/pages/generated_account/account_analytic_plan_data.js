// Generado por odoo2rs — vista data de account.analytic.plan (account_analytic_plan_form_view_inherit_account).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "account_analytic_plan_form_view_inherit_account",
  "name": "account.analytic.plan.inherit.form",
  "model": "account.analytic.plan",
  "type": "data",
  "arch": {
    "tag": "data",
    "children": [
      {
        "tag": "xpath",
        "attrs": {
          "expr": "//field[@name='applicability_ids']//field[@name='business_domain']",
          "position": "after"
        },
        "children": [
          {
            "tag": "field",
            "attrs": {
              "column_invisible": "True",
              "name": "display_account_prefix"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "column_invisible": "True",
              "name": "account_prefix_placeholder"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "not display_account_prefix",
              "name": "account_prefix",
              "options": "{'placeholder_field': 'account_prefix_placeholder'}",
              "widget": "char_with_placeholder_field"
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
      "name": "display_account_prefix",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "account_prefix_placeholder",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "account_prefix",
      "widget": "char_with_placeholder_field",
      "attrs": {
        "invisible": "not display_account_prefix",
        "options": "{'placeholder_field': 'account_prefix_placeholder'}"
      }
    },
    {
      "name": "product_categ_id"
    }
  ]
}

export function renderAccountAnalyticPlanData(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
