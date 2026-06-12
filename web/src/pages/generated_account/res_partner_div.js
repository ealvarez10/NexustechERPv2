// Generado por odoo2rs — vista div de res.partner (partner_view_buttons).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "partner_view_buttons",
  "name": "partner.view.buttons",
  "model": "res.partner",
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
          "context": "{'default_partner_id': id}",
          "groups": "account.group_account_invoice,account.group_account_readonly",
          "icon": "fa-pencil-square-o",
          "name": "action_view_partner_invoices",
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
                  "class": "o_stat_value"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "1",
                      "name": "currency_id"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "total_invoiced",
                      "options": "{'currency_field': 'currency_id'}",
                      "widget": "monetary"
                    }
                  }
                ]
              },
              {
                "tag": "span",
                "attrs": {
                  "class": "o_stat_text"
                },
                "text": "Invoiced"
              }
            ]
          }
        ]
      },
      {
        "tag": "button",
        "attrs": {
          "class": "oe_stat_button",
          "groups": "account.group_account_invoice",
          "help": "Vendor Bills",
          "icon": "fa-pencil-square-o",
          "invisible": "supplier_invoice_count == 0",
          "name": "%(account.res_partner_action_supplier_bills)d",
          "type": "action"
        },
        "children": [
          {
            "tag": "field",
            "attrs": {
              "name": "supplier_invoice_count",
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
      "name": "currency_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "total_invoiced",
      "widget": "monetary",
      "attrs": {
        "options": "{'currency_field': 'currency_id'}"
      }
    },
    {
      "name": "supplier_invoice_count",
      "string": "Vendor Bills",
      "widget": "statinfo"
    }
  ],
  "buttons": [
    {
      "name": "action_view_partner_invoices",
      "type": "object",
      "class": "oe_stat_button"
    },
    {
      "name": "%(account.res_partner_action_supplier_bills)d",
      "type": "action",
      "class": "oe_stat_button"
    }
  ]
}

export function renderResPartnerDiv(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
