// Generado por odoo2rs — vista search de mail.alias.domain (mail_alias_domain_view_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_alias_domain_view_search",
  "name": "mail.alias.domain.view.search",
  "model": "mail.alias.domain",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Alias Domains"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "name"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "bounce_alias"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "catchall_alias"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_multi_company",
          "name": "company_ids"
        }
      },
      {
        "tag": "group",
        "children": [
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by': 'company_ids'}",
              "domain": "[]",
              "groups": "base.group_multi_company",
              "name": "group_by_company_ids",
              "string": "Company"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "name"
    },
    {
      "name": "bounce_alias"
    },
    {
      "name": "catchall_alias"
    },
    {
      "name": "company_ids",
      "attrs": {
        "groups": "base.group_multi_company"
      }
    }
  ]
}

export function renderMailAliasDomainSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
