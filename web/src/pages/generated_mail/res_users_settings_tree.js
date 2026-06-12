// Generado por odoo2rs — vista tree de res.users.settings (res_users_settings_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "res_users_settings_view_tree",
  "name": "res.users.settings.list",
  "model": "res.users.settings",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "User Settings"
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
          "name": "user_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "use_push_to_talk"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "id"
    },
    {
      "name": "user_id"
    },
    {
      "name": "use_push_to_talk"
    }
  ]
}

export function renderResUsersSettingsTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
