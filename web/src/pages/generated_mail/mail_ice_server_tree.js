// Generado por odoo2rs — vista tree de mail.ice.server (view_ice_server_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_ice_server_tree",
  "name": "mail.ice.server.list",
  "model": "mail.ice.server",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "editable": "bottom",
      "sample": "1"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "colspan": "1",
          "name": "server_type"
        }
      },
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
      }
    ]
  },
  "fields": [
    {
      "name": "server_type",
      "attrs": {
        "colspan": "1"
      }
    },
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

export function renderMailIceServerTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
