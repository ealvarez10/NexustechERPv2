// Generado por odoo2rs — vista div de account.analytic.account (account_analytic_account_view_form_inherit).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "account_analytic_account_view_form_inherit",
  "name": "account.analytic.account.form.inherit",
  "model": "account.analytic.account",
  "type": "div",
  "arch": {
    "tag": "div",
    "attrs": {
      "name": "button_box",
      "position": "inside"
    },
    "children": [
      {
        "tag": "button",
        "attrs": {
          "class": "oe_stat_button",
          "icon": "fa-pencil-square-o",
          "invisible": "invoice_count == 0",
          "name": "action_view_invoice",
          "type": "object"
        },
        "children": [
          {
            "tag": "field",
            "attrs": {
              "name": "invoice_count",
              "string": "Customer Invoices",
              "widget": "statinfo"
            }
          }
        ]
      },
      {
        "tag": "button",
        "attrs": {
          "class": "oe_stat_button",
          "icon": "fa-file-text-o",
          "invisible": "vendor_bill_count == 0",
          "name": "action_view_vendor_bill",
          "type": "object"
        },
        "children": [
          {
            "tag": "field",
            "attrs": {
              "name": "vendor_bill_count",
              "string": "Vendor Bills",
              "widget": "statinfo"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "invoice_count",
      "string": "Customer Invoices",
      "widget": "statinfo"
    },
    {
      "name": "vendor_bill_count",
      "string": "Vendor Bills",
      "widget": "statinfo"
    }
  ],
  "buttons": [
    {
      "name": "action_view_invoice",
      "type": "object",
      "class": "oe_stat_button"
    },
    {
      "name": "action_view_vendor_bill",
      "type": "object",
      "class": "oe_stat_button"
    }
  ]
}

export function renderAccountAnalyticAccountDiv(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
