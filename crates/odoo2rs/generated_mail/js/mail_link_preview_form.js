// Generado por odoo2rs — vista form de mail.link.preview (mail_link_preview_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail_link_preview_view_form",
  "name": "mail.link.preview.form",
  "model": "mail.link.preview",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Link Previews"
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
                      "name": "source_url"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "og_type"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "og_title"
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
                      "name": "og_image"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "og_image",
                      "nolabel": "1",
                      "options": "{'size': [150, 150]}",
                      "widget": "image_url"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "og_mimetype"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "image_mimetype"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "create_date"
                    }
                  }
                ]
              }
            ]
          },
          {
            "tag": "label",
            "attrs": {
              "for": "og_description"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "name": "og_description"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "name": "message_link_preview_ids"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "source_url"
    },
    {
      "name": "og_type"
    },
    {
      "name": "og_title"
    },
    {
      "name": "og_image"
    },
    {
      "name": "og_image",
      "widget": "image_url",
      "attrs": {
        "nolabel": "1",
        "options": "{'size': [150, 150]}"
      }
    },
    {
      "name": "og_mimetype"
    },
    {
      "name": "image_mimetype"
    },
    {
      "name": "create_date"
    },
    {
      "name": "og_description"
    },
    {
      "name": "message_link_preview_ids"
    }
  ]
}

export function renderMailLinkPreviewForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.link.preview' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.link.preview/<método> (≈ call_kw)
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
