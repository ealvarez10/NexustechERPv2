// Generado por odoo2rs — vista form de account.cash.rounding (rounding_form_view).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "rounding_form_view",
  "name": "account.cash.rounding.form",
  "model": "account.cash.rounding",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Rounding Form"
    },
    "children": [
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "div",
            "attrs": {
              "class": "oe_title"
            },
            "children": [
              {
                "tag": "label",
                "attrs": {
                  "for": "name"
                }
              },
              {
                "tag": "h1",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "oe_inline",
                      "name": "name"
                    }
                  }
                ]
              }
            ]
          },
          {
            "tag": "group",
            "children": [
              {
                "tag": "group",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "rounding"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "strategy"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "groups": "account.group_account_invoice,account.group_account_readonly",
                      "invisible": "strategy != 'add_invoice_line'",
                      "name": "profit_account_id",
                      "options": "{'no_create': True}",
                      "required": "strategy == 'add_invoice_line'"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "groups": "account.group_account_invoice,account.group_account_readonly",
                      "invisible": "strategy != 'add_invoice_line'",
                      "name": "loss_account_id",
                      "options": "{'no_create': True}",
                      "required": "strategy == 'add_invoice_line'"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "rounding_method"
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
      "name": "name",
      "attrs": {
        "class": "oe_inline"
      }
    },
    {
      "name": "rounding"
    },
    {
      "name": "strategy"
    },
    {
      "name": "profit_account_id",
      "attrs": {
        "groups": "account.group_account_invoice,account.group_account_readonly",
        "invisible": "strategy != 'add_invoice_line'",
        "options": "{'no_create': True}",
        "required": "strategy == 'add_invoice_line'"
      }
    },
    {
      "name": "loss_account_id",
      "attrs": {
        "groups": "account.group_account_invoice,account.group_account_readonly",
        "invisible": "strategy != 'add_invoice_line'",
        "options": "{'no_create': True}",
        "required": "strategy == 'add_invoice_line'"
      }
    },
    {
      "name": "rounding_method"
    }
  ]
}

export function renderAccountCashRoundingForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'account.cash.rounding' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/account.cash.rounding/<método> (≈ call_kw)
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
