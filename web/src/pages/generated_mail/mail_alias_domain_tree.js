// Generado por odoo2rs — vista tree de mail.alias.domain (mail_alias_domain_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_alias_domain_view_tree",
  "name": "mail.alias.domain.view.list",
  "model": "mail.alias.domain",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "duplicate": "false",
      "string": "Alias Domains"
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
          "name": "default_from"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_multi_company",
          "name": "company_ids",
          "optional": "hide",
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
      "name": "bounce_alias"
    },
    {
      "name": "catchall_alias"
    },
    {
      "name": "default_from"
    },
    {
      "name": "company_ids",
      "widget": "many2many_tags",
      "attrs": {
        "groups": "base.group_multi_company",
        "optional": "hide"
      }
    }
  ]
}

export function renderMailAliasDomainTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
