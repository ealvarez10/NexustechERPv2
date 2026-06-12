// Generado por odoo2rs — vista field de account.payment (view_account_various_payment_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_various_payment_tree",
  "name": "account.supplier.payment.list",
  "model": "account.payment",
  "type": "field",
  "arch": {
    "tag": "field",
    "attrs": {
      "name": "partner_id",
      "position": "attributes"
    },
    "children": [
      {
        "tag": "attribute",
        "attrs": {
          "name": "string"
        },
        "text": "Partner"
      }
    ]
  },
  "fields": []
}

export function renderAccountPaymentField(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
