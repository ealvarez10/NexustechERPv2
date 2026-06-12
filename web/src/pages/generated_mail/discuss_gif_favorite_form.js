// Generado por odoo2rs — vista form de discuss.gif.favorite (discuss_gif_favorite_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "discuss_gif_favorite_view_form",
  "name": "discuss.gif.favorite.form",
  "model": "discuss.gif.favorite",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "GIF favorite"
    },
    "children": [
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "group",
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
                  "name": "tenor_gif_id"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "create_uid"
                }
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
      "name": "tenor_gif_id"
    },
    {
      "name": "create_uid"
    }
  ]
}

export function renderDiscussGifFavoriteForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'discuss.gif.favorite' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/discuss.gif.favorite/<método> (≈ call_kw)
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
