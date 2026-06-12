// Generado por odoo2rs — vista tree de account.move.line (view_move_line_payment_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_move_line_payment_tree",
  "name": "account.move.line.payment.list",
  "model": "account.move.line",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "create": "false",
      "decoration-info": "parent_state == 'draft'",
      "default_order": "payment_date asc, id asc",
      "edit": "true",
      "expand": "context.get('expand', False)",
      "multi_edit": "1",
      "sample": "1",
      "string": "Payment Items"
    },
    "children": [
      {
        "tag": "header",
        "children": [
          {
            "tag": "button",
            "attrs": {
              "groups": "account.group_account_user",
              "name": "action_payment_items_register_payment",
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
          "name": "move_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "invoice_date",
          "string": "Bill Date"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "date",
          "optional": "hide",
          "readonly": "1"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "date_maturity",
          "optional": "hide",
          "readonly": "1",
          "string": "Invoice Due Date"
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
          "name": "payment_date",
          "readonly": "1"
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
          "groups": "base.group_multi_company",
          "name": "company_id",
          "optional": "hide",
          "readonly": "1"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "journal_id",
          "optional": "hide",
          "options": "{\"no_open\":True}",
          "readonly": "1"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "move_name",
          "string": "Journal Entry",
          "widget": "open_move_widget"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "partner_id",
          "optional": "show",
          "readonly": "move_type != 'entry'"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "ref",
          "readonly": "False"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "name",
          "optional": "show"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "invisible": "not discount_amount_currency",
          "name": "discount_amount_currency",
          "optional": "show",
          "string": "Discount Amount"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "invisible": "not is_account_reconcile",
          "name": "amount_residual",
          "readonly": "1",
          "string": "Residual",
          "sum": "Total Residual"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_multi_currency",
          "invisible": "is_same_currency or not is_account_reconcile",
          "name": "amount_residual_currency",
          "optional": "hide",
          "readonly": "1",
          "string": "Residual in Currency"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_multi_currency",
          "invisible": "is_same_currency",
          "name": "currency_id",
          "optional": "hide",
          "readonly": "1",
          "string": "Currency"
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
          "name": "move_type"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "is_same_currency"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "is_account_reconcile"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "parent_state"
        }
      },
      {
        "tag": "groupby",
        "attrs": {
          "name": "partner_id"
        },
        "children": [
          {
            "tag": "button",
            "attrs": {
              "icon": "fa-edit",
              "name": "edit",
              "title": "Edit",
              "type": "edit"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "move_id",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "invoice_date",
      "string": "Bill Date"
    },
    {
      "name": "date",
      "attrs": {
        "optional": "hide",
        "readonly": "1"
      }
    },
    {
      "name": "date_maturity",
      "string": "Invoice Due Date",
      "attrs": {
        "optional": "hide",
        "readonly": "1"
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
      "name": "payment_date",
      "attrs": {
        "readonly": "1"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "groups": "base.group_multi_company",
        "optional": "hide",
        "readonly": "1"
      }
    },
    {
      "name": "journal_id",
      "attrs": {
        "optional": "hide",
        "options": "{\"no_open\":True}",
        "readonly": "1"
      }
    },
    {
      "name": "move_name",
      "string": "Journal Entry",
      "widget": "open_move_widget"
    },
    {
      "name": "partner_id",
      "attrs": {
        "optional": "show",
        "readonly": "move_type != 'entry'"
      }
    },
    {
      "name": "ref",
      "attrs": {
        "readonly": "False"
      }
    },
    {
      "name": "name",
      "attrs": {
        "optional": "show"
      }
    },
    {
      "name": "discount_amount_currency",
      "string": "Discount Amount",
      "attrs": {
        "invisible": "not discount_amount_currency",
        "optional": "show"
      }
    },
    {
      "name": "amount_residual",
      "string": "Residual",
      "attrs": {
        "invisible": "not is_account_reconcile",
        "readonly": "1",
        "sum": "Total Residual"
      }
    },
    {
      "name": "amount_residual_currency",
      "string": "Residual in Currency",
      "attrs": {
        "groups": "base.group_multi_currency",
        "invisible": "is_same_currency or not is_account_reconcile",
        "optional": "hide",
        "readonly": "1"
      }
    },
    {
      "name": "currency_id",
      "string": "Currency",
      "attrs": {
        "groups": "base.group_multi_currency",
        "invisible": "is_same_currency",
        "optional": "hide",
        "readonly": "1"
      }
    },
    {
      "name": "company_currency_id",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "move_type",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "is_same_currency",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "is_account_reconcile",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "parent_state",
      "attrs": {
        "column_invisible": "True"
      }
    }
  ],
  "buttons": [
    {
      "name": "action_payment_items_register_payment",
      "string": "Pay",
      "type": "object"
    },
    {
      "name": "edit",
      "type": "edit"
    }
  ]
}

export function renderAccountMoveLineTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
