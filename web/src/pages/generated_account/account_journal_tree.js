// Generado por odoo2rs — vista tree de account.journal (view_account_journal_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_journal_tree",
  "name": "account.journal.list",
  "model": "account.journal",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Account Journal"
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
          "name": "type"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "journal_group_ids",
          "optional": "hide",
          "readonly": "1",
          "widget": "many2many_tags"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_multi_currency",
          "name": "currency_id",
          "optional": "hide"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "code",
          "optional": "show"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "default_account_id",
          "optional": "show"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "active",
          "optional": "hide"
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
          "column_invisible": "True",
          "groups": "!base.group_multi_company",
          "name": "company_id"
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
      "name": "type"
    },
    {
      "name": "journal_group_ids",
      "widget": "many2many_tags",
      "attrs": {
        "optional": "hide",
        "readonly": "1"
      }
    },
    {
      "name": "currency_id",
      "attrs": {
        "groups": "base.group_multi_currency",
        "optional": "hide"
      }
    },
    {
      "name": "code",
      "attrs": {
        "optional": "show"
      }
    },
    {
      "name": "default_account_id",
      "attrs": {
        "optional": "show"
      }
    },
    {
      "name": "active",
      "attrs": {
        "optional": "hide"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "groups": "base.group_multi_company",
        "optional": "hide"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "column_invisible": "True",
        "groups": "!base.group_multi_company"
      }
    }
  ]
}

export function renderAccountJournalTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
