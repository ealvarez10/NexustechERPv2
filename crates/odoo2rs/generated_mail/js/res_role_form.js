// Generado por odoo2rs — vista form de res.role (res_role_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "res_role_view_form",
  "name": "res.role.form",
  "model": "res.role",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Role"
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
                  "name": "name",
                  "string": "Role"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "user_ids",
                  "options": "{'no_create': True}",
                  "widget": "many2many_avatar_user"
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
      "name": "name",
      "string": "Role"
    },
    {
      "name": "user_ids",
      "widget": "many2many_avatar_user",
      "attrs": {
        "options": "{'no_create': True}"
      }
    }
  ]
}

export function renderResRoleForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'res.role' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/res.role/<método> (≈ call_kw)
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
