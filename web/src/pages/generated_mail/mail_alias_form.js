// Generado por odoo2rs — vista form de mail.alias (mail_alias_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail_alias_view_form",
  "name": "mail.alias.view.form",
  "model": "mail.alias",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Alias"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "invisible": "1",
          "name": "alias_status"
        }
      },
      {
        "tag": "div",
        "attrs": {
          "class": "alert alert-danger text-center",
          "invisible": "alias_status != 'invalid'",
          "role": "alert"
        },
        "text": "The last message received on this alias has caused an error."
      },
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "div",
            "attrs": {
              "class": "oe_button_box",
              "name": "button_box"
            },
            "children": [
              {
                "tag": "button",
                "attrs": {
                  "class": "oe_link",
                  "icon": "fa-sitemap",
                  "invisible": "not alias_model_id or alias_force_thread_id == 0",
                  "name": "open_document",
                  "string": "Open Document",
                  "type": "object"
                }
              },
              {
                "tag": "button",
                "attrs": {
                  "class": "oe_link",
                  "icon": "fa-sitemap",
                  "invisible": "not alias_parent_model_id or alias_parent_thread_id == 0",
                  "name": "open_parent_document",
                  "type": "object"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "o_field_widget o_stat_info"
                    },
                    "children": [
                      {
                        "tag": "span",
                        "attrs": {
                          "class": "o_stat_text"
                        },
                        "text": "Open Parent Document"
                      }
                    ]
                  }
                ]
              }
            ]
          },
          {
            "tag": "div",
            "attrs": {
              "class": "d-flex"
            },
            "children": [
              {
                "tag": "h2",
                "attrs": {
                  "class": "flex-grow-1",
                  "dir": "ltr"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "oe_inline",
                      "name": "alias_name",
                      "placeholder": "alias"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "oe_inline",
                      "name": "alias_domain_id",
                      "options": "{'no_create': True, 'no_open': True}",
                      "placeholder": "e.g. mycompany.com"
                    }
                  }
                ],
                "text": "@"
              },
              {
                "tag": "field",
                "attrs": {
                  "decoration-danger": "alias_status == 'invalid'",
                  "decoration-success": "alias_status == 'valid'",
                  "decoration-warning": "alias_status == 'not_tested'",
                  "name": "alias_status",
                  "widget": "badge"
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
                  "name": "alias_model_id"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "alias_force_thread_id"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "alias_defaults"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "alias_contact"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "alias_incoming_local"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "alias_parent_model_id"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "alias_parent_thread_id"
                }
              }
            ]
          },
          {
            "tag": "label",
            "attrs": {
              "for": "alias_bounced_content",
              "invisible": "alias_contact == 'everyone'"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "alias_contact == 'everyone'",
              "name": "alias_bounced_content"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "alias_status",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "alias_name",
      "attrs": {
        "class": "oe_inline",
        "placeholder": "alias"
      }
    },
    {
      "name": "alias_domain_id",
      "attrs": {
        "class": "oe_inline",
        "options": "{'no_create': True, 'no_open': True}",
        "placeholder": "e.g. mycompany.com"
      }
    },
    {
      "name": "alias_status",
      "widget": "badge",
      "attrs": {
        "decoration-danger": "alias_status == 'invalid'",
        "decoration-success": "alias_status == 'valid'",
        "decoration-warning": "alias_status == 'not_tested'"
      }
    },
    {
      "name": "alias_model_id"
    },
    {
      "name": "alias_force_thread_id"
    },
    {
      "name": "alias_defaults"
    },
    {
      "name": "alias_contact"
    },
    {
      "name": "alias_incoming_local"
    },
    {
      "name": "alias_parent_model_id"
    },
    {
      "name": "alias_parent_thread_id"
    },
    {
      "name": "alias_bounced_content",
      "attrs": {
        "invisible": "alias_contact == 'everyone'"
      }
    }
  ],
  "buttons": [
    {
      "name": "open_document",
      "string": "Open Document",
      "type": "object",
      "class": "oe_link"
    },
    {
      "name": "open_parent_document",
      "type": "object",
      "class": "oe_link"
    }
  ]
}

export function renderMailAliasForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.alias' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.alias/<método> (≈ call_kw)
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
