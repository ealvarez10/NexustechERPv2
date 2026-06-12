// Generado por odoo2rs — vista tree de account.account.tag (account_tag_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "account_tag_view_tree",
  "name": "Tags",
  "model": "account.account.tag",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Tags"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "name"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "applicability"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "country_id"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "name"
    },
    {
      "name": "applicability"
    },
    {
      "name": "country_id"
    }
  ]
}

export function renderAccountAccountTagTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
