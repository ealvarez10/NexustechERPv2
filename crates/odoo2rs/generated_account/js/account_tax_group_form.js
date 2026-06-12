// Generado por odoo2rs — vista form de account.tax.group (view_tax_group_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_tax_group_form",
  "name": "account.tax.group.form",
  "model": "account.tax.group",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Account Tax Group"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "invisible": "1",
          "name": "company_id"
        }
      },
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
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "groups": "base.group_multi_company",
                      "name": "company_id"
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
                      "groups": "base.group_no_one",
                      "name": "pos_receipt_label",
                      "string": "Label on PoS Receipts"
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
                      "domain": "[('company_ids', '=', company_id)]",
                      "name": "tax_payable_account_id"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "domain": "[('company_ids', '=', company_id)]",
                      "name": "tax_receivable_account_id"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "domain": "[('company_ids', '=', company_id)]",
                      "name": "advance_tax_payment_account_id"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "preceding_subtotal"
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
      "name": "company_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "name"
    },
    {
      "name": "country_id"
    },
    {
      "name": "company_id",
      "attrs": {
        "groups": "base.group_multi_company"
      }
    },
    {
      "name": "sequence"
    },
    {
      "name": "pos_receipt_label",
      "string": "Label on PoS Receipts",
      "attrs": {
        "groups": "base.group_no_one"
      }
    },
    {
      "name": "tax_payable_account_id",
      "attrs": {
        "domain": "[('company_ids', '=', company_id)]"
      }
    },
    {
      "name": "tax_receivable_account_id",
      "attrs": {
        "domain": "[('company_ids', '=', company_id)]"
      }
    },
    {
      "name": "advance_tax_payment_account_id",
      "attrs": {
        "domain": "[('company_ids', '=', company_id)]"
      }
    },
    {
      "name": "preceding_subtotal"
    }
  ]
}

export function renderAccountTaxGroupForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'account.tax.group' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/account.tax.group/<método> (≈ call_kw)
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
