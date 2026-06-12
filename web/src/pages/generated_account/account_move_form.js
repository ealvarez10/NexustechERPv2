// Generado por odoo2rs — vista form de account.move (view_move_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_move_form",
  "name": "account.move.form",
  "model": "account.move",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "js_class": "account_move_form",
      "string": "Account Entry"
    },
    "children": [
      {
        "tag": "header",
        "children": [
          {
            "tag": "button",
            "attrs": {
              "class": "oe_highlight",
              "context": "{'validate_analytic': True, 'disable_abnormal_invoice_detection': False}",
              "data-hotkey": "q",
              "groups": "account.group_account_invoice",
              "invisible": "hide_post_button or move_type != 'entry'",
              "name": "action_post",
              "string": "Post",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "class": "oe_highlight",
              "context": "{'validate_analytic': True, 'disable_abnormal_invoice_detection': False}",
              "data-hotkey": "q",
              "groups": "account.group_account_invoice",
              "invisible": "hide_post_button or move_type == 'entry' or display_inactive_currency_warning",
              "name": "action_post",
              "string": "Confirm",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "class": "oe_highlight",
              "data-hotkey": "y",
              "invisible": "not display_send_button or not highlight_send_button",
              "name": "action_invoice_sent",
              "string": "Send",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "data-hotkey": "y",
              "invisible": "not display_send_button or highlight_send_button",
              "name": "action_invoice_sent",
              "string": "Send",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "class": "oe_highlight",
              "invisible": "state != 'posted' or is_being_sent or invoice_pdf_report_id or move_type in ('entry', 'in_invoice','in_refund','in_receipt')",
              "name": "action_print_pdf",
              "string": "Print",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "invisible": "state != 'posted' or (not is_being_sent and not invoice_pdf_report_id) or move_type in ('in_invoice','in_refund','in_receipt')",
              "name": "action_print_pdf",
              "string": "Print",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "class": "oe_highlight",
              "context": "{'dont_redirect_to_payments': True, 'display_account_trust': True}",
              "data-hotkey": "g",
              "groups": "account.group_account_invoice",
              "id": "account_invoice_payment_btn",
              "invisible": "(                                     state != 'posted'                                     or payment_state not in ('not_paid', 'partial', 'in_payment')                                     or move_type not in ('out_invoice', 'out_refund', 'in_invoice', 'in_refund', 'out_receipt', 'in_receipt')                                     or invoice_has_outstanding                                 )",
              "name": "action_register_payment",
              "string": "Pay",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "context": "{'dont_redirect_to_payments': True, 'display_account_trust': True}",
              "data-hotkey": "g",
              "groups": "account.group_account_invoice",
              "id": "account_invoice_payment_secondary_btn",
              "invisible": "(                                     state != 'posted'                                     or payment_state not in ('not_paid', 'partial', 'in_payment')                                     or move_type not in ('out_invoice', 'out_refund', 'in_invoice', 'in_refund', 'out_receipt', 'in_receipt')                                     or not invoice_has_outstanding                                 )",
              "name": "action_register_payment",
              "string": "Pay",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "data-hotkey": "o",
              "invisible": "move_type not in ('out_invoice', 'out_refund') or state in ('draft', 'cancel')",
              "name": "preview_invoice",
              "string": "Preview",
              "title": "Preview invoice",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "data-hotkey": "z",
              "groups": "account.group_account_invoice",
              "invisible": "move_type != 'entry' or state != 'posted' or payment_state == 'reversed'",
              "name": "%(action_view_account_move_reversal)d",
              "string": "Reverse Entry",
              "type": "action"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "data-hotkey": "shift+n",
              "groups": "account.group_account_invoice",
              "invisible": "move_type not in ('out_invoice', 'in_invoice') or state != 'posted'",
              "name": "action_reverse",
              "string": "Credit Note",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "data-hotkey": "x",
              "groups": "account.group_account_invoice",
              "invisible": "not id or state != 'draft' or move_type != 'entry'",
              "name": "button_cancel",
              "string": "Cancel Entry",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "data-hotkey": "x",
              "groups": "account.group_account_invoice",
              "invisible": "not id or state != 'draft' or move_type == 'entry'",
              "name": "button_cancel",
              "string": "Cancel",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "data-hotkey": "r",
              "groups": "account.group_account_invoice",
              "invisible": "not show_reset_to_draft_button",
              "name": "button_draft",
              "string": "Reset to Draft",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "groups": "account.group_account_invoice",
              "invisible": "not restrict_mode_hash_table or inalterable_hash or state != 'posted'",
              "name": "button_hash",
              "string": "Lock",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "data-hotkey": "w",
              "groups": "account.group_account_invoice",
              "invisible": "state != 'posted' or show_reset_to_draft_button or not need_cancel_request",
              "name": "button_request_cancel",
              "string": "Request Cancel",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "class": "btn btn-info",
              "groups": "account.group_account_user",
              "invisible": "state != 'posted' or checked",
              "name": "button_set_checked",
              "string": "Reviewed",
              "type": "object"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "groups": "!account.group_account_secured",
              "name": "state",
              "statusbar_visible": "draft,posted",
              "widget": "statusbar"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "groups": "account.group_account_secured",
              "name": "state",
              "statusbar_visible": "draft,posted",
              "widget": "account_move_statusbar_secured"
            }
          }
        ]
      },
      {
        "tag": "div",
        "attrs": {
          "class": "m-0",
          "id": "alerts",
          "invisible": "not alerts"
        },
        "children": [
          {
            "tag": "field",
            "attrs": {
              "class": "o_field_html",
              "name": "alerts",
              "widget": "actionable_errors"
            }
          }
        ]
      },
      {
        "tag": "div",
        "attrs": {
          "class": "d-flex alert alert-warning w-100 d-flex align-items-center gap-1",
          "invisible": "not duplicated_ref_ids",
          "role": "alert"
        },
        "children": [
          {
            "tag": "span",
            "text": "This document might be a duplicate of"
          },
          {
            "tag": "field",
            "attrs": {
              "context": "{'name_as_amount_total': True}",
              "name": "duplicated_ref_ids",
              "nb_records_shown": "1",
              "string": "Duplicated Documents",
              "widget": "x2many_buttons"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "class": "btn btn-link text-danger ms-auto d-flex align-items-center gap-1",
              "invisible": "not is_draft_duplicated_ref_ids",
              "name": "action_delete_duplicates",
              "type": "object"
            },
            "children": [
              {
                "tag": "i",
                "attrs": {
                  "class": "fa fa-trash text-danger"
                }
              },
              {
                "tag": "span",
                "attrs": {
                  "class": "text-danger",
                  "invisible": "duplicated_ref_ids.length == 1"
                },
                "text": "Delete all duplicates"
              },
              {
                "tag": "span",
                "attrs": {
                  "class": "text-danger",
                  "invisible": "duplicated_ref_ids.length > 1"
                },
                "text": "Delete duplicate"
              }
            ]
          }
        ]
      },
      {
        "tag": "div",
        "attrs": {
          "class": "alert alert-info",
          "groups": "account.group_account_invoice,account.group_account_readonly",
          "invisible": "state != 'posted' or move_type not in ('out_invoice', 'out_receipt') or not invoice_has_outstanding or payment_state not in ('not_paid', 'partial')",
          "role": "alert"
        },
        "children": [
          {
            "tag": "bold",
            "children": [
              {
                "tag": "a",
                "attrs": {
                  "class": "alert-link",
                  "href": "#outstanding",
                  "role": "button"
                },
                "text": "outstanding credits"
              }
            ]
          }
        ],
        "text": "You have  listed below for this customer."
      },
      {
        "tag": "div",
        "attrs": {
          "class": "alert alert-info",
          "groups": "account.group_account_invoice,account.group_account_readonly",
          "invisible": "state != 'posted' or move_type not in ('in_invoice', 'in_receipt') or not invoice_has_outstanding or payment_state not in ('not_paid', 'partial')",
          "role": "alert"
        },
        "children": [
          {
            "tag": "bold",
            "children": [
              {
                "tag": "a",
                "attrs": {
                  "class": "alert-link",
                  "href": "#outstanding",
                  "role": "button"
                },
                "text": "outstanding debits"
              }
            ]
          }
        ],
        "text": "You have  listed below for this vendor."
      },
      {
        "tag": "div",
        "attrs": {
          "class": "alert alert-info",
          "groups": "account.group_account_invoice,account.group_account_readonly",
          "invisible": "state != 'posted' or move_type != 'out_refund' or not invoice_has_outstanding or payment_state not in ('not_paid', 'partial')",
          "role": "alert"
        },
        "children": [
          {
            "tag": "bold",
            "children": [
              {
                "tag": "a",
                "attrs": {
                  "class": "alert-link",
                  "href": "#outstanding",
                  "role": "button"
                },
                "text": "outstanding debits"
              }
            ]
          }
        ],
        "text": "You have  listed below for this customer."
      },
      {
        "tag": "div",
        "attrs": {
          "class": "alert alert-info",
          "groups": "account.group_account_invoice,account.group_account_readonly",
          "invisible": "state != 'posted' or move_type != 'in_refund' or not invoice_has_outstanding or payment_state not in ('not_paid', 'partial')",
          "role": "alert"
        },
        "children": [
          {
            "tag": "bold",
            "children": [
              {
                "tag": "a",
                "attrs": {
                  "class": "alert-link",
                  "href": "#outstanding",
                  "role": "button"
                },
                "text": "outstanding credits"
              }
            ]
          }
        ],
        "text": "You have  listed below for this vendor."
      },
      {
        "tag": "div",
        "attrs": {
          "class": "alert alert-warning",
          "invisible": "not display_inactive_currency_warning or move_type not in ('in_invoice', 'in_refund', 'in_receipt')",
          "role": "alert"
        },
        "children": [
          {
            "tag": "button",
            "attrs": {
              "class": "oe_link",
              "name": "action_activate_currency",
              "style": "padding: 0; vertical-align: baseline;",
              "type": "object"
            },
            "text": "activate the currency of the bill"
          }
        ],
        "text": "In order to validate this bill, you must . The journal entries need to be computed by Odoo before being posted in your company's currency."
      },
      {
        "tag": "div",
        "attrs": {
          "class": "alert alert-warning",
          "invisible": "not display_inactive_currency_warning or move_type not in ('out_invoice', 'out_refund', 'out_receipt')",
          "role": "alert"
        },
        "children": [
          {
            "tag": "button",
            "attrs": {
              "class": "oe_link",
              "name": "action_activate_currency",
              "style": "padding: 0; vertical-align: baseline;",
              "type": "object"
            },
            "text": "activate the currency of the invoice"
          }
        ],
        "text": "In order to validate this invoice, you must . The journal entries need to be computed by Odoo before being posted in your company's currency."
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
                  "icon": "fa-bars",
                  "invisible": "move_type != 'entry' or not id or not origin_payment_id",
                  "name": "action_open_business_doc",
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
                        "text": "1 Payment"
                      }
                    ]
                  }
                ]
              },
              {
                "tag": "button",
                "attrs": {
                  "class": "oe_stat_button",
                  "groups": "account.group_account_invoice,account.group_account_readonly",
                  "icon": "fa-bars",
                  "invisible": "not payment_count",
                  "name": "open_payments",
                  "type": "object"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "payment_count",
                      "string": "Payments",
                      "widget": "statinfo"
                    }
                  }
                ]
              },
              {
                "tag": "button",
                "attrs": {
                  "class": "oe_stat_button",
                  "icon": "fa-bars",
                  "invisible": "move_type != 'entry' or not id or not has_reconciled_entries",
                  "name": "open_reconcile_view",
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
                        "text": "Reconciled Items"
                      }
                    ]
                  }
                ]
              },
              {
                "tag": "button",
                "attrs": {
                  "class": "oe_stat_button",
                  "icon": "fa-usd",
                  "invisible": "not tax_cash_basis_created_move_ids",
                  "name": "open_created_caba_entries",
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
                        "text": "Cash Basis Entries"
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
                  "invisible": "not adjusting_entries_move_ids",
                  "name": "open_adjusting_entries",
                  "type": "object"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "adjusting_entries_count",
                      "string": "Adjusting Entries",
                      "widget": "statinfo"
                    }
                  }
                ]
              },
              {
                "tag": "button",
                "attrs": {
                  "class": "oe_stat_button",
                  "icon": "fa-bars",
                  "invisible": "not adjusting_entry_origin_move_ids",
                  "name": "open_adjusting_entry_origin_moves",
                  "type": "object"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "o_stat_info",
                      "invisible": "adjusting_entry_origin_moves_count != 1"
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
                              "name": "adjusting_entry_origin_label"
                            }
                          }
                        ]
                      }
                    ]
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "adjusting_entry_origin_moves_count == 1",
                      "name": "adjusting_entry_origin_moves_count",
                      "string": "Invoices",
                      "widget": "statinfo"
                    }
                  }
                ]
              }
            ]
          },
          {
            "tag": "widget",
            "attrs": {
              "bg_color": "gray_ribbon",
              "invisible": "not is_move_sent or payment_state != 'not_paid' or move_type not in ('out_invoice', 'out_refund', 'out_receipt')",
              "name": "web_ribbon",
              "title": "Sent"
            }
          },
          {
            "tag": "widget",
            "attrs": {
              "bg_color": "text-bg-secondary",
              "invisible": "status_in_payment != 'sent' or move_type not in ('out_invoice', 'out_refund', 'in_invoice', 'in_refund', 'out_receipt', 'in_receipt')",
              "name": "web_ribbon",
              "title": "Sent"
            }
          },
          {
            "tag": "widget",
            "attrs": {
              "invisible": "payment_state != 'paid' or move_type not in ('out_invoice', 'out_refund', 'in_invoice', 'in_refund', 'out_receipt', 'in_receipt')",
              "name": "web_ribbon",
              "title": "Paid"
            }
          },
          {
            "tag": "widget",
            "attrs": {
              "invisible": "payment_state != 'in_payment' or move_type not in ('out_invoice', 'out_refund', 'in_invoice', 'in_refund', 'out_receipt', 'in_receipt')",
              "name": "web_ribbon",
              "title": "In Payment"
            }
          },
          {
            "tag": "widget",
            "attrs": {
              "bg_color": "text-bg-secondary",
              "invisible": "payment_state != 'partial' or move_type not in ('out_invoice', 'out_refund', 'in_invoice', 'in_refund', 'out_receipt', 'in_receipt')",
              "name": "web_ribbon",
              "title": "Partial"
            }
          },
          {
            "tag": "widget",
            "attrs": {
              "invisible": "payment_state != 'reversed'",
              "name": "web_ribbon",
              "title": "Reversed"
            }
          },
          {
            "tag": "widget",
            "attrs": {
              "bg_color": "text-bg-danger",
              "invisible": "payment_state != 'blocked' or move_type not in ('out_invoice', 'out_refund', 'in_invoice', 'in_refund', 'out_receipt', 'in_receipt')",
              "name": "web_ribbon",
              "title": "Blocked"
            }
          },
          {
            "tag": "widget",
            "attrs": {
              "bg_color": "text-bg-info",
              "invisible": "payment_state != 'invoicing_legacy'",
              "name": "web_ribbon",
              "text": "Invoicing App Legacy",
              "tooltip": "This entry has been generated through the Invoicing app, before installing Accounting. It has been disabled by the 'Invoicing Switch Threshold Date' setting so that it does not impact your accounting."
            }
          },
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
              "name": "state"
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
              "name": "journal_id"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "show_name_warning"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "posted_before"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "force_save": "1",
              "invisible": "1",
              "name": "move_type"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "force_save": "1",
              "invisible": "1",
              "name": "payment_state"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "invoice_filter_type_domain"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "suitable_journal_ids"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "currency_id",
              "readonly": "state in ['cancel', 'posted']"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "company_currency_id"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "commercial_partner_id"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "bank_partner_id"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "display_qr_code"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "show_reset_to_draft_button"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "expected_currency_rate"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "invoice_has_outstanding"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "is_move_sent"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "invoice_pdf_report_id"
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
              "name": "has_reconciled_entries"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "restrict_mode_hash_table"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "inalterable_hash"
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
              "name": "display_inactive_currency_warning"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "statement_line_id"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "statement_id"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "origin_payment_id"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "tax_country_id"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "tax_calculation_rounding_method"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "tax_cash_basis_created_move_ids"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "quick_edit_mode"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "hide_post_button"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "quick_encoding_vals"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "show_delivery_date"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "is_being_sent"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "show_update_fpos"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "is_sale_installed"
            }
          },
          {
            "tag": "div",
            "attrs": {
              "class": "oe_title"
            },
            "children": [
              {
                "tag": "span",
                "attrs": {
                  "class": "o_form_label"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "move_type == 'entry'",
                      "name": "move_type",
                      "nolabel": "1",
                      "options": "{'horizontal': true}",
                      "readonly": "state != 'draft'",
                      "widget": "receipt_selector"
                    }
                  }
                ]
              },
              {
                "tag": "div",
                "attrs": {
                  "class": "text-warning",
                  "invisible": "not show_name_warning"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "oe_inline",
                      "name": "highest_name"
                    }
                  }
                ],
                "text": "The current highest number is . You might want to put a higher number here."
              },
              {
                "tag": "h1",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "not (name or name_placeholder or quick_edit_mode)",
                      "name": "name",
                      "options": "{'placeholder_field': 'name_placeholder'}",
                      "readonly": "state != 'draft'"
                    }
                  },
                  {
                    "tag": "span",
                    "attrs": {
                      "invisible": "name or name_placeholder or quick_edit_mode"
                    },
                    "text": "Draft"
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
                  "id": "header_left_group"
                },
                "children": [
                  {
                    "tag": "label",
                    "attrs": {
                      "for": "partner_id",
                      "invisible": "move_type not in ('out_invoice', 'out_refund', 'out_receipt')",
                      "string": "Customer",
                      "style": "font-weight:bold;"
                    }
                  },
                  {
                    "tag": "label",
                    "attrs": {
                      "for": "partner_id",
                      "invisible": "move_type not in ('in_invoice', 'in_refund', 'in_receipt')",
                      "string": "Vendor",
                      "style": "font-weight:bold;"
                    }
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "o_col",
                      "invisible": "move_type not in ('out_invoice', 'out_refund', 'out_receipt', 'in_invoice', 'in_refund', 'in_receipt')"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "context": "{                                             'res_partner_search_mode': (context.get('default_move_type', 'entry') in ('out_invoice', 'out_refund', 'out_receipt') and 'customer') or (context.get('default_move_type', 'entry') in ('in_invoice', 'in_refund', 'in_receipt') and 'supplier') or False,                                             'show_address': 1, 'default_is_company': True, 'show_vat': True}",
                          "default_focus": "1",
                          "invisible": "move_type not in ('out_invoice', 'out_refund', 'in_invoice', 'in_refund', 'out_receipt', 'in_receipt')",
                          "name": "partner_id",
                          "nolabel": "1",
                          "options": "{\"no_quick_create\": True}",
                          "placeholder": "Search a name or Tax ID...",
                          "readonly": "state != 'draft'",
                          "widget": "res_partner_many2one"
                        }
                      },
                      {
                        "tag": "button",
                        "attrs": {
                          "class": "btn-link mb-1 px-0",
                          "help": "Recompute all taxes and accounts based on this fiscal position",
                          "icon": "fa-refresh",
                          "invisible": "not show_update_fpos or state in ['cancel', 'posted']",
                          "name": "action_update_fpos_values",
                          "string": "Update Taxes and Accounts",
                          "type": "object"
                        }
                      }
                    ]
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "groups": "account.group_delivery_invoice_address",
                      "invisible": "move_type not in ('out_invoice', 'out_refund', 'out_receipt')",
                      "name": "partner_shipping_id",
                      "readonly": "state != 'draft'"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "w-50",
                      "invisible": "move_type == 'entry' or not quick_edit_mode",
                      "name": "quick_edit_total_amount",
                      "readonly": "state != 'draft'"
                    }
                  },
                  {
                    "tag": "label",
                    "attrs": {
                      "for": "ref",
                      "invisible": "move_type not in ('in_invoice', 'in_receipt', 'in_refund')",
                      "string": "Bill Reference"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "default_focus": "1",
                      "invisible": "move_type not in ('in_invoice', 'in_receipt', 'in_refund')",
                      "name": "ref",
                      "nolabel": "1"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "default_focus": "1",
                      "invisible": "move_type != 'entry'",
                      "name": "ref"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "not tax_cash_basis_origin_move_id",
                      "name": "tax_cash_basis_origin_move_id"
                    }
                  },
                  {
                    "tag": "label",
                    "attrs": {
                      "class": "oe_edit_only",
                      "for": "invoice_vendor_bill_id",
                      "invisible": "state != 'draft' or move_type not in ('in_invoice', 'in_refund')",
                      "name": "invoice_vendor_bill_id_label",
                      "string": "Auto-Complete"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "oe_edit_only",
                      "context": "{'show_total_amount': True}",
                      "domain": "[('company_id', '=', company_id), ('partner_id', 'child_of', [partner_id]), ('move_type', '=', move_type)]",
                      "invisible": "state != 'draft' or move_type not in ('in_invoice', 'in_refund')",
                      "name": "invoice_vendor_bill_id",
                      "nolabel": "1",
                      "options": "{'no_create': True}",
                      "placeholder": "Select an old purchase document"
                    }
                  }
                ]
              },
              {
                "tag": "group",
                "attrs": {
                  "id": "header_right_group"
                },
                "children": [
                  {
                    "tag": "label",
                    "attrs": {
                      "for": "invoice_date",
                      "invisible": "move_type not in ('out_invoice', 'out_refund', 'out_receipt')",
                      "string": "Invoice Date",
                      "style": "font-weight:bold;"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "move_type not in ('out_invoice', 'out_refund', 'out_receipt')",
                      "name": "invoice_date",
                      "nolabel": "1",
                      "options": "{'warn_future': true}",
                      "placeholder": "Today",
                      "readonly": "state != 'draft'"
                    }
                  },
                  {
                    "tag": "label",
                    "attrs": {
                      "for": "invoice_date",
                      "invisible": "move_type not in ('in_invoice', 'in_refund', 'in_receipt')",
                      "string": "Bill Date",
                      "style": "font-weight:bold;"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "move_type not in ('in_invoice', 'in_refund', 'in_receipt')",
                      "name": "invoice_date",
                      "nolabel": "1",
                      "options": "{'warn_future': true}",
                      "readonly": "state != 'draft'",
                      "required": "move_type in ('in_invoice', 'in_refund', 'in_receipt')"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "move_type in ('out_invoice', 'out_refund', 'out_receipt') and not quick_edit_mode and not (state == 'posted' and date != invoice_date)",
                      "name": "date",
                      "readonly": "state != 'draft'",
                      "string": "Accounting Date"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "move_type not in ('in_invoice', 'in_refund', 'in_receipt')",
                      "name": "payment_reference",
                      "placeholder": "Use Bill Reference",
                      "readonly": "inalterable_hash != False"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "context": "{'default_partner_id': bank_partner_id, 'display_account_trust': True}",
                      "domain": "[('partner_id', '=', bank_partner_id)]",
                      "invisible": "move_type not in ('in_invoice', 'in_refund', 'in_receipt')",
                      "name": "partner_bank_id",
                      "readonly": "is_move_sent and state != 'draft'"
                    }
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "o_td_label",
                      "invisible": "move_type not in ('out_invoice', 'out_refund', 'in_invoice', 'in_refund', 'out_receipt', 'in_receipt')"
                    },
                    "children": [
                      {
                        "tag": "label",
                        "attrs": {
                          "for": "invoice_date_due",
                          "invisible": "invoice_payment_term_id",
                          "string": "Due Date"
                        }
                      },
                      {
                        "tag": "label",
                        "attrs": {
                          "for": "invoice_payment_term_id",
                          "invisible": "not invoice_payment_term_id",
                          "string": "Payment terms"
                        }
                      }
                    ]
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "d-flex",
                      "invisible": "move_type not in ('out_invoice', 'out_refund', 'in_invoice', 'in_refund', 'out_receipt', 'in_receipt')",
                      "name": "due_date"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "force_save": "1",
                          "invisible": "invoice_payment_term_id",
                          "name": "invoice_date_due",
                          "placeholder": "Date"
                        }
                      },
                      {
                        "tag": "span",
                        "attrs": {
                          "class": "o_form_label mx-3 oe_edit_only text-center",
                          "invisible": "state != 'draft' or invoice_payment_term_id",
                          "style": "width: 6ch;"
                        },
                        "text": "or"
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "context": "{'example_date': invoice_date, 'example_amount': tax_totals['total_amount_currency']}",
                          "name": "invoice_payment_term_id",
                          "options": "{'no_quick_create':True}",
                          "placeholder": "Payment Terms",
                          "readonly": "state in ['cancel', 'posted']"
                        }
                      }
                    ]
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "1",
                      "name": "taxable_supply_date_placeholder"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "not show_taxable_supply_date",
                      "name": "taxable_supply_date",
                      "options": "{'placeholder_field': 'taxable_supply_date_placeholder'}",
                      "readonly": "state != 'draft'"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "not show_delivery_date",
                      "name": "delivery_date",
                      "readonly": "state != 'draft'"
                    }
                  },
                  {
                    "tag": "label",
                    "attrs": {
                      "for": "journal_id",
                      "invisible": "not show_journal"
                    }
                  },
                  {
                    "tag": "label",
                    "attrs": {
                      "for": "currency_id",
                      "groups": "base.group_multi_currency",
                      "invisible": "show_journal"
                    }
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "d-flex",
                      "name": "journal_div"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "invisible": "not show_journal",
                          "name": "journal_id",
                          "options": "{'no_create': True, 'no_open': True}",
                          "readonly": "posted_before and name not in (False, '', '/')"
                        }
                      },
                      {
                        "tag": "div",
                        "attrs": {
                          "class": "d-flex flex-column mx-3 text-center",
                          "groups": "base.group_multi_currency",
                          "invisible": "move_type == 'entry' or not show_journal",
                          "name": "in_and_refresh_button_div",
                          "style": "width: 6ch;"
                        },
                        "children": [
                          {
                            "tag": "div",
                            "text": "in"
                          },
                          {
                            "tag": "div",
                            "attrs": {
                              "class": "d-flex flex-column justify-content-center flex-grow-1",
                              "invisible": "state != 'draft' or invoice_currency_rate == expected_currency_rate"
                            },
                            "children": [
                              {
                                "tag": "button",
                                "attrs": {
                                  "class": "btn btn-link p-0",
                                  "icon": "fa-refresh",
                                  "name": "refresh_invoice_currency_rate",
                                  "title": "Reset the currency rate to the default accordingly to the invoice date",
                                  "type": "object"
                                }
                              }
                            ]
                          }
                        ]
                      },
                      {
                        "tag": "div",
                        "attrs": {
                          "class": "w-100",
                          "groups": "base.group_multi_currency",
                          "invisible": "move_type == 'entry'",
                          "name": "currency_div"
                        },
                        "children": [
                          {
                            "tag": "div",
                            "attrs": {
                              "class": "d-flex gap-1"
                            },
                            "children": [
                              {
                                "tag": "field",
                                "attrs": {
                                  "class": "oe_inline",
                                  "name": "currency_id",
                                  "options": "{'no_create': True}",
                                  "readonly": "state != 'draft'"
                                }
                              },
                              {
                                "tag": "widget",
                                "attrs": {
                                  "invisible": "state != 'draft' or currency_id == company_currency_id",
                                  "name": "account_pick_currency_date"
                                }
                              }
                            ]
                          },
                          {
                            "tag": "div",
                            "attrs": {
                              "class": "d-flex gap-1 text-muted",
                              "invisible": "currency_id == company_currency_id",
                              "name": "currency_conversion_div"
                            },
                            "children": [
                              {
                                "tag": "span",
                                "text": "1"
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "class": "w-auto",
                                  "name": "company_currency_id",
                                  "options": "{'no_open': True}",
                                  "readonly": "True"
                                }
                              },
                              {
                                "tag": "span",
                                "text": "="
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "digits": "[12,6]",
                                  "name": "invoice_currency_rate",
                                  "readonly": "state != 'draft'",
                                  "style": "max-width: 21ch;"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "class": "w-auto",
                                  "name": "currency_id",
                                  "options": "{'no_open': True}",
                                  "readonly": "True"
                                }
                              },
                              {
                                "tag": "div",
                                "attrs": {
                                  "invisible": "show_journal or state != 'draft' or invoice_currency_rate == expected_currency_rate"
                                },
                                "children": [
                                  {
                                    "tag": "button",
                                    "attrs": {
                                      "class": "btn btn-link p-0",
                                      "icon": "fa-refresh",
                                      "name": "refresh_invoice_currency_rate",
                                      "title": "Reset the currency rate to the default accordingly to the invoice date",
                                      "type": "object"
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
            "tag": "notebook",
            "children": [
              {
                "tag": "page",
                "attrs": {
                  "id": "invoice_tab",
                  "invisible": "move_type == 'entry'",
                  "name": "invoice_tab",
                  "string": "Invoice Lines"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "aggregated_fields": "price_subtotal,price_total",
                      "context": "{                                            'default_move_type': context.get('default_move_type'),                                            'journal_id': journal_id,                                            'default_partner_id': commercial_partner_id,                                            'default_currency_id': currency_id or company_currency_id,                                            'default_display_type': 'product',                                            'quick_encoding_vals': quick_encoding_vals,                                        }",
                      "mode": "list,kanban",
                      "name": "invoice_line_ids",
                      "options": "{'hide_composition': True, 'hide_prices': True, 'subsections': True}",
                      "readonly": "state != 'draft'",
                      "widget": "product_label_section_and_note_field_o2m"
                    },
                    "children": [
                      {
                        "tag": "list",
                        "attrs": {
                          "default_order": "sequence, id",
                          "editable": "bottom",
                          "name": "journal_items",
                          "string": "Journal Items"
                        },
                        "children": [
                          {
                            "tag": "control",
                            "children": [
                              {
                                "tag": "create",
                                "attrs": {
                                  "name": "add_line_control",
                                  "string": "Add a line"
                                }
                              },
                              {
                                "tag": "create",
                                "attrs": {
                                  "context": "{'default_display_type': 'line_section'}",
                                  "name": "add_section_control",
                                  "string": "Add a section"
                                }
                              },
                              {
                                "tag": "create",
                                "attrs": {
                                  "context": "{'default_display_type': 'line_note'}",
                                  "name": "add_note_control",
                                  "string": "Add a note"
                                }
                              },
                              {
                                "tag": "button",
                                "attrs": {
                                  "class": "btn-link",
                                  "context": "{'order_id': parent.id}",
                                  "name": "action_add_from_catalog",
                                  "string": "Catalog",
                                  "type": "object"
                                }
                              }
                            ]
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
                              "domain": "                                                     context.get('default_move_type') in ('out_invoice', 'out_refund', 'out_receipt')                                                     and [('sale_ok', '=', True)]                                                     or [('purchase_ok', '=', True)]                                                ",
                              "name": "product_id",
                              "optional": "conditional",
                              "widget": "product_label_section_and_note_field"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "name",
                              "optional": "show",
                              "widget": "section_and_note_text"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "context": "{                                                     'partner_id': partner_id,                                                     'move_type': parent.move_type,                                                     'search_default_type_asset_fixed': 'in_' in parent.move_type,                                                     'search_default_type_expense': 'in_' in parent.move_type,                                                     'search_default_account_type': 'out_' in parent.move_type and 'income',                                                     'preferred_account_type': 'out_' in parent.move_type and 'income' or 'in_' in parent.move_type and 'expense',                                                }",
                              "domain": "[('company_ids', 'parent_of', company_id), ('account_type', 'not in', ('asset_receivable', 'liability_payable', 'off_balance'))]",
                              "groups": "account.group_account_readonly",
                              "name": "account_id",
                              "options": "{'no_quick_create': True}",
                              "required": "display_type not in ('line_section', 'line_subsection', 'line_note')"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "business_domain_compute": "parent.move_type in ['out_invoice', 'out_refund', 'out_receipt'] and 'invoice' or parent.move_type in ['in_invoice', 'in_refund', 'in_receipt'] and 'bill' or 'general'",
                              "groups": "analytic.group_analytic_accounting",
                              "name": "analytic_distribution",
                              "optional": "show",
                              "options": "{'product_field': 'product_id', 'account_field': 'account_id', 'amount_field': 'price_subtotal'}",
                              "string": "Analytic",
                              "widget": "analytic_distribution"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "quantity",
                              "optional": "conditional"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "uom.group_uom",
                              "name": "product_uom_id",
                              "optional": "conditional",
                              "options": "{'no_create': True, 'quantity_field': 'quantity'}",
                              "widget": "many2one_uom",
                              "width": "92px"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "True",
                              "name": "product_uom_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "price_unit",
                              "string": "Price"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "discount",
                              "optional": "hide",
                              "string": "Disc.%",
                              "width": "50px"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "context": "{                                                     'append_fields': not parent.invoice_filter_type_domain and ['type_tax_use'],                                                     'active_test': True,                                                     'dynamic_fiscal_position_id': parent.fiscal_position_id,                                                }",
                              "domain": "[('type_tax_use', '=?', parent.invoice_filter_type_domain), ('company_id', 'parent_of', parent.company_id), ('country_id', '=', parent.tax_country_id)]",
                              "name": "tax_ids",
                              "optional": "show",
                              "options": "{'no_create': True}",
                              "widget": "many2many_tax_tags"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "parent.move_type not in ('in_invoice', 'in_refund', 'in_receipt')",
                              "groups": "!account.group_partial_purchase_deductibility",
                              "name": "deductible_amount",
                              "optional": "hide",
                              "string": "Professional %"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "parent.move_type not in ('in_invoice', 'in_refund', 'in_receipt')",
                              "groups": "account.group_partial_purchase_deductibility",
                              "name": "deductible_amount",
                              "optional": "show",
                              "string": "Professional %"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "parent.move_type not in ['in_invoice', 'in_refund', 'in_receipt'] and parent.company_price_include == 'tax_included'",
                              "name": "price_subtotal",
                              "string": "Amount"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "parent.move_type in ['in_invoice', 'in_refund', 'in_receipt'] or parent.company_price_include == 'tax_excluded'",
                              "name": "price_total",
                              "string": "Amount"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "True",
                              "name": "partner_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "True",
                              "name": "currency_id"
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
                              "column_invisible": "True",
                              "name": "company_currency_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "True",
                              "force_save": "1",
                              "name": "display_type"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "True",
                              "name": "collapse_prices"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "True",
                              "name": "collapse_composition"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "kanban",
                        "attrs": {
                          "class": "o_kanban_mobile"
                        },
                        "children": [
                          {
                            "tag": "templates",
                            "children": [
                              {
                                "tag": "t",
                                "attrs": {
                                  "t-name": "card"
                                },
                                "children": [
                                  {
                                    "tag": "div",
                                    "attrs": {
                                      "t-attf-class": "ps-0 pe-0 {{ record.display_type.raw_value ? 'o_is_' + record.display_type.raw_value : '' }}"
                                    },
                                    "children": [
                                      {
                                        "tag": "t",
                                        "attrs": {
                                          "t-if": "!['line_note', 'line_section', 'line_subsection'].includes(record.display_type.raw_value)"
                                        },
                                        "children": [
                                          {
                                            "tag": "div",
                                            "attrs": {
                                              "class": "row g-0"
                                            },
                                            "children": [
                                              {
                                                "tag": "div",
                                                "attrs": {
                                                  "class": "col-2 pe-3"
                                                },
                                                "children": [
                                                  {
                                                    "tag": "field",
                                                    "attrs": {
                                                      "class": "w-100",
                                                      "name": "product_id",
                                                      "options": "{'preview_image': 'image_128'}",
                                                      "widget": "image"
                                                    }
                                                  }
                                                ]
                                              },
                                              {
                                                "tag": "div",
                                                "attrs": {
                                                  "class": "col-10"
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
                                                          "class": "col"
                                                        },
                                                        "children": [
                                                          {
                                                            "tag": "field",
                                                            "attrs": {
                                                              "class": "fw-bold",
                                                              "name": "product_id"
                                                            }
                                                          }
                                                        ]
                                                      },
                                                      {
                                                        "tag": "div",
                                                        "attrs": {
                                                          "class": "col-auto fw-bold float-end text-end"
                                                        },
                                                        "children": [
                                                          {
                                                            "tag": "field",
                                                            "attrs": {
                                                              "name": "price_subtotal",
                                                              "string": "Amount",
                                                              "t-if": "['in_invoice', 'in_refund', 'in_receipt'].includes(record.move_type.raw_value)"
                                                            }
                                                          },
                                                          {
                                                            "tag": "field",
                                                            "attrs": {
                                                              "name": "price_total",
                                                              "string": "Amount",
                                                              "t-if": "!['in_invoice', 'in_refund', 'in_receipt'].includes(record.move_type.raw_value)"
                                                            }
                                                          }
                                                        ]
                                                      }
                                                    ]
                                                  },
                                                  {
                                                    "tag": "div",
                                                    "attrs": {
                                                      "class": "text-muted"
                                                    },
                                                    "children": [
                                                      {
                                                        "tag": "field",
                                                        "attrs": {
                                                          "name": "quantity"
                                                        }
                                                      },
                                                      {
                                                        "tag": "field",
                                                        "attrs": {
                                                          "groups": "uom.group_uom",
                                                          "name": "product_uom_id",
                                                          "widget": "many2one_uom"
                                                        }
                                                      }
                                                    ],
                                                    "text": "Quantity:"
                                                  },
                                                  {
                                                    "tag": "div",
                                                    "attrs": {
                                                      "class": "text-muted"
                                                    },
                                                    "children": [
                                                      {
                                                        "tag": "field",
                                                        "attrs": {
                                                          "name": "price_unit"
                                                        }
                                                      }
                                                    ],
                                                    "text": "Unit Price:"
                                                  }
                                                ]
                                              }
                                            ]
                                          }
                                        ]
                                      },
                                      {
                                        "tag": "t",
                                        "attrs": {
                                          "t-if": "['line_section', 'line_subsection', 'line_note'].includes(record.display_type.raw_value)"
                                        },
                                        "children": [
                                          {
                                            "tag": "div",
                                            "attrs": {
                                              "t-att-class": "{                                                             'fw-bolder': record.display_type.raw_value === 'line_section',                                                             'fw-bold': record.display_type.raw_value === 'line_subsection',                                                             'fst-italic': record.display_type.raw_value === 'line_note',                                                         }"
                                            },
                                            "children": [
                                              {
                                                "tag": "field",
                                                "attrs": {
                                                  "name": "name"
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
                            "tag": "field",
                            "attrs": {
                              "name": "tax_calculation_rounding_method"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "currency_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "company_currency_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "force_save": "1",
                              "name": "display_type"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "move_type"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "form",
                        "children": [
                          {
                            "tag": "sheet",
                            "children": [
                              {
                                "tag": "field",
                                "attrs": {
                                  "invisible": "1",
                                  "name": "tax_calculation_rounding_method"
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
                                  "invisible": "1",
                                  "name": "company_id"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "invisible": "1",
                                  "name": "partner_id"
                                }
                              },
                              {
                                "tag": "group",
                                "children": [
                                  {
                                    "tag": "field",
                                    "attrs": {
                                      "name": "product_id",
                                      "widget": "many2one_barcode"
                                    }
                                  },
                                  {
                                    "tag": "label",
                                    "attrs": {
                                      "for": "name",
                                      "invisible": "display_type in ('line_section', 'line_subsection', 'line_note')",
                                      "string": "Description"
                                    }
                                  },
                                  {
                                    "tag": "label",
                                    "attrs": {
                                      "for": "name",
                                      "invisible": "display_type != 'line_section'",
                                      "string": "Section"
                                    }
                                  },
                                  {
                                    "tag": "label",
                                    "attrs": {
                                      "for": "name",
                                      "invisible": "display_type != 'line_subsection'",
                                      "string": "Subsection"
                                    }
                                  },
                                  {
                                    "tag": "label",
                                    "attrs": {
                                      "for": "name",
                                      "invisible": "display_type != 'line_note'",
                                      "string": "Note"
                                    }
                                  },
                                  {
                                    "tag": "field",
                                    "attrs": {
                                      "name": "name",
                                      "nolabel": "1",
                                      "widget": "text"
                                    }
                                  },
                                  {
                                    "tag": "field",
                                    "attrs": {
                                      "name": "quantity"
                                    }
                                  },
                                  {
                                    "tag": "field",
                                    "attrs": {
                                      "groups": "uom.group_uom",
                                      "name": "product_uom_id",
                                      "widget": "many2one_uom"
                                    }
                                  },
                                  {
                                    "tag": "field",
                                    "attrs": {
                                      "name": "price_unit"
                                    }
                                  },
                                  {
                                    "tag": "field",
                                    "attrs": {
                                      "name": "discount",
                                      "string": "Disc.%"
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
                                      "context": "{'partner_id': partner_id, 'move_type': parent.move_type}",
                                      "domain": "[('company_ids', 'parent_of', company_id)]",
                                      "name": "account_id",
                                      "options": "{'no_create': True}"
                                    }
                                  },
                                  {
                                    "tag": "field",
                                    "attrs": {
                                      "context": "{                                                         'append_fields': not parent.invoice_filter_type_domain and ['type_tax_use'],                                                         'active_test': True,                                                         'dynamic_fiscal_position_id': parent.fiscal_position_id,                                                     }",
                                      "domain": "[('type_tax_use', '=?', parent.invoice_filter_type_domain), ('company_id', 'parent_of', parent.company_id), ('country_id', '=', parent.tax_country_id)]",
                                      "name": "tax_ids",
                                      "options": "{'no_create': True}",
                                      "widget": "many2many_tax_tags"
                                    }
                                  },
                                  {
                                    "tag": "field",
                                    "attrs": {
                                      "groups": "analytic.group_analytic_accounting",
                                      "name": "analytic_distribution",
                                      "widget": "analytic_distribution"
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
                                      "invisible": "parent.move_type not in ['in_invoice', 'in_refund', 'in_receipt'] and parent.company_price_include == 'tax_included'",
                                      "name": "price_subtotal",
                                      "string": "Amount"
                                    }
                                  },
                                  {
                                    "tag": "field",
                                    "attrs": {
                                      "invisible": "parent.move_type in ['in_invoice', 'in_refund', 'in_receipt'] or parent.company_price_include == 'tax_excluded'",
                                      "name": "price_total",
                                      "string": "Amount"
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
                  {
                    "tag": "group",
                    "attrs": {
                      "class": "oe_invoice_lines_tab overflow-hidden",
                      "col": "12"
                    },
                    "children": [
                      {
                        "tag": "group",
                        "attrs": {
                          "colspan": "8"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "narration",
                              "nolabel": "1",
                              "placeholder": "Terms and Conditions"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "colspan": "4"
                        },
                        "children": [
                          {
                            "tag": "group",
                            "attrs": {
                              "class": "oe_subtotal_footer",
                              "invisible": "move_type not in ('out_invoice', 'out_refund', 'in_invoice', 'in_refund', 'out_receipt', 'in_receipt') or payment_state == 'invoicing_legacy'"
                            },
                            "children": [
                              {
                                "tag": "field",
                                "attrs": {
                                  "colspan": "2",
                                  "name": "tax_totals",
                                  "nolabel": "1",
                                  "readonly": "state != 'draft' or (move_type not in ('in_invoice', 'in_refund', 'in_receipt') and not quick_edit_mode)",
                                  "widget": "account-tax-totals-field"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "colspan": "2",
                                  "invisible": "not invoice_payments_widget",
                                  "name": "invoice_payments_widget",
                                  "nolabel": "1",
                                  "widget": "payment"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "class": "oe_subtotal_footer_separator",
                                  "name": "amount_residual"
                                }
                              }
                            ]
                          },
                          {
                            "tag": "group",
                            "attrs": {
                              "class": "oe_subtotal_footer px-4",
                              "groups": "account.group_account_invoice,account.group_account_readonly",
                              "invisible": "state != 'posted' or not invoice_has_outstanding"
                            },
                            "children": [
                              {
                                "tag": "field",
                                "attrs": {
                                  "class": "oe_invoice_outstanding_credits_debits py-3",
                                  "colspan": "2",
                                  "name": "invoice_outstanding_credits_debits_widget",
                                  "nolabel": "1",
                                  "widget": "payment"
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
              {
                "tag": "page",
                "attrs": {
                  "groups": "account.group_account_readonly",
                  "id": "aml_tab",
                  "name": "aml_tab",
                  "string": "Journal Items"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "context": "{                                            'default_move_type': context.get('default_move_type'),                                            'line_ids': line_ids,                                            'journal_id': journal_id,                                            'default_partner_id': commercial_partner_id,                                            'default_currency_id': currency_id or company_currency_id,                                            'kanban_view_ref': 'account.account_move_line_view_kanban_mobile',                                        }",
                      "invisible": "payment_state == 'invoicing_legacy' and move_type != 'entry'",
                      "mode": "list,kanban",
                      "name": "line_ids",
                      "readonly": "state != 'draft'"
                    },
                    "children": [
                      {
                        "tag": "list",
                        "attrs": {
                          "decoration-muted": "display_type in ('line_section', 'line_note')",
                          "default_order": "sequence, id",
                          "editable": "bottom",
                          "string": "Journal Items"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "domain": "[('company_ids', 'parent_of', company_id)]",
                              "invisible": "display_type in ('line_section', 'line_note')",
                              "name": "account_id",
                              "required": "display_type not in ('line_section', 'line_note')"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "parent.move_type != 'entry'",
                              "domain": "['|', ('parent_id', '=', False), ('is_company', '=', True)]",
                              "name": "partner_id",
                              "optional": "show"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "name",
                              "optional": "show",
                              "widget": "section_and_note_text"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "business_domain_compute": "parent.move_type in ['out_invoice', 'out_refund', 'out_receipt'] and 'invoice' or parent.move_type in ['in_invoice', 'in_refund', 'in_receipt'] and 'bill' or 'general'",
                              "groups": "analytic.group_analytic_accounting",
                              "name": "analytic_distribution",
                              "optional": "show",
                              "options": "{'account_field': 'account_id'}",
                              "widget": "analytic_distribution"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "context.get('view_no_maturity')",
                              "invisible": "display_type in ('line_section', 'line_note')",
                              "name": "date_maturity",
                              "optional": "hide"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "base.group_multi_currency",
                              "name": "amount_currency",
                              "optional": "hide"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "parent.move_type != 'entry'",
                              "groups": "base.group_multi_currency",
                              "name": "currency_id",
                              "optional": "hide",
                              "options": "{'no_create': True}"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "context": "{                                                     'append_fields': not parent.invoice_filter_type_domain and ['type_tax_use'],                                                     'active_test': True                                                }",
                              "domain": "[('type_tax_use', '=?', parent.invoice_filter_type_domain), ('company_id', 'parent_of', parent.company_id)]",
                              "force_save": "1",
                              "name": "tax_ids",
                              "optional": "hide",
                              "options": "{'no_create': True}",
                              "readonly": "display_type in ('line_section', 'line_note') or tax_line_id or (parent.move_type in ('out_invoice', 'out_refund', 'in_invoice', 'in_refund', 'out_receipt', 'in_receipt') and account_type in ('asset_receivable', 'liability_payable'))",
                              "widget": "autosave_many2many_tax_tags"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "display_type in ('line_section', 'line_subsection', 'line_note')",
                              "name": "debit",
                              "readonly": "parent.move_type in ('out_invoice', 'out_refund', 'in_invoice', 'in_refund', 'out_receipt', 'in_receipt') and display_type in ('line_section', 'line_note', 'product')",
                              "sum": "Total Debit"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "display_type in ('line_section', 'line_subsection', 'line_note')",
                              "name": "credit",
                              "readonly": "parent.move_type in ('out_invoice', 'out_refund', 'in_invoice', 'in_refund', 'out_receipt', 'in_receipt') and display_type in ('line_section', 'line_note', 'product')",
                              "sum": "Total Credit"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "True",
                              "name": "balance"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "discount_date",
                              "optional": "hide",
                              "string": "Discount Date"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "discount_amount_currency",
                              "optional": "hide",
                              "string": "Discount Amount"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "domain": "[                                                     ('applicability', '=', 'taxes'),                                                     '|', ('country_id', '=', parent.tax_country_id),                                                     ('country_id', '=', False),                                                 ]",
                              "name": "tax_tag_ids",
                              "optional": "show",
                              "options": "{'no_create': True}",
                              "string": "Tax Grids",
                              "widget": "many2many_tags"
                            }
                          },
                          {
                            "tag": "button",
                            "attrs": {
                              "aria-label": "Change Period",
                              "class": "float-end",
                              "column_invisible": "parent.move_type == 'entry' or parent.state != 'posted'",
                              "context": "{'default_action': 'change_period'}",
                              "icon": "fa-calendar",
                              "invisible": "account_internal_group not in ('income', 'expense')",
                              "name": "action_automatic_entry",
                              "string": "Cut-Off",
                              "type": "object"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "True",
                              "name": "tax_line_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "True",
                              "name": "company_currency_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "True",
                              "force_save": "1",
                              "name": "display_type"
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
                              "column_invisible": "True",
                              "name": "sequence"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "True",
                              "name": "id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "True",
                              "name": "account_internal_group"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "True",
                              "name": "account_type"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "form",
                        "children": [
                          {
                            "tag": "group",
                            "children": [
                              {
                                "tag": "field",
                                "attrs": {
                                  "domain": "[('company_ids', 'parent_of', company_id)]",
                                  "name": "account_id"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "domain": "['|', ('parent_id', '=', False), ('is_company', '=', True)]",
                                  "name": "partner_id"
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
                                  "groups": "analytic.group_analytic_accounting",
                                  "name": "analytic_distribution",
                                  "widget": "analytic_distribution"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "groups": "base.group_multi_currency",
                                  "name": "amount_currency"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "invisible": "1",
                                  "name": "company_currency_id"
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
                                  "groups": "base.group_multi_currency",
                                  "name": "currency_id",
                                  "options": "{'no_create': True}"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "name": "debit",
                                  "sum": "Total Debit"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "name": "credit",
                                  "sum": "Total Credit"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "invisible": "1",
                                  "name": "balance"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "context": "{                                                     'append_fields': not parent.invoice_filter_type_domain and ['type_tax_use'],                                                     'active_test': True,                                                }",
                                  "domain": "[('type_tax_use', '=?', parent.invoice_filter_type_domain), ('company_id', 'parent_of', parent.company_id)]",
                                  "name": "tax_ids",
                                  "options": "{'no_create': True}",
                                  "string": "Taxes Applied",
                                  "widget": "many2many_tax_tags"
                                }
                              },
                              {
                                "tag": "field",
                                "attrs": {
                                  "invisible": "context.get('view_no_maturity', False)",
                                  "name": "date_maturity",
                                  "required": "0"
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
                      "class": "alert alert-info text-center mb-0",
                      "invisible": "payment_state != 'invoicing_legacy' or move_type == 'entry'",
                      "role": "alert"
                    },
                    "children": [
                      {
                        "tag": "span",
                        "text": "This entry has been generated through the Invoicing app, before installing Accounting. Its balance has been imported separately."
                      }
                    ]
                  }
                ]
              },
              {
                "tag": "page",
                "attrs": {
                  "id": "other_tab",
                  "invisible": "move_type == 'entry'",
                  "name": "other_info",
                  "string": "Other Info"
                },
                "children": [
                  {
                    "tag": "group",
                    "attrs": {
                      "id": "other_tab_group"
                    },
                    "children": [
                      {
                        "tag": "group",
                        "attrs": {
                          "invisible": "move_type not in ('out_invoice', 'out_refund')",
                          "name": "sale_info_group",
                          "string": "Invoice"
                        },
                        "children": [
                          {
                            "tag": "label",
                            "attrs": {
                              "for": "ref",
                              "string": "Customer Reference"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "ref",
                              "nolabel": "1"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "force_save": "1",
                              "invisible": "1",
                              "name": "user_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "domain": "[('share', '=', False)]",
                              "name": "invoice_user_id",
                              "widget": "many2one_avatar_user"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "force_save": "1",
                              "invisible": "1",
                              "name": "invoice_origin",
                              "string": "Source Document"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "context": "{'default_partner_id': bank_partner_id, 'display_account_trust': True}",
                              "domain": "[('partner_id.ref_company_ids', 'parent_of', company_id)]",
                              "name": "partner_bank_id",
                              "readonly": "is_move_sent and state != 'draft'"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "move_type not in ('out_invoice', 'out_refund')",
                              "name": "payment_reference",
                              "placeholder": "Standard communication",
                              "readonly": "inalterable_hash"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "not display_qr_code",
                              "name": "qr_code_method"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "delivery_date",
                              "readonly": "state != 'draft'"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "name": "accounting_info_group",
                          "string": "Accounting"
                        },
                        "children": [
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
                              "invisible": "1",
                              "name": "invoice_incoterm_placeholder"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "move_type in ('out_receipt', 'in_receipt')",
                              "name": "invoice_incoterm_id",
                              "options": "{'placeholder_field': 'invoice_incoterm_placeholder'}"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "move_type in ('out_receipt', 'in_receipt')",
                              "name": "incoterm_location"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "fiscal_position_id",
                              "readonly": "state in ['cancel', 'posted']"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "account.group_account_secured,base.group_no_one",
                              "name": "secured"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "domain": "[('payment_type', '=', 'inbound'), ('company_id', '=', company_id)]",
                              "invisible": "move_type in ('in_invoice', 'in_refund', 'in_receipt')",
                              "name": "preferred_payment_method_line_id",
                              "string": "Payment Method"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "domain": "[('payment_type', '=', 'outbound'), ('company_id', '=', company_id)]",
                              "invisible": "move_type in ('out_invoice', 'out_refund', 'out_receipt')",
                              "name": "preferred_payment_method_line_id",
                              "string": "Payment Method"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "account.group_cash_rounding",
                              "name": "invoice_cash_rounding_id",
                              "readonly": "state != 'draft'"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "move_type not in ('in_invoice', 'in_refund') or not invoice_source_email",
                              "name": "invoice_source_email",
                              "widget": "email"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "move_type in ('out_receipt', 'in_receipt')",
                              "name": "auto_post",
                              "readonly": "state != 'draft'"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "auto_post in ('no', 'at_date')",
                              "name": "auto_post_until",
                              "readonly": "state != 'draft'"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "checked"
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
                  "id": "other_tab_entry",
                  "invisible": "move_type != 'entry'",
                  "name": "other_info",
                  "string": "Other Info"
                },
                "children": [
                  {
                    "tag": "group",
                    "attrs": {
                      "id": "other_tab_entry_group"
                    },
                    "children": [
                      {
                        "tag": "group",
                        "attrs": {
                          "name": "misc_group"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "move_type != 'entry'",
                              "name": "auto_post",
                              "readonly": "state != 'draft'"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "move_type != 'entry' or not reversed_entry_id",
                              "name": "reversed_entry_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "auto_post in ('no', 'at_date')",
                              "name": "auto_post_until",
                              "readonly": "state != 'draft'"
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
                              "name": "fiscal_position_id",
                              "readonly": "state in ['cancel', 'posted']"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "groups": "base.group_multi_company",
                              "name": "company_id",
                              "required": "1"
                            }
                          }
                        ]
                      }
                    ]
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "height": "50",
                      "name": "narration",
                      "nolabel": "1",
                      "placeholder": "Add an internal note..."
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
          "class": "o_attachment_preview",
          "invisible": "move_type not in ('out_invoice', 'out_refund', 'in_invoice', 'in_refund') or state != 'draft'"
        }
      },
      {
        "tag": "chatter",
        "attrs": {
          "reload_on_attachment": "True"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "state",
      "widget": "statusbar",
      "attrs": {
        "groups": "!account.group_account_secured",
        "statusbar_visible": "draft,posted"
      }
    },
    {
      "name": "state",
      "widget": "account_move_statusbar_secured",
      "attrs": {
        "groups": "account.group_account_secured",
        "statusbar_visible": "draft,posted"
      }
    },
    {
      "name": "alerts",
      "widget": "actionable_errors",
      "attrs": {
        "class": "o_field_html"
      }
    },
    {
      "name": "duplicated_ref_ids",
      "string": "Duplicated Documents",
      "widget": "x2many_buttons",
      "attrs": {
        "context": "{'name_as_amount_total': True}",
        "nb_records_shown": "1"
      }
    },
    {
      "name": "payment_count",
      "string": "Payments",
      "widget": "statinfo"
    },
    {
      "name": "adjusting_entries_count",
      "string": "Adjusting Entries",
      "widget": "statinfo"
    },
    {
      "name": "adjusting_entry_origin_label"
    },
    {
      "name": "adjusting_entry_origin_moves_count",
      "string": "Invoices",
      "widget": "statinfo",
      "attrs": {
        "invisible": "adjusting_entry_origin_moves_count == 1"
      }
    },
    {
      "name": "id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "state",
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
      "name": "journal_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "show_name_warning",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "posted_before",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "move_type",
      "attrs": {
        "force_save": "1",
        "invisible": "1"
      }
    },
    {
      "name": "payment_state",
      "attrs": {
        "force_save": "1",
        "invisible": "1"
      }
    },
    {
      "name": "invoice_filter_type_domain",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "suitable_journal_ids",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "currency_id",
      "attrs": {
        "invisible": "1",
        "readonly": "state in ['cancel', 'posted']"
      }
    },
    {
      "name": "company_currency_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "commercial_partner_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "bank_partner_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "display_qr_code",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "show_reset_to_draft_button",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "expected_currency_rate",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "invoice_has_outstanding",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "is_move_sent",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "invoice_pdf_report_id",
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
      "name": "has_reconciled_entries",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "restrict_mode_hash_table",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "inalterable_hash",
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
      "name": "display_inactive_currency_warning",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "statement_line_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "statement_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "origin_payment_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "tax_country_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "tax_calculation_rounding_method",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "tax_cash_basis_created_move_ids",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "quick_edit_mode",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "hide_post_button",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "quick_encoding_vals",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "show_delivery_date",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "is_being_sent",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "show_update_fpos",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "is_sale_installed",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "move_type",
      "widget": "receipt_selector",
      "attrs": {
        "invisible": "move_type == 'entry'",
        "nolabel": "1",
        "options": "{'horizontal': true}",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "highest_name",
      "attrs": {
        "class": "oe_inline"
      }
    },
    {
      "name": "name",
      "attrs": {
        "invisible": "not (name or name_placeholder or quick_edit_mode)",
        "options": "{'placeholder_field': 'name_placeholder'}",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "partner_id",
      "widget": "res_partner_many2one",
      "attrs": {
        "context": "{                                             'res_partner_search_mode': (context.get('default_move_type', 'entry') in ('out_invoice', 'out_refund', 'out_receipt') and 'customer') or (context.get('default_move_type', 'entry') in ('in_invoice', 'in_refund', 'in_receipt') and 'supplier') or False,                                             'show_address': 1, 'default_is_company': True, 'show_vat': True}",
        "default_focus": "1",
        "invisible": "move_type not in ('out_invoice', 'out_refund', 'in_invoice', 'in_refund', 'out_receipt', 'in_receipt')",
        "nolabel": "1",
        "options": "{\"no_quick_create\": True}",
        "placeholder": "Search a name or Tax ID...",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "partner_shipping_id",
      "attrs": {
        "groups": "account.group_delivery_invoice_address",
        "invisible": "move_type not in ('out_invoice', 'out_refund', 'out_receipt')",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "quick_edit_total_amount",
      "attrs": {
        "class": "w-50",
        "invisible": "move_type == 'entry' or not quick_edit_mode",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "ref",
      "attrs": {
        "default_focus": "1",
        "invisible": "move_type not in ('in_invoice', 'in_receipt', 'in_refund')",
        "nolabel": "1"
      }
    },
    {
      "name": "ref",
      "attrs": {
        "default_focus": "1",
        "invisible": "move_type != 'entry'"
      }
    },
    {
      "name": "tax_cash_basis_origin_move_id",
      "attrs": {
        "invisible": "not tax_cash_basis_origin_move_id"
      }
    },
    {
      "name": "invoice_vendor_bill_id",
      "attrs": {
        "class": "oe_edit_only",
        "context": "{'show_total_amount': True}",
        "domain": "[('company_id', '=', company_id), ('partner_id', 'child_of', [partner_id]), ('move_type', '=', move_type)]",
        "invisible": "state != 'draft' or move_type not in ('in_invoice', 'in_refund')",
        "nolabel": "1",
        "options": "{'no_create': True}",
        "placeholder": "Select an old purchase document"
      }
    },
    {
      "name": "invoice_date",
      "attrs": {
        "invisible": "move_type not in ('out_invoice', 'out_refund', 'out_receipt')",
        "nolabel": "1",
        "options": "{'warn_future': true}",
        "placeholder": "Today",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "invoice_date",
      "attrs": {
        "invisible": "move_type not in ('in_invoice', 'in_refund', 'in_receipt')",
        "nolabel": "1",
        "options": "{'warn_future': true}",
        "readonly": "state != 'draft'",
        "required": "move_type in ('in_invoice', 'in_refund', 'in_receipt')"
      }
    },
    {
      "name": "date",
      "string": "Accounting Date",
      "attrs": {
        "invisible": "move_type in ('out_invoice', 'out_refund', 'out_receipt') and not quick_edit_mode and not (state == 'posted' and date != invoice_date)",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "payment_reference",
      "attrs": {
        "invisible": "move_type not in ('in_invoice', 'in_refund', 'in_receipt')",
        "placeholder": "Use Bill Reference",
        "readonly": "inalterable_hash != False"
      }
    },
    {
      "name": "partner_bank_id",
      "attrs": {
        "context": "{'default_partner_id': bank_partner_id, 'display_account_trust': True}",
        "domain": "[('partner_id', '=', bank_partner_id)]",
        "invisible": "move_type not in ('in_invoice', 'in_refund', 'in_receipt')",
        "readonly": "is_move_sent and state != 'draft'"
      }
    },
    {
      "name": "invoice_date_due",
      "attrs": {
        "force_save": "1",
        "invisible": "invoice_payment_term_id",
        "placeholder": "Date"
      }
    },
    {
      "name": "invoice_payment_term_id",
      "attrs": {
        "context": "{'example_date': invoice_date, 'example_amount': tax_totals['total_amount_currency']}",
        "options": "{'no_quick_create':True}",
        "placeholder": "Payment Terms",
        "readonly": "state in ['cancel', 'posted']"
      }
    },
    {
      "name": "taxable_supply_date_placeholder",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "taxable_supply_date",
      "attrs": {
        "invisible": "not show_taxable_supply_date",
        "options": "{'placeholder_field': 'taxable_supply_date_placeholder'}",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "delivery_date",
      "attrs": {
        "invisible": "not show_delivery_date",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "journal_id",
      "attrs": {
        "invisible": "not show_journal",
        "options": "{'no_create': True, 'no_open': True}",
        "readonly": "posted_before and name not in (False, '', '/')"
      }
    },
    {
      "name": "currency_id",
      "attrs": {
        "class": "oe_inline",
        "options": "{'no_create': True}",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "company_currency_id",
      "attrs": {
        "class": "w-auto",
        "options": "{'no_open': True}",
        "readonly": "True"
      }
    },
    {
      "name": "invoice_currency_rate",
      "attrs": {
        "digits": "[12,6]",
        "readonly": "state != 'draft'",
        "style": "max-width: 21ch;"
      }
    },
    {
      "name": "currency_id",
      "attrs": {
        "class": "w-auto",
        "options": "{'no_open': True}",
        "readonly": "True"
      }
    },
    {
      "name": "invoice_line_ids",
      "widget": "product_label_section_and_note_field_o2m",
      "attrs": {
        "aggregated_fields": "price_subtotal,price_total",
        "context": "{                                            'default_move_type': context.get('default_move_type'),                                            'journal_id': journal_id,                                            'default_partner_id': commercial_partner_id,                                            'default_currency_id': currency_id or company_currency_id,                                            'default_display_type': 'product',                                            'quick_encoding_vals': quick_encoding_vals,                                        }",
        "mode": "list,kanban",
        "options": "{'hide_composition': True, 'hide_prices': True, 'subsections': True}",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "sequence",
      "widget": "handle"
    },
    {
      "name": "product_id",
      "widget": "product_label_section_and_note_field",
      "attrs": {
        "domain": "                                                     context.get('default_move_type') in ('out_invoice', 'out_refund', 'out_receipt')                                                     and [('sale_ok', '=', True)]                                                     or [('purchase_ok', '=', True)]                                                ",
        "optional": "conditional"
      }
    },
    {
      "name": "name",
      "widget": "section_and_note_text",
      "attrs": {
        "optional": "show"
      }
    },
    {
      "name": "account_id",
      "attrs": {
        "context": "{                                                     'partner_id': partner_id,                                                     'move_type': parent.move_type,                                                     'search_default_type_asset_fixed': 'in_' in parent.move_type,                                                     'search_default_type_expense': 'in_' in parent.move_type,                                                     'search_default_account_type': 'out_' in parent.move_type and 'income',                                                     'preferred_account_type': 'out_' in parent.move_type and 'income' or 'in_' in parent.move_type and 'expense',                                                }",
        "domain": "[('company_ids', 'parent_of', company_id), ('account_type', 'not in', ('asset_receivable', 'liability_payable', 'off_balance'))]",
        "groups": "account.group_account_readonly",
        "options": "{'no_quick_create': True}",
        "required": "display_type not in ('line_section', 'line_subsection', 'line_note')"
      }
    },
    {
      "name": "analytic_distribution",
      "string": "Analytic",
      "widget": "analytic_distribution",
      "attrs": {
        "business_domain_compute": "parent.move_type in ['out_invoice', 'out_refund', 'out_receipt'] and 'invoice' or parent.move_type in ['in_invoice', 'in_refund', 'in_receipt'] and 'bill' or 'general'",
        "groups": "analytic.group_analytic_accounting",
        "optional": "show",
        "options": "{'product_field': 'product_id', 'account_field': 'account_id', 'amount_field': 'price_subtotal'}"
      }
    },
    {
      "name": "quantity",
      "attrs": {
        "optional": "conditional"
      }
    },
    {
      "name": "product_uom_id",
      "widget": "many2one_uom",
      "attrs": {
        "groups": "uom.group_uom",
        "optional": "conditional",
        "options": "{'no_create': True, 'quantity_field': 'quantity'}",
        "width": "92px"
      }
    },
    {
      "name": "product_uom_id",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "price_unit",
      "string": "Price"
    },
    {
      "name": "discount",
      "string": "Disc.%",
      "attrs": {
        "optional": "hide",
        "width": "50px"
      }
    },
    {
      "name": "tax_ids",
      "widget": "many2many_tax_tags",
      "attrs": {
        "context": "{                                                     'append_fields': not parent.invoice_filter_type_domain and ['type_tax_use'],                                                     'active_test': True,                                                     'dynamic_fiscal_position_id': parent.fiscal_position_id,                                                }",
        "domain": "[('type_tax_use', '=?', parent.invoice_filter_type_domain), ('company_id', 'parent_of', parent.company_id), ('country_id', '=', parent.tax_country_id)]",
        "optional": "show",
        "options": "{'no_create': True}"
      }
    },
    {
      "name": "deductible_amount",
      "string": "Professional %",
      "attrs": {
        "column_invisible": "parent.move_type not in ('in_invoice', 'in_refund', 'in_receipt')",
        "groups": "!account.group_partial_purchase_deductibility",
        "optional": "hide"
      }
    },
    {
      "name": "deductible_amount",
      "string": "Professional %",
      "attrs": {
        "column_invisible": "parent.move_type not in ('in_invoice', 'in_refund', 'in_receipt')",
        "groups": "account.group_partial_purchase_deductibility",
        "optional": "show"
      }
    },
    {
      "name": "price_subtotal",
      "string": "Amount",
      "attrs": {
        "column_invisible": "parent.move_type not in ['in_invoice', 'in_refund', 'in_receipt'] and parent.company_price_include == 'tax_included'"
      }
    },
    {
      "name": "price_total",
      "string": "Amount",
      "attrs": {
        "column_invisible": "parent.move_type in ['in_invoice', 'in_refund', 'in_receipt'] or parent.company_price_include == 'tax_excluded'"
      }
    },
    {
      "name": "partner_id",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "currency_id",
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
      "name": "company_currency_id",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "display_type",
      "attrs": {
        "column_invisible": "True",
        "force_save": "1"
      }
    },
    {
      "name": "collapse_prices",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "collapse_composition",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "product_id",
      "widget": "image",
      "attrs": {
        "class": "w-100",
        "options": "{'preview_image': 'image_128'}"
      }
    },
    {
      "name": "product_id",
      "attrs": {
        "class": "fw-bold"
      }
    },
    {
      "name": "price_subtotal",
      "string": "Amount",
      "attrs": {
        "t-if": "['in_invoice', 'in_refund', 'in_receipt'].includes(record.move_type.raw_value)"
      }
    },
    {
      "name": "price_total",
      "string": "Amount",
      "attrs": {
        "t-if": "!['in_invoice', 'in_refund', 'in_receipt'].includes(record.move_type.raw_value)"
      }
    },
    {
      "name": "quantity"
    },
    {
      "name": "product_uom_id",
      "widget": "many2one_uom",
      "attrs": {
        "groups": "uom.group_uom"
      }
    },
    {
      "name": "price_unit"
    },
    {
      "name": "name"
    },
    {
      "name": "tax_calculation_rounding_method"
    },
    {
      "name": "currency_id"
    },
    {
      "name": "company_currency_id"
    },
    {
      "name": "display_type",
      "attrs": {
        "force_save": "1"
      }
    },
    {
      "name": "move_type"
    },
    {
      "name": "tax_calculation_rounding_method",
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
      "name": "company_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "partner_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "product_id",
      "widget": "many2one_barcode"
    },
    {
      "name": "name",
      "widget": "text",
      "attrs": {
        "nolabel": "1"
      }
    },
    {
      "name": "quantity"
    },
    {
      "name": "product_uom_id",
      "widget": "many2one_uom",
      "attrs": {
        "groups": "uom.group_uom"
      }
    },
    {
      "name": "price_unit"
    },
    {
      "name": "discount",
      "string": "Disc.%"
    },
    {
      "name": "account_id",
      "attrs": {
        "context": "{'partner_id': partner_id, 'move_type': parent.move_type}",
        "domain": "[('company_ids', 'parent_of', company_id)]",
        "options": "{'no_create': True}"
      }
    },
    {
      "name": "tax_ids",
      "widget": "many2many_tax_tags",
      "attrs": {
        "context": "{                                                         'append_fields': not parent.invoice_filter_type_domain and ['type_tax_use'],                                                         'active_test': True,                                                         'dynamic_fiscal_position_id': parent.fiscal_position_id,                                                     }",
        "domain": "[('type_tax_use', '=?', parent.invoice_filter_type_domain), ('company_id', 'parent_of', parent.company_id), ('country_id', '=', parent.tax_country_id)]",
        "options": "{'no_create': True}"
      }
    },
    {
      "name": "analytic_distribution",
      "widget": "analytic_distribution",
      "attrs": {
        "groups": "analytic.group_analytic_accounting"
      }
    },
    {
      "name": "price_subtotal",
      "string": "Amount",
      "attrs": {
        "invisible": "parent.move_type not in ['in_invoice', 'in_refund', 'in_receipt'] and parent.company_price_include == 'tax_included'"
      }
    },
    {
      "name": "price_total",
      "string": "Amount",
      "attrs": {
        "invisible": "parent.move_type in ['in_invoice', 'in_refund', 'in_receipt'] or parent.company_price_include == 'tax_excluded'"
      }
    },
    {
      "name": "narration",
      "attrs": {
        "nolabel": "1",
        "placeholder": "Terms and Conditions"
      }
    },
    {
      "name": "tax_totals",
      "widget": "account-tax-totals-field",
      "attrs": {
        "colspan": "2",
        "nolabel": "1",
        "readonly": "state != 'draft' or (move_type not in ('in_invoice', 'in_refund', 'in_receipt') and not quick_edit_mode)"
      }
    },
    {
      "name": "invoice_payments_widget",
      "widget": "payment",
      "attrs": {
        "colspan": "2",
        "invisible": "not invoice_payments_widget",
        "nolabel": "1"
      }
    },
    {
      "name": "amount_residual",
      "attrs": {
        "class": "oe_subtotal_footer_separator"
      }
    },
    {
      "name": "invoice_outstanding_credits_debits_widget",
      "widget": "payment",
      "attrs": {
        "class": "oe_invoice_outstanding_credits_debits py-3",
        "colspan": "2",
        "nolabel": "1"
      }
    },
    {
      "name": "line_ids",
      "attrs": {
        "context": "{                                            'default_move_type': context.get('default_move_type'),                                            'line_ids': line_ids,                                            'journal_id': journal_id,                                            'default_partner_id': commercial_partner_id,                                            'default_currency_id': currency_id or company_currency_id,                                            'kanban_view_ref': 'account.account_move_line_view_kanban_mobile',                                        }",
        "invisible": "payment_state == 'invoicing_legacy' and move_type != 'entry'",
        "mode": "list,kanban",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "account_id",
      "attrs": {
        "domain": "[('company_ids', 'parent_of', company_id)]",
        "invisible": "display_type in ('line_section', 'line_note')",
        "required": "display_type not in ('line_section', 'line_note')"
      }
    },
    {
      "name": "partner_id",
      "attrs": {
        "column_invisible": "parent.move_type != 'entry'",
        "domain": "['|', ('parent_id', '=', False), ('is_company', '=', True)]",
        "optional": "show"
      }
    },
    {
      "name": "name",
      "widget": "section_and_note_text",
      "attrs": {
        "optional": "show"
      }
    },
    {
      "name": "analytic_distribution",
      "widget": "analytic_distribution",
      "attrs": {
        "business_domain_compute": "parent.move_type in ['out_invoice', 'out_refund', 'out_receipt'] and 'invoice' or parent.move_type in ['in_invoice', 'in_refund', 'in_receipt'] and 'bill' or 'general'",
        "groups": "analytic.group_analytic_accounting",
        "optional": "show",
        "options": "{'account_field': 'account_id'}"
      }
    },
    {
      "name": "date_maturity",
      "attrs": {
        "column_invisible": "context.get('view_no_maturity')",
        "invisible": "display_type in ('line_section', 'line_note')",
        "optional": "hide"
      }
    },
    {
      "name": "amount_currency",
      "attrs": {
        "groups": "base.group_multi_currency",
        "optional": "hide"
      }
    },
    {
      "name": "currency_id",
      "attrs": {
        "column_invisible": "parent.move_type != 'entry'",
        "groups": "base.group_multi_currency",
        "optional": "hide",
        "options": "{'no_create': True}"
      }
    },
    {
      "name": "tax_ids",
      "widget": "autosave_many2many_tax_tags",
      "attrs": {
        "context": "{                                                     'append_fields': not parent.invoice_filter_type_domain and ['type_tax_use'],                                                     'active_test': True                                                }",
        "domain": "[('type_tax_use', '=?', parent.invoice_filter_type_domain), ('company_id', 'parent_of', parent.company_id)]",
        "force_save": "1",
        "optional": "hide",
        "options": "{'no_create': True}",
        "readonly": "display_type in ('line_section', 'line_note') or tax_line_id or (parent.move_type in ('out_invoice', 'out_refund', 'in_invoice', 'in_refund', 'out_receipt', 'in_receipt') and account_type in ('asset_receivable', 'liability_payable'))"
      }
    },
    {
      "name": "debit",
      "attrs": {
        "invisible": "display_type in ('line_section', 'line_subsection', 'line_note')",
        "readonly": "parent.move_type in ('out_invoice', 'out_refund', 'in_invoice', 'in_refund', 'out_receipt', 'in_receipt') and display_type in ('line_section', 'line_note', 'product')",
        "sum": "Total Debit"
      }
    },
    {
      "name": "credit",
      "attrs": {
        "invisible": "display_type in ('line_section', 'line_subsection', 'line_note')",
        "readonly": "parent.move_type in ('out_invoice', 'out_refund', 'in_invoice', 'in_refund', 'out_receipt', 'in_receipt') and display_type in ('line_section', 'line_note', 'product')",
        "sum": "Total Credit"
      }
    },
    {
      "name": "balance",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "discount_date",
      "string": "Discount Date",
      "attrs": {
        "optional": "hide"
      }
    },
    {
      "name": "discount_amount_currency",
      "string": "Discount Amount",
      "attrs": {
        "optional": "hide"
      }
    },
    {
      "name": "tax_tag_ids",
      "string": "Tax Grids",
      "widget": "many2many_tags",
      "attrs": {
        "domain": "[                                                     ('applicability', '=', 'taxes'),                                                     '|', ('country_id', '=', parent.tax_country_id),                                                     ('country_id', '=', False),                                                 ]",
        "optional": "show",
        "options": "{'no_create': True}"
      }
    },
    {
      "name": "tax_line_id",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "company_currency_id",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "display_type",
      "attrs": {
        "column_invisible": "True",
        "force_save": "1"
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
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "id",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "account_internal_group",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "account_type",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "account_id",
      "attrs": {
        "domain": "[('company_ids', 'parent_of', company_id)]"
      }
    },
    {
      "name": "partner_id",
      "attrs": {
        "domain": "['|', ('parent_id', '=', False), ('is_company', '=', True)]"
      }
    },
    {
      "name": "name"
    },
    {
      "name": "analytic_distribution",
      "widget": "analytic_distribution",
      "attrs": {
        "groups": "analytic.group_analytic_accounting"
      }
    },
    {
      "name": "amount_currency",
      "attrs": {
        "groups": "base.group_multi_currency"
      }
    },
    {
      "name": "company_currency_id",
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
      "name": "currency_id",
      "attrs": {
        "groups": "base.group_multi_currency",
        "options": "{'no_create': True}"
      }
    },
    {
      "name": "debit",
      "attrs": {
        "sum": "Total Debit"
      }
    },
    {
      "name": "credit",
      "attrs": {
        "sum": "Total Credit"
      }
    },
    {
      "name": "balance",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "tax_ids",
      "string": "Taxes Applied",
      "widget": "many2many_tax_tags",
      "attrs": {
        "context": "{                                                     'append_fields': not parent.invoice_filter_type_domain and ['type_tax_use'],                                                     'active_test': True,                                                }",
        "domain": "[('type_tax_use', '=?', parent.invoice_filter_type_domain), ('company_id', 'parent_of', parent.company_id)]",
        "options": "{'no_create': True}"
      }
    },
    {
      "name": "date_maturity",
      "attrs": {
        "invisible": "context.get('view_no_maturity', False)",
        "required": "0"
      }
    },
    {
      "name": "ref",
      "attrs": {
        "nolabel": "1"
      }
    },
    {
      "name": "user_id",
      "attrs": {
        "force_save": "1",
        "invisible": "1"
      }
    },
    {
      "name": "invoice_user_id",
      "widget": "many2one_avatar_user",
      "attrs": {
        "domain": "[('share', '=', False)]"
      }
    },
    {
      "name": "invoice_origin",
      "string": "Source Document",
      "attrs": {
        "force_save": "1",
        "invisible": "1"
      }
    },
    {
      "name": "partner_bank_id",
      "attrs": {
        "context": "{'default_partner_id': bank_partner_id, 'display_account_trust': True}",
        "domain": "[('partner_id.ref_company_ids', 'parent_of', company_id)]",
        "readonly": "is_move_sent and state != 'draft'"
      }
    },
    {
      "name": "payment_reference",
      "attrs": {
        "invisible": "move_type not in ('out_invoice', 'out_refund')",
        "placeholder": "Standard communication",
        "readonly": "inalterable_hash"
      }
    },
    {
      "name": "qr_code_method",
      "attrs": {
        "invisible": "not display_qr_code"
      }
    },
    {
      "name": "delivery_date",
      "attrs": {
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "groups": "base.group_multi_company"
      }
    },
    {
      "name": "invoice_incoterm_placeholder",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "invoice_incoterm_id",
      "attrs": {
        "invisible": "move_type in ('out_receipt', 'in_receipt')",
        "options": "{'placeholder_field': 'invoice_incoterm_placeholder'}"
      }
    },
    {
      "name": "incoterm_location",
      "attrs": {
        "invisible": "move_type in ('out_receipt', 'in_receipt')"
      }
    },
    {
      "name": "fiscal_position_id",
      "attrs": {
        "readonly": "state in ['cancel', 'posted']"
      }
    },
    {
      "name": "secured",
      "attrs": {
        "groups": "account.group_account_secured,base.group_no_one"
      }
    },
    {
      "name": "preferred_payment_method_line_id",
      "string": "Payment Method",
      "attrs": {
        "domain": "[('payment_type', '=', 'inbound'), ('company_id', '=', company_id)]",
        "invisible": "move_type in ('in_invoice', 'in_refund', 'in_receipt')"
      }
    },
    {
      "name": "preferred_payment_method_line_id",
      "string": "Payment Method",
      "attrs": {
        "domain": "[('payment_type', '=', 'outbound'), ('company_id', '=', company_id)]",
        "invisible": "move_type in ('out_invoice', 'out_refund', 'out_receipt')"
      }
    },
    {
      "name": "invoice_cash_rounding_id",
      "attrs": {
        "groups": "account.group_cash_rounding",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "invoice_source_email",
      "widget": "email",
      "attrs": {
        "invisible": "move_type not in ('in_invoice', 'in_refund') or not invoice_source_email"
      }
    },
    {
      "name": "auto_post",
      "attrs": {
        "invisible": "move_type in ('out_receipt', 'in_receipt')",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "auto_post_until",
      "attrs": {
        "invisible": "auto_post in ('no', 'at_date')",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "checked"
    },
    {
      "name": "auto_post",
      "attrs": {
        "invisible": "move_type != 'entry'",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "reversed_entry_id",
      "attrs": {
        "invisible": "move_type != 'entry' or not reversed_entry_id"
      }
    },
    {
      "name": "auto_post_until",
      "attrs": {
        "invisible": "auto_post in ('no', 'at_date')",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "fiscal_position_id",
      "attrs": {
        "readonly": "state in ['cancel', 'posted']"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "groups": "base.group_multi_company",
        "required": "1"
      }
    },
    {
      "name": "narration",
      "attrs": {
        "height": "50",
        "nolabel": "1",
        "placeholder": "Add an internal note..."
      }
    }
  ],
  "buttons": [
    {
      "name": "action_post",
      "string": "Post",
      "type": "object",
      "class": "oe_highlight"
    },
    {
      "name": "action_post",
      "string": "Confirm",
      "type": "object",
      "class": "oe_highlight"
    },
    {
      "name": "action_invoice_sent",
      "string": "Send",
      "type": "object",
      "class": "oe_highlight"
    },
    {
      "name": "action_invoice_sent",
      "string": "Send",
      "type": "object"
    },
    {
      "name": "action_print_pdf",
      "string": "Print",
      "type": "object",
      "class": "oe_highlight"
    },
    {
      "name": "action_print_pdf",
      "string": "Print",
      "type": "object"
    },
    {
      "name": "action_register_payment",
      "string": "Pay",
      "type": "object",
      "class": "oe_highlight"
    },
    {
      "name": "action_register_payment",
      "string": "Pay",
      "type": "object"
    },
    {
      "name": "preview_invoice",
      "string": "Preview",
      "type": "object"
    },
    {
      "name": "%(action_view_account_move_reversal)d",
      "string": "Reverse Entry",
      "type": "action"
    },
    {
      "name": "action_reverse",
      "string": "Credit Note",
      "type": "object"
    },
    {
      "name": "button_cancel",
      "string": "Cancel Entry",
      "type": "object"
    },
    {
      "name": "button_cancel",
      "string": "Cancel",
      "type": "object"
    },
    {
      "name": "button_draft",
      "string": "Reset to Draft",
      "type": "object"
    },
    {
      "name": "button_hash",
      "string": "Lock",
      "type": "object"
    },
    {
      "name": "button_request_cancel",
      "string": "Request Cancel",
      "type": "object"
    },
    {
      "name": "button_set_checked",
      "string": "Reviewed",
      "type": "object",
      "class": "btn btn-info"
    },
    {
      "name": "action_delete_duplicates",
      "type": "object",
      "class": "btn btn-link text-danger ms-auto d-flex align-items-center gap-1"
    },
    {
      "name": "action_activate_currency",
      "type": "object",
      "class": "oe_link"
    },
    {
      "name": "action_activate_currency",
      "type": "object",
      "class": "oe_link"
    },
    {
      "name": "action_open_business_doc",
      "type": "object",
      "class": "oe_stat_button"
    },
    {
      "name": "open_payments",
      "type": "object",
      "class": "oe_stat_button"
    },
    {
      "name": "open_reconcile_view",
      "type": "object",
      "class": "oe_stat_button"
    },
    {
      "name": "open_created_caba_entries",
      "type": "object",
      "class": "oe_stat_button"
    },
    {
      "name": "open_adjusting_entries",
      "type": "object",
      "class": "oe_stat_button"
    },
    {
      "name": "open_adjusting_entry_origin_moves",
      "type": "object",
      "class": "oe_stat_button"
    },
    {
      "name": "action_update_fpos_values",
      "string": "Update Taxes and Accounts",
      "type": "object",
      "class": "btn-link mb-1 px-0"
    },
    {
      "name": "refresh_invoice_currency_rate",
      "type": "object",
      "class": "btn btn-link p-0"
    },
    {
      "name": "refresh_invoice_currency_rate",
      "type": "object",
      "class": "btn btn-link p-0"
    },
    {
      "name": "action_add_from_catalog",
      "string": "Catalog",
      "type": "object",
      "class": "btn-link"
    },
    {
      "name": "action_automatic_entry",
      "string": "Cut-Off",
      "type": "object",
      "class": "float-end"
    }
  ]
}

export function renderAccountMoveForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'account.move' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/account.move/<método> (≈ call_kw)
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
