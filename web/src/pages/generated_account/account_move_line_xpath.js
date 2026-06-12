// Generado por odoo2rs — vista xpath de account.move.line (account_move_line_view_kanban_mobile).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "account_move_line_view_kanban_mobile",
  "name": "account.move.line.kanban.mobile",
  "model": "account.move.line",
  "type": "xpath",
  "arch": {
    "tag": "xpath",
    "attrs": {
      "expr": "//kanban[hasclass('o_kanban_mobile')]",
      "position": "attributes"
    },
    "children": [
      {
        "tag": "attribute",
        "attrs": {
          "name": "create"
        },
        "text": "true"
      }
    ]
  },
  "fields": []
}

export function renderAccountMoveLineXpath(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
