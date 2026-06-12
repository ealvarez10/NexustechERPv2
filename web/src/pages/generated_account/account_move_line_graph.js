// Generado por odoo2rs — vista graph de account.move.line (account_move_line_graph_date).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "account_move_line_graph_date",
  "name": "account.move.line.graph",
  "model": "account.move.line",
  "type": "graph",
  "arch": {
    "tag": "graph",
    "attrs": {
      "sample": "1",
      "string": "Account Statistics"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "date"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "balance",
          "operator": "+",
          "type": "measure"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "date"
    },
    {
      "name": "balance",
      "attrs": {
        "operator": "+",
        "type": "measure"
      }
    }
  ]
}

export function renderAccountMoveLineGraph(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
