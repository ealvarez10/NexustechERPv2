// Generado por odoo2rs — vista tree de account.account (view_account_list).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_list",
  "name": "account.account.list",
  "model": "account.account",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "create": "1",
      "delete": "1",
      "multi_edit": "1",
      "string": "Chart of accounts"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "1",
          "name": "placeholder_code"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "code",
          "options": "{'placeholder_field': 'placeholder_code'}",
          "string": "Code",
          "widget": "char_with_placeholder_field"
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
          "name": "account_type",
          "widget": "account_type_selection"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "group_id",
          "optional": "hide"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "internal_group"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "invisible": "account_type in ('asset_cash', 'liability_credit_card', 'off_balance')",
          "name": "reconcile",
          "widget": "boolean_toggle"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "active",
          "optional": "hide",
          "widget": "boolean_toggle"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "invisible": "account_type not in ('liability_payable', 'asset_receivable')",
          "name": "non_trade",
          "optional": "hide",
          "widget": "boolean_toggle"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "tax_ids",
          "optional": "hide",
          "widget": "many2many_tax_tags"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "domain": "[('applicability', '=', 'accounts')]",
          "name": "tag_ids",
          "optional": "hide",
          "widget": "many2many_tags"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_multi_currency",
          "name": "currency_id",
          "options": "{'no_create': True}"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_multi_company",
          "name": "company_ids",
          "readonly": "True",
          "widget": "many2many_tags"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "placeholder_code",
      "attrs": {
        "column_invisible": "1"
      }
    },
    {
      "name": "code",
      "string": "Code",
      "widget": "char_with_placeholder_field",
      "attrs": {
        "options": "{'placeholder_field': 'placeholder_code'}"
      }
    },
    {
      "name": "name"
    },
    {
      "name": "account_type",
      "widget": "account_type_selection"
    },
    {
      "name": "group_id",
      "attrs": {
        "optional": "hide"
      }
    },
    {
      "name": "internal_group",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "reconcile",
      "widget": "boolean_toggle",
      "attrs": {
        "invisible": "account_type in ('asset_cash', 'liability_credit_card', 'off_balance')"
      }
    },
    {
      "name": "active",
      "widget": "boolean_toggle",
      "attrs": {
        "optional": "hide"
      }
    },
    {
      "name": "non_trade",
      "widget": "boolean_toggle",
      "attrs": {
        "invisible": "account_type not in ('liability_payable', 'asset_receivable')",
        "optional": "hide"
      }
    },
    {
      "name": "tax_ids",
      "widget": "many2many_tax_tags",
      "attrs": {
        "optional": "hide"
      }
    },
    {
      "name": "tag_ids",
      "widget": "many2many_tags",
      "attrs": {
        "domain": "[('applicability', '=', 'accounts')]",
        "optional": "hide"
      }
    },
    {
      "name": "currency_id",
      "attrs": {
        "groups": "base.group_multi_currency",
        "options": "{'no_create': True}"
      }
    },
    {
      "name": "company_ids",
      "widget": "many2many_tags",
      "attrs": {
        "groups": "base.group_multi_company",
        "readonly": "True"
      }
    }
  ]
}

export function renderAccountAccountTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
