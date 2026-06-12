// Generado por odoo2rs — vista data de account.analytic.line (view_account_analytic_line_filter_inherit_account).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_analytic_line_filter_inherit_account",
  "name": "account.analytic.line.select.inherit.account",
  "model": "account.analytic.line",
  "type": "data",
  "arch": {
    "tag": "data",
    "children": [
      {
        "tag": "xpath",
        "attrs": {
          "expr": "//field[@name='date']",
          "position": "after"
        },
        "children": [
          {
            "tag": "field",
            "attrs": {
              "name": "auto_account_id"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "name": "account_id"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "name": "product_id"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "name": "general_account_id"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "filter_domain": "[('partner_id','child_of',self)]",
              "name": "partner_id"
            }
          }
        ]
      },
      {
        "tag": "xpath",
        "attrs": {
          "expr": "//filter[@name='month']",
          "position": "after"
        },
        "children": [
          {
            "tag": "filter",
            "attrs": {
              "domain": "[('fiscal_year_search', '=', True)]",
              "invisible": "1",
              "name": "fiscal_date",
              "string": "From last fiscal year"
            }
          }
        ]
      },
      {
        "tag": "xpath",
        "attrs": {
          "expr": "//group[@name='groupby']",
          "position": "inside"
        },
        "children": [
          {
            "tag": "separator"
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'account_id'}",
              "name": "account_id"
            }
          },
          {
            "tag": "separator"
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by':'general_account_id'}",
              "name": "group_by_financial_account",
              "string": "Financial Account"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'category'}",
              "name": "category",
              "string": "Category"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by':'product_id'}",
              "name": "product",
              "string": "Product"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by':'partner_id'}",
              "name": "partner",
              "string": "Partner"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "auto_account_id"
    },
    {
      "name": "account_id"
    },
    {
      "name": "product_id"
    },
    {
      "name": "general_account_id"
    },
    {
      "name": "partner_id",
      "attrs": {
        "filter_domain": "[('partner_id','child_of',self)]"
      }
    }
  ]
}

export function renderAccountAnalyticLineData(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
