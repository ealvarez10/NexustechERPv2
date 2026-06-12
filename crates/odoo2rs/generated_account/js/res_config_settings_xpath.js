// Generado por odoo2rs — vista xpath de res.config.settings (res_config_settings_view_form_base_setup).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "res_config_settings_view_form_base_setup",
  "name": "res.config.settings.view.form.inherit.base_setup",
  "model": "res.config.settings",
  "type": "xpath",
  "arch": {
    "tag": "xpath",
    "attrs": {
      "expr": "//button[@name='%(web.action_base_document_layout_configurator)d']",
      "position": "attributes"
    },
    "children": [
      {
        "tag": "attribute",
        "attrs": {
          "name": "name"
        },
        "text": "%(account.action_base_document_layout_configurator)d"
      },
      {
        "tag": "attribute",
        "attrs": {
          "name": "context"
        },
        "text": "{'default_from_invoice': True}"
      }
    ]
  },
  "fields": []
}

export function renderResConfigSettingsXpath(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
