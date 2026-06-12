// Generado por odoo2rs — vista tree de account.tax.group (view_tax_group_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_tax_group_tree",
  "name": "account.tax.group.list",
  "model": "account.tax.group",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "editable": "bottom",
      "open_form_view": "True",
      "string": "Account Tax Group"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "sequence",
          "widget": "handle"
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
          "name": "country_id"
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
          "name": "company_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "country_code"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "tax_payable_account_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "tax_receivable_account_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "advance_tax_payment_account_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "preceding_subtotal",
          "optional": "hide"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "sequence",
      "widget": "handle"
    },
    {
      "name": "name"
    },
    {
      "name": "country_id"
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
        "groups": "base.group_multi_company"
      }
    },
    {
      "name": "country_code",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "tax_payable_account_id"
    },
    {
      "name": "tax_receivable_account_id"
    },
    {
      "name": "advance_tax_payment_account_id"
    },
    {
      "name": "preceding_subtotal",
      "attrs": {
        "optional": "hide"
      }
    }
  ]
}

export function renderAccountTaxGroupTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
