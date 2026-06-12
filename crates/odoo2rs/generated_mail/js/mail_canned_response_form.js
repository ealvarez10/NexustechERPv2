// Generado por odoo2rs — vista form de mail.canned.response (mail_canned_response_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail_canned_response_view_form",
  "name": "mail.canned.response.form",
  "model": "mail.canned.response",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Canned response"
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
                  "name": "source",
                  "readonly": "not is_editable",
                  "widget": "shortcut"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "substitution",
                  "placeholder": "e.g. Hello, how may I help you?",
                  "readonly": "not is_editable"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "group_ids",
                  "readonly": "not is_editable",
                  "widget": "many2many_tags"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "invisible": "True",
                  "name": "is_editable"
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
      "name": "source",
      "widget": "shortcut",
      "attrs": {
        "readonly": "not is_editable"
      }
    },
    {
      "name": "substitution",
      "attrs": {
        "placeholder": "e.g. Hello, how may I help you?",
        "readonly": "not is_editable"
      }
    },
    {
      "name": "group_ids",
      "widget": "many2many_tags",
      "attrs": {
        "readonly": "not is_editable"
      }
    },
    {
      "name": "is_editable",
      "attrs": {
        "invisible": "True"
      }
    }
  ]
}

export function renderMailCannedResponseForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.canned.response' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.canned.response/<método> (≈ call_kw)
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
