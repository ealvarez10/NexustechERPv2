// Generado por odoo2rs — vista form de account.account (view_account_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_account_form",
  "name": "account.account.form",
  "model": "account.account",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Account"
    },
    "children": [
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "div",
            "attrs": {
              "class": "oe_button_box",
              "name": "button_box"
            },
            "children": [
              {
                "tag": "button",
                "attrs": {
                  "class": "oe_stat_button",
                  "icon": "fa-bars",
                  "invisible": "related_taxes_amount == 0",
                  "name": "action_open_related_taxes",
                  "type": "object"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "o_stat_info"
                    },
                    "children": [
                      {
                        "tag": "span",
                        "attrs": {
                          "class": "o_stat_value"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "related_taxes_amount"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "span",
                        "attrs": {
                          "class": "o_stat_text"
                        },
                        "text": "Taxes"
                      }
                    ]
                  }
                ]
              },
              {
                "tag": "button",
                "attrs": {
                  "class": "oe_stat_button",
                  "icon": "fa-bars",
                  "name": "account.action_move_line_select",
                  "type": "action"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "o_stat_info"
                    },
                    "children": [
                      {
                        "tag": "span",
                        "attrs": {
                          "class": "o_stat_value"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "current_balance"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "span",
                        "attrs": {
                          "class": "o_stat_text"
                        },
                        "text": "Balance"
                      }
                    ]
                  }
                ]
              }
            ]
          },
          {
            "tag": "div",
            "children": [
              {
                "tag": "h1",
                "attrs": {
                  "style": "font-size: 1.9rem;"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "row"
                    },
                    "children": [
                      {
                        "tag": "div",
                        "attrs": {
                          "class": "col col-md-8"
                        },
                        "children": [
                          {
                            "tag": "label",
                            "attrs": {
                              "for": "name",
                              "string": "Account Name"
                            }
                          },
                          {
                            "tag": "div",
                            "children": [
                              {
                                "tag": "field",
                                "attrs": {
                                  "name": "name",
                                  "placeholder": "e.g. Current Assets",
                                  "style": "width:80% !important;"
                                }
                              }
                            ]
                          }
                        ]
                      }
                    ]
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "row"
                    },
                    "children": [
                      {
                        "tag": "div",
                        "attrs": {
                          "class": "col col-md-auto"
                        },
                        "children": [
                          {
                            "tag": "label",
                            "attrs": {
                              "for": "code",
                              "string": "Code"
                            }
                          },
                          {
                            "tag": "div",
                            "children": [
                              {
                                "tag": "field",
                                "attrs": {
                                  "invisible": "1",
                                  "name": "placeholder_code"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "class": "oe_inline",
                                  "name": "code",
                                  "options": "{'placeholder_field': 'placeholder_code'}",
                                  "placeholder": "e.g. 101000"
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
            "tag": "notebook",
            "children": [
              {
                "tag": "page",
                "attrs": {
                  "name": "accounting",
                  "string": "Accounting"
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
                              "name": "account_type",
                              "widget": "account_type_selection"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "account_type == 'off_balance'",
                              "name": "tax_ids",
                              "options": "{'no_quick_create': True}",
                              "widget": "many2many_tax_tags"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "context": "{'default_applicability': 'accounts'}",
                              "domain": "[('applicability', '=', 'accounts')]",
                              "name": "tag_ids",
                              "options": "{'no_create_edit': True}",
                              "widget": "many2many_tags"
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
                              "name": "internal_group",
                              "readonly": "1"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "base.group_multi_currency",
                              "name": "currency_id",
                              "options": "{'no_create': True}",
                              "placeholder": "Any currency",
                              "string": "Currency"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "active",
                              "widget": "boolean_toggle"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "base.group_no_one",
                              "name": "group_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "base.group_multi_company",
                              "name": "company_ids",
                              "options": "{'no_create': True}",
                              "widget": "many2many_tags"
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
                  "name": "Description"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "description"
                    }
                  }
                ]
              },
              {
                "tag": "page",
                "attrs": {
                  "groups": "base.group_multi_company",
                  "invisible": "not display_mapping_tab",
                  "name": "mapping",
                  "string": "Mapping"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "code_mapping_ids",
                      "nolabel": "1"
                    },
                    "children": [
                      {
                        "tag": "list",
                        "attrs": {
                          "editable": "bottom"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "force_save": "1",
                              "name": "company_id"
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
      "name": "related_taxes_amount"
    },
    {
      "name": "current_balance"
    },
    {
      "name": "name",
      "attrs": {
        "placeholder": "e.g. Current Assets",
        "style": "width:80% !important;"
      }
    },
    {
      "name": "placeholder_code",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "code",
      "attrs": {
        "class": "oe_inline",
        "options": "{'placeholder_field': 'placeholder_code'}",
        "placeholder": "e.g. 101000"
      }
    },
    {
      "name": "account_type",
      "widget": "account_type_selection"
    },
    {
      "name": "tax_ids",
      "widget": "many2many_tax_tags",
      "attrs": {
        "invisible": "account_type == 'off_balance'",
        "options": "{'no_quick_create': True}"
      }
    },
    {
      "name": "tag_ids",
      "widget": "many2many_tags",
      "attrs": {
        "context": "{'default_applicability': 'accounts'}",
        "domain": "[('applicability', '=', 'accounts')]",
        "options": "{'no_create_edit': True}"
      }
    },
    {
      "name": "internal_group",
      "attrs": {
        "invisible": "1",
        "readonly": "1"
      }
    },
    {
      "name": "currency_id",
      "string": "Currency",
      "attrs": {
        "groups": "base.group_multi_currency",
        "options": "{'no_create': True}",
        "placeholder": "Any currency"
      }
    },
    {
      "name": "active",
      "widget": "boolean_toggle"
    },
    {
      "name": "group_id",
      "attrs": {
        "groups": "base.group_no_one"
      }
    },
    {
      "name": "company_ids",
      "widget": "many2many_tags",
      "attrs": {
        "groups": "base.group_multi_company",
        "options": "{'no_create': True}"
      }
    },
    {
      "name": "description"
    },
    {
      "name": "code_mapping_ids",
      "attrs": {
        "nolabel": "1"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "force_save": "1"
      }
    },
    {
      "name": "code"
    }
  ],
  "buttons": [
    {
      "name": "action_open_related_taxes",
      "type": "object",
      "class": "oe_stat_button"
    },
    {
      "name": "account.action_move_line_select",
      "type": "action",
      "class": "oe_stat_button"
    }
  ]
}

export function renderAccountAccountForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'account.account' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/account.account/<método> (≈ call_kw)
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
