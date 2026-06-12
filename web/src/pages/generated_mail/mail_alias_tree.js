// Generado por odoo2rs — vista tree de mail.alias (mail_alias_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_alias_view_tree",
  "name": "mail.alias.view.list",
  "model": "mail.alias",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Alias"
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
          "name": "alias_force_thread_id",
          "optional": "hide"
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
          "name": "alias_parent_thread_id",
          "optional": "hide"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "alias_defaults",
          "optional": "hide"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "alias_contact"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "alias_incoming_local",
          "optional": "hide"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "decoration-danger": "alias_status == 'invalid'",
          "decoration-success": "alias_status == 'valid'",
          "decoration-warning": "alias_status == 'not_tested'",
          "name": "alias_status",
          "widget": "badge"
        }
      },
      {
        "tag": "button",
        "attrs": {
          "icon": "fa-sitemap",
          "invisible": "not alias_model_id or alias_force_thread_id == 0",
          "name": "open_document",
          "string": "Open Document",
          "type": "object"
        }
      },
      {
        "tag": "button",
        "attrs": {
          "icon": "fa-sitemap",
          "invisible": "not alias_parent_model_id or alias_parent_thread_id == 0",
          "name": "open_parent_document",
          "string": "Open Owner",
          "type": "object"
        }
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
      "name": "alias_force_thread_id",
      "attrs": {
        "optional": "hide"
      }
    },
    {
      "name": "alias_parent_model_id"
    },
    {
      "name": "alias_parent_thread_id",
      "attrs": {
        "optional": "hide"
      }
    },
    {
      "name": "alias_defaults",
      "attrs": {
        "optional": "hide"
      }
    },
    {
      "name": "alias_contact"
    },
    {
      "name": "alias_incoming_local",
      "attrs": {
        "optional": "hide"
      }
    },
    {
      "name": "alias_status",
      "widget": "badge",
      "attrs": {
        "decoration-danger": "alias_status == 'invalid'",
        "decoration-success": "alias_status == 'valid'",
        "decoration-warning": "alias_status == 'not_tested'"
      }
    }
  ],
  "buttons": [
    {
      "name": "open_document",
      "string": "Open Document",
      "type": "object"
    },
    {
      "name": "open_parent_document",
      "string": "Open Owner",
      "type": "object"
    }
  ]
}

export function renderMailAliasTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
