// Generado por odoo2rs — vista tree de mail.mail (view_mail_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_mail_tree",
  "name": "mail.mail.list",
  "model": "mail.mail",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Emails"
    },
    "children": [
      {
        "tag": "header",
        "children": [
          {
            "tag": "button",
            "attrs": {
              "name": "action_retry",
              "string": "Retry",
              "type": "object"
            }
          }
        ]
      },
      {
        "tag": "field",
        "attrs": {
          "name": "date"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "subject"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "author_id",
          "string": "User"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "message_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "recipient_ids"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "model"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "res_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "email_from"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "message_type"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "decoration-danger": "state=='exception'",
          "decoration-info": "state=='outgoing'",
          "decoration-muted": "state in ('sent', 'cancel')",
          "name": "state",
          "widget": "badge"
        }
      },
      {
        "tag": "button",
        "attrs": {
          "icon": "fa-paper-plane",
          "invisible": "state != 'outgoing' or message_type == 'user_notification'",
          "name": "send",
          "string": "Send Now",
          "type": "object"
        }
      },
      {
        "tag": "button",
        "attrs": {
          "icon": "fa-repeat",
          "invisible": "state not in ('exception', 'cancel')",
          "name": "mark_outgoing",
          "string": "Retry",
          "type": "object"
        }
      },
      {
        "tag": "button",
        "attrs": {
          "icon": "fa-times-circle",
          "invisible": "state != 'outgoing'",
          "name": "cancel",
          "string": "Cancel Email",
          "type": "object"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "date"
    },
    {
      "name": "subject"
    },
    {
      "name": "author_id",
      "string": "User"
    },
    {
      "name": "message_id",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "recipient_ids",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "model",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "res_id",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "email_from",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "message_type",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "state",
      "widget": "badge",
      "attrs": {
        "decoration-danger": "state=='exception'",
        "decoration-info": "state=='outgoing'",
        "decoration-muted": "state in ('sent', 'cancel')"
      }
    }
  ],
  "buttons": [
    {
      "name": "action_retry",
      "string": "Retry",
      "type": "object"
    },
    {
      "name": "send",
      "string": "Send Now",
      "type": "object"
    },
    {
      "name": "mark_outgoing",
      "string": "Retry",
      "type": "object"
    },
    {
      "name": "cancel",
      "string": "Cancel Email",
      "type": "object"
    }
  ]
}

export function renderMailMailTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
