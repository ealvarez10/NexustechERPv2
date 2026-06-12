// Generado por odoo2rs — vista xpath de sale.order (sale_order_view_list_inherit_mercadily).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "sale_order_view_list_inherit_mercadily",
  "name": "sale.order.list.inherit.mercadily",
  "model": "sale.order",
  "type": "xpath",
  "arch": {
    "tag": "xpath",
    "attrs": {
      "expr": "//field[@name='state']",
      "position": "after"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "mercadily_status",
          "optional": "hide",
          "string": "Estatus Tienda"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "mercadily_status",
      "string": "Estatus Tienda",
      "attrs": {
        "optional": "hide"
      }
    }
  ]
}

export function renderSaleOrderXpath(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
