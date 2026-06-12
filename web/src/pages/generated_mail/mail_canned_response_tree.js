// Generado por odoo2rs — vista tree de mail.canned.response (mail_canned_response_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_canned_response_view_tree",
  "name": "mail.canned.response.list",
  "model": "mail.canned.response",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "default_order": "is_shared",
      "editable": "bottom",
      "sample": "1",
      "string": "Canned responses"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "source",
          "placeholder": "e.g. hello",
          "readonly": "not is_editable",
          "widget": "shortcut"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "substitution",
          "placeholder": "e.g. Hello, how may I help you?",
          "readonly": "not is_editable"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "create_uid",
          "optional": "hide",
          "widget": "many2one_avatar_user"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "group_ids",
          "optional": "hide",
          "readonly": "not is_editable",
          "widget": "many2many_tags"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "last_used",
          "optional": "hide",
          "readonly": "1"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "is_editable"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "is_shared"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "source",
      "widget": "shortcut",
      "attrs": {
        "placeholder": "e.g. hello",
        "readonly": "not is_editable"
      }
    },
    {
      "name": "substitution",
      "attrs": {
        "placeholder": "e.g. Hello, how may I help you?",
        "readonly": "not is_editable"
      }
    },
    {
      "name": "create_uid",
      "widget": "many2one_avatar_user",
      "attrs": {
        "optional": "hide"
      }
    },
    {
      "name": "group_ids",
      "widget": "many2many_tags",
      "attrs": {
        "optional": "hide",
        "readonly": "not is_editable"
      }
    },
    {
      "name": "last_used",
      "attrs": {
        "optional": "hide",
        "readonly": "1"
      }
    },
    {
      "name": "is_editable",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "is_shared",
      "attrs": {
        "column_invisible": "True"
      }
    }
  ]
}

export function renderMailCannedResponseTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
