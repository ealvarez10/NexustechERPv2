// Generado por odoo2rs — vista kanban de mail.canned.response (mail_canned_response_view_kanban).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { kanbanViewHtml } from '../components/kanban_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail_canned_response_view_kanban",
  "name": "mail.canned.response.kanban",
  "model": "mail.canned.response",
  "type": "kanban",
  "arch": {
    "tag": "kanban",
    "attrs": {
      "class": "o_kanban_mobile",
      "sample": "1"
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
                "tag": "div",
                "attrs": {
                  "class": "oe_kanban_global_click p-2 d-flex flex-column gap-1"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "fw-bold fs-5"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "source",
                          "widget": "shortcut"
                        }
                      }
                    ]
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "text-truncate",
                      "t-att-title": "record.substitution.value"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "substitution"
                        }
                      }
                    ]
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "pt-1"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "group_ids",
                          "widget": "many2many_tags"
                        }
                      }
                    ]
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
      "name": "source",
      "widget": "shortcut"
    },
    {
      "name": "substitution"
    },
    {
      "name": "group_ids",
      "widget": "many2many_tags"
    }
  ]
}

export function renderMailCannedResponseKanban(records = []) {
  return kanbanViewHtml({
    // TODO(odoo2rs): columnas desde la selection del campo state del modelo
    columns: [],
    records,
    stateField: 'state',
  })
}
