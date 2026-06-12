// Generado por odoo2rs — vista calendar de mail.activity (mail_activity_view_calendar).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_activity_view_calendar",
  "name": "mail.activity.view.calendar",
  "model": "mail.activity",
  "type": "calendar",
  "arch": {
    "tag": "calendar",
    "attrs": {
      "color": "activity_type_id",
      "create": "0",
      "date_start": "date_deadline",
      "js_class": "activity_calendar",
      "mode": "month",
      "string": "Activity"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "avatar_field": "avatar_128",
          "name": "user_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "res_name",
          "string": "Name"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "date_deadline"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "summary"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "filters": "1",
          "invisible": "1",
          "name": "activity_type_id"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "user_id",
      "attrs": {
        "avatar_field": "avatar_128"
      }
    },
    {
      "name": "res_name",
      "string": "Name"
    },
    {
      "name": "date_deadline"
    },
    {
      "name": "summary"
    },
    {
      "name": "activity_type_id",
      "attrs": {
        "filters": "1",
        "invisible": "1"
      }
    }
  ]
}

export function renderMailActivityCalendar(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
