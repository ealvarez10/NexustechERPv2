// Generado por odoo2rs — vista form de discuss.call.history (discuss_call_history_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "discuss_call_history_view_form",
  "name": "discuss.call.history.view.form",
  "model": "discuss.call.history",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Call History"
    },
    "children": [
      {
        "tag": "group",
        "attrs": {
          "class": "oe_form_field"
        },
        "children": [
          {
            "tag": "field",
            "attrs": {
              "name": "channel_id"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "name": "start_dt"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "name": "end_dt"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "name": "duration_hour",
              "options": "{'displaySeconds': True}",
              "widget": "float_time"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "channel_id"
    },
    {
      "name": "start_dt"
    },
    {
      "name": "end_dt"
    },
    {
      "name": "duration_hour",
      "widget": "float_time",
      "attrs": {
        "options": "{'displaySeconds': True}"
      }
    }
  ]
}

export function renderDiscussCallHistoryForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'discuss.call.history' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/discuss.call.history/<método> (≈ call_kw)
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
