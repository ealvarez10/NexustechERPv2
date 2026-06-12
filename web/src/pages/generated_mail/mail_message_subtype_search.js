// Generado por odoo2rs — vista search de mail.message.subtype (mail_message_subtype_view_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_message_subtype_view_search",
  "name": "mail.message.subtype.view.search",
  "model": "mail.message.subtype",
  "type": "search",
  "arch": {
    "tag": "search",
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
          "name": "res_model"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "description"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('default', '=', True)]",
          "name": "filter_default",
          "string": "Default"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "name"
    },
    {
      "name": "res_model"
    },
    {
      "name": "description"
    }
  ]
}

export function renderMailMessageSubtypeSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
