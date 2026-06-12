// Generado por odoo2rs — vista tree de account.reconcile.model (view_account_reconcile_model_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_reconcile_model_tree",
  "name": "account.reconcile.model.list",
  "model": "account.reconcile.model",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Bank Reconciliation Move Presets"
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
          "name": "trigger"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "match_journal_ids",
          "optional": "hidden",
          "widget": "many2many_tags"
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
      "name": "trigger"
    },
    {
      "name": "match_journal_ids",
      "widget": "many2many_tags",
      "attrs": {
        "optional": "hidden"
      }
    }
  ]
}

export function renderAccountReconcileModelTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
