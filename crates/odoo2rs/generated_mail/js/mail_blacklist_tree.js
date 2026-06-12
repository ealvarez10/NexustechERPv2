// Generado por odoo2rs — vista tree de mail.blacklist (mail_blacklist_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_blacklist_view_tree",
  "name": "mail.blacklist.view.list",
  "model": "mail.blacklist",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Email Blacklist"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "create_date",
          "string": "Blacklist Date"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "email"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "create_date",
      "string": "Blacklist Date"
    },
    {
      "name": "email"
    }
  ]
}

export function renderMailBlacklistTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
