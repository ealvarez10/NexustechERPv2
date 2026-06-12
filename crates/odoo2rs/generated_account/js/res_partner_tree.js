// Generado por odoo2rs — vista tree de res.partner (partner_missing_account_list_view).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "partner_missing_account_list_view",
  "name": "res.partner.list",
  "model": "res.partner",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "create": "false",
      "delete": "false",
      "edit": "false",
      "import": "false",
      "open_form_view": "True",
      "string": "Partners Missing a bank account"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "name"
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

export function renderResPartnerTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
