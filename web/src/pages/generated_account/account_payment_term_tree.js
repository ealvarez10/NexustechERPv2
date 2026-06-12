// Generado por odoo2rs — vista tree de account.payment.term (view_payment_term_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_payment_term_tree",
  "name": "account.payment.term.list",
  "model": "account.payment.term",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Payment Terms"
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
          "groups": "base.group_multi_company",
          "name": "company_id"
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
      "name": "company_id",
      "attrs": {
        "groups": "base.group_multi_company"
      }
    }
  ]
}

export function renderAccountPaymentTermTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
