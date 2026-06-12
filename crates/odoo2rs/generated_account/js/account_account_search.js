// Generado por odoo2rs — vista search de account.account (view_account_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_search",
  "name": "account.account.search",
  "model": "account.account",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Accounts"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "filter_domain": "['|', '|', ('name', 'ilike', self), ('code', '=ilike', self + '%'), ('description', 'ilike', self)]",
          "name": "name",
          "string": "Account"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('account_type','=','asset_receivable')]",
          "name": "receivableacc",
          "string": "Receivable"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('account_type','=','liability_payable')]",
          "name": "payableacc",
          "string": "Payable"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('internal_group','=', 'equity')]",
          "name": "equityacc",
          "string": "Equity"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('internal_group','=', 'asset')]",
          "name": "assetsacc",
          "string": "Assets"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('internal_group','=', 'liability')]",
          "name": "liabilityacc",
          "string": "Liability"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('internal_group','=', 'income')]",
          "name": "incomeacc",
          "string": "Income"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('internal_group','=', 'expense')]",
          "name": "expensesacc",
          "string": "Expenses"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('account_type', '=', 'asset_fixed')]",
          "invisible": "True",
          "name": "type_asset_fixed",
          "string": "Fixed Assets"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('account_type', '=', 'expense')]",
          "invisible": "True",
          "name": "type_expense",
          "string": "Frequent Expenses"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('used', '=', True)]",
          "name": "used",
          "string": "Account with Entries"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('active', '=', False)]",
          "name": "inactiveacc",
          "string": "Inactive Accounts"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "field",
        "attrs": {
          "name": "account_type"
        }
      },
      {
        "tag": "group",
        "children": [
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by':'account_type'}",
              "domain": "",
              "name": "accounttype",
              "string": "Account Type"
            }
          }
        ]
      },
      {
        "tag": "searchpanel",
        "attrs": {
          "class": "account_root w-auto"
        },
        "children": [
          {
            "tag": "field",
            "attrs": {
              "icon": "fa-filter",
              "limit": "false",
              "name": "root_id"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "name",
      "string": "Account",
      "attrs": {
        "filter_domain": "['|', '|', ('name', 'ilike', self), ('code', '=ilike', self + '%'), ('description', 'ilike', self)]"
      }
    },
    {
      "name": "account_type"
    },
    {
      "name": "root_id",
      "attrs": {
        "icon": "fa-filter",
        "limit": "false"
      }
    }
  ]
}

export function renderAccountAccountSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
