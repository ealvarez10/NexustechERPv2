// Generado por odoo2rs — vista tree de account.group (view_account_group_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_account_group_tree",
  "name": "account.group.list",
  "model": "account.group",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Account Group"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "code_prefix_start"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "code_prefix_end"
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
          "groups": "base.group_multi_company",
          "name": "company_id"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "code_prefix_start"
    },
    {
      "name": "code_prefix_end"
    },
    {
      "name": "name"
    },
    {
      "name": "company_id",
      "attrs": {
        "groups": "base.group_multi_company"
      }
    }
  ]
}

export function renderAccountGroupTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
