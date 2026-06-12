// Generado por odoo2rs — vista data de res.users (view_users_form_mail).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_users_form_mail",
  "name": "res.users.form.mail",
  "model": "res.users",
  "type": "data",
  "arch": {
    "tag": "data",
    "children": [
      {
        "tag": "xpath",
        "attrs": {
          "expr": "//group[@name='other_preferences']",
          "position": "inside"
        },
        "children": [
          {
            "tag": "field",
            "attrs": {
              "invisible": "share",
              "name": "notification_type",
              "options": "{'horizontal': true}",
              "widget": "radio"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "outgoing_mail_server_id"
            }
          },
          {
            "tag": "span",
            "attrs": {
              "class": "o_form_label fw-bold",
              "invisible": "not has_external_mail_server"
            },
            "text": "Outgoing Mail Server"
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "not has_external_mail_server",
              "name": "outgoing_mail_server_type",
              "nolabel": "1",
              "string": "Outgoing Mail Server",
              "widget": "mail_server_configurator_selection"
            }
          }
        ]
      },
      {
        "tag": "field",
        "attrs": {
          "name": "signature",
          "position": "attributes"
        },
        "children": [
          {
            "tag": "attribute",
            "attrs": {
              "name": "widget"
            },
            "text": "html_mail"
          }
        ]
      },
      {
        "tag": "group",
        "attrs": {
          "name": "calendar_preferences",
          "position": "inside"
        },
        "children": [
          {
            "tag": "label",
            "attrs": {
              "for": "out_of_office_from",
              "string": "Out-of-office"
            }
          },
          {
            "tag": "div",
            "attrs": {
              "class": "o_row"
            },
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "name": "out_of_office_from",
                  "options": "{'end_date_field': 'out_of_office_to', 'show_time': False}",
                  "placeholder": "None planned",
                  "widget": "daterange"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "invisible": "1",
                  "name": "out_of_office_to"
                }
              }
            ]
          },
          {
            "tag": "field",
            "attrs": {
              "class": "border border-secondary w-100",
              "name": "out_of_office_message",
              "options": "{'height': 112}",
              "placeholder": "Your out-of-office message...",
              "string": ""
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "notification_type",
      "widget": "radio",
      "attrs": {
        "invisible": "share",
        "options": "{'horizontal': true}"
      }
    },
    {
      "name": "outgoing_mail_server_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "outgoing_mail_server_type",
      "string": "Outgoing Mail Server",
      "widget": "mail_server_configurator_selection",
      "attrs": {
        "invisible": "not has_external_mail_server",
        "nolabel": "1"
      }
    },
    {
      "name": "signature",
      "attrs": {
        "position": "attributes"
      }
    },
    {
      "name": "out_of_office_from",
      "widget": "daterange",
      "attrs": {
        "options": "{'end_date_field': 'out_of_office_to', 'show_time': False}",
        "placeholder": "None planned"
      }
    },
    {
      "name": "out_of_office_to",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "out_of_office_message",
      "string": "",
      "attrs": {
        "class": "border border-secondary w-100",
        "options": "{'height': 112}",
        "placeholder": "Your out-of-office message..."
      }
    }
  ]
}

export function renderResUsersData(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
