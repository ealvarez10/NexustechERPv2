// Generado por odoo2rs — vista field de ir.model (model_search_view).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "model_search_view",
  "model": "ir.model",
  "type": "field",
  "arch": {
    "tag": "field",
    "attrs": {
      "name": "model",
      "position": "after"
    },
    "children": [
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('is_mail_thread', '=', True)]",
          "name": "is_mail_thread",
          "string": "Mail Thread"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('is_mail_activity', '=', True)]",
          "name": "is_mail_activity",
          "string": "Mail Activity"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('is_mail_blacklist', '=', True)]",
          "name": "is_mail_blacklist",
          "string": "Mail Blacklist"
        }
      }
    ]
  },
  "fields": []
}

export function renderIrModelField(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
