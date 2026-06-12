// Generado por odoo2rs — vista field de ir.actions.report (ir_actions_report_form_inherit_account).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "ir_actions_report_form_inherit_account",
  "name": "ir.actions.report.form.inherit.account",
  "model": "ir.actions.report",
  "type": "field",
  "arch": {
    "tag": "field",
    "attrs": {
      "name": "paperformat_id",
      "position": "after"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "invisible": "model != 'account.move'",
          "name": "is_invoice_report"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "is_invoice_report",
      "attrs": {
        "invisible": "model != 'account.move'"
      }
    }
  ]
}

export function renderIrActionsReportField(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
