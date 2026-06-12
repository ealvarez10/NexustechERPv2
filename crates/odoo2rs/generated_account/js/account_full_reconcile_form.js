// Generado por odoo2rs — vista form de account.full.reconcile (view_full_reconcile_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_full_reconcile_form",
  "name": "account.full.reconcile.form",
  "model": "account.full.reconcile",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Matching"
    },
    "children": [
      {
        "tag": "group",
        "children": [
          {
            "tag": "div",
            "attrs": {
              "class": "oe_title",
              "colspan": "4"
            },
            "children": [
              {
                "tag": "h1",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "id",
                      "readonly": "1"
                    }
                  }
                ]
              }
            ]
          },
          {
            "tag": "separator",
            "attrs": {
              "colspan": "4",
              "string": "Matched Journal Items"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "colspan": "4",
              "name": "reconciled_line_ids",
              "nolabel": "1",
              "readonly": "1"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "id",
      "attrs": {
        "readonly": "1"
      }
    },
    {
      "name": "reconciled_line_ids",
      "attrs": {
        "colspan": "4",
        "nolabel": "1",
        "readonly": "1"
      }
    }
  ]
}

export function renderAccountFullReconcileForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'account.full.reconcile' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/account.full.reconcile/<método> (≈ call_kw)
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
