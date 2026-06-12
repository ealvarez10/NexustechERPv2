// Generado por odoo2rs — vista search de account.group (view_account_group_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_group_search",
  "name": "account.group.search",
  "model": "account.group",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Account groups"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "filter_domain": "['|', ('code_prefix_start', '=like', self + '%'), ('name', 'ilike', self)]",
          "name": "name",
          "string": "Account group"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "name",
      "string": "Account group",
      "attrs": {
        "filter_domain": "['|', ('code_prefix_start', '=like', self + '%'), ('name', 'ilike', self)]"
      }
    }
  ]
}

export function renderAccountGroupSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
