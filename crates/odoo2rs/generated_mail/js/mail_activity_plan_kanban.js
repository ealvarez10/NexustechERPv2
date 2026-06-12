// Generado por odoo2rs — vista kanban de mail.activity.plan (mail_activity_plan_view_kanban).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { kanbanViewHtml } from '../components/kanban_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail_activity_plan_view_kanban",
  "name": "mail.activity.plan.view.kanban",
  "model": "mail.activity.plan",
  "type": "kanban",
  "arch": {
    "tag": "kanban",
    "attrs": {
      "class": "o_kanban_mobile"
    },
    "children": [
      {
        "tag": "templates",
        "children": [
          {
            "tag": "t",
            "attrs": {
              "t-name": "card"
            },
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "class": "fw-bolder",
                  "name": "name"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "res_model_id"
                }
              },
              {
                "tag": "div",
                "children": [
                  {
                    "tag": "i",
                    "attrs": {
                      "aria-label": "Steps count",
                      "class": "fa fa-cogs fa-fw me-2",
                      "role": "img",
                      "title": "Steps count"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "steps_count"
                    }
                  }
                ]
              }
            ]
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "name",
      "attrs": {
        "class": "fw-bolder"
      }
    },
    {
      "name": "res_model_id"
    },
    {
      "name": "steps_count"
    }
  ]
}

export function renderMailActivityPlanKanban(records = []) {
  return kanbanViewHtml({
    // TODO(odoo2rs): columnas desde la selection del campo state del modelo
    columns: [],
    records,
    stateField: 'state',
  })
}
