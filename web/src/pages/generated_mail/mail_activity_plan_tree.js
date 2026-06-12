// Generado por odoo2rs — vista tree de mail.activity.plan (mail_activity_plan_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_activity_plan_view_tree",
  "name": "mail.activity.plan.view.list",
  "model": "mail.activity.plan",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "sample": "1",
      "string": "Planning"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "name"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "res_model_id",
          "optional": "hide"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "steps_count"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_multi_company",
          "name": "company_id",
          "optional": "hide"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "name"
    },
    {
      "name": "res_model_id",
      "attrs": {
        "optional": "hide"
      }
    },
    {
      "name": "steps_count"
    },
    {
      "name": "company_id",
      "attrs": {
        "groups": "base.group_multi_company",
        "optional": "hide"
      }
    }
  ]
}

export function renderMailActivityPlanTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
