// Generado por odoo2rs — vista search de account.bank.statement (view_bank_statement_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_bank_statement_search",
  "name": "account.bank.statement.search",
  "model": "account.bank.statement",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Search Statements"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "name",
          "string": "Statement"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "date"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('line_ids','=',False)]",
          "name": "empty",
          "string": "Empty"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "['|', ('is_valid', '=', False),('is_complete', '=', False)]",
          "name": "invalid",
          "string": "Invalid"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "date": "date",
          "name": "filter_date"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "domain": "[('type', 'in', ('bank', 'cash', 'credit'))]",
          "name": "journal_id"
        }
      },
      {
        "tag": "group",
        "children": [
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'journal_id'}",
              "name": "journal",
              "string": "Journal"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'date'}",
              "name": "date",
              "string": "Date"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "name",
      "string": "Statement"
    },
    {
      "name": "date"
    },
    {
      "name": "journal_id",
      "attrs": {
        "domain": "[('type', 'in', ('bank', 'cash', 'credit'))]"
      }
    }
  ]
}

export function renderAccountBankStatementSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
