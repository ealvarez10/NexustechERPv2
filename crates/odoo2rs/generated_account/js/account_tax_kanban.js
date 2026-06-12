// Generado por odoo2rs — vista kanban de account.tax (view_tax_kanban).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { kanbanViewHtml } from '../components/kanban_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_tax_kanban",
  "name": "account.tax.kanban",
  "model": "account.tax",
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
                  "class": "row mb4"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "col-6"
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
                      "class": "col-6 text-end"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "class": "badge rounded-pill",
                          "name": "type_tax_use"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "class": "badge rounded-pill",
                          "name": "tax_scope"
                        }
                      }
                    ]
                  }
                ]
              },
              {
                "tag": "field",
                "attrs": {
                  "class": "text-muted",
                  "name": "description"
                }
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
      "name": "type_tax_use",
      "attrs": {
        "class": "badge rounded-pill"
      }
    },
    {
      "name": "tax_scope",
      "attrs": {
        "class": "badge rounded-pill"
      }
    },
    {
      "name": "description",
      "attrs": {
        "class": "text-muted"
      }
    }
  ]
}

export function renderAccountTaxKanban(records = []) {
  return kanbanViewHtml({
    // TODO(odoo2rs): columnas desde la selection del campo state del modelo
    columns: [],
    records,
    stateField: 'state',
  })
}
