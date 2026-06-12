// Generado por odoo2rs — vista xpath de product.product (product_view_search_catalog).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "product_view_search_catalog",
  "name": "product.view.search.catalog.inherit.account",
  "model": "product.product",
  "type": "xpath",
  "arch": {
    "tag": "xpath",
    "attrs": {
      "expr": "//field[@name='product_tmpl_id']",
      "position": "after"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "seller_ids",
          "string": "Vendor"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "seller_ids",
      "string": "Vendor"
    }
  ]
}

export function renderProductProductXpath(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
