// Generado por odoo2rs — vista pivot de account.bank.statement (account_bank_statement_pivot).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "account_bank_statement_pivot",
  "name": "account.bank.statement.pivot",
  "model": "account.bank.statement",
  "type": "pivot",
  "arch": {
    "tag": "pivot",
    "attrs": {
      "sample": "1",
      "string": "Account Statistics"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "date",
          "type": "row"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "balance_start",
          "type": "measure"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "balance_end",
          "type": "measure"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "date",
      "attrs": {
        "type": "row"
      }
    },
    {
      "name": "balance_start",
      "attrs": {
        "type": "measure"
      }
    },
    {
      "name": "balance_end",
      "attrs": {
        "type": "measure"
      }
    }
  ]
}

export function renderAccountBankStatementPivot(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
