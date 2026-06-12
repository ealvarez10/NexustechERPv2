// Generado por odoo2rs — vista form de mail.activity.type (mail_activity_type_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail_activity_type_view_form",
  "name": "mail.activity.type.view.form",
  "model": "mail.activity.type",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Activities"
    },
    "children": [
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "widget",
            "attrs": {
              "bg_color": "text-bg-danger",
              "invisible": "active",
              "name": "web_ribbon",
              "title": "Archived"
            }
          },
          {
            "tag": "div",
            "attrs": {
              "class": "oe_title"
            },
            "children": [
              {
                "tag": "label",
                "attrs": {
                  "class": "oe_edit_only",
                  "for": "name"
                }
              },
              {
                "tag": "h1",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "name",
                      "options": "{'line_breaks': False}",
                      "placeholder": "e.g. Schedule a meeting",
                      "widget": "text"
                    }
                  }
                ]
              }
            ]
          },
          {
            "tag": "group",
            "children": [
              {
                "tag": "group",
                "attrs": {
                  "name": "activity_details",
                  "string": "Activity Settings"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "1",
                      "name": "active"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "category"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "domain": "[('share', '=', False)]",
                      "name": "default_user_id",
                      "options": "{'no_create': True}",
                      "widget": "many2one_avatar_user"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "context.get('default_res_model')",
                      "name": "res_model",
                      "placeholder": "Available everywhere"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "summary",
                      "placeholder": "e.g. \"Discuss proposal\""
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "groups": "base.group_no_one",
                      "name": "icon"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "groups": "base.group_no_one",
                      "name": "decoration_type"
                    }
                  },
                  {
                    "tag": "label",
                    "attrs": {
                      "for": "delay_count"
                    }
                  },
                  {
                    "tag": "div",
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "class": "oe_inline pe-1 o_input_3ch",
                          "name": "delay_count"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "class": "oe_inline ps-1 pe-2",
                          "name": "delay_unit"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "class": "oe_inline",
                          "name": "delay_from"
                        }
                      }
                    ]
                  }
                ]
              },
              {
                "tag": "group",
                "attrs": {
                  "name": "activity_planning",
                  "string": "Next Activity"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "category == 'upload_file'",
                      "name": "chaining_type"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "context": "{'default_res_model': res_model}",
                      "invisible": "chaining_type == 'suggest' and category != 'upload_file'",
                      "name": "triggered_next_type_id",
                      "options": "{'no_open': True}",
                      "required": "chaining_type == 'trigger' and category != 'upload_file'"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "context": "{'default_res_model': res_model}",
                      "invisible": "chaining_type == 'trigger' or category == 'upload_file'",
                      "name": "suggested_next_type_ids",
                      "widget": "many2many_tags"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "context": "{'default_model': res_model}",
                      "domain": "[('model_id.model', '=', res_model)]",
                      "invisible": "not res_model",
                      "name": "mail_template_ids",
                      "widget": "many2many_tags"
                    }
                  }
                ]
              }
            ]
          },
          {
            "tag": "label",
            "attrs": {
              "class": "fw-bold",
              "for": "default_note"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "class": "oe-bordered-editor",
              "name": "default_note",
              "nolabel": "1",
              "placeholder": "e.g. \"Go over the offer and discuss details\""
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "name",
      "widget": "text",
      "attrs": {
        "options": "{'line_breaks': False}",
        "placeholder": "e.g. Schedule a meeting"
      }
    },
    {
      "name": "active",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "category"
    },
    {
      "name": "default_user_id",
      "widget": "many2one_avatar_user",
      "attrs": {
        "domain": "[('share', '=', False)]",
        "options": "{'no_create': True}"
      }
    },
    {
      "name": "res_model",
      "attrs": {
        "invisible": "context.get('default_res_model')",
        "placeholder": "Available everywhere"
      }
    },
    {
      "name": "summary",
      "attrs": {
        "placeholder": "e.g. \"Discuss proposal\""
      }
    },
    {
      "name": "icon",
      "attrs": {
        "groups": "base.group_no_one"
      }
    },
    {
      "name": "decoration_type",
      "attrs": {
        "groups": "base.group_no_one"
      }
    },
    {
      "name": "delay_count",
      "attrs": {
        "class": "oe_inline pe-1 o_input_3ch"
      }
    },
    {
      "name": "delay_unit",
      "attrs": {
        "class": "oe_inline ps-1 pe-2"
      }
    },
    {
      "name": "delay_from",
      "attrs": {
        "class": "oe_inline"
      }
    },
    {
      "name": "chaining_type",
      "attrs": {
        "invisible": "category == 'upload_file'"
      }
    },
    {
      "name": "triggered_next_type_id",
      "attrs": {
        "context": "{'default_res_model': res_model}",
        "invisible": "chaining_type == 'suggest' and category != 'upload_file'",
        "options": "{'no_open': True}",
        "required": "chaining_type == 'trigger' and category != 'upload_file'"
      }
    },
    {
      "name": "suggested_next_type_ids",
      "widget": "many2many_tags",
      "attrs": {
        "context": "{'default_res_model': res_model}",
        "invisible": "chaining_type == 'trigger' or category == 'upload_file'"
      }
    },
    {
      "name": "mail_template_ids",
      "widget": "many2many_tags",
      "attrs": {
        "context": "{'default_model': res_model}",
        "domain": "[('model_id.model', '=', res_model)]",
        "invisible": "not res_model"
      }
    },
    {
      "name": "default_note",
      "attrs": {
        "class": "oe-bordered-editor",
        "nolabel": "1",
        "placeholder": "e.g. \"Go over the offer and discuss details\""
      }
    }
  ]
}

export function renderMailActivityTypeForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.activity.type' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.activity.type/<método> (≈ call_kw)
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
