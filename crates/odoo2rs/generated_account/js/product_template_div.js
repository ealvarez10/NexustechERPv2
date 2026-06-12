// Generado por odoo2rs — vista div de product.template (product_template_form_view).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "product_template_form_view",
  "name": "product.template.form.inherit",
  "model": "product.template",
  "type": "div",
  "arch": {
    "tag": "div",
    "attrs": {
      "name": "options",
      "position": "inside"
    },
    "children": [
      {
        "tag": "span",
        "attrs": {
          "class": "d-inline-flex",
          "invisible": "type == 'combo'"
        },
        "children": [
          {
            "tag": "field",
            "attrs": {
              "name": "purchase_ok"
            }
          },
          {
            "tag": "label",
            "attrs": {
              "for": "purchase_ok"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "purchase_ok"
    }
  ]
}

export function renderProductTemplateDiv(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
