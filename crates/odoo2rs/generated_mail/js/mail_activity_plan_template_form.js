// Generado por odoo2rs — vista form de mail.activity.plan.template (mail_activity_plan_template_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail_activity_plan_template_view_form",
  "name": "mail.activity.plan.template.view.form",
  "model": "mail.activity.plan.template",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Activity"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "invisible": "1",
          "name": "company_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "invisible": "1",
          "name": "res_model"
        }
      },
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "group",
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "name": "activity_type_id"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "summary",
                  "placeholder": "e.g. Discuss Proposal"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "responsible_type"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "invisible": "responsible_type != 'other'",
                  "name": "responsible_id",
                  "required": "responsible_type == 'other'"
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
            "tag": "field",
            "attrs": {
              "class": "oe-bordered-editor",
              "name": "note",
              "nolabel": "1",
              "placeholder": "e.g. Log a note",
              "widget": "html_mail"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "company_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "res_model",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "activity_type_id"
    },
    {
      "name": "summary",
      "attrs": {
        "placeholder": "e.g. Discuss Proposal"
      }
    },
    {
      "name": "responsible_type"
    },
    {
      "name": "responsible_id",
      "attrs": {
        "invisible": "responsible_type != 'other'",
        "required": "responsible_type == 'other'"
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
      "name": "note",
      "widget": "html_mail",
      "attrs": {
        "class": "oe-bordered-editor",
        "nolabel": "1",
        "placeholder": "e.g. Log a note"
      }
    }
  ]
}

export function renderMailActivityPlanTemplateForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.activity.plan.template' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.activity.plan.template/<método> (≈ call_kw)
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
