// Generado por odoo2rs — vista search de account.journal (view_account_journal_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_journal_search",
  "name": "account.journal.search",
  "model": "account.journal",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Search Account Journal"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "filter_domain": "['|', ('name', 'ilike', self), ('code', 'ilike', self)]",
          "name": "name",
          "string": "Journal"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "activity_user_id",
          "string": "Activities of"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "activity_type_id",
          "string": "Activity type"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('show_on_dashboard', '=', True)]",
          "name": "dashboard",
          "string": "Favorites"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('type', '=', 'sale')]",
          "name": "sales",
          "string": "Sales"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('type', '=', 'purchase')]",
          "name": "purchases",
          "string": "Purchases"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('type', 'in', ('cash', 'bank', 'credit'))]",
          "name": "liquidity",
          "string": "Liquidity"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('type', 'not in', ['sale', 'purchase', 'cash', 'bank', 'credit'])]",
          "name": "miscellaneous",
          "string": "Miscellaneous"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('active', '=', False)]",
          "name": "inactive",
          "string": "Archived"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "name",
      "string": "Journal",
      "attrs": {
        "filter_domain": "['|', ('name', 'ilike', self), ('code', 'ilike', self)]"
      }
    },
    {
      "name": "activity_user_id",
      "string": "Activities of"
    },
    {
      "name": "activity_type_id",
      "string": "Activity type"
    }
  ]
}

export function renderAccountJournalSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
