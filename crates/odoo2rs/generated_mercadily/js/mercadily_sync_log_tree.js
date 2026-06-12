// Generado por odoo2rs — vista tree de mercadily.sync.log (mercadily_sync_log_list_view).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mercadily_sync_log_list_view",
  "name": "mercadily.sync.log.list",
  "model": "mercadily.sync.log",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "create": "false",
      "edit": "false",
      "string": "Logs de Sincronización"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "create_date"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "backend_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "sync_type"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "created_count"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "updated_count"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "decoration-danger": "error_count > 0",
          "name": "error_count"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "create_date"
    },
    {
      "name": "backend_id"
    },
    {
      "name": "sync_type"
    },
    {
      "name": "created_count"
    },
    {
      "name": "updated_count"
    },
    {
      "name": "error_count",
      "attrs": {
        "decoration-danger": "error_count > 0"
      }
    }
  ]
}

export function renderMercadilySyncLogTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
