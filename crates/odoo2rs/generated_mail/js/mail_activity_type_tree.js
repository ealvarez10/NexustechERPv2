// Generado por odoo2rs — vista tree de mail.activity.type (mail_activity_type_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_activity_type_view_tree",
  "name": "mail.activity.type.view.list",
  "model": "mail.activity.type",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "sample": "1",
      "string": "Activities"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "sequence",
          "widget": "handle"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "name"
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
          "class": "text-end",
          "name": "delay_label",
          "string": "Planned in"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "delay_from",
          "string": "Type"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "context.get('default_res_model')",
          "name": "res_model"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_no_one",
          "name": "icon"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "triggered_next_type_id",
          "optional": "hide",
          "string": "Triggered Next"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "suggested_next_type_ids",
          "optional": "hide",
          "string": "Suggested Next",
          "widget": "many2many_tags"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "sequence",
      "widget": "handle"
    },
    {
      "name": "name"
    },
    {
      "name": "summary"
    },
    {
      "name": "delay_label",
      "string": "Planned in",
      "attrs": {
        "class": "text-end"
      }
    },
    {
      "name": "delay_from",
      "string": "Type"
    },
    {
      "name": "res_model",
      "attrs": {
        "column_invisible": "context.get('default_res_model')"
      }
    },
    {
      "name": "icon",
      "attrs": {
        "groups": "base.group_no_one"
      }
    },
    {
      "name": "triggered_next_type_id",
      "string": "Triggered Next",
      "attrs": {
        "optional": "hide"
      }
    },
    {
      "name": "suggested_next_type_ids",
      "string": "Suggested Next",
      "widget": "many2many_tags",
      "attrs": {
        "optional": "hide"
      }
    }
  ]
}

export function renderMailActivityTypeTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
