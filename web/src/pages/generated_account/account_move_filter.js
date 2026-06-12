// Generado por odoo2rs — vista filter de account.move (view_account_move_with_gaps_in_sequence_filter).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_move_with_gaps_in_sequence_filter",
  "name": "account.move.with.gaps.in.sequence.filter",
  "model": "account.move",
  "type": "filter",
  "arch": {
    "tag": "filter",
    "attrs": {
      "name": "due_date",
      "position": "after"
    },
    "children": [
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "['|', ('made_sequence_gap', '=', True), '&', '&', ('state', 'in', ('draft', 'cancel')), ('sequence_number', '!=', 0), ('name', '!=', '/')]",
          "name": "irregular_sequences",
          "string": "Irregular Sequences"
        }
      }
    ]
  },
  "fields": []
}

export function renderAccountMoveFilter(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
