// Generado por odoo2rs — vista tree de fetchmail.server (view_email_server_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_email_server_tree",
  "name": "fetchmail.server.list",
  "model": "fetchmail.server",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "POP/IMAP Servers"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "name"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "server_type"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "user",
          "readonly": "state != 'draft'"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "date"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "decoration-info": "state == 'draft'",
          "decoration-success": "state == 'done'",
          "name": "state",
          "widget": "badge"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "name"
    },
    {
      "name": "server_type"
    },
    {
      "name": "user",
      "attrs": {
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "date"
    },
    {
      "name": "state",
      "widget": "badge",
      "attrs": {
        "decoration-info": "state == 'draft'",
        "decoration-success": "state == 'done'"
      }
    }
  ]
}

export function renderFetchmailServerTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
