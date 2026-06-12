// Generado por odoo2rs — vista search de account.incoterms (account_incoterms_view_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "account_incoterms_view_search",
  "name": "account.incoterms.search",
  "model": "account.incoterms",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Incoterms"
    },
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
          "name": "inactive",
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

export function renderAccountIncotermsSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
