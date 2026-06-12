// Generado por odoo2rs — vista tree de mail.followers (view_followers_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_followers_tree",
  "name": "mail.followers.list",
  "model": "mail.followers",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Followers"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "res_model"
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
          "name": "partner_id"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "res_model"
    },
    {
      "name": "res_id"
    },
    {
      "name": "partner_id"
    }
  ]
}

export function renderMailFollowersTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
