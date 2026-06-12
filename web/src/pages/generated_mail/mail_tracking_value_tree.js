// Generado por odoo2rs — vista tree de mail.tracking.value (view_mail_tracking_value_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_mail_tracking_value_tree",
  "name": "mail.tracking.value.list",
  "model": "mail.tracking.value",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Tracking Value"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "field_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "old_value_integer"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "old_value_float"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "old_value_char"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "old_value_text"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "old_value_datetime"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "new_value_integer"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "new_value_float"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "new_value_char"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "new_value_text"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "new_value_datetime"
        }
      },
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
      "name": "field_id"
    },
    {
      "name": "old_value_integer"
    },
    {
      "name": "old_value_float"
    },
    {
      "name": "old_value_char"
    },
    {
      "name": "old_value_text"
    },
    {
      "name": "old_value_datetime"
    },
    {
      "name": "new_value_integer"
    },
    {
      "name": "new_value_float"
    },
    {
      "name": "new_value_char"
    },
    {
      "name": "new_value_text"
    },
    {
      "name": "new_value_datetime"
    },
    {
      "name": "mail_message_id"
    }
  ]
}

export function renderMailTrackingValueTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
