// Generado por odoo2rs — vista tree de discuss.call.history (discuss_call_history_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "discuss_call_history_view_tree",
  "name": "discuss.call.history.view.list",
  "model": "discuss.call.history",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "sample": "1",
      "string": "Call History"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "channel_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "start_dt"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "end_dt"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "duration_hour",
          "options": "{'displaySeconds': True}",
          "widget": "float_time"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "channel_id"
    },
    {
      "name": "start_dt"
    },
    {
      "name": "end_dt"
    },
    {
      "name": "duration_hour",
      "widget": "float_time",
      "attrs": {
        "options": "{'displaySeconds': True}"
      }
    }
  ]
}

export function renderDiscussCallHistoryTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
