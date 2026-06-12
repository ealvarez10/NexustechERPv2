// Generado por odoo2rs — vista form de mail.guest (mail_guest_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail_guest_view_form",
  "name": "mail.guest.form",
  "model": "mail.guest",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Guest"
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
                      "name": "name"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "country_id"
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
                      "name": "lang"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "timezone"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "channel_ids",
                      "widget": "many2many_tags"
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
      "name": "country_id"
    },
    {
      "name": "lang"
    },
    {
      "name": "timezone"
    },
    {
      "name": "channel_ids",
      "widget": "many2many_tags"
    }
  ]
}

export function renderMailGuestForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.guest' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.guest/<método> (≈ call_kw)
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
