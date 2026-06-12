// Generado por odoo2rs — vista search de mail.message (view_message_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_message_search",
  "name": "mail.message.search",
  "model": "mail.message",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Messages Search"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "filter_domain": "['|', ('subject', 'ilike', self), ('body', 'ilike', self)]",
          "name": "body",
          "string": "Content"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "subject"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "message_type"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "author_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "partner_ids"
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
      },
      {
        "tag": "field",
        "attrs": {
          "name": "parent_id"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('partner_ids.user_ids', 'in', [uid])]",
          "name": "filter_has_mentions",
          "string": "Has Mentions"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('needaction', '=', True)]",
          "help": "Unread messages",
          "name": "message_needaction",
          "string": "Need Action"
        }
      },
      {
        "tag": "separator"
      }
    ]
  },
  "fields": [
    {
      "name": "body",
      "string": "Content",
      "attrs": {
        "filter_domain": "['|', ('subject', 'ilike', self), ('body', 'ilike', self)]"
      }
    },
    {
      "name": "subject"
    },
    {
      "name": "message_type"
    },
    {
      "name": "author_id"
    },
    {
      "name": "partner_ids"
    },
    {
      "name": "model"
    },
    {
      "name": "res_id"
    },
    {
      "name": "parent_id"
    }
  ]
}

export function renderMailMessageSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
