// Generado por odoo2rs — vista form de mercadily.sync.log (mercadily_sync_log_form_view).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "mercadily_sync_log_form_view",
  "name": "mercadily.sync.log.form",
  "model": "mercadily.sync.log",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "create": "false",
      "edit": "false",
      "string": "Log de Sincronización"
    },
    "children": [
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "group",
            "children": [
              {
                "tag": "group",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "backend_id"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "sync_type"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "create_date"
                    }
                  }
                ]
              },
              {
                "tag": "group",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "created_count"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "updated_count"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "error_count"
                    }
                  }
                ]
              }
            ]
          },
          {
            "tag": "group",
            "attrs": {
              "invisible": "not error_details",
              "string": "Detalle de Errores"
            },
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "name": "error_details",
                  "nolabel": "1"
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
      "name": "backend_id"
    },
    {
      "name": "sync_type"
    },
    {
      "name": "create_date"
    },
    {
      "name": "created_count"
    },
    {
      "name": "updated_count"
    },
    {
      "name": "error_count"
    },
    {
      "name": "error_details",
      "attrs": {
        "nolabel": "1"
      }
    }
  ]
}

export function renderMercadilySyncLogForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mercadily.sync.log' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mercadily.sync.log/<método> (≈ call_kw)
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
