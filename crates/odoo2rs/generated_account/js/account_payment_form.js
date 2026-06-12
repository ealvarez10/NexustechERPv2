// Generado por odoo2rs — vista form de account.payment (view_account_payment_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_account_payment_form",
  "name": "account.payment.form",
  "model": "account.payment",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Pay"
    },
    "children": [
      {
        "tag": "header",
        "children": [
          {
            "tag": "button",
            "attrs": {
              "class": "oe_highlight",
              "data-hotkey": "q",
              "invisible": "state != 'draft'",
              "name": "action_post",
              "string": "Confirm",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "class": "oe_highlight",
              "data-hotkey": "q",
              "invisible": "state != 'in_process' or move_id",
              "name": "action_validate",
              "string": "Validate",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "data-hotkey": "q",
              "invisible": "state != 'in_process' or not is_sent",
              "name": "action_reject",
              "string": "Reject",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "class": "btn btn-secondary",
              "data-hotkey": "w",
              "groups": "account.group_account_invoice",
              "invisible": "state in ('draft')",
              "name": "action_draft",
              "string": "Reset to Draft",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "data-hotkey": "w",
              "groups": "account.group_account_invoice",
              "invisible": "state != 'in_process' or not move_id or not need_cancel_request",
              "name": "button_request_cancel",
              "string": "Request Cancel",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "data-hotkey": "v",
              "invisible": "state != 'in_process' or is_sent or payment_method_code != 'manual'",
              "name": "mark_as_sent",
              "string": "Mark as Sent",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "data-hotkey": "k",
              "invisible": "state != 'in_process' or not is_sent or payment_method_code != 'manual'",
              "name": "unmark_as_sent",
              "string": "Unmark as Sent",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "data-hotkey": "x",
              "invisible": "not id or not (state == 'draft' or (state == 'in_process' and is_sent))",
              "name": "action_cancel",
              "string": "Cancel",
              "type": "object"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "name": "state",
              "statusbar_visible": "draft,in_process,paid",
              "widget": "statusbar"
            }
          }
        ]
      },
      {
        "tag": "div",
        "attrs": {
          "class": "alert alert-warning mb-2",
          "invisible": "not duplicate_payment_ids or state!='draft'",
          "role": "alert"
        },
        "children": [
          {
            "tag": "span",
            "text": "This payment has the same partner, amount and date as"
          },
          {
            "tag": "field",
            "attrs": {
              "name": "duplicate_payment_ids",
              "string": "Duplicated Payments",
              "widget": "x2many_buttons"
            }
          }
        ]
      },
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "id"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "is_sent"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "need_cancel_request"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "is_reconciled"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "is_matched"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "payment_method_code"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "show_partner_bank_account"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "require_partner_bank_account"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "available_payment_method_line_ids"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "available_partner_bank_ids"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "country_code"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "partner_type"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "reconciled_invoices_type"
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
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "paired_internal_transfer_payment_id"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "available_journal_ids"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "currency_id"
            }
          },
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
                  "invisible": "reconciled_invoices_count == 0",
                  "name": "button_open_invoices",
                  "type": "object"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "o_form_field o_stat_info"
                    },
                    "children": [
                      {
                        "tag": "span",
                        "attrs": {
                          "class": "o_stat_text"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "reconciled_invoices_count"
                            }
                          },
                          {
                            "tag": "span",
                            "attrs": {
                              "invisible": "reconciled_invoices_type != 'invoice'"
                            },
                            "text": "Invoice"
                          },
                          {
                            "tag": "span",
                            "attrs": {
                              "invisible": "reconciled_invoices_type == 'invoice'"
                            },
                            "text": "Credit Note"
                          }
                        ]
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
                  "invisible": "reconciled_bills_count == 0",
                  "name": "button_open_bills",
                  "type": "object"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "o_form_field o_stat_info"
                    },
                    "children": [
                      {
                        "tag": "span",
                        "attrs": {
                          "class": "o_stat_text"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "reconciled_bills_count"
                            }
                          },
                          {
                            "tag": "span",
                            "text": "Bill"
                          }
                        ]
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
                  "invisible": "reconciled_statement_lines_count == 0",
                  "name": "button_open_statement_lines",
                  "type": "object"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "o_form_field o_stat_info"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "reconciled_statement_lines_count"
                        }
                      },
                      {
                        "tag": "span",
                        "attrs": {
                          "class": "o_stat_text"
                        },
                        "text": "Transaction"
                      }
                    ]
                  }
                ]
              },
              {
                "tag": "button",
                "attrs": {
                  "class": "oe_stat_button",
                  "groups": "account.group_account_user,account.group_account_readonly",
                  "icon": "fa-bars",
                  "invisible": "not move_id",
                  "name": "button_open_journal_entry",
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
                          "class": "o_stat_text"
                        },
                        "text": "Journal Entry"
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
              "bg_color": "text-bg-info",
              "invisible": "state != 'invoicing_legacy'",
              "name": "web_ribbon",
              "text": "Invoicing App Legacy",
              "tooltip": "This payment has been generated through the Invoicing app, before installing Accounting. It has been disabled by the 'Invoicing Switch Threshold Date' setting so that it does not impact your accounting."
            }
          },
          {
            "tag": "div",
            "attrs": {
              "class": "oe_title"
            },
            "children": [
              {
                "tag": "h1",
                "attrs": {
                  "invisible": "state != 'draft'"
                },
                "children": [
                  {
                    "tag": "span",
                    "text": "Draft"
                  }
                ]
              },
              {
                "tag": "h1",
                "attrs": {
                  "invisible": "state == 'draft'"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "name",
                      "readonly": "1"
                    }
                  }
                ]
              }
            ]
          },
          {
            "tag": "group",
            "attrs": {
              "name": "main_group"
            },
            "children": [
              {
                "tag": "group",
                "attrs": {
                  "name": "group1"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "payment_type",
                      "options": "{'horizontal': True}",
                      "readonly": "state != 'draft'",
                      "widget": "radio"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "context": "{'default_is_company': True}",
                      "invisible": "partner_type != 'customer'",
                      "name": "partner_id",
                      "options": "{'no_quick_create': True}",
                      "readonly": "state != 'draft'",
                      "string": "Customer"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "context": "{'default_is_company': True}",
                      "invisible": "partner_type != 'supplier'",
                      "name": "partner_id",
                      "options": "{'no_quick_create': True}",
                      "readonly": "state != 'draft'",
                      "string": "Vendor"
                    }
                  },
                  {
                    "tag": "label",
                    "attrs": {
                      "for": "amount"
                    }
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "o_row",
                      "name": "amount_div"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "amount",
                          "readonly": "state != 'draft'"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "groups": "base.group_multi_currency",
                          "name": "currency_id",
                          "options": "{'no_create': True, 'no_open': True}",
                          "readonly": "state != 'draft'",
                          "required": "1"
                        }
                      }
                    ]
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "date",
                      "readonly": "state != 'draft'"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "memo",
                      "string": "Memo"
                    }
                  }
                ]
              },
              {
                "tag": "group",
                "attrs": {
                  "name": "group2"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "domain": "[('id', 'in', available_journal_ids)]",
                      "name": "journal_id",
                      "readonly": "state != 'draft'"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "context": "{'hide_payment_journal_id': 1}",
                      "name": "payment_method_line_id",
                      "options": "{'no_create': True, 'no_open': True}",
                      "readonly": "state != 'draft'",
                      "required": "1"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "context": "{'default_partner_id': partner_id, 'display_account_trust': True}",
                      "invisible": "not show_partner_bank_account or partner_type != 'customer' or payment_type == 'inbound'",
                      "name": "partner_bank_id",
                      "required": "require_partner_bank_account",
                      "string": "Customer Bank Account"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "context": "{'default_partner_id': partner_id, 'display_account_trust': True}",
                      "invisible": "not show_partner_bank_account or partner_type != 'supplier' or payment_type == 'inbound'",
                      "name": "partner_bank_id",
                      "required": "require_partner_bank_account",
                      "string": "Vendor Bank Account"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "context": "{'default_partner_id': partner_id, 'display_account_trust': True}",
                      "invisible": "not show_partner_bank_account or payment_type == 'outbound'",
                      "name": "partner_bank_id",
                      "required": "require_partner_bank_account",
                      "string": "Company Bank Account"
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
                      "name": "qr_code"
                    }
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "text-center",
                      "colspan": "2",
                      "invisible": "not qr_code"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "qr_code",
                          "widget": "html"
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
            "children": [
              {
                "tag": "notebook",
                "attrs": {
                  "colspan": "2",
                  "name": "payment_notebook"
                }
              }
            ]
          }
        ]
      },
      {
        "tag": "div",
        "attrs": {
          "class": "o_attachment_preview"
        }
      },
      {
        "tag": "chatter"
      }
    ]
  },
  "fields": [
    {
      "name": "state",
      "widget": "statusbar",
      "attrs": {
        "statusbar_visible": "draft,in_process,paid"
      }
    },
    {
      "name": "duplicate_payment_ids",
      "string": "Duplicated Payments",
      "widget": "x2many_buttons"
    },
    {
      "name": "id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "is_sent",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "need_cancel_request",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "is_reconciled",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "is_matched",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "payment_method_code",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "show_partner_bank_account",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "require_partner_bank_account",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "available_payment_method_line_ids",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "available_partner_bank_ids",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "country_code",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "partner_type",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "reconciled_invoices_type",
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
      "name": "paired_internal_transfer_payment_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "available_journal_ids",
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
      "name": "reconciled_invoices_count"
    },
    {
      "name": "reconciled_bills_count"
    },
    {
      "name": "reconciled_statement_lines_count"
    },
    {
      "name": "name",
      "attrs": {
        "readonly": "1"
      }
    },
    {
      "name": "payment_type",
      "widget": "radio",
      "attrs": {
        "options": "{'horizontal': True}",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "partner_id",
      "string": "Customer",
      "attrs": {
        "context": "{'default_is_company': True}",
        "invisible": "partner_type != 'customer'",
        "options": "{'no_quick_create': True}",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "partner_id",
      "string": "Vendor",
      "attrs": {
        "context": "{'default_is_company': True}",
        "invisible": "partner_type != 'supplier'",
        "options": "{'no_quick_create': True}",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "amount",
      "attrs": {
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "currency_id",
      "attrs": {
        "groups": "base.group_multi_currency",
        "options": "{'no_create': True, 'no_open': True}",
        "readonly": "state != 'draft'",
        "required": "1"
      }
    },
    {
      "name": "date",
      "attrs": {
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "memo",
      "string": "Memo"
    },
    {
      "name": "journal_id",
      "attrs": {
        "domain": "[('id', 'in', available_journal_ids)]",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "payment_method_line_id",
      "attrs": {
        "context": "{'hide_payment_journal_id': 1}",
        "options": "{'no_create': True, 'no_open': True}",
        "readonly": "state != 'draft'",
        "required": "1"
      }
    },
    {
      "name": "partner_bank_id",
      "string": "Customer Bank Account",
      "attrs": {
        "context": "{'default_partner_id': partner_id, 'display_account_trust': True}",
        "invisible": "not show_partner_bank_account or partner_type != 'customer' or payment_type == 'inbound'",
        "required": "require_partner_bank_account"
      }
    },
    {
      "name": "partner_bank_id",
      "string": "Vendor Bank Account",
      "attrs": {
        "context": "{'default_partner_id': partner_id, 'display_account_trust': True}",
        "invisible": "not show_partner_bank_account or partner_type != 'supplier' or payment_type == 'inbound'",
        "required": "require_partner_bank_account"
      }
    },
    {
      "name": "partner_bank_id",
      "string": "Company Bank Account",
      "attrs": {
        "context": "{'default_partner_id': partner_id, 'display_account_trust': True}",
        "invisible": "not show_partner_bank_account or payment_type == 'outbound'",
        "required": "require_partner_bank_account"
      }
    },
    {
      "name": "qr_code",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "qr_code",
      "widget": "html"
    }
  ],
  "buttons": [
    {
      "name": "action_post",
      "string": "Confirm",
      "type": "object",
      "class": "oe_highlight"
    },
    {
      "name": "action_validate",
      "string": "Validate",
      "type": "object",
      "class": "oe_highlight"
    },
    {
      "name": "action_reject",
      "string": "Reject",
      "type": "object"
    },
    {
      "name": "action_draft",
      "string": "Reset to Draft",
      "type": "object",
      "class": "btn btn-secondary"
    },
    {
      "name": "button_request_cancel",
      "string": "Request Cancel",
      "type": "object"
    },
    {
      "name": "mark_as_sent",
      "string": "Mark as Sent",
      "type": "object"
    },
    {
      "name": "unmark_as_sent",
      "string": "Unmark as Sent",
      "type": "object"
    },
    {
      "name": "action_cancel",
      "string": "Cancel",
      "type": "object"
    },
    {
      "name": "button_open_invoices",
      "type": "object",
      "class": "oe_stat_button"
    },
    {
      "name": "button_open_bills",
      "type": "object",
      "class": "oe_stat_button"
    },
    {
      "name": "button_open_statement_lines",
      "type": "object",
      "class": "oe_stat_button"
    },
    {
      "name": "button_open_journal_entry",
      "type": "object",
      "class": "oe_stat_button"
    }
  ]
}

export function renderAccountPaymentForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'account.payment' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/account.payment/<método> (≈ call_kw)
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
