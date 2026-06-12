// Generado por odoo2rs — vista kanban de account.move (view_account_move_kanban).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { kanbanViewHtml } from '../components/kanban_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_account_move_kanban",
  "name": "account.move.kanban",
  "model": "account.move",
  "type": "kanban",
  "arch": {
    "tag": "kanban",
    "attrs": {
      "class": "o_kanban_mobile",
      "js_class": "account_documents_kanban",
      "sample": "1"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "currency_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "checked"
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
                  "class": "d-flex align-items-baseline mb-2"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "fw-bolder fs-5 me-2",
                      "invisible": "not partner_id",
                      "name": "partner_id",
                      "readonly": "state != 'draft'"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "fw-bolder fs-5 me-2",
                      "invisible": "partner_id",
                      "name": "journal_id"
                    }
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "groups": "account.group_account_user",
                      "invisible": "checked or state != 'posted'"
                    },
                    "children": [
                      {
                        "tag": "span",
                        "attrs": {
                          "class": "badge text-bg-info"
                        },
                        "text": "To Review"
                      }
                    ]
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "fw-bolder ms-auto flex-shrink-0",
                      "name": "amount_total_in_currency_signed",
                      "widget": "monetary"
                    }
                  }
                ]
              },
              {
                "tag": "footer",
                "attrs": {
                  "class": "align-items-end"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "d-flex flex-wrap gap-1 text-muted text-nowrap"
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
                          "name": "date"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "activity_ids",
                          "widget": "kanban_activity"
                        }
                      }
                    ]
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "ms-auto",
                      "name": "state",
                      "options": "{'classes': {'draft': 'default', 'posted': 'success'}}",
                      "widget": "label_selection"
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
      "name": "currency_id"
    },
    {
      "name": "checked"
    },
    {
      "name": "partner_id",
      "attrs": {
        "class": "fw-bolder fs-5 me-2",
        "invisible": "not partner_id",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "journal_id",
      "attrs": {
        "class": "fw-bolder fs-5 me-2",
        "invisible": "partner_id"
      }
    },
    {
      "name": "amount_total_in_currency_signed",
      "widget": "monetary",
      "attrs": {
        "class": "fw-bolder ms-auto flex-shrink-0"
      }
    },
    {
      "name": "name"
    },
    {
      "name": "date"
    },
    {
      "name": "activity_ids",
      "widget": "kanban_activity"
    },
    {
      "name": "state",
      "widget": "label_selection",
      "attrs": {
        "class": "ms-auto",
        "options": "{'classes': {'draft': 'default', 'posted': 'success'}}"
      }
    }
  ]
}

export function renderAccountMoveKanban(records = []) {
  return kanbanViewHtml({
    // TODO(odoo2rs): columnas desde la selection del campo state del modelo
    columns: [],
    records,
    stateField: 'state',
  })
}
