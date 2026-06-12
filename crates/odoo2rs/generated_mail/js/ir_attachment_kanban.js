// Generado por odoo2rs — vista kanban de ir.attachment (view_document_file_kanban).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { kanbanViewHtml } from '../components/kanban_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_document_file_kanban",
  "name": "ir.attachment kanban",
  "model": "ir.attachment",
  "type": "kanban",
  "arch": {
    "tag": "kanban",
    "attrs": {
      "edit": "false",
      "import": "false"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "mimetype"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "type"
        }
      },
      {
        "tag": "templates",
        "children": [
          {
            "tag": "t",
            "attrs": {
              "t-name": "menu"
            },
            "children": [
              {
                "tag": "a",
                "attrs": {
                  "class": "dropdown-item",
                  "download": "",
                  "t-attf-href": "/web/content/ir.attachment/#{record.id.raw_value}/datas?download=true"
                },
                "text": "Download"
              },
              {
                "tag": "a",
                "attrs": {
                  "class": "dropdown-item",
                  "role": "menuitem",
                  "t-if": "widget.deletable",
                  "type": "delete"
                },
                "text": "Delete"
              }
            ]
          },
          {
            "tag": "t",
            "attrs": {
              "class": "o_kanban_attachment flex-row",
              "t-name": "card"
            },
            "children": [
              {
                "tag": "aside",
                "attrs": {
                  "class": "o_kanban_image m-1"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "o_kanban_image_wrapper d-flex align-items-center justify-content-center"
                    },
                    "children": [
                      {
                        "tag": "t",
                        "attrs": {
                          "t-set": "webimage",
                          "t-value": "new RegExp('image.*(gif|jpeg|jpg|png|webp)').test(record.mimetype.value)"
                        }
                      },
                      {
                        "tag": "div",
                        "attrs": {
                          "aria-label": "Image is a link",
                          "class": "fa fa-link fa-3x text-muted",
                          "t-if": "record.type.raw_value == 'url'"
                        }
                      },
                      {
                        "tag": "img",
                        "attrs": {
                          "alt": "Document",
                          "class": "o_attachment_image",
                          "height": "100",
                          "t-attf-src": "/web/image/#{record.id.raw_value}",
                          "t-elif": "webimage",
                          "width": "100"
                        }
                      },
                      {
                        "tag": "div",
                        "attrs": {
                          "class": "o_image o_image_thumbnail",
                          "t-att-data-mimetype": "record.mimetype.value",
                          "t-else": "!webimage"
                        }
                      }
                    ]
                  }
                ]
              },
              {
                "tag": "main",
                "attrs": {
                  "class": "ms-1"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "o_kanban_details_wrapper d-flex flex-column"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "class": "text-truncate fw-bold fs-5",
                          "name": "name"
                        }
                      },
                      {
                        "tag": "div",
                        "attrs": {
                          "class": "d-flex flex-grow-1 align-items-center"
                        },
                        "children": [
                          {
                            "tag": "t",
                            "attrs": {
                              "t-if": "record.type.raw_value == 'url'"
                            },
                            "children": [
                              {
                                "tag": "i",
                                "attrs": {
                                  "aria-label": "Document url",
                                  "class": "fa fa-globe"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "name": "url",
                                  "widget": "url"
                                }
                              }
                            ]
                          }
                        ]
                      },
                      {
                        "tag": "div",
                        "attrs": {
                          "class": "d-flex"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "create_date"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "class": "ms-auto",
                              "name": "create_uid",
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
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "id"
    },
    {
      "name": "mimetype"
    },
    {
      "name": "type"
    },
    {
      "name": "name",
      "attrs": {
        "class": "text-truncate fw-bold fs-5"
      }
    },
    {
      "name": "url",
      "widget": "url"
    },
    {
      "name": "create_date"
    },
    {
      "name": "create_uid",
      "widget": "many2one_avatar_user",
      "attrs": {
        "class": "ms-auto"
      }
    }
  ]
}

export function renderIrAttachmentKanban(records = []) {
  return kanbanViewHtml({
    // TODO(odoo2rs): columnas desde la selection del campo state del modelo
    columns: [],
    records,
    stateField: 'state',
  })
}
