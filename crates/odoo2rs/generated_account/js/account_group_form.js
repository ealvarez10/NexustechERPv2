// Generado por odoo2rs — vista form de account.group (view_account_group_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_account_group_form",
  "name": "account.group.form",
  "model": "account.group",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Account Group"
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
                  "name": "name"
                }
              },
              {
                "tag": "label",
                "attrs": {
                  "for": "code_prefix_start",
                  "string": "Code Prefix"
                }
              },
              {
                "tag": "div",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "oe_inline",
                      "name": "code_prefix_start"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "oe_inline",
                      "name": "code_prefix_end"
                    }
                  }
                ],
                "text": "From  to"
              },
              {
                "tag": "field",
                "attrs": {
                  "groups": "base.group_multi_company",
                  "name": "company_id",
                  "options": "{'no_create': True}"
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
      "name": "name"
    },
    {
      "name": "code_prefix_start",
      "attrs": {
        "class": "oe_inline"
      }
    },
    {
      "name": "code_prefix_end",
      "attrs": {
        "class": "oe_inline"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "groups": "base.group_multi_company",
        "options": "{'no_create': True}"
      }
    }
  ]
}

export function renderAccountGroupForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'account.group' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/account.group/<método> (≈ call_kw)
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
