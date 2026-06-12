// Generado por odoo2rs — vista search de mail.message.schedule (mail_message_schedule_view_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_message_schedule_view_search",
  "name": "mail.message.schedule.view.search",
  "model": "mail.message.schedule",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Scheduled Messages"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "mail_message_id"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "mail_message_id"
    }
  ]
}

export function renderMailMessageScheduleSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
