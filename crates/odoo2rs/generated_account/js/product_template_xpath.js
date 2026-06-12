// Generado por odoo2rs — vista xpath de product.template (product_template_list_view_purchasable_inherit).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "product_template_list_view_purchasable_inherit",
  "name": "product.template.list.purchasable.inherit",
  "model": "product.template",
  "type": "xpath",
  "arch": {
    "tag": "xpath",
    "attrs": {
      "expr": "//field[@name='standard_price']",
      "position": "after"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "context": "{'search_default_domestictax': True, 'append_fields': ['company_id']}",
          "name": "supplier_taxes_id",
          "widget": "many2many_tax_tags"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "supplier_taxes_id",
      "widget": "many2many_tax_tags",
      "attrs": {
        "context": "{'search_default_domestictax': True, 'append_fields': ['company_id']}"
      }
    }
  ]
}

export function renderProductTemplateXpath(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
