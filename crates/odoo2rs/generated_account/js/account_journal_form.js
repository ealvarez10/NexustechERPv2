// Generado por odoo2rs — vista form de account.journal (view_account_journal_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_account_journal_form",
  "name": "account.journal.form",
  "model": "account.journal",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Account Journal"
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
        "tag": "field",
        "attrs": {
          "invisible": "1",
          "name": "bank_statements_source"
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
                  "context": "{'search_default_journal_id':id}",
                  "icon": "fa-book",
                  "name": "%(action_account_moves_all_a)d",
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
                          "class": "o_stat_text"
                        },
                        "text": "Journal Entries"
                      }
                    ]
                  }
                ]
              }
            ]
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
                  "for": "name"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "force_save": "1",
                  "invisible": "1",
                  "name": "name_placeholder"
                }
              },
              {
                "tag": "h1",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "name",
                      "options": "{'placeholder_field': 'name_placeholder'}",
                      "required": "not type"
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
                      "invisible": "1",
                      "name": "active"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "type"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "code",
                      "placeholder": "e.g. INV"
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
                      "groups": "base.group_multi_company",
                      "name": "company_id",
                      "options": "{'no_create': True}"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "1",
                      "name": "country_code"
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
                  "name": "bank_account",
                  "string": "Journal Entries"
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
                              "invisible": "1",
                              "name": "default_account_type"
                            }
                          },
                          {
                            "tag": "label",
                            "attrs": {
                              "for": "default_account_id",
                              "groups": "account.group_account_readonly",
                              "invisible": "type != 'bank'",
                              "string": "Bank Account"
                            }
                          },
                          {
                            "tag": "label",
                            "attrs": {
                              "for": "default_account_id",
                              "groups": "account.group_account_readonly",
                              "invisible": "type != 'credit'",
                              "string": "Journal Account"
                            }
                          },
                          {
                            "tag": "label",
                            "attrs": {
                              "for": "default_account_id",
                              "groups": "account.group_account_readonly",
                              "invisible": "type != 'cash'",
                              "string": "Cash Account"
                            }
                          },
                          {
                            "tag": "label",
                            "attrs": {
                              "for": "default_account_id",
                              "groups": "account.group_account_readonly",
                              "invisible": "type != 'sale'",
                              "string": "Default Income Account"
                            }
                          },
                          {
                            "tag": "label",
                            "attrs": {
                              "for": "default_account_id",
                              "groups": "account.group_account_readonly",
                              "invisible": "type != 'purchase'",
                              "string": "Default Expense Account"
                            }
                          },
                          {
                            "tag": "label",
                            "attrs": {
                              "for": "default_account_id",
                              "groups": "account.group_account_readonly",
                              "help": "If set, this account is used to automatically balance entries.",
                              "invisible": "type != 'general'",
                              "string": "Default Account"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "account.group_account_readonly",
                              "invisible": "not type or type in ('sale', 'purchase', 'general')",
                              "name": "default_account_id",
                              "nolabel": "1",
                              "options": "{'no_quick_create': True}",
                              "placeholder": "Create new account",
                              "required": "(id and type in ('bank', 'cash', 'credit'))"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "account.group_account_readonly",
                              "invisible": "type not in ('sale', 'purchase', 'general')",
                              "name": "default_account_id",
                              "nolabel": "1",
                              "options": "{'no_quick_create': True}",
                              "required": "type in ('sale', 'purchase')"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "account.group_account_readonly",
                              "invisible": "type not in ('bank', 'cash', 'credit')",
                              "name": "suspense_account_id",
                              "options": "{'no_quick_create': True}",
                              "required": "type in ('bank', 'cash', 'credit')"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "account.group_account_readonly",
                              "invisible": "type != 'purchase'",
                              "name": "non_deductible_account_id",
                              "options": "{'no_quick_create': True}"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "account.group_account_readonly",
                              "invisible": "type not in ('cash', 'bank')",
                              "name": "profit_account_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "account.group_account_readonly",
                              "invisible": "type not in ('cash', 'bank')",
                              "name": "loss_account_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "type not in ['sale', 'purchase']",
                              "name": "refund_sequence"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "type not in ('bank', 'cash', 'credit')",
                              "name": "payment_sequence"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "not display_invoice_template_pdf_report_id",
                              "name": "invoice_template_pdf_report_id",
                              "options": "{'no_create': True, 'no_edit': True}"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "base.group_multi_currency",
                              "name": "currency_id",
                              "options": "{'no_create': True}"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "invisible": "type != 'bank'",
                          "name": "bank_account_number"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "1",
                              "name": "company_partner_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "context": "{'default_partner_id': company_partner_id}",
                              "name": "bank_account_id",
                              "string": "Bank Account Number"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "not bank_account_id",
                              "name": "bank_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "account.group_account_basic",
                              "name": "bank_statements_source",
                              "required": "type == 'bank'",
                              "widget": "radio"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "invisible": "type != 'credit'",
                          "name": "bank_source"
                        },
                        "children": [
                          {
                            "tag": "label",
                            "attrs": {
                              "for": "bank_statements_source",
                              "groups": "account.group_account_readonly",
                              "invisible": "type != 'credit'",
                              "string": "Transaction Feeds"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "account.group_account_basic",
                              "name": "bank_statements_source",
                              "nolabel": "1",
                              "required": "type == 'credit'",
                              "widget": "radio"
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
                  "id": "inbound_payment_settings",
                  "invisible": "type not in ['cash', 'bank', 'credit']",
                  "name": "page_incoming_payments",
                  "string": "Incoming Payments"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "1",
                      "name": "available_payment_method_ids"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "context": "{'default_payment_type': 'inbound'}",
                      "name": "inbound_payment_method_line_ids",
                      "nolabel": "1"
                    },
                    "children": [
                      {
                        "tag": "list",
                        "attrs": {
                          "editable": "bottom",
                          "string": "Payment Methods"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "True",
                              "name": "available_payment_method_ids"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "True",
                              "name": "payment_type"
                            }
                          },
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
                              "name": "payment_method_id",
                              "options": "{'no_create': True, 'no_open': True}"
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
                              "groups": "account.group_account_readonly",
                              "name": "payment_account_id",
                              "options": "{'no_quick_create': True}",
                              "placeholder": "No payment journal entries",
                              "string": "Outstanding Receipts accounts"
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
                  "id": "outbound_payment_settings",
                  "invisible": "type not in ['cash', 'bank', 'credit']",
                  "name": "page_outgoing_payments",
                  "string": "Outgoing Payments"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "context": "{'default_payment_type': 'outbound'}",
                      "name": "outbound_payment_method_line_ids",
                      "nolabel": "1"
                    },
                    "children": [
                      {
                        "tag": "list",
                        "attrs": {
                          "editable": "bottom",
                          "string": "Payment Methods"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "True",
                              "name": "available_payment_method_ids"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "True",
                              "name": "payment_type"
                            }
                          },
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
                              "name": "payment_method_id",
                              "options": "{'no_create': True, 'no_open': True}"
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
                              "groups": "account.group_account_readonly",
                              "name": "payment_account_id",
                              "options": "{'no_quick_create': True}",
                              "placeholder": "No payment journal entries",
                              "string": "Outstanding Payments accounts"
                            }
                          }
                        ]
                      }
                    ]
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "1",
                      "name": "selected_payment_method_codes"
                    }
                  },
                  {
                    "tag": "group",
                    "attrs": {
                      "name": "outgoing_payment"
                    }
                  }
                ]
              },
              {
                "tag": "page",
                "attrs": {
                  "name": "advanced_settings",
                  "string": "Advanced Settings"
                },
                "children": [
                  {
                    "tag": "group",
                    "children": [
                      {
                        "tag": "group",
                        "attrs": {
                          "groups": "account.group_account_manager",
                          "string": "Automation"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "account.group_account_readonly",
                              "invisible": "type not in ['sale', 'purchase', 'general']",
                              "name": "restrict_mode_hash_table"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "type != 'purchase'",
                              "name": "is_self_billing"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "invisible": "type not in ('general', 'sale', 'purchase')",
                          "name": "group_email_alias",
                          "string": "Emails"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "1",
                              "name": "display_alias_fields"
                            }
                          },
                          {
                            "tag": "div",
                            "attrs": {
                              "class": "o_row",
                              "colspan": "2",
                              "invisible": "display_alias_fields"
                            },
                            "children": [
                              {
                                "tag": "a",
                                "attrs": {
                                  "class": "btn btn-link",
                                  "name": "%(action_open_settings)d",
                                  "role": "button",
                                  "type": "action"
                                },
                                "children": [
                                  {
                                    "tag": "i",
                                    "attrs": {
                                      "class": "oi oi-fw o_button_icon oi-arrow-right"
                                    }
                                  }
                                ],
                                "text": "Configure Alias Domain"
                              }
                            ]
                          },
                          {
                            "tag": "div",
                            "attrs": {
                              "class": "o_row",
                              "colspan": "2",
                              "dir": "ltr",
                              "invisible": "not display_alias_fields"
                            },
                            "children": [
                              {
                                "tag": "label",
                                "attrs": {
                                  "for": "alias_name",
                                  "string": "Email Alias"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "name": "alias_name",
                                  "placeholder": "alias"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "name": "alias_domain_id",
                                  "options": "{'no_create': True, 'no_open': True}",
                                  "placeholder": "e.g. mycompany.com"
                                }
                              }
                            ],
                            "text": "@"
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "class": "w-100",
                              "name": "incoming_einvoice_notification_email",
                              "placeholder": "e.g. finance@example.com; accountant@example.com"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "invisible": "1",
                          "name": "group_edi_config",
                          "string": "Electronic Data Interchange"
                        }
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "invisible": "type != 'sale'",
                          "string": "Payment Communications"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "invoice_reference_type"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "invoice_reference_type == 'none'",
                              "name": "invoice_reference_model"
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
      "name": "bank_statements_source",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "name_placeholder",
      "attrs": {
        "force_save": "1",
        "invisible": "1"
      }
    },
    {
      "name": "name",
      "attrs": {
        "options": "{'placeholder_field': 'name_placeholder'}",
        "required": "not type"
      }
    },
    {
      "name": "active",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "type"
    },
    {
      "name": "code",
      "attrs": {
        "placeholder": "e.g. INV"
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
      "name": "country_code",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "default_account_type",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "default_account_id",
      "attrs": {
        "groups": "account.group_account_readonly",
        "invisible": "not type or type in ('sale', 'purchase', 'general')",
        "nolabel": "1",
        "options": "{'no_quick_create': True}",
        "placeholder": "Create new account",
        "required": "(id and type in ('bank', 'cash', 'credit'))"
      }
    },
    {
      "name": "default_account_id",
      "attrs": {
        "groups": "account.group_account_readonly",
        "invisible": "type not in ('sale', 'purchase', 'general')",
        "nolabel": "1",
        "options": "{'no_quick_create': True}",
        "required": "type in ('sale', 'purchase')"
      }
    },
    {
      "name": "suspense_account_id",
      "attrs": {
        "groups": "account.group_account_readonly",
        "invisible": "type not in ('bank', 'cash', 'credit')",
        "options": "{'no_quick_create': True}",
        "required": "type in ('bank', 'cash', 'credit')"
      }
    },
    {
      "name": "non_deductible_account_id",
      "attrs": {
        "groups": "account.group_account_readonly",
        "invisible": "type != 'purchase'",
        "options": "{'no_quick_create': True}"
      }
    },
    {
      "name": "profit_account_id",
      "attrs": {
        "groups": "account.group_account_readonly",
        "invisible": "type not in ('cash', 'bank')"
      }
    },
    {
      "name": "loss_account_id",
      "attrs": {
        "groups": "account.group_account_readonly",
        "invisible": "type not in ('cash', 'bank')"
      }
    },
    {
      "name": "refund_sequence",
      "attrs": {
        "invisible": "type not in ['sale', 'purchase']"
      }
    },
    {
      "name": "payment_sequence",
      "attrs": {
        "invisible": "type not in ('bank', 'cash', 'credit')"
      }
    },
    {
      "name": "invoice_template_pdf_report_id",
      "attrs": {
        "invisible": "not display_invoice_template_pdf_report_id",
        "options": "{'no_create': True, 'no_edit': True}"
      }
    },
    {
      "name": "currency_id",
      "attrs": {
        "groups": "base.group_multi_currency",
        "options": "{'no_create': True}"
      }
    },
    {
      "name": "company_partner_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "bank_account_id",
      "string": "Bank Account Number",
      "attrs": {
        "context": "{'default_partner_id': company_partner_id}"
      }
    },
    {
      "name": "bank_id",
      "attrs": {
        "invisible": "not bank_account_id"
      }
    },
    {
      "name": "bank_statements_source",
      "widget": "radio",
      "attrs": {
        "groups": "account.group_account_basic",
        "required": "type == 'bank'"
      }
    },
    {
      "name": "bank_statements_source",
      "widget": "radio",
      "attrs": {
        "groups": "account.group_account_basic",
        "nolabel": "1",
        "required": "type == 'credit'"
      }
    },
    {
      "name": "available_payment_method_ids",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "inbound_payment_method_line_ids",
      "attrs": {
        "context": "{'default_payment_type': 'inbound'}",
        "nolabel": "1"
      }
    },
    {
      "name": "available_payment_method_ids",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "payment_type",
      "attrs": {
        "column_invisible": "True"
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
      "name": "payment_method_id",
      "attrs": {
        "options": "{'no_create': True, 'no_open': True}"
      }
    },
    {
      "name": "name"
    },
    {
      "name": "payment_account_id",
      "string": "Outstanding Receipts accounts",
      "attrs": {
        "groups": "account.group_account_readonly",
        "options": "{'no_quick_create': True}",
        "placeholder": "No payment journal entries"
      }
    },
    {
      "name": "outbound_payment_method_line_ids",
      "attrs": {
        "context": "{'default_payment_type': 'outbound'}",
        "nolabel": "1"
      }
    },
    {
      "name": "available_payment_method_ids",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "payment_type",
      "attrs": {
        "column_invisible": "True"
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
      "name": "payment_method_id",
      "attrs": {
        "options": "{'no_create': True, 'no_open': True}"
      }
    },
    {
      "name": "name"
    },
    {
      "name": "payment_account_id",
      "string": "Outstanding Payments accounts",
      "attrs": {
        "groups": "account.group_account_readonly",
        "options": "{'no_quick_create': True}",
        "placeholder": "No payment journal entries"
      }
    },
    {
      "name": "selected_payment_method_codes",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "restrict_mode_hash_table",
      "attrs": {
        "groups": "account.group_account_readonly",
        "invisible": "type not in ['sale', 'purchase', 'general']"
      }
    },
    {
      "name": "is_self_billing",
      "attrs": {
        "invisible": "type != 'purchase'"
      }
    },
    {
      "name": "display_alias_fields",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "alias_name",
      "attrs": {
        "placeholder": "alias"
      }
    },
    {
      "name": "alias_domain_id",
      "attrs": {
        "options": "{'no_create': True, 'no_open': True}",
        "placeholder": "e.g. mycompany.com"
      }
    },
    {
      "name": "incoming_einvoice_notification_email",
      "attrs": {
        "class": "w-100",
        "placeholder": "e.g. finance@example.com; accountant@example.com"
      }
    },
    {
      "name": "invoice_reference_type"
    },
    {
      "name": "invoice_reference_model",
      "attrs": {
        "invisible": "invoice_reference_type == 'none'"
      }
    }
  ],
  "buttons": [
    {
      "name": "%(action_account_moves_all_a)d",
      "type": "action",
      "class": "oe_stat_button"
    }
  ]
}

export function renderAccountJournalForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'account.journal' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/account.journal/<método> (≈ call_kw)
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
