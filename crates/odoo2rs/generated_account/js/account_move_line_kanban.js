// Generado por odoo2rs — vista kanban de account.move.line (account_move_line_view_kanban).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { kanbanViewHtml } from '../components/kanban_view.js'

export const DESCRIPTOR = {
  "xml_id": "account_move_line_view_kanban",
  "name": "account.move.line.kanban",
  "model": "account.move.line",
  "type": "kanban",
  "arch": {
    "tag": "kanban",
    "attrs": {
      "class": "o_kanban_mobile",
      "create": "false",
      "group_create": "false"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "company_currency_id"
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
                  "class": "row mb4"
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
                          "name": "account_id"
                        }
                      }
                    ]
                  },
                  {
                    "tag": "strong",
                    "attrs": {
                      "class": "col-4 ps-0 text-end",
                      "t-if": "record.date_maturity.raw_value"
                    },
                    "children": [
                      {
                        "tag": "i",
                        "attrs": {
                          "aria-label": "Date",
                          "class": "fa fa-clock-o",
                          "role": "img",
                          "title": "Date"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "date_maturity"
                        }
                      }
                    ]
                  }
                ]
              },
              {
                "tag": "div",
                "attrs": {
                  "class": "row mb4",
                  "style": "min-height: 60px;"
                },
                "children": [
                  {
                    "tag": "em",
                    "attrs": {
                      "class": "col-10"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "name"
                        }
                      }
                    ]
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "col-2 text-end"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "partner_id",
                          "options": "{'preview_image': 'avatar_128'}",
                          "widget": "image"
                        }
                      }
                    ]
                  }
                ]
              },
              {
                "tag": "div",
                "attrs": {
                  "class": "row"
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
                          "name": "tax_ids",
                          "widget": "many2many_tax_tags"
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
                        "tag": "t",
                        "attrs": {
                          "t-if": "record.debit.raw_value > 0"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "debit"
                            }
                          },
                          {
                            "tag": "span",
                            "text": "(DR)"
                          }
                        ]
                      },
                      {
                        "tag": "t",
                        "attrs": {
                          "t-if": "record.credit.raw_value > 0"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "credit"
                            }
                          },
                          {
                            "tag": "span",
                            "text": "(CR)"
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
      }
    ]
  },
  "fields": [
    {
      "name": "company_currency_id"
    },
    {
      "name": "account_id"
    },
    {
      "name": "date_maturity"
    },
    {
      "name": "name"
    },
    {
      "name": "partner_id",
      "widget": "image",
      "attrs": {
        "options": "{'preview_image': 'avatar_128'}"
      }
    },
    {
      "name": "tax_ids",
      "widget": "many2many_tax_tags"
    },
    {
      "name": "debit"
    },
    {
      "name": "credit"
    }
  ]
}

export function renderAccountMoveLineKanban(records = []) {
  return kanbanViewHtml({
    // TODO(odoo2rs): columnas desde la selection del campo state del modelo
    columns: [],
    records,
    stateField: 'state',
  })
}
