// Generado por odoo2rs — vista form de mail.activity (mail_activity_view_form_without_record_access).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail_activity_view_form_without_record_access",
  "name": "mail.activity.view.form.without.record.access",
  "model": "mail.activity",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "create": "false",
      "delete": "false",
      "string": "Log an Activity"
    },
    "children": [
      {
        "tag": "header",
        "children": [
          {
            "tag": "button",
            "attrs": {
              "class": "btn-primary",
              "name": "action_done_redirect_to_other",
              "string": "Mark Done",
              "type": "object"
            }
          }
        ]
      },
      {
        "tag": "sheet",
        "attrs": {
          "string": "Activity"
        },
        "children": [
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "display_name"
            }
          },
          {
            "tag": "group",
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "iconField": "icon",
                  "name": "activity_type_id",
                  "nolabel": "1",
                  "required": "1",
                  "widget": "selection_badge_icons"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "summary",
                  "placeholder": "e.g. Discuss proposal"
                }
              },
              {
                "tag": "group",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "date_deadline"
                    }
                  }
                ]
              }
            ]
          },
          {
            "tag": "field",
            "attrs": {
              "class": "oe-bordered-editor embedded-editor-height-4",
              "name": "note",
              "placeholder": "Log a note...",
              "widget": "html_mail"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "display_name",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "activity_type_id",
      "widget": "selection_badge_icons",
      "attrs": {
        "iconField": "icon",
        "nolabel": "1",
        "required": "1"
      }
    },
    {
      "name": "summary",
      "attrs": {
        "placeholder": "e.g. Discuss proposal"
      }
    },
    {
      "name": "date_deadline"
    },
    {
      "name": "note",
      "widget": "html_mail",
      "attrs": {
        "class": "oe-bordered-editor embedded-editor-height-4",
        "placeholder": "Log a note..."
      }
    }
  ],
  "buttons": [
    {
      "name": "action_done_redirect_to_other",
      "string": "Mark Done",
      "type": "object",
      "class": "btn-primary"
    }
  ]
}

export function renderMailActivityForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.activity' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.activity/<método> (≈ call_kw)
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
