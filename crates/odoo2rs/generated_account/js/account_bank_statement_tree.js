// Generado por odoo2rs — vista tree de account.bank.statement (view_bank_statement_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_bank_statement_tree",
  "name": "account.bank.statement.list",
  "model": "account.bank.statement",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "create": "false",
      "decoration-danger": "journal_id and not is_complete or not is_valid",
      "decoration-muted": "not journal_id",
      "string": "Statements"
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
          "name": "date"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "journal_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_multi_company",
          "name": "company_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "balance_start"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "balance_end_real"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "balance_end"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "currency_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "is_complete"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "is_valid"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "name"
    },
    {
      "name": "date"
    },
    {
      "name": "journal_id"
    },
    {
      "name": "company_id",
      "attrs": {
        "groups": "base.group_multi_company"
      }
    },
    {
      "name": "balance_start"
    },
    {
      "name": "balance_end_real"
    },
    {
      "name": "balance_end",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "currency_id",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "is_complete",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "is_valid",
      "attrs": {
        "column_invisible": "True"
      }
    }
  ]
}

export function renderAccountBankStatementTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
