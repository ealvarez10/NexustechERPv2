// Generado por odoo2rs — vista xpath de crm.lead (crm_lead_view_list_inherit_mercadily).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "crm_lead_view_list_inherit_mercadily",
  "name": "crm.lead.list.inherit.mercadily",
  "model": "crm.lead",
  "type": "xpath",
  "arch": {
    "tag": "xpath",
    "attrs": {
      "expr": "//field[@name='email_from']",
      "position": "after"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "mercadily_status",
          "optional": "hide"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "mercadily_status",
      "attrs": {
        "optional": "hide"
      }
    }
  ]
}

export function renderCrmLeadXpath(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
