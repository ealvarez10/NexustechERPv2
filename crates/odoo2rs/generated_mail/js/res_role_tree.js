// Generado por odoo2rs — vista tree de res.role (res_role_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "res_role_view_tree",
  "name": "res.role.list",
  "model": "res.role",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "editable": "bottom",
      "sample": "1",
      "string": "Role"
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
          "name": "user_ids",
          "widget": "many2many_avatar_user"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "name",
      "string": "Role"
    },
    {
      "name": "user_ids",
      "widget": "many2many_avatar_user"
    }
  ]
}

export function renderResRoleTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
