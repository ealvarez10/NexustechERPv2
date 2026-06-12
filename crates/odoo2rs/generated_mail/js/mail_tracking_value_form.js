// Generado por odoo2rs — vista form de mail.tracking.value (view_mail_tracking_value_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_mail_tracking_value_form",
  "name": "mail.tracking.value.form",
  "model": "mail.tracking.value",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Tracking Value"
    },
    "children": [
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "group",
            "attrs": {
              "name": "field_details",
              "string": "Field details"
            },
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "name": "field_id"
                }
              }
            ]
          },
          {
            "tag": "group",
            "attrs": {
              "name": "values"
            },
            "children": [
              {
                "tag": "group",
                "attrs": {
                  "name": "old_values",
                  "string": "Old values"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "old_value_integer"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "old_value_float"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "old_value_char"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "old_value_text"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "old_value_datetime"
                    }
                  }
                ]
              },
              {
                "tag": "group",
                "attrs": {
                  "name": "new_values",
                  "string": "New values"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "new_value_integer"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "new_value_float"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "new_value_char"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "new_value_text"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "new_value_datetime"
                    }
                  }
                ]
              }
            ]
          },
          {
            "tag": "group",
            "attrs": {
              "string": "Related Message"
            },
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "name": "mail_message_id"
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
      "name": "field_id"
    },
    {
      "name": "old_value_integer"
    },
    {
      "name": "old_value_float"
    },
    {
      "name": "old_value_char"
    },
    {
      "name": "old_value_text"
    },
    {
      "name": "old_value_datetime"
    },
    {
      "name": "new_value_integer"
    },
    {
      "name": "new_value_float"
    },
    {
      "name": "new_value_char"
    },
    {
      "name": "new_value_text"
    },
    {
      "name": "new_value_datetime"
    },
    {
      "name": "mail_message_id"
    }
  ]
}

export function renderMailTrackingValueForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.tracking.value' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.tracking.value/<método> (≈ call_kw)
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
