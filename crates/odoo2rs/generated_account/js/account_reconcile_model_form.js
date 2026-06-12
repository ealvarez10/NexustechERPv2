// Generado por odoo2rs — vista form de account.reconcile.model (view_account_reconcile_model_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_account_reconcile_model_form",
  "name": "account.reconcile.model.form",
  "model": "account.reconcile.model",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Operation Templates"
    },
    "children": [
      {
        "tag": "header",
        "children": [
          {
            "tag": "button",
            "attrs": {
              "class": "oe_highlight",
              "data-hotkey": "y",
              "invisible": "trigger == 'manual'",
              "name": "action_set_manual",
              "string": "Set Manual",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "class": "oe_highlight",
              "data-hotkey": "q",
              "invisible": "trigger == 'auto_reconcile'",
              "name": "action_set_auto_reconcile",
              "string": "Automate",
              "type": "object"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "name": "trigger",
              "widget": "statusbar"
            }
          }
        ]
      },
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
          "name": "company_id"
        }
      },
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
                  "icon": "fa-book",
                  "name": "action_reconcile_stat",
                  "string": "Journal Entries",
                  "type": "object"
                }
              }
            ]
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
                  "placeholder": "Model Name"
                }
              },
              {
                "tag": "h1",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "name",
                      "placeholder": "e.g. Bank Fees"
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
                "attrs": {
                  "id": "filters_left_column"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "match_journal_ids",
                      "options": "{'no_create': True}",
                      "placeholder": "All bank & cash journals",
                      "widget": "many2many_tags"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "match_partner_ids",
                      "options": "{'no_quick_create': True}",
                      "placeholder": "All partners",
                      "widget": "many2many_tags"
                    }
                  },
                  {
                    "tag": "label",
                    "attrs": {
                      "for": "match_amount"
                    }
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "d-flex gap-2"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "match_amount",
                          "placeholder": "Any amount"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "invisible": "match_amount in (False, 'lower')",
                          "name": "match_amount_min",
                          "required": "match_amount"
                        }
                      },
                      {
                        "tag": "span",
                        "attrs": {
                          "class": "o_form_label",
                          "invisible": "match_amount != 'between'"
                        },
                        "text": "and"
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "invisible": "match_amount in (False, 'greater')",
                          "name": "match_amount_max",
                          "required": "match_amount == 'between'"
                        }
                      }
                    ]
                  },
                  {
                    "tag": "label",
                    "attrs": {
                      "for": "match_label"
                    }
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "d-flex gap-3"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "match_label",
                          "placeholder": "Any label"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "invisible": "not match_label",
                          "name": "match_label_param",
                          "placeholder": "BRT *([\\d,\\.]+)",
                          "required": "match_label"
                        }
                      }
                    ]
                  }
                ]
              },
              {
                "tag": "group",
                "attrs": {
                  "class": "col-6",
                  "id": "counterpart_entry_right_column"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "next_activity_type_id",
                      "placeholder": "Nothing to do"
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
                  "id": "counterpart_items_tab",
                  "name": "counterpart_items",
                  "string": "Counterpart Items"
                },
                "children": [
                  {
                    "tag": "group",
                    "attrs": {
                      "class": "oe_inline"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "default": "{'default_model_id': self, 'default_company_id': self.company_id}",
                          "name": "line_ids",
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
                                  "column_invisible": "True",
                                  "name": "company_id"
                                }
                              },
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
                                  "name": "partner_id"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "name": "account_id"
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
                                  "name": "amount_string"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "name": "tax_ids",
                                  "optional": "hide",
                                  "widget": "many2many_tax_tags"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "groups": "analytic.group_analytic_accounting",
                                  "name": "analytic_distribution",
                                  "options": "{'account_field': 'account_id', 'business_domain': 'general'}",
                                  "string": "Analytic",
                                  "widget": "analytic_distribution"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "name": "label"
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
      "name": "trigger",
      "widget": "statusbar"
    },
    {
      "name": "active",
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
        "placeholder": "e.g. Bank Fees"
      }
    },
    {
      "name": "match_journal_ids",
      "widget": "many2many_tags",
      "attrs": {
        "options": "{'no_create': True}",
        "placeholder": "All bank & cash journals"
      }
    },
    {
      "name": "match_partner_ids",
      "widget": "many2many_tags",
      "attrs": {
        "options": "{'no_quick_create': True}",
        "placeholder": "All partners"
      }
    },
    {
      "name": "match_amount",
      "attrs": {
        "placeholder": "Any amount"
      }
    },
    {
      "name": "match_amount_min",
      "attrs": {
        "invisible": "match_amount in (False, 'lower')",
        "required": "match_amount"
      }
    },
    {
      "name": "match_amount_max",
      "attrs": {
        "invisible": "match_amount in (False, 'greater')",
        "required": "match_amount == 'between'"
      }
    },
    {
      "name": "match_label",
      "attrs": {
        "placeholder": "Any label"
      }
    },
    {
      "name": "match_label_param",
      "attrs": {
        "invisible": "not match_label",
        "placeholder": "BRT *([\\d,\\.]+)",
        "required": "match_label"
      }
    },
    {
      "name": "next_activity_type_id",
      "attrs": {
        "placeholder": "Nothing to do"
      }
    },
    {
      "name": "line_ids",
      "attrs": {
        "default": "{'default_model_id': self, 'default_company_id': self.company_id}",
        "nolabel": "1"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "sequence",
      "widget": "handle"
    },
    {
      "name": "partner_id"
    },
    {
      "name": "account_id"
    },
    {
      "name": "amount_type"
    },
    {
      "name": "amount_string"
    },
    {
      "name": "tax_ids",
      "widget": "many2many_tax_tags",
      "attrs": {
        "optional": "hide"
      }
    },
    {
      "name": "analytic_distribution",
      "string": "Analytic",
      "widget": "analytic_distribution",
      "attrs": {
        "groups": "analytic.group_analytic_accounting",
        "options": "{'account_field': 'account_id', 'business_domain': 'general'}"
      }
    },
    {
      "name": "label"
    }
  ],
  "buttons": [
    {
      "name": "action_set_manual",
      "string": "Set Manual",
      "type": "object",
      "class": "oe_highlight"
    },
    {
      "name": "action_set_auto_reconcile",
      "string": "Automate",
      "type": "object",
      "class": "oe_highlight"
    },
    {
      "name": "action_reconcile_stat",
      "string": "Journal Entries",
      "type": "object",
      "class": "oe_stat_button"
    }
  ]
}

export function renderAccountReconcileModelForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'account.reconcile.model' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/account.reconcile.model/<método> (≈ call_kw)
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
