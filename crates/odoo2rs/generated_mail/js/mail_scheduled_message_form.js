// Generado por odoo2rs — vista form de mail.scheduled.message (mail_scheduled_message_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail_scheduled_message_view_form",
  "name": "mail.scheduled.message.view.form",
  "model": "mail.scheduled.message",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Scheduled Message"
    },
    "children": [
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "group",
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "invisible": "1",
                  "name": "composition_comment_option"
                }
              },
              {
                "tag": "label",
                "attrs": {
                  "for": "partner_ids",
                  "invisible": "is_note",
                  "string": "To"
                }
              },
              {
                "tag": "div",
                "attrs": {
                  "class": "d-flex gap-3",
                  "invisible": "is_note"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "w-auto flex-grow-1",
                      "context": "{'force_email': True, 'show_email': True, 'form_view_ref': 'base.view_partner_simple_form'}",
                      "name": "partner_ids",
                      "options": "{'edit_tags': True}",
                      "placeholder": "Add contacts to notify...",
                      "widget": "many2many_tags_email"
                    }
                  }
                ]
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "subject",
                  "required": "True"
                }
              }
            ]
          },
          {
            "tag": "group",
            "attrs": {
              "col": "1"
            },
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "force_save": "1",
                  "name": "body",
                  "nolabel": "1",
                  "placeholder": "Write your message here...",
                  "widget": "html_composer_message"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "attachment_ids",
                  "nolabel": "1",
                  "widget": "mail_composer_attachment_list"
                }
              }
            ]
          }
        ]
      },
      {
        "tag": "footer",
        "children": [
          {
            "tag": "button",
            "attrs": {
              "class": "btn-primary",
              "data-hotkey": "S",
              "invisible": "not scheduled_date",
              "special": "save",
              "string": "Save"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "data-hotkey": "q",
              "invisible": "is_note",
              "name": "post_message",
              "string": "Send Now",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "data-hotkey": "q",
              "invisible": "not is_note",
              "name": "post_message",
              "string": "Log Now",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "class": "btn-secondary",
              "data-hotkey": "x",
              "special": "cancel",
              "string": "Discard"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "name": "attachment_ids",
              "widget": "mail_composer_attachment_selector"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "name": "scheduled_date",
              "widget": "datetime_scheduled_date"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "composition_comment_option",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "partner_ids",
      "widget": "many2many_tags_email",
      "attrs": {
        "class": "w-auto flex-grow-1",
        "context": "{'force_email': True, 'show_email': True, 'form_view_ref': 'base.view_partner_simple_form'}",
        "options": "{'edit_tags': True}",
        "placeholder": "Add contacts to notify..."
      }
    },
    {
      "name": "subject",
      "attrs": {
        "required": "True"
      }
    },
    {
      "name": "body",
      "widget": "html_composer_message",
      "attrs": {
        "force_save": "1",
        "nolabel": "1",
        "placeholder": "Write your message here..."
      }
    },
    {
      "name": "attachment_ids",
      "widget": "mail_composer_attachment_list",
      "attrs": {
        "nolabel": "1"
      }
    },
    {
      "name": "attachment_ids",
      "widget": "mail_composer_attachment_selector"
    },
    {
      "name": "scheduled_date",
      "widget": "datetime_scheduled_date"
    }
  ],
  "buttons": [
    {
      "string": "Save",
      "class": "btn-primary"
    },
    {
      "name": "post_message",
      "string": "Send Now",
      "type": "object"
    },
    {
      "name": "post_message",
      "string": "Log Now",
      "type": "object"
    },
    {
      "string": "Discard",
      "class": "btn-secondary"
    }
  ]
}

export function renderMailScheduledMessageForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.scheduled.message' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.scheduled.message/<método> (≈ call_kw)
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
