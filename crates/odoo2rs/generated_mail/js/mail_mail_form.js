// Generado por odoo2rs — vista form de mail.mail (view_mail_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_mail_form",
  "name": "mail.mail.form",
  "model": "mail.mail",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "duplicate": "0",
      "string": "Email message"
    },
    "children": [
      {
        "tag": "header",
        "children": [
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "message_type"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "class": "oe_highlight",
              "invisible": "state != 'outgoing' or message_type == 'user_notification'",
              "name": "action_send_and_close",
              "string": "Send & Close",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "invisible": "state not in ('exception', 'cancel')",
              "name": "mark_outgoing",
              "string": "Retry",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "invisible": "state != 'outgoing'",
              "name": "cancel",
              "string": "Cancel",
              "type": "object"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "name": "state",
              "statusbar_visible": "outgoing,sent,received,exception,cancel",
              "widget": "statusbar"
            }
          }
        ]
      },
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "model"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "res_id"
            }
          },
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
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "mail_message_id_int",
              "required": "0"
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
                  "class": "oe_edit_only",
                  "for": "subject"
                }
              },
              {
                "tag": "h2",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "subject"
                    }
                  }
                ]
              }
            ]
          },
          {
            "tag": "div",
            "attrs": {
              "style": "vertical-align: top;"
            },
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "class": "oe_inline",
                  "name": "author_id",
                  "string": "User"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "class": "oe_inline",
                  "name": "date",
                  "readonly": "1"
                }
              },
              {
                "tag": "button",
                "attrs": {
                  "context": "{'default_composition_mode':'comment', 'default_parent_id': mail_message_id_int}",
                  "icon": "fa-reply text-warning",
                  "invisible": "state not in ('received', 'sent', 'exception', 'cancel')",
                  "name": "%(action_email_compose_message_wizard)d",
                  "string": "Reply",
                  "type": "action"
                }
              }
            ],
            "text": "by  on"
          },
          {
            "tag": "group",
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "name": "email_from"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "email_to"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "domain": "[('active', '=', True)]",
                  "name": "recipient_ids",
                  "widget": "many2many_tags"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "email_cc"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "reply_to"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "scheduled_date",
                  "placeholder": "YYYY-MM-DD HH:MM:SS"
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
                  "name": "body",
                  "string": "Body"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "body_content"
                    }
                  }
                ]
              },
              {
                "tag": "page",
                "attrs": {
                  "groups": "base.group_no_one",
                  "name": "advanced",
                  "string": "Advanced"
                },
                "children": [
                  {
                    "tag": "group",
                    "children": [
                      {
                        "tag": "group",
                        "attrs": {
                          "string": "Status"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "state != 'outgoing' and state != 'exception'",
                              "name": "auto_delete"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "is_notification"
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
                              "name": "mail_server_id"
                            }
                          },
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
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "string": "Headers"
                        },
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
                              "name": "references"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "fetchmail_server_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "headers"
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
                  "name": "attachments",
                  "string": "Attachments"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "alert alert-warning",
                      "invisible": "restricted_attachment_count == 0",
                      "role": "alert"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "restricted_attachment_count"
                        }
                      }
                    ],
                    "text": "You do not have access to \n                                    attachment(s) of this email."
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "domain": "[('res_field','=', False)]",
                      "name": "unrestricted_attachment_ids"
                    }
                  }
                ]
              },
              {
                "tag": "page",
                "attrs": {
                  "invisible": "state != 'exception'",
                  "name": "failure_reason",
                  "string": "Failure Reason"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "failure_reason"
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
      "name": "message_type",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "state",
      "widget": "statusbar",
      "attrs": {
        "statusbar_visible": "outgoing,sent,received,exception,cancel"
      }
    },
    {
      "name": "model",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "res_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "mail_message_id_int",
      "attrs": {
        "invisible": "1",
        "required": "0"
      }
    },
    {
      "name": "subject"
    },
    {
      "name": "author_id",
      "string": "User",
      "attrs": {
        "class": "oe_inline"
      }
    },
    {
      "name": "date",
      "attrs": {
        "class": "oe_inline",
        "readonly": "1"
      }
    },
    {
      "name": "email_from"
    },
    {
      "name": "email_to"
    },
    {
      "name": "recipient_ids",
      "widget": "many2many_tags",
      "attrs": {
        "domain": "[('active', '=', True)]"
      }
    },
    {
      "name": "email_cc"
    },
    {
      "name": "reply_to"
    },
    {
      "name": "scheduled_date",
      "attrs": {
        "placeholder": "YYYY-MM-DD HH:MM:SS"
      }
    },
    {
      "name": "body_content"
    },
    {
      "name": "auto_delete",
      "attrs": {
        "invisible": "state != 'outgoing' and state != 'exception'"
      }
    },
    {
      "name": "is_notification"
    },
    {
      "name": "message_type"
    },
    {
      "name": "mail_server_id"
    },
    {
      "name": "model"
    },
    {
      "name": "res_id"
    },
    {
      "name": "message_id"
    },
    {
      "name": "references"
    },
    {
      "name": "fetchmail_server_id"
    },
    {
      "name": "headers"
    },
    {
      "name": "restricted_attachment_count"
    },
    {
      "name": "unrestricted_attachment_ids",
      "attrs": {
        "domain": "[('res_field','=', False)]"
      }
    },
    {
      "name": "failure_reason"
    }
  ],
  "buttons": [
    {
      "name": "action_send_and_close",
      "string": "Send & Close",
      "type": "object",
      "class": "oe_highlight"
    },
    {
      "name": "mark_outgoing",
      "string": "Retry",
      "type": "object"
    },
    {
      "name": "cancel",
      "string": "Cancel",
      "type": "object"
    },
    {
      "name": "action_open_document",
      "type": "object",
      "class": "oe_link"
    },
    {
      "name": "%(action_email_compose_message_wizard)d",
      "string": "Reply",
      "type": "action"
    }
  ]
}

export function renderMailMailForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.mail' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.mail/<método> (≈ call_kw)
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
