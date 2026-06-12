// Generado por odoo2rs — vista kanban de discuss.channel (mail.discuss_channel_view_kanban).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { kanbanViewHtml } from '../components/kanban_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail.discuss_channel_view_kanban",
  "name": "discuss.channel.kanban",
  "model": "discuss.channel",
  "type": "kanban",
  "arch": {
    "tag": "kanban",
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "is_member"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "group_ids"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "active"
        }
      },
      {
        "tag": "templates",
        "children": [
          {
            "tag": "t",
            "attrs": {
              "class": "row g-0",
              "t-name": "card"
            },
            "children": [
              {
                "tag": "widget",
                "attrs": {
                  "bg_color": "text-bg-danger",
                  "class": "d-flex",
                  "invisible": "active",
                  "name": "web_ribbon",
                  "title": "Archived"
                }
              },
              {
                "tag": "aside",
                "attrs": {
                  "class": "col-2 my-auto"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "alt": "Channel",
                      "name": "avatar_128",
                      "options": "{'size': [50, 50], 'img_class': 'bg-transparent'}",
                      "widget": "image"
                    }
                  }
                ]
              },
              {
                "tag": "main",
                "attrs": {
                  "class": "col me-4 ms-2"
                },
                "children": [
                  {
                    "tag": "span",
                    "attrs": {
                      "class": "fw-bold fs-5"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "name"
                        }
                      }
                    ],
                    "text": "#"
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "groups": "base.group_no_one",
                      "name": "channel_type"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "text-muted lh-1 small",
                      "name": "description"
                    }
                  },
                  {
                    "tag": "button",
                    "attrs": {
                      "class": "btn btn-primary ms-auto mt-auto",
                      "invisible": "is_member or group_ids",
                      "name": "channel_join",
                      "type": "object"
                    },
                    "text": "Join"
                  },
                  {
                    "tag": "button",
                    "attrs": {
                      "class": "btn btn-secondary ms-auto mt-auto",
                      "invisible": "not is_member or group_ids",
                      "name": "action_unfollow",
                      "type": "object"
                    },
                    "text": "Leave"
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
      "name": "is_member"
    },
    {
      "name": "group_ids"
    },
    {
      "name": "active"
    },
    {
      "name": "avatar_128",
      "widget": "image",
      "attrs": {
        "alt": "Channel",
        "options": "{'size': [50, 50], 'img_class': 'bg-transparent'}"
      }
    },
    {
      "name": "name"
    },
    {
      "name": "channel_type",
      "attrs": {
        "groups": "base.group_no_one"
      }
    },
    {
      "name": "description",
      "attrs": {
        "class": "text-muted lh-1 small"
      }
    }
  ],
  "buttons": [
    {
      "name": "channel_join",
      "type": "object",
      "class": "btn btn-primary ms-auto mt-auto"
    },
    {
      "name": "action_unfollow",
      "type": "object",
      "class": "btn btn-secondary ms-auto mt-auto"
    }
  ]
}

export function renderDiscussChannelKanban(records = []) {
  return kanbanViewHtml({
    // TODO(odoo2rs): columnas desde la selection del campo state del modelo
    columns: [],
    records,
    stateField: 'state',
  })
}
