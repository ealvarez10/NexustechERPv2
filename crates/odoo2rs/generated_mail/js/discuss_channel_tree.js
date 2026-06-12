// Generado por odoo2rs — vista tree de discuss.channel (mail.discuss_channel_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail.discuss_channel_view_tree",
  "name": "discuss.channel.list",
  "model": "discuss.channel",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Groups"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "name"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "name"
    }
  ]
}

export function renderDiscussChannelTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
