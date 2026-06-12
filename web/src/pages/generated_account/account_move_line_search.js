// Generado por odoo2rs — vista search de account.move.line (view_account_move_line_payment_filter).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_move_line_payment_filter",
  "name": "account.move.line.payment.search",
  "model": "account.move.line",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Search Journal Items"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "filter_domain": "[                         '|', '|', '|',                         ('name', 'ilike', self), ('ref', 'ilike', self), ('account_id', 'ilike', self), ('partner_id', 'ilike', self)]",
          "name": "name",
          "string": "Journal Item"
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
          "name": "move_id"
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
          "name": "payment_date"
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
          "name": "journal_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_multi_currency",
          "invisible": "1",
          "name": "currency_id"
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
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('parent_state', '=', 'posted')]",
          "help": "Posted Journal Items",
          "invisible": "1",
          "name": "posted",
          "string": "Posted"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('amount_residual', '>', '0')]",
          "invisible": "context.get('journal_type') != 'sale'",
          "name": "invoices",
          "string": "Invoices"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('amount_residual', '<', '0')]",
          "invisible": "context.get('journal_type') != 'sale'",
          "name": "credit_notes",
          "string": "Credit Notes"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('amount_residual', '<', '0')]",
          "invisible": "context.get('journal_type') != 'purchase'",
          "name": "bills",
          "string": "Bills"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('amount_residual', '>', '0')]",
          "invisible": "context.get('journal_type') != 'purchase'",
          "name": "refunds",
          "string": "Refunds"
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
          "date": "payment_date",
          "name": "payment_date",
          "string": "Next Payment Date"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('date_maturity', '<', 'today')]",
          "help": "Overdue payments, due date passed",
          "name": "late",
          "string": "Overdue"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('discount_date', '!=', False), ('discount_date', '>', 'today')]",
          "name": "early_discount",
          "string": "Early Discount"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('date', '>=', context.get('date_from')), ('date', '<=', context.get('date_to'))]",
          "invisible": "1",
          "name": "date_between",
          "string": "Report Dates"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('date', '<=', context.get('date_to'))]",
          "invisible": "1",
          "name": "date_before",
          "string": "Report Dates"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "group",
        "children": [
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'currency_id'}",
              "domain": "[]",
              "name": "group_by_currencies",
              "string": "Currency"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'partner_id'}",
              "domain": "[]",
              "name": "group_by_partner",
              "string": "Partner"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'journal_id'}",
              "domain": "[]",
              "name": "journal",
              "string": "Journal"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'invoice_date'}",
              "domain": "[]",
              "name": "groupby_invoice_date",
              "string": "Invoice Date"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'payment_date'}",
              "domain": "[]",
              "name": "groupby_payment_date",
              "string": "Next Payment Date"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "name",
      "string": "Journal Item",
      "attrs": {
        "filter_domain": "[                         '|', '|', '|',                         ('name', 'ilike', self), ('ref', 'ilike', self), ('account_id', 'ilike', self), ('partner_id', 'ilike', self)]"
      }
    },
    {
      "name": "name"
    },
    {
      "name": "move_id"
    },
    {
      "name": "ref"
    },
    {
      "name": "payment_date"
    },
    {
      "name": "partner_id"
    },
    {
      "name": "journal_id"
    },
    {
      "name": "currency_id",
      "attrs": {
        "groups": "base.group_multi_currency",
        "invisible": "1"
      }
    },
    {
      "name": "company_currency_id",
      "attrs": {
        "invisible": "1"
      }
    }
  ]
}

export function renderAccountMoveLineSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
