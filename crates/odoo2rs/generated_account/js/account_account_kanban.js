// Generado por odoo2rs — vista kanban de account.account (view_account_account_kanban).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { kanbanViewHtml } from '../components/kanban_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_account_account_kanban",
  "name": "account.account.kanban",
  "model": "account.account",
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
                "tag": "div",
                "attrs": {
                  "class": "row"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "col-8"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "class": "fw-bolder",
                          "name": "name"
                        }
                      }
                    ]
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "col-4 text-end"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "class": "badge rounded-pill",
                          "name": "code"
                        }
                      }
                    ]
                  }
                ]
              },
              {
                "tag": "div",
                "children": [
                  {
                    "tag": "strong",
                    "text": "Type:"
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "account_type"
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
      "name": "code",
      "attrs": {
        "class": "badge rounded-pill"
      }
    },
    {
      "name": "account_type"
    }
  ]
}

export function renderAccountAccountKanban(records = []) {
  return kanbanViewHtml({
    // TODO(odoo2rs): columnas desde la selection del campo state del modelo
    columns: [],
    records,
    stateField: 'state',
  })
}
