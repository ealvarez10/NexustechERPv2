// Generado por odoo2rs — vista xpath de mail.activity.plan (mail_activity_plan_view_form_fixed_model).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_activity_plan_view_form_fixed_model",
  "name": "mail.activity.plan.view.form.fixed.model",
  "model": "mail.activity.plan",
  "type": "xpath",
  "arch": {
    "tag": "xpath",
    "attrs": {
      "expr": "//list",
      "position": "attributes"
    },
    "children": [
      {
        "tag": "attribute",
        "attrs": {
          "name": "editable"
        },
        "text": "bottom"
      }
    ]
  },
  "fields": []
}

export function renderMailActivityPlanXpath(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
