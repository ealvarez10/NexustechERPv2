// Generado por odoo2rs — vista form de discuss.channel (mail.discuss_channel_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail.discuss_channel_view_form",
  "name": "discuss.channel.form",
  "model": "discuss.channel",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Mail Channel Form"
    },
    "children": [
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "div",
            "attrs": {
              "class": "oe_button_box",
              "name": "button_box"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "avatar_128"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "is_editable"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "class": "oe_avatar",
              "name": "image_128",
              "options": "{'size': [90, 90], 'preview_image':'avatar_128'}",
              "readonly": "not is_editable",
              "widget": "image"
            }
          },
          {
            "tag": "widget",
            "attrs": {
              "bg_color": "text-bg-danger",
              "invisible": "active",
              "name": "web_ribbon",
              "title": "Archived"
            }
          },
          {
            "tag": "div",
            "attrs": {
              "class": "oe_title"
            },
            "children": [
              {
                "tag": "label",
                "attrs": {
                  "for": "name",
                  "string": "Group Name"
                }
              },
              {
                "tag": "h1",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "oe_inline",
                      "default_focus": "1",
                      "name": "name",
                      "placeholder": "e.g. support",
                      "readonly": "not is_editable"
                    }
                  }
                ],
                "text": "#"
              }
            ]
          },
          {
            "tag": "group",
            "attrs": {
              "class": "o_label_nowrap"
            },
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
                  "name": "description",
                  "placeholder": "Topics discussed in this group...",
                  "readonly": "not is_editable"
                }
              }
            ]
          },
          {
            "tag": "notebook",
            "children": [
              {
                "tag": "page",
                "attrs": {
                  "name": "privacy",
                  "string": "Privacy"
                },
                "children": [
                  {
                    "tag": "group",
                    "attrs": {
                      "class": "o_label_nowrap"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "invisible": "channel_type != 'channel' or parent_channel_id",
                          "name": "group_public_id",
                          "readonly": "not is_editable"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "invisible": "channel_type != 'channel'",
                          "name": "group_ids",
                          "readonly": "not is_editable",
                          "string": "Auto Subscribe Groups",
                          "widget": "many2many_tags"
                        }
                      }
                    ]
                  }
                ]
              },
              {
                "tag": "page",
                "attrs": {
                  "name": "members",
                  "string": "Members"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "1",
                      "name": "channel_type"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "context": "{'active_test': False}",
                      "mode": "list",
                      "name": "channel_member_ids",
                      "readonly": "channel_type == 'chat'"
                    },
                    "children": [
                      {
                        "tag": "list",
                        "attrs": {
                          "editable": "bottom",
                          "string": "Members"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "partner_id",
                              "readonly": "id or guest_id",
                              "required": "not guest_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "guest_id",
                              "readonly": "id or partner_id",
                              "required": "not partner_id"
                            }
                          }
                        ]
                      }
                    ]
                  }
                ]
              },
              {
                "tag": "page",
                "attrs": {
                  "groups": "base.group_no_one",
                  "name": "extra_info",
                  "string": "Extra info"
                },
                "children": [
                  {
                    "tag": "group",
                    "attrs": {
                      "class": "o_label_nowrap"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "channel_type"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "parent_channel_id"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "sub_channel_ids"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "from_message_id"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "sfu_channel_uuid"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "sfu_server_url"
                        }
                      }
                    ]
                  }
                ]
              },
              {
                "tag": "page",
                "attrs": {
                  "invisible": "1",
                  "name": "discuss_channel_integrations",
                  "string": "Integrations"
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
      "name": "avatar_128",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "is_editable",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "image_128",
      "widget": "image",
      "attrs": {
        "class": "oe_avatar",
        "options": "{'size': [90, 90], 'preview_image':'avatar_128'}",
        "readonly": "not is_editable"
      }
    },
    {
      "name": "name",
      "attrs": {
        "class": "oe_inline",
        "default_focus": "1",
        "placeholder": "e.g. support",
        "readonly": "not is_editable"
      }
    },
    {
      "name": "active",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "description",
      "attrs": {
        "placeholder": "Topics discussed in this group...",
        "readonly": "not is_editable"
      }
    },
    {
      "name": "group_public_id",
      "attrs": {
        "invisible": "channel_type != 'channel' or parent_channel_id",
        "readonly": "not is_editable"
      }
    },
    {
      "name": "group_ids",
      "string": "Auto Subscribe Groups",
      "widget": "many2many_tags",
      "attrs": {
        "invisible": "channel_type != 'channel'",
        "readonly": "not is_editable"
      }
    },
    {
      "name": "channel_type",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "channel_member_ids",
      "attrs": {
        "context": "{'active_test': False}",
        "mode": "list",
        "readonly": "channel_type == 'chat'"
      }
    },
    {
      "name": "partner_id",
      "attrs": {
        "readonly": "id or guest_id",
        "required": "not guest_id"
      }
    },
    {
      "name": "guest_id",
      "attrs": {
        "readonly": "id or partner_id",
        "required": "not partner_id"
      }
    },
    {
      "name": "channel_type"
    },
    {
      "name": "parent_channel_id"
    },
    {
      "name": "sub_channel_ids"
    },
    {
      "name": "from_message_id"
    },
    {
      "name": "sfu_channel_uuid"
    },
    {
      "name": "sfu_server_url"
    }
  ]
}

export function renderDiscussChannelForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'discuss.channel' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/discuss.channel/<método> (≈ call_kw)
      onClick: `alert('TODO: ${b.name}')`,
    })) || [],
    fieldGroups: [{
      fields: DESCRIPTOR.fields.map(f => ({
        label: f.string || f.name,
        value: record[f.name] ?? '',
      })),
    }],
    id: record.id || '',
  })
}
