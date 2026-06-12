// Generado por odoo2rs — vista search de mail.alias (mail_alias_view_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_alias_view_search",
  "name": "mail.alias.view.search",
  "model": "mail.alias",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Search Alias"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "alias_name"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "alias_domain_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "alias_model_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "create_uid"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "alias_force_thread_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "alias_parent_model_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "alias_parent_thread_id"
        }
      },
      {
        "tag": "separator"
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('alias_name', '!=', False)]",
          "name": "active",
          "string": "Active"
        }
      },
      {
        "tag": "group",
        "children": [
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'create_uid'}",
              "name": "groupby_create_uid",
              "string": "Creator"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'alias_domain_id'}",
              "name": "groupby_alias_domain_id",
              "string": "Alias Domain"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'alias_model_id'}",
              "name": "groupby_alias_model_id",
              "string": "Document Model"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'alias_parent_model_id'}",
              "name": "groupby_alias_model_id",
              "string": "Container Model"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "alias_name"
    },
    {
      "name": "alias_domain_id"
    },
    {
      "name": "alias_model_id"
    },
    {
      "name": "create_uid"
    },
    {
      "name": "alias_force_thread_id"
    },
    {
      "name": "alias_parent_model_id"
    },
    {
      "name": "alias_parent_thread_id"
    }
  ]
}

export function renderMailAliasSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
