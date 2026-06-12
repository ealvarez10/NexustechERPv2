// Generado por odoo2rs — vista kanban de mail.ice.server (view_ice_server_kanban).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { kanbanViewHtml } from '../components/kanban_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_ice_server_kanban",
  "name": "mail.ice.server.kanban",
  "model": "mail.ice.server",
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
                  "class": "oe_kanban_global_click"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "oe_kanban_content p-2"
                    },
                    "children": [
                      {
                        "tag": "div",
                        "attrs": {
                          "class": "d-flex align-items-center mb-2"
                        },
                        "children": [
                          {
                            "tag": "span",
                            "attrs": {
                              "class": "fw-bold me-1"
                            },
                            "text": "Type:"
                          },
                          {
                            "tag": "span",
                            "attrs": {
                              "class": "fw-bold"
                            },
                            "children": [
                              {
                                "tag": "field",
                                "attrs": {
                                  "name": "server_type"
                                }
                              }
                            ]
                          }
                        ]
                      },
                      {
                        "tag": "div",
                        "attrs": {
                          "class": "d-flex align-items-center mb-2"
                        },
                        "children": [
                          {
                            "tag": "span",
                            "attrs": {
                              "class": "fw-bold me-1"
                            },
                            "text": "URI:"
                          },
                          {
                            "tag": "span",
                            "attrs": {
                              "class": "text-primary fw-bold"
                            },
                            "children": [
                              {
                                "tag": "field",
                                "attrs": {
                                  "name": "uri"
                                }
                              }
                            ]
                          }
                        ]
                      },
                      {
                        "tag": "t",
                        "attrs": {
                          "t-if": "record.username.raw_value"
                        },
                        "children": [
                          {
                            "tag": "div",
                            "attrs": {
                              "class": "d-flex align-items-center mb-2"
                            },
                            "children": [
                              {
                                "tag": "span",
                                "attrs": {
                                  "class": "fw-bold me-1"
                                },
                                "text": "Username:"
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "name": "username"
                                }
                              }
                            ]
                          }
                        ]
                      },
                      {
                        "tag": "t",
                        "attrs": {
                          "t-if": "record.credential.raw_value"
                        },
                        "children": [
                          {
                            "tag": "div",
                            "attrs": {
                              "class": "d-flex align-items-center"
                            },
                            "children": [
                              {
                                "tag": "span",
                                "attrs": {
                                  "class": "fw-bold me-1"
                                },
                                "text": "Credential:"
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "name": "credential"
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
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "server_type"
    },
    {
      "name": "uri"
    },
    {
      "name": "username"
    },
    {
      "name": "credential"
    }
  ]
}

export function renderMailIceServerKanban(records = []) {
  return kanbanViewHtml({
    // TODO(odoo2rs): columnas desde la selection del campo state del modelo
    columns: [],
    records,
    stateField: 'state',
  })
}
