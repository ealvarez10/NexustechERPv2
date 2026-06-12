// Generado por odoo2rs — vista search de account.move (view_account_invoice_filter).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_invoice_filter",
  "name": "account.invoice.select",
  "model": "account.move",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Search Invoice"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "filter_domain": "[                                 '|', '|' , '|', '|',                                 ('name', 'ilike', self), ('invoice_origin', 'ilike', self),                                 ('ref', 'ilike', self), ('payment_reference', 'ilike', self),                                 ('partner_id', 'child_of', self)]",
          "name": "name",
          "string": "Invoice"
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
          "name": "ref"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "payment_reference"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "amount_total"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "journal_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "journal_group_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "partner_id",
          "operator": "child_of"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "domain": "[('share', '=', False)]",
          "name": "invoice_user_id",
          "string": "Salesperson"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "date",
          "string": "Period"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "next_payment_date"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "line_ids",
          "string": "Invoice Line"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "activity_user_id",
          "string": "Activities of"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "activity_type_id",
          "string": "Activity type"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('invoice_user_id', '=', uid)]",
          "help": "My Invoices",
          "name": "myinvoices"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('state','=','draft')]",
          "name": "draft",
          "string": "Draft"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('state', '=', 'posted')]",
          "name": "posted",
          "string": "Posted"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('state', '=', 'cancel')]",
          "name": "cancel",
          "string": "Cancelled"
        }
      },
      {
        "tag": "separator",
        "attrs": {
          "groups": "account.group_account_secured,base.group_no_one"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('secured', '=', False), ('state', '=', 'posted')]",
          "groups": "account.group_account_secured,base.group_no_one",
          "name": "not_secured",
          "string": "Not Secured"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('is_move_sent', '=', False)]",
          "invisible": "context.get('default_move_type') in ('in_invoice', 'in_refund', 'in_receipt')",
          "name": "not_sent",
          "string": "Not Sent"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('move_type', '=', 'out_invoice')]",
          "name": "out_invoice",
          "string": "Invoices"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('move_type', '=', 'out_receipt')]",
          "name": "out_receipt",
          "string": "Receipts"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('move_type', '=', 'out_refund')]",
          "name": "out_refund",
          "string": "Credit Notes"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('checked', '=', False), ('state', '!=', 'draft')]",
          "name": "to_check",
          "string": "To Review"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('state', '!=', 'cancel'), ('payment_state', 'in', ('not_paid', 'partial')), ('move_type', '!=', 'entry')]",
          "name": "open",
          "string": "To pay"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('state', '=', 'posted'), ('payment_state', '=', 'in_payment')]",
          "name": "in_payment",
          "string": "In payment"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[                         ('invoice_date_due', '<', 'today'),                         ('state', '=', 'posted'),                         ('payment_state', 'in', ('not_paid', 'partial')),                         ('move_type', '!=', 'entry')                     ]",
          "help": "Overdue invoices, maturity date passed",
          "name": "late",
          "string": "Overdue"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "date": "invoice_date",
          "name": "invoice_date",
          "string": "Invoice Date"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "date": "date",
          "invisible": "context.get('default_move_type') in ('out_invoice', 'out_refund', 'out_receipt')",
          "name": "date",
          "string": "Accounting Date"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "date": "invoice_date_due",
          "name": "due_date",
          "string": "Due Date"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('activity_user_id', '=', uid)]",
          "invisible": "1",
          "name": "filter_activities_my",
          "string": "My Activities"
        }
      },
      {
        "tag": "separator",
        "attrs": {
          "invisible": "1"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('my_activity_date_deadline', '<', 'today')]",
          "help": "Show all records whose next activity date is past",
          "invisible": "1",
          "name": "activities_overdue",
          "string": "Late Activities"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('my_activity_date_deadline', '=', 'today')]",
          "invisible": "1",
          "name": "activities_today",
          "string": "Today Activities"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('my_activity_date_deadline', '>', 'today')]",
          "invisible": "1",
          "name": "activities_upcoming_all",
          "string": "Future Activities"
        }
      },
      {
        "tag": "group",
        "children": [
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by':'invoice_user_id'}",
              "name": "salesperson",
              "string": "Salesperson"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by':'partner_id'}",
              "name": "partner",
              "string": "Partner"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by':'state'}",
              "name": "status",
              "string": "Status"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'preferred_payment_method_line_id'}",
              "groups": "account.group_account_invoice,account.group_account_readonly",
              "name": "preferred_payment_method_line",
              "string": "Payment Method"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'journal_id'}",
              "domain": "[]",
              "name": "groupy_by_journal",
              "string": "Journal"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'company_id'}",
              "groups": "base.group_multi_company",
              "name": "group_by_company",
              "string": "Company"
            }
          },
          {
            "tag": "separator"
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'invoice_date'}",
              "name": "invoicedate",
              "string": "Invoice Date"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'invoice_date_due'}",
              "name": "duedate",
              "string": "Due Date"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'date'}",
              "name": "accounting_date",
              "string": "Accounting Date"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'sequence_prefix'}",
              "invisible": "1",
              "name": "group_by_sequence_prefix",
              "string": "Sequence Prefix"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "name",
      "string": "Invoice",
      "attrs": {
        "filter_domain": "[                                 '|', '|' , '|', '|',                                 ('name', 'ilike', self), ('invoice_origin', 'ilike', self),                                 ('ref', 'ilike', self), ('payment_reference', 'ilike', self),                                 ('partner_id', 'child_of', self)]"
      }
    },
    {
      "name": "name"
    },
    {
      "name": "ref"
    },
    {
      "name": "payment_reference"
    },
    {
      "name": "amount_total"
    },
    {
      "name": "journal_id"
    },
    {
      "name": "journal_group_id"
    },
    {
      "name": "partner_id",
      "attrs": {
        "operator": "child_of"
      }
    },
    {
      "name": "invoice_user_id",
      "string": "Salesperson",
      "attrs": {
        "domain": "[('share', '=', False)]"
      }
    },
    {
      "name": "date",
      "string": "Period"
    },
    {
      "name": "next_payment_date"
    },
    {
      "name": "line_ids",
      "string": "Invoice Line"
    },
    {
      "name": "activity_user_id",
      "string": "Activities of"
    },
    {
      "name": "activity_type_id",
      "string": "Activity type"
    }
  ]
}

export function renderAccountMoveSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
