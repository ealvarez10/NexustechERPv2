// Generado por odoo2rs — vista tree de account.payment.method.line (view_account_payment_method_line_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_payment_method_line_tree",
  "name": "account.payment.method.line.list",
  "model": "account.payment.method.line",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "edit": "0"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "name",
          "string": "Payment Method Name"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "journal_id"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "name",
      "string": "Payment Method Name"
    },
    {
      "name": "journal_id"
    }
  ]
}

export function renderAccountPaymentMethodLineTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
