// Generado por odoo2rs — vista form de mail.message.reaction (mail_message_reaction_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail_message_reaction_view_form",
  "name": "mail.message.reaction.form",
  "model": "mail.message.reaction",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "create": "0",
      "edit": "0",
      "string": "Reactions"
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
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "message_id"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "content"
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
                      "name": "partner_id"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "guest_id"
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
      "name": "message_id"
    },
    {
      "name": "content"
    },
    {
      "name": "partner_id"
    },
    {
      "name": "guest_id"
    }
  ]
}

export function renderMailMessageReactionForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.message.reaction' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.message.reaction/<método> (≈ call_kw)
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
