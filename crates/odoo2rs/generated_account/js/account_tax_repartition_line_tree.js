// Generado por odoo2rs — vista tree de account.tax.repartition.line (tax_repartition_line_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "tax_repartition_line_tree",
  "name": "account.tax.repartition.line.list",
  "model": "account.tax.repartition.line",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "create": "1",
      "delete": "1",
      "editable": "bottom"
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
          "invisible": "repartition_type == 'base'",
          "name": "factor_percent",
          "widget": "account_tax_repartition_line_factor_percent"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "repartition_type"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "invisible": "repartition_type == 'base'",
          "name": "account_id",
          "options": "{'no_create': True}"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "domain": "tag_ids_domain",
          "name": "tag_ids",
          "options": "{'no_create': True}",
          "widget": "many2many_tags"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "invisible": "repartition_type == 'base'",
          "name": "use_in_tax_closing",
          "optional": "hidden"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "company_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "column_invisible": "True",
          "name": "tag_ids_domain"
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
      "name": "factor_percent",
      "widget": "account_tax_repartition_line_factor_percent",
      "attrs": {
        "invisible": "repartition_type == 'base'"
      }
    },
    {
      "name": "repartition_type"
    },
    {
      "name": "account_id",
      "attrs": {
        "invisible": "repartition_type == 'base'",
        "options": "{'no_create': True}"
      }
    },
    {
      "name": "tag_ids",
      "widget": "many2many_tags",
      "attrs": {
        "domain": "tag_ids_domain",
        "options": "{'no_create': True}"
      }
    },
    {
      "name": "use_in_tax_closing",
      "attrs": {
        "invisible": "repartition_type == 'base'",
        "optional": "hidden"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "column_invisible": "True"
      }
    },
    {
      "name": "tag_ids_domain",
      "attrs": {
        "column_invisible": "True"
      }
    }
  ]
}

export function renderAccountTaxRepartitionLineTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
