// Generado por odoo2rs — vista pivot de account.move.line (view_move_line_pivot).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_move_line_pivot",
  "name": "account.move.line.pivot",
  "model": "account.move.line",
  "type": "pivot",
  "arch": {
    "tag": "pivot",
    "attrs": {
      "sample": "1",
      "string": "Journal Items"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "journal_id",
          "type": "row"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "date",
          "type": "col"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "balance",
          "type": "measure"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "journal_id",
      "attrs": {
        "type": "row"
      }
    },
    {
      "name": "date",
      "attrs": {
        "type": "col"
      }
    },
    {
      "name": "balance",
      "attrs": {
        "type": "measure"
      }
    }
  ]
}

export function renderAccountMoveLinePivot(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
