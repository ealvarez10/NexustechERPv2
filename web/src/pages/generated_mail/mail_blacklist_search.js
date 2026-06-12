// Generado por odoo2rs — vista search de mail.blacklist (mail_blacklist_view_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_blacklist_view_search",
  "name": "mail.blacklist.view.search",
  "model": "mail.blacklist",
  "type": "search",
  "arch": {
    "tag": "search",
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "email"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('active','=',False)]",
          "name": "inactive",
          "string": "Archived"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "email"
    }
  ]
}

export function renderMailBlacklistSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
