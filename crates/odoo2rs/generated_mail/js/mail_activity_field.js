// Generado por odoo2rs — vista field de mail.activity (mail_activity_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_activity_view_form",
  "name": "mail.activity.view.form",
  "model": "mail.activity",
  "type": "field",
  "arch": {
    "tag": "field",
    "attrs": {
      "name": "activity_type_id",
      "position": "before"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "res_name",
          "readonly": "1",
          "string": "Document"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "res_name",
      "string": "Document",
      "attrs": {
        "readonly": "1"
      }
    }
  ]
}

export function renderMailActivityField(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
