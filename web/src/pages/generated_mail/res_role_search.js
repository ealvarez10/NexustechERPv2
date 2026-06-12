// Generado por odoo2rs — vista search de res.role (res_role_view_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "res_role_view_search",
  "name": "res.role.view.search",
  "model": "res.role",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Roles Search"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "name",
          "string": "Role"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "user_ids"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('user_ids', '=', uid)]",
          "name": "my_role_ids",
          "string": "My Roles"
        }
      },
      {
        "tag": "group",
        "children": [
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'user_ids'}",
              "domain": "[]",
              "name": "filter_user_ids",
              "string": "Users"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "name",
      "string": "Role"
    },
    {
      "name": "user_ids"
    }
  ]
}

export function renderResRoleSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
