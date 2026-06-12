// Generado por odoo2rs — vista graph de account.bank.statement (account_bank_statement_graph).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "account_bank_statement_graph",
  "name": "account.bank.statement.graph",
  "model": "account.bank.statement",
  "type": "graph",
  "arch": {
    "tag": "graph",
    "attrs": {
      "sample": "1",
      "string": "Account Statistics"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "date"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "balance_start",
          "operator": "+"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "balance_end",
          "operator": "+"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "date"
    },
    {
      "name": "balance_start",
      "attrs": {
        "operator": "+"
      }
    },
    {
      "name": "balance_end",
      "attrs": {
        "operator": "+"
      }
    }
  ]
}

export function renderAccountBankStatementGraph(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
