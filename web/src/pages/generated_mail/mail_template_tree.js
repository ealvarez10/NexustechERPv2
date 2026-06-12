// Generado por odoo2rs — vista tree de mail.template (email_template_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "email_template_tree",
  "name": "email.template.list",
  "model": "mail.template",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Templates"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "mail_server_id"
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
          "groups": "base.group_no_one",
          "name": "model_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "user_id",
          "optional": "show",
          "widget": "many2one_avatar_user"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "description"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "subject",
          "optional": "hidden"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "email_from",
          "optional": "hidden"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "email_to",
          "optional": "hidden"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "partner_to",
          "optional": "hidden"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "mail_server_id",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "name"
    },
    {
      "name": "model_id",
      "attrs": {
        "groups": "base.group_no_one"
      }
    },
    {
      "name": "user_id",
      "widget": "many2one_avatar_user",
      "attrs": {
        "optional": "show"
      }
    },
    {
      "name": "description"
    },
    {
      "name": "subject",
      "attrs": {
        "optional": "hidden"
      }
    },
    {
      "name": "email_from",
      "attrs": {
        "optional": "hidden"
      }
    },
    {
      "name": "email_to",
      "attrs": {
        "optional": "hidden"
      }
    },
    {
      "name": "partner_to",
      "attrs": {
        "optional": "hidden"
      }
    }
  ]
}

export function renderMailTemplateTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
