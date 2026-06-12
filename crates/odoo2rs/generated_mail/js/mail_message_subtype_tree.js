// Generado por odoo2rs — vista tree de mail.message.subtype (view_message_subtype_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_message_subtype_tree",
  "name": "mail.message.subtype.list",
  "model": "mail.message.subtype",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Subtype"
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
          "name": "res_model"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "default"
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
      "name": "res_model"
    },
    {
      "name": "default"
    }
  ]
}

export function renderMailMessageSubtypeTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
