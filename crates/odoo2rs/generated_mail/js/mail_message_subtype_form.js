// Generado por odoo2rs — vista form de mail.message.subtype (view_mail_message_subtype_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_mail_message_subtype_form",
  "name": "mail.message.subtype.form",
  "model": "mail.message.subtype",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Email message"
    },
    "children": [
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "group",
            "children": [
              {
                "tag": "group",
                "attrs": {
                  "string": "Description"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "name"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "sequence"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "res_model"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "description"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "default"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "internal"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "hidden"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "track_recipients"
                    }
                  }
                ]
              },
              {
                "tag": "group",
                "attrs": {
                  "string": "Auto subscription"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "parent_id"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "relation_field"
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
      "name": "name"
    },
    {
      "name": "sequence"
    },
    {
      "name": "res_model"
    },
    {
      "name": "description"
    },
    {
      "name": "default"
    },
    {
      "name": "internal"
    },
    {
      "name": "hidden"
    },
    {
      "name": "track_recipients"
    },
    {
      "name": "parent_id"
    },
    {
      "name": "relation_field"
    }
  ]
}

export function renderMailMessageSubtypeForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.message.subtype' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.message.subtype/<método> (≈ call_kw)
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
