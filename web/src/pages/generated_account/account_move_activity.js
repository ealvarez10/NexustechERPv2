// Generado por odoo2rs — vista activity de account.move (account_move_view_activity).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "account_move_view_activity",
  "name": "account.move.view.activity",
  "model": "account.move",
  "type": "activity",
  "arch": {
    "tag": "activity",
    "attrs": {
      "string": "Account Entry"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "currency_id"
        }
      },
      {
        "tag": "templates",
        "children": [
          {
            "tag": "div",
            "attrs": {
              "class": "d-block",
              "t-name": "activity-box"
            },
            "children": [
              {
                "tag": "div",
                "attrs": {
                  "class": "d-flex justify-content-between"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "o_text_block o_text_bold",
                      "name": "name",
                      "string": "Entry Name"
                    }
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "m-1"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "amount_total",
                      "widget": "monetary"
                    }
                  }
                ]
              },
              {
                "tag": "div",
                "attrs": {
                  "class": "d-flex justify-content-between"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "o_text_block",
                      "name": "commercial_partner_id",
                      "string": "Commercial Entity"
                    }
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "m-1"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "decoration-info": "state == 'draft'",
                      "decoration-success": "state == 'posted'",
                      "name": "state",
                      "widget": "badge"
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
      "name": "currency_id"
    },
    {
      "name": "name",
      "string": "Entry Name",
      "attrs": {
        "class": "o_text_block o_text_bold"
      }
    },
    {
      "name": "amount_total",
      "widget": "monetary"
    },
    {
      "name": "commercial_partner_id",
      "string": "Commercial Entity",
      "attrs": {
        "class": "o_text_block"
      }
    },
    {
      "name": "state",
      "widget": "badge",
      "attrs": {
        "decoration-info": "state == 'draft'",
        "decoration-success": "state == 'posted'"
      }
    }
  ]
}

export function renderAccountMoveActivity(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
