// Generado por odoo2rs — vista xpath de res.partner.bank (view_partner_bank_search_inherit).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_partner_bank_search_inherit",
  "name": "res.partner.bank.search.inherit",
  "model": "res.partner.bank",
  "type": "xpath",
  "arch": {
    "tag": "xpath",
    "attrs": {
      "expr": "//search",
      "position": "inside"
    },
    "children": [
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('allow_out_payment', '=', True)]",
          "name": "trusted",
          "string": "Trusted"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('allow_out_payment', '=', False)]",
          "name": "untrusted",
          "string": "Untrusted"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('allow_out_payment', '=', False), ('related_moves', '!=', False)]",
          "name": "to_validate",
          "string": "To validate"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('partner_customer_rank','>', 0)]",
          "name": "customer",
          "string": "Customers"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('partner_supplier_rank','>', 0)]",
          "name": "supplier",
          "string": "Vendors"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('has_money_transfer_warning', '!=', False)]",
          "name": "high_phishing_risk",
          "string": "Phishing risk: High"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('has_iban_warning', '!=', False)]",
          "name": "medium_phishing_risk",
          "string": "Phishing risk: Medium"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "date": "create_date",
          "name": "create_date",
          "string": "Created On"
        }
      },
      {
        "tag": "group",
        "children": [
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'create_date'}",
              "name": "groupby_create_date",
              "string": "Created On"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'create_uid'}",
              "name": "groupby_create_by",
              "string": "Created By"
            }
          }
        ]
      }
    ]
  },
  "fields": []
}

export function renderResPartnerBankXpath(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
