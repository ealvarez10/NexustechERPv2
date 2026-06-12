// Generado por odoo2rs — vista field de ir.model.fields (field_form_view).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "field_form_view",
  "model": "ir.model.fields",
  "type": "field",
  "arch": {
    "tag": "field",
    "attrs": {
      "name": "copied",
      "position": "after"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "invisible": "1",
          "name": "state"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "invisible": "ttype in ['binary', 'html']",
          "name": "tracking",
          "readonly": "state != 'manual'"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "state",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "tracking",
      "attrs": {
        "invisible": "ttype in ['binary', 'html']",
        "readonly": "state != 'manual'"
      }
    }
  ]
}

export function renderIrModelFieldsField(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
