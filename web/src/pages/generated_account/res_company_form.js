// Generado por odoo2rs — vista form de res.company (res_company_form_view_onboarding_sale_tax).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "res_company_form_view_onboarding_sale_tax",
  "name": "res.company.form.view.onboarding.sale.tax",
  "model": "res.company",
  "type": "form",
  "arch": {
    "tag": "form",
    "children": [
      {
        "tag": "div",
        "attrs": {
          "class": "mb16"
        },
        "text": "Choose a default sales tax for your products."
      },
      {
        "tag": "label",
        "attrs": {
          "for": "account_sale_tax_id",
          "string": "Sales Tax"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "account_sale_tax_id"
        }
      },
      {
        "tag": "footer",
        "children": [
          {
            "tag": "button",
            "attrs": {
              "class": "btn btn-primary",
              "data-hotkey": "q",
              "name": "action_save_onboarding_sale_tax",
              "string": "Apply",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "class": "btn-secondary",
              "data-hotkey": "x",
              "special": "cancel",
              "string": "Cancel"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "account_sale_tax_id"
    }
  ],
  "buttons": [
    {
      "name": "action_save_onboarding_sale_tax",
      "string": "Apply",
      "type": "object",
      "class": "btn btn-primary"
    },
    {
      "string": "Cancel",
      "class": "btn-secondary"
    }
  ]
}

export function renderResCompanyForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'res.company' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/res.company/<método> (≈ call_kw)
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
