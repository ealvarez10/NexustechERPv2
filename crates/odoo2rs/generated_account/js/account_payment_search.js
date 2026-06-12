// Generado por odoo2rs — vista search de account.payment (view_account_payment_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_payment_search",
  "name": "account.payment.search",
  "model": "account.payment",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Payments"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "filter_domain": "['|', '|', '|', '|', ('name', 'ilike', self), ('partner_id', 'ilike', self), ('memo', 'ilike', self), ('amount_company_currency_signed' , 'ilike', self), ('amount', 'ilike', self)]",
          "name": "name",
          "string": "Payment"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "partner_id",
          "string": "Customer/Vendor"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "journal_id"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('partner_type', '=', 'customer')]",
          "name": "inbound_filter",
          "string": "Customer Payments"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('partner_type', '=', 'supplier')]",
          "name": "outbound_filter",
          "string": "Vendor Payments"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('state', '=', 'draft')]",
          "name": "state_draft",
          "string": "Draft"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('state', '=', 'in_process')]",
          "name": "state_in_process",
          "string": "In Process"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('is_sent', '=', 'True')]",
          "name": "state_sent",
          "string": "Sent"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('is_sent', '=', 'False')]",
          "name": "state_sent",
          "string": "Not Sent"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('is_matched', '=', False)]",
          "name": "unmatched",
          "string": "No Bank Matching"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('is_reconciled', '=', True)]",
          "name": "reconciled",
          "string": "Reconciled"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "date": "date",
          "name": "date",
          "string": "Payment Date"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_multi_company",
          "name": "company_id"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "context": "{'group_by': 'partner_id'}",
          "domain": "[]",
          "name": "partner",
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
          "context": "{'group_by': 'payment_method_line_id'}",
          "domain": "[]",
          "name": "paymentmethodline",
          "string": "Payment Method Line"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "context": "{'group_by': 'state'}",
          "domain": "[]",
          "name": "state",
          "string": "Status"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "context": "{'group_by': 'date'}",
          "domain": "[]",
          "name": "groupby_date",
          "string": "Payment Date"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "context": "{'group_by': 'currency_id'}",
          "domain": "[]",
          "groups": "base.group_multi_currency",
          "name": "currency",
          "string": "Currency"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "context": "{'group_by': 'company_id'}",
          "domain": "[]",
          "groups": "base.group_multi_company",
          "name": "company",
          "string": "Company"
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
      }
    ]
  },
  "fields": [
    {
      "name": "name",
      "string": "Payment",
      "attrs": {
        "filter_domain": "['|', '|', '|', '|', ('name', 'ilike', self), ('partner_id', 'ilike', self), ('memo', 'ilike', self), ('amount_company_currency_signed' , 'ilike', self), ('amount', 'ilike', self)]"
      }
    },
    {
      "name": "partner_id",
      "string": "Customer/Vendor"
    },
    {
      "name": "journal_id"
    },
    {
      "name": "company_id",
      "attrs": {
        "groups": "base.group_multi_company"
      }
    }
  ]
}

export function renderAccountPaymentSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
