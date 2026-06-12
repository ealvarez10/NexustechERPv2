// Generado por odoo2rs — vista search de fetchmail.server (view_email_server_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_email_server_search",
  "name": "fetchmail.server.search",
  "model": "fetchmail.server",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Search Incoming Mail Servers"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "name",
          "string": "Incoming Mail Server"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "user"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('server_type', '=', 'imap')]",
          "help": "Server type IMAP.",
          "name": "imap",
          "string": "IMAP"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('server_type', '=', 'pop')]",
          "help": "Server type POP.",
          "name": "pop",
          "string": "POP"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('is_ssl', '=', True)]",
          "help": "If SSL required.",
          "name": "ssl",
          "string": "SSL"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('active', '=', False)]",
          "name": "inactive",
          "string": "Archived"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "name",
      "string": "Incoming Mail Server"
    },
    {
      "name": "user"
    }
  ]
}

export function renderFetchmailServerSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
