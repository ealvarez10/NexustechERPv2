// Generado por odoo2rs — vista xpath de digest.digest (digest_digest_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "digest_digest_view_form",
  "name": "digest.digest.view.form.inherit.account.account",
  "model": "digest.digest",
  "type": "xpath",
  "arch": {
    "tag": "xpath",
    "attrs": {
      "expr": "//group[@name='kpis']/group[last()]",
      "position": "before"
    },
    "children": [
      {
        "tag": "group",
        "attrs": {
          "groups": "account.group_account_manager",
          "name": "kpi_account",
          "string": "Invoicing"
        },
        "children": [
          {
            "tag": "field",
            "attrs": {
              "name": "kpi_account_total_revenue"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "kpi_account_total_revenue"
    }
  ]
}

export function renderDigestDigestXpath(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
