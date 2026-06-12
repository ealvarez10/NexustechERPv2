// Generado por odoo2rs — vista filter de res.partner (res_partner_view_search_inherit_mail).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "res_partner_view_search_inherit_mail",
  "name": "res.partner.view.search.inherit.mail",
  "model": "res.partner",
  "type": "filter",
  "arch": {
    "tag": "filter",
    "attrs": {
      "name": "inactive",
      "position": "after"
    },
    "children": [
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('activity_user_id', '=', uid)]",
          "invisible": "1",
          "name": "filter_activities_my",
          "string": "My Activities"
        }
      },
      {
        "tag": "separator",
        "attrs": {
          "invisible": "1"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('my_activity_date_deadline', '<', 'today')]",
          "help": "Show all records whose next activity date is past",
          "invisible": "1",
          "name": "activities_overdue",
          "string": "Late Activities"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('my_activity_date_deadline', '=', 'today')]",
          "invisible": "1",
          "name": "activities_today",
          "string": "Today Activities"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('my_activity_date_deadline', '>', 'today')]",
          "invisible": "1",
          "name": "activities_upcoming_all",
          "string": "Future Activities"
        }
      },
      {
        "tag": "separator"
      }
    ]
  },
  "fields": []
}

export function renderResPartnerFilter(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
