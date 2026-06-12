// Generado por odoo2rs — vista form de mail.blacklist (mail_blacklist_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail_blacklist_view_form",
  "name": "mail.blacklist.view.form",
  "model": "mail.blacklist",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "duplicate": "false",
      "string": "Add Email Blacklist"
    },
    "children": [
      {
        "tag": "header",
        "children": [
          {
            "tag": "button",
            "attrs": {
              "class": "oe_highlight",
              "context": "{'default_email': email}",
              "invisible": "not active or not email",
              "name": "mail_action_blacklist_remove",
              "string": "Unblacklist",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "class": "oe_highlight",
              "invisible": "active or not email",
              "name": "action_add",
              "string": "Blacklist",
              "type": "object"
            }
          }
        ]
      },
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "div",
            "attrs": {
              "class": "badge rounded-pill text-bg-danger float-end fs-6 border-0",
              "invisible": "active"
            },
            "text": "Archived"
          },
          {
            "tag": "group",
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "name": "email"
                }
              }
            ]
          }
        ]
      },
      {
        "tag": "chatter"
      }
    ]
  },
  "fields": [
    {
      "name": "email"
    }
  ],
  "buttons": [
    {
      "name": "mail_action_blacklist_remove",
      "string": "Unblacklist",
      "type": "object",
      "class": "oe_highlight"
    },
    {
      "name": "action_add",
      "string": "Blacklist",
      "type": "object",
      "class": "oe_highlight"
    }
  ]
}

export function renderMailBlacklistForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.blacklist' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.blacklist/<método> (≈ call_kw)
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
