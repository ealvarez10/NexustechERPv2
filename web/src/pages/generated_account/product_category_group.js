// Generado por odoo2rs — vista group de product.category (view_category_property_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_category_property_form",
  "name": "product.category.property.form.inherit",
  "model": "product.category",
  "type": "group",
  "arch": {
    "tag": "group",
    "attrs": {
      "name": "first",
      "position": "after"
    },
    "children": [
      {
        "tag": "group",
        "attrs": {
          "name": "account_property"
        },
        "children": [
          {
            "tag": "group",
            "attrs": {
              "groups": "account.group_account_readonly",
              "string": "Account Properties"
            },
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "name": "property_account_income_categ_id"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "property_account_expense_categ_id"
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
      "name": "property_account_income_categ_id"
    },
    {
      "name": "property_account_expense_categ_id"
    }
  ]
}

export function renderProductCategoryGroup(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
