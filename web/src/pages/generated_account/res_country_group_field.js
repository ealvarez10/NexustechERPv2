// Generado por odoo2rs — vista field de res.country.group (country_group_form_inherit_account).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "country_group_form_inherit_account",
  "name": "res.country.group.form.inherit.account",
  "model": "res.country.group",
  "type": "field",
  "arch": {
    "tag": "field",
    "attrs": {
      "name": "country_ids",
      "position": "after"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "exclude_state_ids",
          "options": "{'no_open': True, 'no_create': True}",
          "widget": "many2many_tags"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "exclude_state_ids",
      "widget": "many2many_tags",
      "attrs": {
        "options": "{'no_open': True, 'no_create': True}"
      }
    }
  ]
}

export function renderResCountryGroupField(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
