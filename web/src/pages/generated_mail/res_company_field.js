// Generado por odoo2rs — vista field de res.company (res_company_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "res_company_view_form",
  "name": "res.company.view.form.inherit.mail",
  "model": "res.company",
  "type": "field",
  "arch": {
    "tag": "field",
    "attrs": {
      "name": "parent_id",
      "position": "after"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "alias_domain_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_no_one",
          "name": "bounce_formatted"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_no_one",
          "name": "catchall_formatted"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_no_one",
          "name": "default_from_email"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "alias_domain_id"
    },
    {
      "name": "bounce_formatted",
      "attrs": {
        "groups": "base.group_no_one"
      }
    },
    {
      "name": "catchall_formatted",
      "attrs": {
        "groups": "base.group_no_one"
      }
    },
    {
      "name": "default_from_email",
      "attrs": {
        "groups": "base.group_no_one"
      }
    }
  ]
}

export function renderResCompanyField(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
