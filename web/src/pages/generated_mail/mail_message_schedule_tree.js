// Generado por odoo2rs — vista tree de mail.message.schedule (mail_message_schedule_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_message_schedule_view_tree",
  "name": "mail.message.schedule.view.list",
  "model": "mail.message.schedule",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Emails"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "mail_message_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "scheduled_datetime"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "mail_message_id"
    },
    {
      "name": "scheduled_datetime"
    }
  ]
}

export function renderMailMessageScheduleTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
