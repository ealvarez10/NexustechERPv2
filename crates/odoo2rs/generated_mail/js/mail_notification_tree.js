// Generado por odoo2rs — vista tree de mail.notification (mail_notification_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_notification_view_tree",
  "name": "mail.notification.view.list",
  "model": "mail.notification",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Notifications"
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
          "name": "notification_type"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "res_partner_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "is_read"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "failure_type"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "mail_message_id"
    },
    {
      "name": "notification_type"
    },
    {
      "name": "res_partner_id"
    },
    {
      "name": "is_read"
    },
    {
      "name": "failure_type"
    }
  ]
}

export function renderMailNotificationTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
