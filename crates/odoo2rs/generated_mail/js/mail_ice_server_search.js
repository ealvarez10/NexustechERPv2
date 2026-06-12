// Generado por odoo2rs — vista search de mail.ice.server (view_ice_server_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_ice_server_search",
  "name": "mail.ice.server.search",
  "model": "mail.ice.server",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Search ICE Servers"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "uri"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "username"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "credential"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('server_type','=','stun')]",
          "name": "stun",
          "string": "STUN"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('server_type','=','turn')]",
          "name": "turn",
          "string": "TURN"
        }
      },
      {
        "tag": "group",
        "children": [
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'server_type'}",
              "name": "group_by_server_type",
              "string": "Server Type"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "uri"
    },
    {
      "name": "username"
    },
    {
      "name": "credential"
    }
  ]
}

export function renderMailIceServerSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
