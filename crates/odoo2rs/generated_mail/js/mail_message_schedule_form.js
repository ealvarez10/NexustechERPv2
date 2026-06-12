// Generado por odoo2rs — vista form de mail.message.schedule (mail_message_schedule_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail_message_schedule_view_form",
  "name": "mail.message.schedule.view.form",
  "model": "mail.message.schedule",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "duplicate": "0",
      "string": "Scheduled Message"
    },
    "children": [
      {
        "tag": "header",
        "children": [
          {
            "tag": "button",
            "attrs": {
              "name": "force_send",
              "string": "Force Send",
              "type": "object"
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
                "tag": "field",
                "attrs": {
                  "name": "mail_message_id"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "scheduled_datetime"
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
                  "name": "notification_parameters"
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
      "name": "mail_message_id"
    },
    {
      "name": "scheduled_datetime"
    },
    {
      "name": "notification_parameters"
    }
  ],
  "buttons": [
    {
      "name": "force_send",
      "string": "Force Send",
      "type": "object"
    }
  ]
}

export function renderMailMessageScheduleForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.message.schedule' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.message.schedule/<método> (≈ call_kw)
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
