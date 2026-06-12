// Generado por odoo2rs — vista search de mail.canned.response (mail_canned_response_view_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_canned_response_view_search",
  "name": "mail.canned.response.view.search",
  "model": "mail.canned.response",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Canned Responses Search"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "source"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "substitution"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('is_shared', '=', False)]",
          "name": "filter_create_uid",
          "string": "Private"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('is_shared', '=', True)]",
          "name": "filter_is_shared",
          "string": "Shared"
        }
      },
      {
        "tag": "group",
        "children": [
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'group_ids'}",
              "name": "group_by_group_ids",
              "string": "Authorized Groups"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "source"
    },
    {
      "name": "substitution"
    }
  ]
}

export function renderMailCannedResponseSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
