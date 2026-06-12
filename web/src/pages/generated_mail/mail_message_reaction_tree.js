// Generado por odoo2rs — vista tree de mail.message.reaction (mail_message_reaction_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_message_reaction_view_tree",
  "name": "mail.message.reaction.list",
  "model": "mail.message.reaction",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "create": "0",
      "edit": "0",
      "string": "Reactions"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "message_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "content"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "partner_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "guest_id"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "id"
    },
    {
      "name": "message_id"
    },
    {
      "name": "content"
    },
    {
      "name": "partner_id"
    },
    {
      "name": "guest_id"
    }
  ]
}

export function renderMailMessageReactionTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
