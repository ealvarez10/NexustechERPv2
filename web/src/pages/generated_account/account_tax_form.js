// Generado por odoo2rs — vista form de account.tax (view_tax_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_tax_form",
  "name": "account.tax.form",
  "model": "account.tax",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Account Tax"
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
                      "name": "amount_type"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "active",
                      "widget": "boolean_toggle"
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
                      "invisible": "1",
                      "name": "is_used"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "type_tax_use"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "tax_scope"
                    }
                  },
                  {
                    "tag": "label",
                    "attrs": {
                      "for": "amount",
                      "invisible": "amount_type not in ('fixed', 'percent', 'division')"
                    }
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "invisible": "amount_type not in ('fixed', 'percent', 'division')"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "class": "oe_inline",
                          "name": "amount",
                          "nolabel": "1"
                        }
                      },
                      {
                        "tag": "span",
                        "attrs": {
                          "class": "o_form_label oe_inline",
                          "invisible": "amount_type == 'fixed'"
                        },
                        "text": "%"
                      }
                    ]
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "fiscal_position_ids",
                      "placeholder": "all",
                      "widget": "many2many_tags"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "not display_alternative_taxes_field",
                      "name": "original_tax_ids",
                      "options": "{'no_create': True}",
                      "widget": "many2many_tags"
                    }
                  }
                ]
              }
            ]
          },
          {
            "tag": "notebook",
            "children": [
              {
                "tag": "page",
                "attrs": {
                  "name": "definition",
                  "string": "Definition"
                },
                "children": [
                  {
                    "tag": "group",
                    "attrs": {
                      "invisible": "amount_type == 'group'"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "invisible": "1",
                          "name": "country_code"
                        }
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "class": "mw-100",
                          "string": "Distribution for Invoices"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "colspan": "2",
                              "name": "invoice_repartition_line_ids",
                              "nolabel": "1"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "class": "mw-100",
                          "string": "Distribution for Refunds"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "colspan": "2",
                              "name": "refund_repartition_line_ids",
                              "nolabel": "1"
                            }
                          }
                        ]
                      }
                    ]
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "domain": "[('type_tax_use','in',('none',type_tax_use)), ('amount_type','!=','group')]",
                      "invisible": "amount_type != 'group' or type_tax_use == 'none'",
                      "name": "children_tax_ids"
                    },
                    "children": [
                      {
                        "tag": "list",
                        "attrs": {
                          "string": "Children Taxes"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "sequence",
                              "widget": "handle"
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
                              "name": "amount_type"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "amount"
                            }
                          }
                        ]
                      }
                    ]
                  }
                ]
              },
              {
                "tag": "page",
                "attrs": {
                  "name": "advanced_options",
                  "string": "Advanced Options"
                },
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
                              "name": "invoice_label"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "description"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "amount_type == 'group'",
                              "name": "tax_group_id",
                              "required": "amount_type != 'group'"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "analytic.group_analytic_accounting",
                              "invisible": "amount_type == 'group'",
                              "name": "analytic"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "base.group_multi_company",
                              "name": "company_id",
                              "options": "{'no_create': True}"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "country_id",
                              "required": "True"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "invoice_legal_notes"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "name": "advanced_booleans"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "1",
                              "name": "price_include"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "amount_type == 'group' or has_negative_factor",
                              "name": "price_include_override",
                              "placeholder": "Default"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "amount_type == 'group'",
                              "name": "include_base_amount"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "base.group_no_one",
                              "invisible": "amount_type == 'group' or price_include",
                              "name": "is_base_affected"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "1",
                              "name": "hide_tax_exigibility"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "amount_type == 'group' or not hide_tax_exigibility",
                              "name": "tax_exigibility",
                              "widget": "radio"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "account.group_account_readonly",
                              "invisible": "tax_exigibility == 'on_invoice'",
                              "name": "cash_basis_transition_account_id",
                              "options": "{'no_create': True}",
                              "required": "tax_exigibility == 'on_payment'"
                            }
                          }
                        ]
                      }
                    ]
                  }
                ]
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
      "name": "company_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "name"
    },
    {
      "name": "amount_type"
    },
    {
      "name": "active",
      "widget": "boolean_toggle"
    },
    {
      "name": "is_used",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "type_tax_use"
    },
    {
      "name": "tax_scope"
    },
    {
      "name": "amount",
      "attrs": {
        "class": "oe_inline",
        "nolabel": "1"
      }
    },
    {
      "name": "fiscal_position_ids",
      "widget": "many2many_tags",
      "attrs": {
        "placeholder": "all"
      }
    },
    {
      "name": "original_tax_ids",
      "widget": "many2many_tags",
      "attrs": {
        "invisible": "not display_alternative_taxes_field",
        "options": "{'no_create': True}"
      }
    },
    {
      "name": "country_code",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "invoice_repartition_line_ids",
      "attrs": {
        "colspan": "2",
        "nolabel": "1"
      }
    },
    {
      "name": "refund_repartition_line_ids",
      "attrs": {
        "colspan": "2",
        "nolabel": "1"
      }
    },
    {
      "name": "children_tax_ids",
      "attrs": {
        "domain": "[('type_tax_use','in',('none',type_tax_use)), ('amount_type','!=','group')]",
        "invisible": "amount_type != 'group' or type_tax_use == 'none'"
      }
    },
    {
      "name": "sequence",
      "widget": "handle"
    },
    {
      "name": "name"
    },
    {
      "name": "amount_type"
    },
    {
      "name": "amount"
    },
    {
      "name": "invoice_label"
    },
    {
      "name": "description"
    },
    {
      "name": "tax_group_id",
      "attrs": {
        "invisible": "amount_type == 'group'",
        "required": "amount_type != 'group'"
      }
    },
    {
      "name": "analytic",
      "attrs": {
        "groups": "analytic.group_analytic_accounting",
        "invisible": "amount_type == 'group'"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "groups": "base.group_multi_company",
        "options": "{'no_create': True}"
      }
    },
    {
      "name": "country_id",
      "attrs": {
        "required": "True"
      }
    },
    {
      "name": "invoice_legal_notes"
    },
    {
      "name": "price_include",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "price_include_override",
      "attrs": {
        "invisible": "amount_type == 'group' or has_negative_factor",
        "placeholder": "Default"
      }
    },
    {
      "name": "include_base_amount",
      "attrs": {
        "invisible": "amount_type == 'group'"
      }
    },
    {
      "name": "is_base_affected",
      "attrs": {
        "groups": "base.group_no_one",
        "invisible": "amount_type == 'group' or price_include"
      }
    },
    {
      "name": "hide_tax_exigibility",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "tax_exigibility",
      "widget": "radio",
      "attrs": {
        "invisible": "amount_type == 'group' or not hide_tax_exigibility"
      }
    },
    {
      "name": "cash_basis_transition_account_id",
      "attrs": {
        "groups": "account.group_account_readonly",
        "invisible": "tax_exigibility == 'on_invoice'",
        "options": "{'no_create': True}",
        "required": "tax_exigibility == 'on_payment'"
      }
    }
  ]
}

export function renderAccountTaxForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'account.tax' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/account.tax/<método> (≈ call_kw)
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
