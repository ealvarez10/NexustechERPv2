// Generado por odoo2rs — vista xpath de mail.activity (mail_activity_view_tree_open_target).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_activity_view_tree_open_target",
  "name": "mail.activity.view.list.open.target",
  "model": "mail.activity",
  "type": "xpath",
  "arch": {
    "tag": "xpath",
    "attrs": {
      "expr": "//list",
      "position": "attributes"
    },
    "children": [
      {
        "tag": "attribute",
        "attrs": {
          "name": "action"
        },
        "text": "action_open_document"
      },
      {
        "tag": "attribute",
        "attrs": {
          "name": "type"
        },
        "text": "object"
      },
      {
        "tag": "attribute",
        "attrs": {
          "name": "js_class"
        },
        "text": "archive_disabled_activity_list"
      },
      {
        "tag": "attribute",
        "attrs": {
          "name": "multi_edit"
        },
        "text": "1"
      }
    ]
  },
  "fields": []
}

export function renderMailActivityXpath(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
