// Generado por odoo2rs — vista search de mail.activity.plan (mail_activity_plan_view_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_activity_plan_view_search",
  "name": "mail.activity.plan.view.search",
  "model": "mail.activity.plan",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Plan"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "name"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('active', '=', False)]",
          "name": "filter_inactive",
          "string": "Archived"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "context": "{'group_by': 'res_model_id'}",
          "domain": "[]",
          "name": "group_by_model",
          "string": "Model"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "name"
    }
  ]
}

export function renderMailActivityPlanSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
