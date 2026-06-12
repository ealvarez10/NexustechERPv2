// Generado por odoo2rs — vista tree de account.journal.group (view_account_journal_group_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_journal_group_tree",
  "name": "account.journal.group.list",
  "model": "account.journal.group",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "editable": "bottom"
    },
    "children": [
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
          "name": "sequence",
          "widget": "handle"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "name",
          "placeholder": "e.g. GAAP, IFRS, ..."
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "excluded_journal_ids",
          "options": "{'no_create': True}",
          "widget": "many2many_tags_journals"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_multi_company",
          "name": "company_id",
          "options": "{'no_create': True}",
          "placeholder": "Visible to all"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "company_id",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "sequence",
      "widget": "handle"
    },
    {
      "name": "name",
      "attrs": {
        "placeholder": "e.g. GAAP, IFRS, ..."
      }
    },
    {
      "name": "excluded_journal_ids",
      "widget": "many2many_tags_journals",
      "attrs": {
        "options": "{'no_create': True}"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "groups": "base.group_multi_company",
        "options": "{'no_create': True}",
        "placeholder": "Visible to all"
      }
    }
  ]
}

export function renderAccountJournalGroupTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
