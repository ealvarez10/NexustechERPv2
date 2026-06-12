// Generado por odoo2rs — vista form de mail.ice.server (view_ice_server_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_ice_server_form",
  "name": "mail.ice.server.form",
  "model": "mail.ice.server",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "ICE Server Configuration"
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
                  "class": "oe_inline",
                  "name": "server_type"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "uri",
                  "placeholder": "stun:stun.google.com:19302 or turn:turn.example.com:3478"
                }
              }
            ]
          },
          {
            "tag": "group",
            "attrs": {
              "string": "Authentication"
            },
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "name": "username"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "credential"
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
      "name": "server_type",
      "attrs": {
        "class": "oe_inline"
      }
    },
    {
      "name": "uri",
      "attrs": {
        "placeholder": "stun:stun.google.com:19302 or turn:turn.example.com:3478"
      }
    },
    {
      "name": "username"
    },
    {
      "name": "credential"
    }
  ]
}

export function renderMailIceServerForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.ice.server' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.ice.server/<método> (≈ call_kw)
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
