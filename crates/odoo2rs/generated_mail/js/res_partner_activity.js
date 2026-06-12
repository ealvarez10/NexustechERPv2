// Generado por odoo2rs — vista activity de res.partner (res_partner_view_activity).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "res_partner_view_activity",
  "name": "res.partner.activity",
  "model": "res.partner",
  "type": "activity",
  "arch": {
    "tag": "activity",
    "attrs": {
      "string": "Contacts"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "id"
        }
      },
      {
        "tag": "templates",
        "children": [
          {
            "tag": "div",
            "attrs": {
              "t-name": "activity-box"
            },
            "children": [
              {
                "tag": "img",
                "attrs": {
                  "role": "img",
                  "t-att-alt": "record.name.value",
                  "t-att-src": "activity_image('res.partner', 'avatar_128', record.id.raw_value)",
                  "t-att-title": "record.name.value"
                }
              },
              {
                "tag": "div",
                "attrs": {
                  "class": "ms-2"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "o_text_block",
                      "display": "full",
                      "name": "name"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "o_text_block",
                      "display": "full",
                      "muted": "1",
                      "name": "parent_id"
                    }
                  }
                ]
              }
            ]
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "id"
    },
    {
      "name": "name",
      "attrs": {
        "class": "o_text_block",
        "display": "full"
      }
    },
    {
      "name": "parent_id",
      "attrs": {
        "class": "o_text_block",
        "display": "full",
        "muted": "1"
      }
    }
  ]
}

export function renderResPartnerActivity(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
