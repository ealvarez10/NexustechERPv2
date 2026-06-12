// Generado por odoo2rs — vista kanban de mail.activity (mail_activity_view_kanban_open_target).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { kanbanViewHtml } from '../components/kanban_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail_activity_view_kanban_open_target",
  "name": "mail.activity.view.kanban.open.target",
  "model": "mail.activity",
  "type": "kanban",
  "arch": {
    "tag": "kanban",
    "attrs": {
      "action": "action_open_document",
      "js_class": "mail_activity_my_kanban",
      "string": "Activity",
      "type": "object"
    },
    "children": [
      {
        "tag": "templates",
        "children": [
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "active"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "icon"
            }
          },
          {
            "tag": "t",
            "attrs": {
              "t-name": "card"
            },
            "children": [
              {
                "tag": "div",
                "attrs": {
                  "class": "d-flex justify-content-between"
                },
                "children": [
                  {
                    "tag": "span",
                    "attrs": {
                      "class": "text-truncate",
                      "invisible": "not res_name"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "class": "fw-bold",
                          "name": "res_name"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "class": "text-muted",
                          "name": "res_model_id"
                        }
                      }
                    ],
                    "text": "()"
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "text-truncate",
                      "invisible": "res_name",
                      "name": "summary"
                    }
                  },
                  {
                    "tag": "span",
                    "attrs": {
                      "class": "badge rounded-pill text-bg-300"
                    },
                    "children": [
                      {
                        "tag": "i",
                        "attrs": {
                          "t-attf-class": "fa {{record.icon.raw_value}} me-1",
                          "t-if": "record.icon"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "activity_type_id"
                        }
                      }
                    ]
                  }
                ]
              },
              {
                "tag": "field",
                "attrs": {
                  "class": "text-truncate",
                  "invisible": "not res_name",
                  "name": "summary"
                }
              },
              {
                "tag": "footer",
                "attrs": {
                  "class": "align-items-center"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "user_id",
                      "widget": "many2one_avatar_user"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "ms-2",
                      "name": "date_deadline",
                      "widget": "remaining_days"
                    }
                  },
                  {
                    "tag": "button",
                    "attrs": {
                      "class": "btn btn-link btn-sm ms-auto me-1",
                      "invisible": "active == False",
                      "name": "action_done",
                      "type": "object"
                    },
                    "children": [
                      {
                        "tag": "i",
                        "attrs": {
                          "class": "fa fa-check"
                        }
                      }
                    ],
                    "text": "Done"
                  },
                  {
                    "tag": "button",
                    "attrs": {
                      "class": "btn btn-link text-danger btn-sm",
                      "name": "unlink",
                      "type": "object"
                    },
                    "children": [
                      {
                        "tag": "i",
                        "attrs": {
                          "class": "fa fa-times"
                        }
                      }
                    ],
                    "text": "Cancel"
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
      "name": "active",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "icon",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "res_name",
      "attrs": {
        "class": "fw-bold"
      }
    },
    {
      "name": "res_model_id",
      "attrs": {
        "class": "text-muted"
      }
    },
    {
      "name": "summary",
      "attrs": {
        "class": "text-truncate",
        "invisible": "res_name"
      }
    },
    {
      "name": "activity_type_id"
    },
    {
      "name": "summary",
      "attrs": {
        "class": "text-truncate",
        "invisible": "not res_name"
      }
    },
    {
      "name": "user_id",
      "widget": "many2one_avatar_user"
    },
    {
      "name": "date_deadline",
      "widget": "remaining_days",
      "attrs": {
        "class": "ms-2"
      }
    }
  ],
  "buttons": [
    {
      "name": "action_done",
      "type": "object",
      "class": "btn btn-link btn-sm ms-auto me-1"
    },
    {
      "name": "unlink",
      "type": "object",
      "class": "btn btn-link text-danger btn-sm"
    }
  ]
}

export function renderMailActivityKanban(records = []) {
  return kanbanViewHtml({
    // TODO(odoo2rs): columnas desde la selection del campo state del modelo
    columns: [],
    records,
    stateField: 'state',
  })
}
