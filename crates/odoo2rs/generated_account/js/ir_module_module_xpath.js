// Generado por odoo2rs — vista xpath de ir.module.module (view_module_filter_inherit_account).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_module_filter_inherit_account",
  "name": "ir.module.module.list.inherit.account",
  "model": "ir.module.module",
  "type": "xpath",
  "arch": {
    "tag": "xpath",
    "attrs": {
      "expr": "//searchpanel",
      "position": "replace"
    }
  },
  "fields": []
}

export function renderIrModuleModuleXpath(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
