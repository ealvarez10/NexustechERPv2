// Generado por odoo2rs — vista search de account.account.tag (account_tag_view_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "account_tag_view_search",
  "name": "account.tag.view.search",
  "model": "account.account.tag",
  "type": "search",
  "arch": {
    "tag": "search",
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "name"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('active', '=', False)]",
          "name": "archived",
          "string": "Archived"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "name"
    }
  ]
}

export function renderAccountAccountTagSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
