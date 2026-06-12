// Generado por odoo2rs — vista graph de account.payment (view_account_payment_graph).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_payment_graph",
  "name": "account.payment.graph",
  "model": "account.payment",
  "type": "graph",
  "arch": {
    "tag": "graph",
    "attrs": {
      "sample": "1",
      "string": "Invoices"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "payment_type"
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
          "name": "amount",
          "type": "measure"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "payment_type"
    },
    {
      "name": "journal_id"
    },
    {
      "name": "amount",
      "attrs": {
        "type": "measure"
      }
    }
  ]
}

export function renderAccountPaymentGraph(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
