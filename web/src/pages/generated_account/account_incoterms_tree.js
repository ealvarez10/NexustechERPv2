// Generado por odoo2rs — vista tree de account.incoterms (view_incoterms_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_incoterms_tree",
  "name": "account.incoterms.list",
  "model": "account.incoterms",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "editable": "bottom",
      "string": "Incoterms"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "active"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "code"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "colspan": "4",
          "name": "name"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "active",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "code"
    },
    {
      "name": "name",
      "attrs": {
        "colspan": "4"
      }
    }
  ]
}

export function renderAccountIncotermsTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
