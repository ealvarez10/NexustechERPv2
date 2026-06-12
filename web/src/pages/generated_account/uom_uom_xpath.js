// Generado por odoo2rs — vista xpath de uom.uom (product_uom_form_view_inherit).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "product_uom_form_view_inherit",
  "name": "product_uom_form_view_inherit",
  "model": "uom.uom",
  "type": "xpath",
  "arch": {
    "tag": "xpath",
    "attrs": {
      "expr": "//field[@name='name']",
      "position": "before"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "invisible": "1",
          "name": "fiscal_country_codes"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "fiscal_country_codes",
      "attrs": {
        "invisible": "1"
      }
    }
  ]
}

export function renderUomUomXpath(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
