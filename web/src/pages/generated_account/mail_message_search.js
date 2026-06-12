// Generado por odoo2rs — vista search de mail.message (view_message_tree_audit_log_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_message_tree_audit_log_search",
  "name": "mail.message.search",
  "model": "mail.message",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Messages Search"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "account_audit_log_move_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "account_audit_log_account_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "account_audit_log_tax_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "account_audit_log_partner_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "account_audit_log_company_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "author_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "date"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('model', '=', 'account.move')]",
          "name": "account_move",
          "string": "Journal Entry"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('model', '=', 'account.account')]",
          "name": "account_account",
          "string": "Account"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('model', '=', 'account.tax')]",
          "name": "account_tax",
          "string": "Taxes"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('model', '=', 'res.partner')]",
          "name": "res_partner",
          "string": "Partners"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('model', '=', 'res.company')]",
          "name": "res_company",
          "string": "Company"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "field",
        "attrs": {
          "filter_domain": "[                     '|', ('tracking_value_ids.old_value_char', 'ilike', self),                     '|', ('tracking_value_ids.new_value_char', 'ilike', self),                     '|', ('tracking_value_ids.old_value_text', 'ilike', self),                          ('tracking_value_ids.new_value_text', 'ilike', self),                 ]",
          "name": "tracking_value_ids",
          "string": "Field Value"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "filter_domain": "[('tracking_value_ids.field_id', 'ilike', self)]",
          "name": "tracking_value_ids",
          "string": "Field"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('tracking_value_ids', '!=', False)]",
          "groups": "base.group_system",
          "name": "update_only",
          "string": "Update"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('tracking_value_ids', '=', False)]",
          "groups": "base.group_system",
          "name": "create_only",
          "string": "Create"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('account_audit_log_restricted', '=', True)]",
          "name": "restricted_by_audit_trail",
          "string": "Restricted"
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
          "string": "Date"
        }
      },
      {
        "tag": "group",
        "children": [
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'date'}",
              "domain": "[]",
              "name": "group_by_date",
              "string": "Date"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'res_id'}",
              "domain": "[]",
              "name": "group_by_res_id",
              "string": "Updated Data"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "account_audit_log_move_id"
    },
    {
      "name": "account_audit_log_account_id"
    },
    {
      "name": "account_audit_log_tax_id"
    },
    {
      "name": "account_audit_log_partner_id"
    },
    {
      "name": "account_audit_log_company_id"
    },
    {
      "name": "author_id"
    },
    {
      "name": "date"
    },
    {
      "name": "tracking_value_ids",
      "string": "Field Value",
      "attrs": {
        "filter_domain": "[                     '|', ('tracking_value_ids.old_value_char', 'ilike', self),                     '|', ('tracking_value_ids.new_value_char', 'ilike', self),                     '|', ('tracking_value_ids.old_value_text', 'ilike', self),                          ('tracking_value_ids.new_value_text', 'ilike', self),                 ]"
      }
    },
    {
      "name": "tracking_value_ids",
      "string": "Field",
      "attrs": {
        "filter_domain": "[('tracking_value_ids.field_id', 'ilike', self)]"
      }
    }
  ]
}

export function renderMailMessageSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
