// Generado por odoo2rs — vista xpath de ir.mail_server (ir_mail_server_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "ir_mail_server_view_form",
  "name": "ir.mail_server.view.form.inherit.mail",
  "model": "ir.mail_server",
  "type": "xpath",
  "arch": {
    "tag": "xpath",
    "attrs": {
      "expr": "//label[@for='smtp_user']",
      "position": "attributes"
    },
    "children": [
      {
        "tag": "attribute",
        "attrs": {
          "add": "owner_user_id",
          "name": "invisible",
          "separator": "or"
        }
      }
    ]
  },
  "fields": []
}

export function renderIrMailServerXpath(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
