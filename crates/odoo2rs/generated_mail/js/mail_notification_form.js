// Generado por odoo2rs — vista form de mail.notification (mail_notification_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail_notification_view_form",
  "name": "mail.notification.view.form",
  "model": "mail.notification",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "duplicate": "0",
      "string": "Notification"
    },
    "children": [
      {
        "tag": "header",
        "children": [
          {
            "tag": "field",
            "attrs": {
              "name": "notification_status",
              "statusbar_visible": "ready,sent",
              "widget": "statusbar"
            }
          }
        ]
      },
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "group",
            "children": [
              {
                "tag": "group",
                "attrs": {
                  "string": "Source"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "mail_message_id"
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
                      "name": "mail_mail_id"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "res_partner_id"
                    }
                  }
                ]
              },
              {
                "tag": "group",
                "attrs": {
                  "string": "Status"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "is_read"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "read_date"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "failure_type"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "o_text_overflow",
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
      "name": "notification_status",
      "widget": "statusbar",
      "attrs": {
        "statusbar_visible": "ready,sent"
      }
    },
    {
      "name": "mail_message_id"
    },
    {
      "name": "notification_type"
    },
    {
      "name": "mail_mail_id"
    },
    {
      "name": "res_partner_id"
    },
    {
      "name": "is_read"
    },
    {
      "name": "read_date"
    },
    {
      "name": "failure_type"
    },
    {
      "name": "failure_reason",
      "attrs": {
        "class": "o_text_overflow"
      }
    }
  ]
}

export function renderMailNotificationForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.notification' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.notification/<método> (≈ call_kw)
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
