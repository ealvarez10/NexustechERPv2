// Generado por odoo2rs — vista form de account.move.line (view_move_line_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_move_line_form",
  "name": "account.move.line.form",
  "model": "account.move.line",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "create": "false",
      "string": "Journal Item"
    },
    "children": [
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "company_id"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "parent_state"
            }
          },
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
                  "domain": "['|', ('parent_id', '=', False), ('is_company', '=', True)]",
                  "name": "partner_id",
                  "readonly": "1"
                }
              }
            ]
          },
          {
            "tag": "notebook",
            "attrs": {
              "colspan": "4"
            },
            "children": [
              {
                "tag": "page",
                "attrs": {
                  "name": "information",
                  "string": "Information"
                },
                "children": [
                  {
                    "tag": "group",
                    "children": [
                      {
                        "tag": "group",
                        "attrs": {
                          "string": "Amount"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "domain": "[('company_ids', 'parent_of', company_id)]",
                              "name": "account_id",
                              "options": "{'no_create': True}",
                              "readonly": "1"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "debit",
                              "readonly": "1"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "credit",
                              "readonly": "1"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "balance",
                              "readonly": "1"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "quantity",
                              "readonly": "1"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "string": "Dates"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "account.group_account_readonly",
                              "name": "date"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "date_maturity"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "invisible": "not tax_line_id and not tax_ids",
                          "string": "Taxes"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "not tax_line_id",
                              "name": "tax_line_id",
                              "readonly": "1"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "not tax_ids",
                              "name": "tax_ids",
                              "readonly": "1",
                              "widget": "many2many_tax_tags"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "invisible": "not matched_debit_ids and not matched_credit_ids",
                          "string": "Matching"
                        },
                        "children": [
                          {
                            "tag": "label",
                            "attrs": {
                              "for": "full_reconcile_id"
                            }
                          },
                          {
                            "tag": "div",
                            "children": [
                              {
                                "tag": "field",
                                "attrs": {
                                  "invisible": "not full_reconcile_id",
                                  "name": "full_reconcile_id"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "invisible": "1",
                                  "name": "matched_debit_ids"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "invisible": "1",
                                  "name": "matched_credit_ids"
                                }
                              },
                              {
                                "tag": "button",
                                "attrs": {
                                  "class": "oe_link",
                                  "invisible": "full_reconcile_id or not matched_debit_ids and not matched_credit_ids",
                                  "name": "open_reconcile_view",
                                  "string": "-> View partially reconciled entries",
                                  "type": "object"
                                }
                              }
                            ]
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "groups": "base.group_multi_currency",
                          "string": "Currency"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "1",
                              "name": "currency_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "1",
                              "name": "display_type"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "amount_currency",
                              "readonly": "display_type != 'tax' or parent_state != 'draft'"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "invisible": "not product_id",
                          "string": "Product"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "product_id",
                              "readonly": "1"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "groups": "analytic.group_analytic_accounting",
                          "string": "Analytic"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "analytic.group_analytic_accounting",
                              "name": "analytic_distribution",
                              "readonly": "1",
                              "widget": "analytic_distribution"
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
                  "groups": "analytic.group_analytic_accounting",
                  "name": "analytic_lines",
                  "string": "Analytic Lines"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "1",
                      "name": "date"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "context": "{'list_view_ref':'analytic.view_account_analytic_line_tree', 'default_general_account_id':account_id, 'default_name': name, 'default_date':date, 'amount': (debit or 0.0)-(credit or 0.0)}",
                      "name": "analytic_line_ids"
                    }
                  }
                ]
              },
              {
                "tag": "page",
                "attrs": {
                  "name": "page_accounting_documents",
                  "string": "Accounting documents"
                },
                "children": [
                  {
                    "tag": "label",
                    "attrs": {
                      "for": "move_id",
                      "string": "Journal Entry"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "move_id",
                      "nolabel": "1",
                      "readonly": "1"
                    }
                  },
                  {
                    "tag": "group",
                    "attrs": {
                      "invisible": "not statement_line_id",
                      "readonly": "1"
                    },
                    "children": [
                      {
                        "tag": "label",
                        "attrs": {
                          "for": "statement_line_id",
                          "string": "Originator Statement Line"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "statement_line_id"
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
  "fields": [
    {
      "name": "company_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "parent_state",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "name"
    },
    {
      "name": "partner_id",
      "attrs": {
        "domain": "['|', ('parent_id', '=', False), ('is_company', '=', True)]",
        "readonly": "1"
      }
    },
    {
      "name": "account_id",
      "attrs": {
        "domain": "[('company_ids', 'parent_of', company_id)]",
        "options": "{'no_create': True}",
        "readonly": "1"
      }
    },
    {
      "name": "debit",
      "attrs": {
        "readonly": "1"
      }
    },
    {
      "name": "credit",
      "attrs": {
        "readonly": "1"
      }
    },
    {
      "name": "balance",
      "attrs": {
        "readonly": "1"
      }
    },
    {
      "name": "quantity",
      "attrs": {
        "readonly": "1"
      }
    },
    {
      "name": "date",
      "attrs": {
        "groups": "account.group_account_readonly"
      }
    },
    {
      "name": "date_maturity"
    },
    {
      "name": "tax_line_id",
      "attrs": {
        "invisible": "not tax_line_id",
        "readonly": "1"
      }
    },
    {
      "name": "tax_ids",
      "widget": "many2many_tax_tags",
      "attrs": {
        "invisible": "not tax_ids",
        "readonly": "1"
      }
    },
    {
      "name": "full_reconcile_id",
      "attrs": {
        "invisible": "not full_reconcile_id"
      }
    },
    {
      "name": "matched_debit_ids",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "matched_credit_ids",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "currency_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "display_type",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "amount_currency",
      "attrs": {
        "readonly": "display_type != 'tax' or parent_state != 'draft'"
      }
    },
    {
      "name": "product_id",
      "attrs": {
        "readonly": "1"
      }
    },
    {
      "name": "analytic_distribution",
      "widget": "analytic_distribution",
      "attrs": {
        "groups": "analytic.group_analytic_accounting",
        "readonly": "1"
      }
    },
    {
      "name": "date",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "analytic_line_ids",
      "attrs": {
        "context": "{'list_view_ref':'analytic.view_account_analytic_line_tree', 'default_general_account_id':account_id, 'default_name': name, 'default_date':date, 'amount': (debit or 0.0)-(credit or 0.0)}"
      }
    },
    {
      "name": "move_id",
      "attrs": {
        "nolabel": "1",
        "readonly": "1"
      }
    },
    {
      "name": "statement_line_id"
    }
  ],
  "buttons": [
    {
      "name": "open_reconcile_view",
      "string": "-> View partially reconciled entries",
      "type": "object",
      "class": "oe_link"
    }
  ]
}

export function renderAccountMoveLineForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'account.move.line' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/account.move.line/<método> (≈ call_kw)
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
