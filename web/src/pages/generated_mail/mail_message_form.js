// Generado por odoo2rs — vista form de mail.message (mail_message_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail_message_view_form",
  "name": "mail.message.view.form",
  "model": "mail.message",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "duplicate": "0",
      "string": "Message"
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
            },
            "children": [
              {
                "tag": "button",
                "attrs": {
                  "class": "oe_link",
                  "icon": "fa-file-text-o",
                  "invisible": "not model or res_id == 0",
                  "name": "action_open_document",
                  "type": "object"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "o_field_widget o_stat_info"
                    },
                    "children": [
                      {
                        "tag": "span",
                        "attrs": {
                          "class": "o_stat_text"
                        },
                        "text": "Open Document"
                      }
                    ]
                  }
                ]
              }
            ]
          },
          {
            "tag": "group",
            "children": [
              {
                "tag": "group",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "subject"
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
                      "name": "email_from"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "author_id"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "message_type"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "subtype_id"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "is_internal"
                    }
                  }
                ]
              },
              {
                "tag": "group",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "model"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "res_id"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "record_name"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "parent_id"
                    }
                  }
                ]
              }
            ]
          },
          {
            "tag": "notebook",
            "children": [
              {
                "tag": "page",
                "attrs": {
                  "name": "body",
                  "string": "Body"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "body",
                      "widget": "html_mail"
                    }
                  }
                ]
              },
              {
                "tag": "page",
                "attrs": {
                  "name": "gateway",
                  "string": "Gateway"
                },
                "children": [
                  {
                    "tag": "group",
                    "children": [
                      {
                        "tag": "group",
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "reply_to"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "reply_to_force_new"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "message_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "mail_server_id"
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
                  "name": "recipients",
                  "string": "Recipients"
                },
                "children": [
                  {
                    "tag": "group",
                    "children": [
                      {
                        "tag": "group",
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "partner_ids",
                              "widget": "many2many_tags"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "incoming_email_to"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "incoming_email_cc"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "notified_partner_ids",
                              "widget": "many2many_tags"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "starred_partner_ids",
                              "widget": "many2many_tags"
                            }
                          }
                        ]
                      }
                    ]
                  },
                  {
                    "tag": "group",
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "notification_ids"
                        },
                        "children": [
                          {
                            "tag": "list",
                            "children": [
                              {
                                "tag": "field",
                                "attrs": {
                                  "name": "res_partner_id"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "name": "is_read"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "name": "notification_type"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "name": "notification_status"
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
              {
                "tag": "page",
                "attrs": {
                  "name": "page_tracking",
                  "string": "Tracking"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "tracking_value_ids"
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
      "name": "subject"
    },
    {
      "name": "date"
    },
    {
      "name": "email_from"
    },
    {
      "name": "author_id"
    },
    {
      "name": "message_type"
    },
    {
      "name": "subtype_id"
    },
    {
      "name": "is_internal"
    },
    {
      "name": "model"
    },
    {
      "name": "res_id"
    },
    {
      "name": "record_name"
    },
    {
      "name": "parent_id"
    },
    {
      "name": "body",
      "widget": "html_mail"
    },
    {
      "name": "reply_to"
    },
    {
      "name": "reply_to_force_new"
    },
    {
      "name": "message_id"
    },
    {
      "name": "mail_server_id"
    },
    {
      "name": "partner_ids",
      "widget": "many2many_tags"
    },
    {
      "name": "incoming_email_to"
    },
    {
      "name": "incoming_email_cc"
    },
    {
      "name": "notified_partner_ids",
      "widget": "many2many_tags"
    },
    {
      "name": "starred_partner_ids",
      "widget": "many2many_tags"
    },
    {
      "name": "notification_ids"
    },
    {
      "name": "res_partner_id"
    },
    {
      "name": "is_read"
    },
    {
      "name": "notification_type"
    },
    {
      "name": "notification_status"
    },
    {
      "name": "tracking_value_ids"
    }
  ],
  "buttons": [
    {
      "name": "action_open_document",
      "type": "object",
      "class": "oe_link"
    }
  ]
}

export function renderMailMessageForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.message' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.message/<método> (≈ call_kw)
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
