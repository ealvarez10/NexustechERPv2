// Generado por odoo2rs — vista tree de mail.guest (mail_guest_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_guest_view_tree",
  "name": "mail.guest.list",
  "model": "mail.guest",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Guests"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "name"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "country_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "lang"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "timezone"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "id"
    },
    {
      "name": "name"
    },
    {
      "name": "country_id"
    },
    {
      "name": "lang"
    },
    {
      "name": "timezone"
    }
  ]
}

export function renderMailGuestTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
