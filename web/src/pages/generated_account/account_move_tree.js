// Generado por odoo2rs — vista tree de account.move (view_invoice_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_invoice_tree",
  "name": "account.invoice.list",
  "model": "account.move",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "decoration-info": "state == 'draft'",
      "decoration-muted": "state == 'cancel'",
      "expand": "context.get('expand', False)",
      "js_class": "account_tree",
      "sample": "1",
      "string": "Invoices"
    },
    "children": [
      {
        "tag": "header",
        "children": [
          {
            "tag": "button",
            "attrs": {
              "groups": "account.group_account_invoice",
              "invisible": "context.get('default_move_type') not in ('out_invoice', 'out_refund', 'out_receipt', 'in_invoice', 'in_refund','in_receipt')",
              "name": "action_force_register_payment",
              "string": "Pay",
              "type": "object"
            }
          }
        ]
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "made_sequence_gap"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "duplicated_ref_ids"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "decoration-bf": "1",
          "decoration-danger": "made_sequence_gap and state == 'posted'",
          "name": "name",
          "placeholder": "/",
          "widget": "char_with_placeholder_field_to_check"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "groups": "account.group_account_user",
          "name": "checked"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "context.get('default_move_type') not in ('in_invoice', 'in_refund', 'in_receipt')",
          "groups": "base.group_user",
          "name": "invoice_partner_display_name",
          "string": "Vendor"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "context.get('default_move_type') not in ('out_invoice', 'out_refund', 'out_receipt')",
          "groups": "base.group_user",
          "name": "invoice_partner_display_name",
          "string": "Customer"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "context.get('default_move_type') not in ('in_invoice', 'in_refund', 'in_receipt')",
          "decoration-warning": "abnormal_date_warning",
          "name": "invoice_date",
          "optional": "show",
          "readonly": "state != 'draft'",
          "string": "Bill Date"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "context.get('default_move_type') not in ('out_invoice', 'out_refund', 'out_receipt')",
          "decoration-warning": "abnormal_date_warning",
          "name": "invoice_date",
          "optional": "show",
          "readonly": "state != 'draft'",
          "string": "Invoice Date"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "date",
          "optional": "hide",
          "readonly": "state in ['cancel', 'posted']",
          "string": "Accounting Date"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "invisible": "payment_state in ('paid', 'in_payment', 'reversed') or state == 'cancel'",
          "name": "invoice_date_due",
          "optional": "show",
          "widget": "remaining_days"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "invoice_origin",
          "optional": "hide",
          "string": "Source Document"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "context.get('default_move_type') in ('out_invoice', 'out_refund', 'out_receipt')",
          "name": "payment_reference",
          "optional": "hide"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "ref",
          "optional": "hide"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "context.get('default_move_type') not in ('out_invoice', 'out_refund', 'out_receipt')",
          "name": "invoice_user_id",
          "optional": "hide",
          "string": "Salesperson",
          "widget": "many2one_avatar_user"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "activity_ids",
          "optional": "hide",
          "widget": "list_activity"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_multi_company",
          "name": "company_id",
          "optional": "hide",
          "options": "{'no_create': True}"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "groups": "!base.group_multi_company",
          "name": "company_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "amount_untaxed_in_currency_signed",
          "optional": "show",
          "string": "Tax Excluded",
          "sum": "Total"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "amount_tax_signed",
          "optional": "hide",
          "string": "Tax",
          "sum": "Total"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "decoration-bf": "1",
          "decoration-warning": "abnormal_amount_warning",
          "name": "amount_total_in_currency_signed",
          "optional": "show",
          "string": "Total",
          "sum": "Total"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "amount_residual_signed",
          "optional": "hide",
          "string": "Amount Due",
          "sum": "Amount Due"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "currency_id",
          "optional": "hide",
          "readonly": "state in ['cancel', 'posted']"
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
          "decoration-danger": "status_in_payment == 'cancel'",
          "decoration-info": "status_in_payment == 'draft'",
          "decoration-muted": "status_in_payment in ('posted', 'sent', 'partial')",
          "decoration-success": "status_in_payment in ('in_payment', 'paid', 'reversed')",
          "invisible": "payment_state == 'invoicing_legacy' or move_type == 'entry'",
          "name": "status_in_payment",
          "optional": "show",
          "string": "Status",
          "widget": "badge"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "context.get('default_move_type') not in ('out_invoice', 'out_refund', 'out_receipt')",
          "decoration-danger": "move_sent_values == 'not_sent'",
          "decoration-success": "move_sent_values == 'sent'",
          "name": "move_sent_values",
          "optional": "hide",
          "string": "Sent",
          "widget": "badge"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "context.get('default_move_type', True)",
          "name": "move_type"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "1",
          "name": "abnormal_amount_warning"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "1",
          "name": "abnormal_date_warning"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "made_sequence_gap",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "duplicated_ref_ids",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "name",
      "widget": "char_with_placeholder_field_to_check",
      "attrs": {
        "decoration-bf": "1",
        "decoration-danger": "made_sequence_gap and state == 'posted'",
        "placeholder": "/"
      }
    },
    {
      "name": "checked",
      "attrs": {
        "column_invisible": "True",
        "groups": "account.group_account_user"
      }
    },
    {
      "name": "invoice_partner_display_name",
      "string": "Vendor",
      "attrs": {
        "column_invisible": "context.get('default_move_type') not in ('in_invoice', 'in_refund', 'in_receipt')",
        "groups": "base.group_user"
      }
    },
    {
      "name": "invoice_partner_display_name",
      "string": "Customer",
      "attrs": {
        "column_invisible": "context.get('default_move_type') not in ('out_invoice', 'out_refund', 'out_receipt')",
        "groups": "base.group_user"
      }
    },
    {
      "name": "invoice_date",
      "string": "Bill Date",
      "attrs": {
        "column_invisible": "context.get('default_move_type') not in ('in_invoice', 'in_refund', 'in_receipt')",
        "decoration-warning": "abnormal_date_warning",
        "optional": "show",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "invoice_date",
      "string": "Invoice Date",
      "attrs": {
        "column_invisible": "context.get('default_move_type') not in ('out_invoice', 'out_refund', 'out_receipt')",
        "decoration-warning": "abnormal_date_warning",
        "optional": "show",
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "date",
      "string": "Accounting Date",
      "attrs": {
        "optional": "hide",
        "readonly": "state in ['cancel', 'posted']"
      }
    },
    {
      "name": "invoice_date_due",
      "widget": "remaining_days",
      "attrs": {
        "invisible": "payment_state in ('paid', 'in_payment', 'reversed') or state == 'cancel'",
        "optional": "show"
      }
    },
    {
      "name": "invoice_origin",
      "string": "Source Document",
      "attrs": {
        "optional": "hide"
      }
    },
    {
      "name": "payment_reference",
      "attrs": {
        "column_invisible": "context.get('default_move_type') in ('out_invoice', 'out_refund', 'out_receipt')",
        "optional": "hide"
      }
    },
    {
      "name": "ref",
      "attrs": {
        "optional": "hide"
      }
    },
    {
      "name": "invoice_user_id",
      "string": "Salesperson",
      "widget": "many2one_avatar_user",
      "attrs": {
        "column_invisible": "context.get('default_move_type') not in ('out_invoice', 'out_refund', 'out_receipt')",
        "optional": "hide"
      }
    },
    {
      "name": "activity_ids",
      "widget": "list_activity",
      "attrs": {
        "optional": "hide"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "groups": "base.group_multi_company",
        "optional": "hide",
        "options": "{'no_create': True}"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "column_invisible": "True",
        "groups": "!base.group_multi_company"
      }
    },
    {
      "name": "amount_untaxed_in_currency_signed",
      "string": "Tax Excluded",
      "attrs": {
        "optional": "show",
        "sum": "Total"
      }
    },
    {
      "name": "amount_tax_signed",
      "string": "Tax",
      "attrs": {
        "optional": "hide",
        "sum": "Total"
      }
    },
    {
      "name": "amount_total_in_currency_signed",
      "string": "Total",
      "attrs": {
        "decoration-bf": "1",
        "decoration-warning": "abnormal_amount_warning",
        "optional": "show",
        "sum": "Total"
      }
    },
    {
      "name": "amount_residual_signed",
      "string": "Amount Due",
      "attrs": {
        "optional": "hide",
        "sum": "Amount Due"
      }
    },
    {
      "name": "currency_id",
      "attrs": {
        "optional": "hide",
        "readonly": "state in ['cancel', 'posted']"
      }
    },
    {
      "name": "company_currency_id",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "status_in_payment",
      "string": "Status",
      "widget": "badge",
      "attrs": {
        "decoration-danger": "status_in_payment == 'cancel'",
        "decoration-info": "status_in_payment == 'draft'",
        "decoration-muted": "status_in_payment in ('posted', 'sent', 'partial')",
        "decoration-success": "status_in_payment in ('in_payment', 'paid', 'reversed')",
        "invisible": "payment_state == 'invoicing_legacy' or move_type == 'entry'",
        "optional": "show"
      }
    },
    {
      "name": "move_sent_values",
      "string": "Sent",
      "widget": "badge",
      "attrs": {
        "column_invisible": "context.get('default_move_type') not in ('out_invoice', 'out_refund', 'out_receipt')",
        "decoration-danger": "move_sent_values == 'not_sent'",
        "decoration-success": "move_sent_values == 'sent'",
        "optional": "hide"
      }
    },
    {
      "name": "move_type",
      "attrs": {
        "column_invisible": "context.get('default_move_type', True)"
      }
    },
    {
      "name": "abnormal_amount_warning",
      "attrs": {
        "column_invisible": "1"
      }
    },
    {
      "name": "abnormal_date_warning",
      "attrs": {
        "column_invisible": "1"
      }
    }
  ],
  "buttons": [
    {
      "name": "action_force_register_payment",
      "string": "Pay",
      "type": "object"
    }
  ]
}

export function renderAccountMoveTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
