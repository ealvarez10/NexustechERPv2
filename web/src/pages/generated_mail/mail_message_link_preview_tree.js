// Generado por odoo2rs — vista tree de mail.message.link.preview (message_link_preview_list).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "message_link_preview_list",
  "name": "mail.message.link.preview.list",
  "model": "mail.message.link.preview",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "messages"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "author_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "is_hidden"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "author_id"
    },
    {
      "name": "is_hidden"
    }
  ]
}

export function renderMailMessageLinkPreviewTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
