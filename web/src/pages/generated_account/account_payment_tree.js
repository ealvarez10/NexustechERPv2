// Generado por odoo2rs — vista tree de account.payment (view_account_payment_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_payment_tree",
  "name": "account.payment.list",
  "model": "account.payment",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "decoration-info": "state == 'draft'",
      "decoration-muted": "state == 'canceled'",
      "edit": "false",
      "sample": "1"
    },
    "children": [
      {
        "tag": "header",
        "children": [
          {
            "tag": "button",
            "attrs": {
              "name": "action_post",
              "string": "Confirm",
              "type": "object"
            }
          }
        ]
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
          "name": "available_payment_method_line_ids"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "date",
          "readonly": "state in ['cancel', 'in_process']"
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
          "name": "journal_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_multi_company",
          "name": "company_id",
          "optional": "hide"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "context": "{'hide_payment_journal_id': 1}",
          "name": "payment_method_line_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "partner_id",
          "string": "Customer"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "!base.group_multi_currency",
          "name": "amount_signed",
          "optional": "hide",
          "string": "Amount in Currency"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_multi_currency",
          "name": "amount_signed",
          "optional": "show",
          "string": "Amount in Currency"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "currency_id",
          "optional": "hide",
          "string": "Payment Currency"
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
          "name": "amount_company_currency_signed",
          "string": "Amount",
          "sum": "Total",
          "widget": "monetary"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "decoration-info": "state == 'draft'",
          "decoration-success": "state == 'paid'",
          "decoration-warning": "state == 'in_process'",
          "name": "state",
          "widget": "badge"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "company_currency_id",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "available_payment_method_line_ids",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "date",
      "attrs": {
        "readonly": "state in ['cancel', 'in_process']"
      }
    },
    {
      "name": "name"
    },
    {
      "name": "journal_id"
    },
    {
      "name": "company_id",
      "attrs": {
        "groups": "base.group_multi_company",
        "optional": "hide"
      }
    },
    {
      "name": "payment_method_line_id",
      "attrs": {
        "context": "{'hide_payment_journal_id': 1}"
      }
    },
    {
      "name": "partner_id",
      "string": "Customer"
    },
    {
      "name": "amount_signed",
      "string": "Amount in Currency",
      "attrs": {
        "groups": "!base.group_multi_currency",
        "optional": "hide"
      }
    },
    {
      "name": "amount_signed",
      "string": "Amount in Currency",
      "attrs": {
        "groups": "base.group_multi_currency",
        "optional": "show"
      }
    },
    {
      "name": "currency_id",
      "string": "Payment Currency",
      "attrs": {
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
      "name": "amount_company_currency_signed",
      "string": "Amount",
      "widget": "monetary",
      "attrs": {
        "sum": "Total"
      }
    },
    {
      "name": "state",
      "widget": "badge",
      "attrs": {
        "decoration-info": "state == 'draft'",
        "decoration-success": "state == 'paid'",
        "decoration-warning": "state == 'in_process'"
      }
    }
  ],
  "buttons": [
    {
      "name": "action_post",
      "string": "Confirm",
      "type": "object"
    }
  ]
}

export function renderAccountPaymentTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
