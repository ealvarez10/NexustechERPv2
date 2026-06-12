// Generado por odoo2rs — vista field de product.product (product_product_view_form_normalized_account).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "product_product_view_form_normalized_account",
  "name": "product.product.view.form.normalized.account.inherit",
  "model": "product.product",
  "type": "field",
  "arch": {
    "tag": "field",
    "attrs": {
      "name": "list_price",
      "position": "after"
    },
    "children": [
      {
        "tag": "label",
        "attrs": {
          "for": "taxes_id"
        }
      },
      {
        "tag": "div",
        "attrs": {
          "class": "o_row",
          "name": "tax_info"
        },
        "children": [
          {
            "tag": "field",
            "attrs": {
              "context": "{'search_default_sale': 1}",
              "name": "taxes_id",
              "options": "{'create': false, 'create_edit': false}",
              "widget": "many2many_tags"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "name": "tax_string"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "taxes_id",
      "widget": "many2many_tags",
      "attrs": {
        "context": "{'search_default_sale': 1}",
        "options": "{'create': false, 'create_edit': false}"
      }
    },
    {
      "name": "tax_string"
    }
  ]
}

export function renderProductProductField(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
