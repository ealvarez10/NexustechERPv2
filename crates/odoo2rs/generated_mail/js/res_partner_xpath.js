// Generado por odoo2rs — vista xpath de res.partner (res_partner_view_tree_inherit_mail).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "res_partner_view_tree_inherit_mail",
  "name": "res.partner.view.list.inherit.mail",
  "model": "res.partner",
  "type": "xpath",
  "arch": {
    "tag": "xpath",
    "attrs": {
      "expr": "//field[@name='user_id']",
      "position": "after"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "activity_ids",
          "optional": "show",
          "widget": "list_activity"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "activity_ids",
      "widget": "list_activity",
      "attrs": {
        "optional": "show"
      }
    }
  ]
}

export function renderResPartnerXpath(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
