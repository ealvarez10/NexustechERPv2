// Generado por odoo2rs — vista xpath de res.partner (res_partner_view_form_inherit_mercadily).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "res_partner_view_form_inherit_mercadily",
  "name": "res.partner.form.inherit.mercadily",
  "model": "res.partner",
  "type": "xpath",
  "arch": {
    "tag": "xpath",
    "attrs": {
      "expr": "//notebook",
      "position": "inside"
    },
    "children": [
      {
        "tag": "page",
        "attrs": {
          "invisible": "not mercadily_backend_id",
          "name": "mercadily_tab",
          "string": "Mercadily"
        },
        "children": [
          {
            "tag": "group",
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "name": "mercadily_backend_id",
                  "readonly": "1"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "mercadily_id",
                  "readonly": "1"
                }
              }
            ]
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "mercadily_backend_id",
      "attrs": {
        "readonly": "1"
      }
    },
    {
      "name": "mercadily_id",
      "attrs": {
        "readonly": "1"
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
