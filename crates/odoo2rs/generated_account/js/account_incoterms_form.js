// Generado por odoo2rs — vista form de account.incoterms (account_incoterms_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "account_incoterms_form",
  "name": "account.incoterms.form",
  "model": "account.incoterms",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Incoterms"
    },
    "children": [
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "widget",
            "attrs": {
              "bg_color": "text-bg-danger",
              "invisible": "active",
              "name": "web_ribbon",
              "title": "Archived"
            }
          },
          {
            "tag": "group",
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "invisible": "1",
                  "name": "active"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "name"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "code"
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
      "name": "active",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "name"
    },
    {
      "name": "code"
    }
  ]
}

export function renderAccountIncotermsForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'account.incoterms' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/account.incoterms/<método> (≈ call_kw)
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
