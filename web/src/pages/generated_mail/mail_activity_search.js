// Generado por odoo2rs — vista search de mail.activity (mail_activity_view_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_activity_view_search",
  "name": "mail.activity.view.search",
  "model": "mail.activity",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Activity"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "filter_domain": "[                     '|',                         ('res_name', 'ilike', self),                         ('summary', 'ilike', self),                 ]",
          "name": "res_name",
          "string": "Name"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "user_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "activity_type_id"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('user_id', '=', uid)]",
          "name": "filter_user_id_uid",
          "string": "My Activities"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('user_id', '=', False)]",
          "name": "filter_user_id_no_user",
          "string": "Unassigned Activities"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "separator",
        "attrs": {
          "invisible": "1"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('date_deadline', '<', 'today')]",
          "help": "Show all records whose next activity date is past",
          "name": "filter_date_deadline_past",
          "string": "Overdue"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('date_deadline', '=', 'today')]",
          "name": "filter_date_deadline_today",
          "string": "Today"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('date_deadline', '=', 'today +1d')]",
          "help": "Show all records whose next action date is tomorrow",
          "name": "filter_date_deadline_tomorrow",
          "string": "Tomorrow"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[                             ('date_deadline', '>=', '=week_start'),                             ('date_deadline', '<', '=week_start +1w')                         ]",
          "help": "Show all records whose next action date is this week",
          "name": "filter_date_deadline_week",
          "string": "This week"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('date_deadline', '>', 'today')                         ]",
          "name": "filter_date_deadline_future",
          "string": "Future"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('active', '=', False)]",
          "name": "filter_archived",
          "string": "Done"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "group",
        "children": [
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'date_deadline'}",
              "name": "date_deadline",
              "string": "Deadline"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'res_model_id'}",
              "name": "group_by_res_model_id",
              "string": "Document Model"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'user_id'}",
              "name": "group_by_user_id",
              "string": "Assigned To"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'create_uid'}",
              "name": "createdby",
              "string": "Created By"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'activity_type_id'}",
              "name": "activittype",
              "string": "Activity Type"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "res_name",
      "string": "Name",
      "attrs": {
        "filter_domain": "[                     '|',                         ('res_name', 'ilike', self),                         ('summary', 'ilike', self),                 ]"
      }
    },
    {
      "name": "user_id"
    },
    {
      "name": "activity_type_id"
    }
  ]
}

export function renderMailActivitySearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
