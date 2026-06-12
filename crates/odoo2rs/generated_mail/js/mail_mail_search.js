// Generado por odoo2rs — vista search de mail.mail (view_mail_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_mail_search",
  "name": "mail.mail.search",
  "model": "mail.mail",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Email Search"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "filter_domain": "['|', '|',('email_from','ilike',self), ('email_to','ilike',self), ('subject','ilike',self)]",
          "name": "email_from",
          "string": "Email"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "date"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('state','=','received')]",
          "name": "received",
          "string": "Received"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('state','=','outgoing')]",
          "name": "outgoing",
          "string": "Outgoing"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('state','=','sent')]",
          "name": "sent",
          "string": "Sent"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('state','=','exception')]",
          "name": "exception",
          "string": "Failed"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('message_type','=','email_outgoing')]",
          "name": "filter_type_email_outgoing",
          "string": "Outgoing Email"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('message_type','=','email')]",
          "name": "filter_type_email",
          "string": "Incoming Email"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('message_type','=','comment')]",
          "name": "filter_type_comment",
          "string": "Comment"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('message_type','=','notification')]",
          "name": "filter_type_notification",
          "string": "Notification"
        }
      },
      {
        "tag": "group",
        "children": [
          {
            "tag": "field",
            "attrs": {
              "name": "author_id"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "name": "recipient_ids"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "name": "model"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "name": "res_id"
            }
          }
        ]
      },
      {
        "tag": "group",
        "children": [
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by':'state'}",
              "domain": "[]",
              "name": "status",
              "string": "Status"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by':'author_id'}",
              "name": "author",
              "string": "Author"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by':'message_id'}",
              "domain": "[]",
              "name": "thread",
              "string": "Thread"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by':'date'}",
              "domain": "[]",
              "help": "Creation Date",
              "name": "month",
              "string": "Date"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "email_from",
      "string": "Email",
      "attrs": {
        "filter_domain": "['|', '|',('email_from','ilike',self), ('email_to','ilike',self), ('subject','ilike',self)]"
      }
    },
    {
      "name": "date"
    },
    {
      "name": "author_id"
    },
    {
      "name": "recipient_ids"
    },
    {
      "name": "model"
    },
    {
      "name": "res_id"
    }
  ]
}

export function renderMailMailSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
