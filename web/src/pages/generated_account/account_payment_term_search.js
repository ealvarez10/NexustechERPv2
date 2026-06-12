// Generado por odoo2rs — vista search de account.payment.term (view_payment_term_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_payment_term_search",
  "name": "account.payment.term.search",
  "model": "account.payment.term",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Payment Terms"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "name",
          "string": "Payment Terms"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "active"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('active', '=', False)]",
          "name": "archived",
          "string": "Archived"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "name",
      "string": "Payment Terms"
    },
    {
      "name": "active"
    }
  ]
}

export function renderAccountPaymentTermSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
