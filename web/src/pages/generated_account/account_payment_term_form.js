// Generado por odoo2rs — vista form de account.payment.term (view_payment_term_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_payment_term_form",
  "name": "account.payment.term.form",
  "model": "account.payment.term",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Payment Terms"
    },
    "children": [
      {
        "tag": "sheet",
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
              "invisible": "1",
              "name": "fiscal_country_codes"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "company_id"
            }
          },
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
            "tag": "div",
            "attrs": {
              "class": "oe_title"
            },
            "children": [
              {
                "tag": "label",
                "attrs": {
                  "for": "name",
                  "string": "Payment Terms"
                }
              },
              {
                "tag": "h1",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "name",
                      "nolabel": "1",
                      "placeholder": "e.g. 30 days"
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
                "tag": "field",
                "attrs": {
                  "class": "w-25",
                  "groups": "base.group_multi_company",
                  "name": "company_id",
                  "options": "{'no_create': True}",
                  "placeholder": "Visible to all"
                }
              },
              {
                "tag": "label",
                "attrs": {
                  "for": "early_discount"
                }
              },
              {
                "tag": "div",
                "attrs": {
                  "class": "o_field_highlight"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "early_discount"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "text-end o_field_highlight o_input oe_inline",
                      "invisible": "not early_discount",
                      "name": "discount_percentage"
                    }
                  },
                  {
                    "tag": "span",
                    "attrs": {
                      "invisible": "not early_discount"
                    },
                    "text": "% if paid within"
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "text-end o_field_highlight o_input oe_inline",
                      "invisible": "not early_discount",
                      "name": "discount_days"
                    }
                  },
                  {
                    "tag": "span",
                    "attrs": {
                      "invisible": "not early_discount"
                    },
                    "text": "days"
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "invisible": "not early_discount"
                    },
                    "children": [
                      {
                        "tag": "span",
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "class": "w-auto",
                              "name": "early_pay_discount_computation"
                            }
                          }
                        ],
                        "text": "Reduced tax:"
                      }
                    ]
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
                "attrs": {
                  "string": "Due Terms"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "line_ids",
                      "nolabel": "1",
                      "widget": "payment_term_line_ids"
                    },
                    "children": [
                      {
                        "tag": "list",
                        "attrs": {
                          "editable": "bottom",
                          "no_open": "True",
                          "string": "Payment Terms"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "value_amount"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "value",
                              "nolabel": "1"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "nb_days",
                              "string": "After"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "delay_type",
                              "nolabel": "1"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "1",
                              "name": "display_days_next_month"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "not display_days_next_month",
                              "name": "days_next_month",
                              "nolabel": "1",
                              "width": "30px"
                            }
                          }
                        ]
                      }
                    ]
                  }
                ]
              },
              {
                "tag": "group",
                "attrs": {
                  "string": "Preview"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "colspan": "2"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "display_on_invoice",
                          "nolabel": "1"
                        }
                      },
                      {
                        "tag": "label",
                        "attrs": {
                          "for": "display_on_invoice"
                        }
                      }
                    ]
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "d-flex gap-2",
                      "col": "4",
                      "colspan": "2"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "class": "oe_inline",
                          "name": "example_amount"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "class": "fw-bold border-bottom o_example_date",
                          "name": "example_date"
                        }
                      }
                    ],
                    "text": "Example:\n                                    \n                                    on"
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "py-1 bg-secondary",
                      "colspan": "2"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "class": "border-bottom o_example_note",
                          "name": "note",
                          "placeholder": "Description on invoice (e.g. Payment terms: 30 days after invoice date)"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "class": "ps-2",
                          "invisible": "not early_discount or not display_on_invoice",
                          "name": "example_preview_discount"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "class": "ps-2",
                          "invisible": "not display_on_invoice",
                          "name": "example_preview"
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
      "name": "active",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "fiscal_country_codes",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "name",
      "attrs": {
        "nolabel": "1",
        "placeholder": "e.g. 30 days"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "class": "w-25",
        "groups": "base.group_multi_company",
        "options": "{'no_create': True}",
        "placeholder": "Visible to all"
      }
    },
    {
      "name": "early_discount"
    },
    {
      "name": "discount_percentage",
      "attrs": {
        "class": "text-end o_field_highlight o_input oe_inline",
        "invisible": "not early_discount"
      }
    },
    {
      "name": "discount_days",
      "attrs": {
        "class": "text-end o_field_highlight o_input oe_inline",
        "invisible": "not early_discount"
      }
    },
    {
      "name": "early_pay_discount_computation",
      "attrs": {
        "class": "w-auto"
      }
    },
    {
      "name": "line_ids",
      "widget": "payment_term_line_ids",
      "attrs": {
        "nolabel": "1"
      }
    },
    {
      "name": "value_amount"
    },
    {
      "name": "value",
      "attrs": {
        "nolabel": "1"
      }
    },
    {
      "name": "nb_days",
      "string": "After"
    },
    {
      "name": "delay_type",
      "attrs": {
        "nolabel": "1"
      }
    },
    {
      "name": "display_days_next_month",
      "attrs": {
        "column_invisible": "1"
      }
    },
    {
      "name": "days_next_month",
      "attrs": {
        "invisible": "not display_days_next_month",
        "nolabel": "1",
        "width": "30px"
      }
    },
    {
      "name": "display_on_invoice",
      "attrs": {
        "nolabel": "1"
      }
    },
    {
      "name": "example_amount",
      "attrs": {
        "class": "oe_inline"
      }
    },
    {
      "name": "example_date",
      "attrs": {
        "class": "fw-bold border-bottom o_example_date"
      }
    },
    {
      "name": "note",
      "attrs": {
        "class": "border-bottom o_example_note",
        "placeholder": "Description on invoice (e.g. Payment terms: 30 days after invoice date)"
      }
    },
    {
      "name": "example_preview_discount",
      "attrs": {
        "class": "ps-2",
        "invisible": "not early_discount or not display_on_invoice"
      }
    },
    {
      "name": "example_preview",
      "attrs": {
        "class": "ps-2",
        "invisible": "not display_on_invoice"
      }
    }
  ]
}

export function renderAccountPaymentTermForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'account.payment.term' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/account.payment.term/<método> (≈ call_kw)
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
