// Generado por odoo2rs — vista form de mail.template (email_template_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "email_template_form",
  "name": "email.template.form",
  "model": "mail.template",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "class": "o_mail_template_form_view",
      "string": "Templates"
    },
    "children": [
      {
        "tag": "header",
        "children": [
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "ref_ir_act_window"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "template_fs"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "is_template_editor"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "class": "btn-primary",
              "name": "action_open_mail_preview",
              "string": "Preview",
              "target": "new",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "groups": "mail.group_mail_template_editor",
              "invisible": "not template_fs",
              "name": "%(mail_template_reset_action)d",
              "string": "Reset Template",
              "type": "action"
            }
          },
          {
            "tag": "t",
            "attrs": {
              "groups": "base.group_no_one"
            },
            "children": [
              {
                "tag": "button",
                "attrs": {
                  "class": "btn btn-secondary",
                  "groups": "base.group_system",
                  "help": "Display an option on related documents to open a composition wizard with this template",
                  "invisible": "ref_ir_act_window",
                  "name": "create_action",
                  "string": "Add Context Action",
                  "type": "object"
                }
              },
              {
                "tag": "button",
                "attrs": {
                  "class": "btn btn-secondary",
                  "help": "Remove the contextual action to use this template on related documents",
                  "invisible": "not ref_ir_act_window",
                  "name": "unlink_action",
                  "string": "Remove Context Action",
                  "type": "object"
                }
              }
            ]
          }
        ]
      },
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "div",
            "attrs": {
              "class": "oe_title"
            },
            "children": [
              {
                "tag": "label",
                "attrs": {
                  "for": "name"
                }
              },
              {
                "tag": "h1",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "w-100",
                      "name": "name",
                      "placeholder": "e.g. \"Welcome email\"",
                      "required": "1",
                      "string": "Template Name"
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
                      "groups": "!base.group_no_one",
                      "invisible": "context.get('default_model')",
                      "name": "model_id",
                      "options": "{'no_create': True}",
                      "placeholder": "e.g. Contact",
                      "required": "1"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "groups": "base.group_no_one",
                      "name": "model_id",
                      "options": "{'no_create': True}",
                      "placeholder": "e.g. Contact",
                      "required": "1"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "subject",
                      "options": "{'dynamic_placeholder': true}",
                      "placeholder": "e.g. \"Welcome to MyCompany\" or \"Nice to meet you, {{ object.name }}\""
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "1",
                      "name": "model"
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
                  "name": "content",
                  "string": "Body"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "1",
                      "name": "can_write"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "oe-bordered-editor",
                      "name": "body_html",
                      "options": "{'codeview': true, 'dynamic_placeholder': true, 'allowCommandVideo': false}",
                      "placeholder": "Write your message here...",
                      "readonly": "not can_write and id",
                      "widget": "html_mail"
                    }
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "fst-italic text-muted",
                      "invisible": "not can_write"
                    },
                    "text": "Tip: Write /field to insert dynamic content!"
                  }
                ]
              },
              {
                "tag": "page",
                "attrs": {
                  "groups": "base.group_no_one",
                  "name": "email_configuration",
                  "string": "Settings"
                },
                "children": [
                  {
                    "tag": "group",
                    "attrs": {
                      "col": "2"
                    },
                    "children": [
                      {
                        "tag": "group",
                        "children": [
                          {
                            "tag": "separator",
                            "attrs": {
                              "string": "Sender & Recipients"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "email_from",
                              "placeholder": "Sender's email"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "use_default_to"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "use_default_to",
                              "name": "email_to",
                              "placeholder": "Comma-separated recipient addresses"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "use_default_to",
                              "name": "partner_to",
                              "placeholder": "Comma-separated ids of recipient partners"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "use_default_to",
                              "name": "email_cc",
                              "placeholder": "Comma-separated carbon copy of recipients addresses"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "reply_to",
                              "placeholder": "Capture replies in the chatter"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "children": [
                          {
                            "tag": "separator",
                            "attrs": {
                              "string": "Technical"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "lang",
                              "options": "{'dynamic_placeholder': true}",
                              "placeholder": "Main partner's language"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "1",
                              "name": "has_mail_server"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "not has_mail_server",
                              "name": "mail_server_id",
                              "placeholder": "By order of server priority"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "auto_delete"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "scheduled_date",
                              "options": "{'dynamic_placeholder': true}",
                              "placeholder": "Send Instantly",
                              "string": "Scheduled Send Date"
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
                  "name": "email_settings",
                  "string": "Options"
                },
                "children": [
                  {
                    "tag": "group",
                    "attrs": {
                      "col": "2"
                    },
                    "children": [
                      {
                        "tag": "group",
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "1",
                              "name": "has_dynamic_reports"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "attachment_ids",
                              "widget": "many2many_binary"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "domain": "[('model','=',model)]",
                              "invisible": "not has_dynamic_reports",
                              "name": "report_template_ids",
                              "options": "{'no_create': True}",
                              "placeholder": "None",
                              "widget": "many2many_tags"
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
                              "invisible": "not is_template_editor",
                              "name": "user_id",
                              "placeholder": "Shared with all users",
                              "readonly": "not is_template_editor",
                              "widget": "many2one_avatar_user"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "description",
                              "placeholder": "Describe when this template should be used"
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
      "name": "ref_ir_act_window",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "template_fs",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "is_template_editor",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "name",
      "string": "Template Name",
      "attrs": {
        "class": "w-100",
        "placeholder": "e.g. \"Welcome email\"",
        "required": "1"
      }
    },
    {
      "name": "model_id",
      "attrs": {
        "groups": "!base.group_no_one",
        "invisible": "context.get('default_model')",
        "options": "{'no_create': True}",
        "placeholder": "e.g. Contact",
        "required": "1"
      }
    },
    {
      "name": "model_id",
      "attrs": {
        "groups": "base.group_no_one",
        "options": "{'no_create': True}",
        "placeholder": "e.g. Contact",
        "required": "1"
      }
    },
    {
      "name": "subject",
      "attrs": {
        "options": "{'dynamic_placeholder': true}",
        "placeholder": "e.g. \"Welcome to MyCompany\" or \"Nice to meet you, {{ object.name }}\""
      }
    },
    {
      "name": "model",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "can_write",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "body_html",
      "widget": "html_mail",
      "attrs": {
        "class": "oe-bordered-editor",
        "options": "{'codeview': true, 'dynamic_placeholder': true, 'allowCommandVideo': false}",
        "placeholder": "Write your message here...",
        "readonly": "not can_write and id"
      }
    },
    {
      "name": "email_from",
      "attrs": {
        "placeholder": "Sender's email"
      }
    },
    {
      "name": "use_default_to"
    },
    {
      "name": "email_to",
      "attrs": {
        "invisible": "use_default_to",
        "placeholder": "Comma-separated recipient addresses"
      }
    },
    {
      "name": "partner_to",
      "attrs": {
        "invisible": "use_default_to",
        "placeholder": "Comma-separated ids of recipient partners"
      }
    },
    {
      "name": "email_cc",
      "attrs": {
        "invisible": "use_default_to",
        "placeholder": "Comma-separated carbon copy of recipients addresses"
      }
    },
    {
      "name": "reply_to",
      "attrs": {
        "placeholder": "Capture replies in the chatter"
      }
    },
    {
      "name": "lang",
      "attrs": {
        "options": "{'dynamic_placeholder': true}",
        "placeholder": "Main partner's language"
      }
    },
    {
      "name": "has_mail_server",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "mail_server_id",
      "attrs": {
        "invisible": "not has_mail_server",
        "placeholder": "By order of server priority"
      }
    },
    {
      "name": "auto_delete"
    },
    {
      "name": "scheduled_date",
      "string": "Scheduled Send Date",
      "attrs": {
        "options": "{'dynamic_placeholder': true}",
        "placeholder": "Send Instantly"
      }
    },
    {
      "name": "has_dynamic_reports",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "attachment_ids",
      "widget": "many2many_binary"
    },
    {
      "name": "report_template_ids",
      "widget": "many2many_tags",
      "attrs": {
        "domain": "[('model','=',model)]",
        "invisible": "not has_dynamic_reports",
        "options": "{'no_create': True}",
        "placeholder": "None"
      }
    },
    {
      "name": "user_id",
      "widget": "many2one_avatar_user",
      "attrs": {
        "invisible": "not is_template_editor",
        "placeholder": "Shared with all users",
        "readonly": "not is_template_editor"
      }
    },
    {
      "name": "description",
      "attrs": {
        "placeholder": "Describe when this template should be used"
      }
    }
  ],
  "buttons": [
    {
      "name": "action_open_mail_preview",
      "string": "Preview",
      "type": "object",
      "class": "btn-primary"
    },
    {
      "name": "%(mail_template_reset_action)d",
      "string": "Reset Template",
      "type": "action"
    },
    {
      "name": "create_action",
      "string": "Add Context Action",
      "type": "object",
      "class": "btn btn-secondary"
    },
    {
      "name": "unlink_action",
      "string": "Remove Context Action",
      "type": "object",
      "class": "btn btn-secondary"
    }
  ]
}

export function renderMailTemplateForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.template' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.template/<método> (≈ call_kw)
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
