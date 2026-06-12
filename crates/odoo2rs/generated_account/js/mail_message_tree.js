// Generado por odoo2rs — vista tree de mail.message (view_message_tree_audit_log).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_message_tree_audit_log",
  "name": "mail.message.list.inherit.audit.log",
  "model": "mail.message",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "action": "action_open_document",
      "create": "0",
      "delete": "0",
      "edit": "0",
      "type": "object"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "date"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "author_id",
          "widget": "many2one_avatar"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "res_id",
          "string": "Name"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "account_audit_log_preview"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "date"
    },
    {
      "name": "author_id",
      "widget": "many2one_avatar"
    },
    {
      "name": "res_id",
      "string": "Name"
    },
    {
      "name": "account_audit_log_preview"
    }
  ]
}

export function renderMailMessageTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
