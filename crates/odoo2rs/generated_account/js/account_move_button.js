// Generado por odoo2rs — vista button de account.move (view_out_credit_note_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_out_credit_note_tree",
  "name": "account.out.invoice.list",
  "model": "account.move",
  "type": "button",
  "arch": {
    "tag": "button",
    "attrs": {
      "name": "action_force_register_payment",
      "position": "before"
    },
    "children": [
      {
        "tag": "button",
        "attrs": {
          "name": "action_send_and_print",
          "string": "Send",
          "type": "object"
        }
      }
    ]
  },
  "fields": [],
  "buttons": [
    {
      "name": "action_send_and_print",
      "string": "Send",
      "type": "object"
    }
  ]
}

export function renderAccountMoveButton(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
