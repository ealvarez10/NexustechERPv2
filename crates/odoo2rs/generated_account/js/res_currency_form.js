// Generado por odoo2rs — vista form de res.currency (res_currency_form_inherit).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "res_currency_form_inherit",
  "name": "res.currency.form.inherit",
  "model": "res.currency",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "position": "attributes"
    },
    "children": [
      {
        "tag": "attribute",
        "attrs": {
          "name": "js_class"
        },
        "text": "currency_form"
      }
    ]
  },
  "fields": []
}

export function renderResCurrencyForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'res.currency' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/res.currency/<método> (≈ call_kw)
      onClick: `alert('TODO: ${b.name}')`,
    })) || [],
    fieldGroups: [{
      fields: DESCRIPTOR.fields.map(f => ({
        label: f.string || f.name,
        value: record[f.name] ?? '',
      })),
    }],
    id: record.id || '',
  })
}
