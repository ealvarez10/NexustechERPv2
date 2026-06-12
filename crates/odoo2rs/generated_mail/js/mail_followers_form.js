// Generado por odoo2rs — vista form de mail.followers (view_mail_subscription_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_mail_subscription_form",
  "name": "mail.followers.form",
  "model": "mail.followers",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Followers Form"
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
                      "name": "res_model"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "partner_id"
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
                      "name": "res_id"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "subtype_ids",
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
      "name": "res_model"
    },
    {
      "name": "partner_id"
    },
    {
      "name": "res_id"
    },
    {
      "name": "subtype_ids",
      "widget": "many2many_tags"
    }
  ]
}

export function renderMailFollowersForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.followers' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.followers/<método> (≈ call_kw)
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
