// Generado por odoo2rs — vista kanban de mail.activity.type (mail_activity_type_view_kanban).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { kanbanViewHtml } from '../components/kanban_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail_activity_type_view_kanban",
  "name": "mail.activity.type.view.kanban",
  "model": "mail.activity.type",
  "type": "kanban",
  "arch": {
    "tag": "kanban",
    "attrs": {
      "class": "o_kanban_mobile"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "icon"
        }
      },
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
                "tag": "div",
                "attrs": {
                  "class": "fw-bold fs-5"
                },
                "children": [
                  {
                    "tag": "i",
                    "attrs": {
                      "aria-label": "Activity Type Name",
                      "role": "img",
                      "t-attf-class": "fa #{record.icon.value} fa-fw",
                      "t-if": "record.icon.value",
                      "title": "Activity Type Name"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "name"
                    }
                  }
                ]
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "res_model",
                  "t-if": "record.res_model.value"
                }
              },
              {
                "tag": "div",
                "attrs": {
                  "t-if": "record.summary.raw_value"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "summary"
                    }
                  }
                ],
                "text": "Default Summary:"
              },
              {
                "tag": "footer",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "ms-auto",
                      "name": "default_user_id",
                      "readonly": "1",
                      "widget": "many2one_avatar_user"
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
      "name": "icon"
    },
    {
      "name": "name"
    },
    {
      "name": "res_model",
      "attrs": {
        "t-if": "record.res_model.value"
      }
    },
    {
      "name": "summary"
    },
    {
      "name": "default_user_id",
      "widget": "many2one_avatar_user",
      "attrs": {
        "class": "ms-auto",
        "readonly": "1"
      }
    }
  ]
}

export function renderMailActivityTypeKanban(records = []) {
  return kanbanViewHtml({
    // TODO(odoo2rs): columnas desde la selection del campo state del modelo
    columns: [],
    records,
    stateField: 'state',
  })
}
