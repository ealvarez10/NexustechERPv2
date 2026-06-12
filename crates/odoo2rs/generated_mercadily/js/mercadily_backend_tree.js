// Generado por odoo2rs — vista tree de mercadily.backend (mercadily_backend_list_view).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mercadily_backend_list_view",
  "name": "mercadily.backend.list",
  "model": "mercadily.backend",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Backends Mercadily"
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
          "name": "api_url",
          "widget": "url"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "groups": "base.group_multi_company",
          "name": "company_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "last_lead_sync"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "last_order_sync"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "last_customer_sync"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "name"
    },
    {
      "name": "api_url",
      "widget": "url"
    },
    {
      "name": "company_id",
      "attrs": {
        "groups": "base.group_multi_company"
      }
    },
    {
      "name": "last_lead_sync"
    },
    {
      "name": "last_order_sync"
    },
    {
      "name": "last_customer_sync"
    }
  ]
}

export function renderMercadilyBackendTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
