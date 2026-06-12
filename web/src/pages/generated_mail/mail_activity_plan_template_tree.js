// Generado por odoo2rs — vista tree de mail.activity.plan.template (mail_activity_plan_template_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_activity_plan_template_view_tree",
  "name": "mail.activity.plan.template.view.list",
  "model": "mail.activity.plan.template",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Activities"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "activity_type_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "summary",
          "placeholder": "e.g. Discuss Proposal"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "responsible_type"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "delay_count"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "delay_unit"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "delay_from"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "activity_type_id"
    },
    {
      "name": "summary",
      "attrs": {
        "placeholder": "e.g. Discuss Proposal"
      }
    },
    {
      "name": "responsible_type"
    },
    {
      "name": "delay_count"
    },
    {
      "name": "delay_unit"
    },
    {
      "name": "delay_from"
    }
  ]
}

export function renderMailActivityPlanTemplateTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
