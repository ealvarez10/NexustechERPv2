// Generado por odoo2rs — vista tree de mail.activity (mail_activity_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_activity_view_tree",
  "name": "mail.activity.view.list",
  "model": "mail.activity",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "create": "true",
      "default_order": "date_deadline",
      "string": "Next Activities"
    },
    "children": [
      {
        "tag": "header",
        "children": [
          {
            "tag": "button",
            "attrs": {
              "icon": "fa-check",
              "name": "action_done",
              "string": "Done",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "icon": "fa-times",
              "name": "action_cancel",
              "string": "Cancel",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "icon": "fa-arrow-down",
              "name": "action_reschedule_today",
              "string": "Today",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "icon": "fa-calendar-plus-o",
              "name": "action_reschedule_tomorrow",
              "string": "Tomorrow",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "icon": "fa-calendar-o",
              "name": "action_reschedule_nextweek",
              "string": "Next Week",
              "type": "object"
            }
          }
        ]
      },
      {
        "tag": "field",
        "attrs": {
          "name": "summary",
          "string": "Summary"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "activity_type_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "user_id",
          "widget": "many2one_avatar_user"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "res_name",
          "string": "Linked to"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "date_deadline",
          "widget": "remaining_days"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "date_done",
          "optional": "hide",
          "string": "Done Date"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "feedback",
          "optional": "hide"
        }
      },
      {
        "tag": "widget",
        "attrs": {
          "name": "mail_activity_list_reschedule_dropdown"
        }
      },
      {
        "tag": "button",
        "attrs": {
          "icon": "fa-check",
          "invisible": "active == False",
          "name": "action_done",
          "string": "Done",
          "type": "object"
        }
      },
      {
        "tag": "button",
        "attrs": {
          "class": "text-danger",
          "icon": "fa-times",
          "invisible": "active == False",
          "name": "unlink",
          "string": "Cancel",
          "type": "object"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "summary",
      "string": "Summary"
    },
    {
      "name": "activity_type_id"
    },
    {
      "name": "user_id",
      "widget": "many2one_avatar_user"
    },
    {
      "name": "res_name",
      "string": "Linked to"
    },
    {
      "name": "date_deadline",
      "widget": "remaining_days"
    },
    {
      "name": "date_done",
      "string": "Done Date",
      "attrs": {
        "optional": "hide"
      }
    },
    {
      "name": "feedback",
      "attrs": {
        "optional": "hide"
      }
    }
  ],
  "buttons": [
    {
      "name": "action_done",
      "string": "Done",
      "type": "object"
    },
    {
      "name": "action_cancel",
      "string": "Cancel",
      "type": "object"
    },
    {
      "name": "action_reschedule_today",
      "string": "Today",
      "type": "object"
    },
    {
      "name": "action_reschedule_tomorrow",
      "string": "Tomorrow",
      "type": "object"
    },
    {
      "name": "action_reschedule_nextweek",
      "string": "Next Week",
      "type": "object"
    },
    {
      "name": "action_done",
      "string": "Done",
      "type": "object"
    },
    {
      "name": "unlink",
      "string": "Cancel",
      "type": "object",
      "class": "text-danger"
    }
  ]
}

export function renderMailActivityTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
