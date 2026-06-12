// Generado por odoo2rs — vista search de account.reconcile.model (view_account_reconcile_model_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_reconcile_model_search",
  "name": "account.reconcile.model.search",
  "model": "account.reconcile.model",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Bank Reconciliation Move preset"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "name"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('trigger', '=', 'auto_reconcile')]",
          "name": "auto_validate",
          "string": "Automated"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('line_ids.tax_ids', '!=', False)]",
          "name": "withtax",
          "string": "With tax"
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
      },
      {
        "tag": "group",
        "children": [
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'match_journal_ids'}",
              "name": "group_by_journal",
              "string": "Journals Availability"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'trigger'}",
              "name": "group_by_auto_validate",
              "string": "Automation"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "name"
    }
  ]
}

export function renderAccountReconcileModelSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
