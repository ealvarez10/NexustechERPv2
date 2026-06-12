// Generado por odoo2rs — vista form de res.users.settings (res_users_settings_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "res_users_settings_view_form",
  "name": "res.users.settings.form",
  "model": "res.users.settings",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "User Settings"
    },
    "children": [
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "div",
            "attrs": {
              "class": "oe_title"
            },
            "children": [
              {
                "tag": "h1",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "user_id",
                      "readonly": "id != False"
                    }
                  }
                ]
              }
            ]
          },
          {
            "tag": "group",
            "attrs": {
              "name": "discuss_user_settings"
            },
            "children": [
              {
                "tag": "group",
                "attrs": {
                  "string": "Discuss sidebar"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "is_discuss_sidebar_category_channel_open"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "is_discuss_sidebar_category_chat_open"
                    }
                  }
                ]
              },
              {
                "tag": "group",
                "attrs": {
                  "string": "Voice"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "use_push_to_talk"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "not use_push_to_talk",
                      "name": "push_to_talk_key",
                      "placeholder": "e.g. true.true..f"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "use_push_to_talk",
                      "name": "voice_active_duration"
                    }
                  }
                ]
              }
            ]
          },
          {
            "tag": "notebook",
            "attrs": {
              "colspan": "4"
            },
            "children": [
              {
                "tag": "page",
                "attrs": {
                  "name": "page_volume_per_partner",
                  "string": "Volume per partner"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "volume_settings_ids"
                    },
                    "children": [
                      {
                        "tag": "list",
                        "attrs": {
                          "editable": "bottom"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "partner_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "volume"
                            }
                          }
                        ]
                      }
                    ]
                  }
                ]
              }
            ]
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "user_id",
      "attrs": {
        "readonly": "id != False"
      }
    },
    {
      "name": "is_discuss_sidebar_category_channel_open"
    },
    {
      "name": "is_discuss_sidebar_category_chat_open"
    },
    {
      "name": "use_push_to_talk"
    },
    {
      "name": "push_to_talk_key",
      "attrs": {
        "invisible": "not use_push_to_talk",
        "placeholder": "e.g. true.true..f"
      }
    },
    {
      "name": "voice_active_duration",
      "attrs": {
        "invisible": "use_push_to_talk"
      }
    },
    {
      "name": "volume_settings_ids"
    },
    {
      "name": "partner_id"
    },
    {
      "name": "volume"
    }
  ]
}

export function renderResUsersSettingsForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'res.users.settings' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/res.users.settings/<método> (≈ call_kw)
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
