// Generado por odoo2rs — vista form de mail.activity.plan (mail_activity_plan_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail_activity_plan_view_form",
  "name": "mail.activity.plan.view.form",
  "model": "mail.activity.plan",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Planning"
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
          "name": "active"
        }
      },
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
                  "for": "name",
                  "string": "Plan Name"
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
                      "placeholder": "e.g. Onboarding",
                      "widget": "text"
                    }
                  }
                ]
              }
            ]
          },
          {
            "tag": "group",
            "attrs": {
              "name": "group_plan_fields"
            },
            "children": [
              {
                "tag": "group",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "res_model"
                    }
                  }
                ]
              },
              {
                "tag": "group",
                "attrs": {
                  "groups": "base.group_multi_company",
                  "name": "company_id"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "domain": "[('id', '=', allowed_company_ids)]",
                      "name": "company_id",
                      "placeholder": "Visible to all"
                    }
                  }
                ]
              }
            ]
          },
          {
            "tag": "notebook",
            "children": [
              {
                "tag": "page",
                "attrs": {
                  "string": "Activities To Create"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "context": "{'default_res_model': res_model}",
                      "name": "template_ids",
                      "nolabel": "1"
                    },
                    "children": [
                      {
                        "tag": "list",
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "1",
                              "name": "company_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "column_invisible": "1",
                              "name": "note"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "sequence",
                              "widget": "handle"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "activity_type_id",
                              "options": "{'no_quick_create': True}"
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
                              "name": "responsible_id",
                              "readonly": "responsible_type != 'other'",
                              "widget": "many2one_avatar_user"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "delay_count"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "delay_unit",
                              "string": "Unit"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "delay_from"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "not next_activity_ids",
                              "name": "next_activity_ids",
                              "optional": "hide",
                              "options": "{                                                 'no_quick_create': True,                                                 'edit_tags': True,                                             }",
                              "widget": "many2many_tags"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "kanban",
                        "attrs": {
                          "class": "o_kanban_mobile"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "icon"
                            }
                          },
                          {
                            "tag": "templates",
                            "children": [
                              {
                                "tag": "t",
                                "attrs": {
                                  "t-name": "card"
                                },
                                "children": [
                                  {
                                    "tag": "div",
                                    "attrs": {
                                      "class": "fw-bold fs-5"
                                    },
                                    "children": [
                                      {
                                        "tag": "i",
                                        "attrs": {
                                          "aria-label": "Activity Type",
                                          "role": "img",
                                          "t-attf-class": "fa #{record.icon.value} fa-fw ",
                                          "t-if": "record.icon.value",
                                          "title": "Activity Type"
                                        }
                                      },
                                      {
                                        "tag": "field",
                                        "attrs": {
                                          "name": "activity_type_id"
                                        }
                                      }
                                    ]
                                  },
                                  {
                                    "tag": "field",
                                    "attrs": {
                                      "name": "summary"
                                    }
                                  },
                                  {
                                    "tag": "div",
                                    "children": [
                                      {
                                        "tag": "field",
                                        "attrs": {
                                          "name": "delay_count"
                                        }
                                      },
                                      {
                                        "tag": "field",
                                        "attrs": {
                                          "name": "delay_unit"
                                        }
                                      },
                                      {
                                        "tag": "field",
                                        "attrs": {
                                          "name": "delay_from"
                                        }
                                      }
                                    ],
                                    "text": "()"
                                  },
                                  {
                                    "tag": "field",
                                    "attrs": {
                                      "invisible": "not next_activity_ids",
                                      "name": "next_activity_ids",
                                      "widget": "many2many_tags"
                                    }
                                  },
                                  {
                                    "tag": "footer",
                                    "attrs": {
                                      "class": "p-0"
                                    },
                                    "children": [
                                      {
                                        "tag": "field",
                                        "attrs": {
                                          "name": "responsible_type"
                                        }
                                      },
                                      {
                                        "tag": "field",
                                        "attrs": {
                                          "class": "ms-auto",
                                          "name": "responsible_id",
                                          "readonly": "1",
                                          "widget": "many2one_avatar_user"
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
              }
            ]
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
      "name": "active",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "name",
      "widget": "text",
      "attrs": {
        "options": "{'line_breaks': False}",
        "placeholder": "e.g. Onboarding"
      }
    },
    {
      "name": "res_model"
    },
    {
      "name": "company_id",
      "attrs": {
        "domain": "[('id', '=', allowed_company_ids)]",
        "placeholder": "Visible to all"
      }
    },
    {
      "name": "template_ids",
      "attrs": {
        "context": "{'default_res_model': res_model}",
        "nolabel": "1"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "column_invisible": "1"
      }
    },
    {
      "name": "note",
      "attrs": {
        "column_invisible": "1"
      }
    },
    {
      "name": "sequence",
      "widget": "handle"
    },
    {
      "name": "activity_type_id",
      "attrs": {
        "options": "{'no_quick_create': True}"
      }
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
      "widget": "many2one_avatar_user",
      "attrs": {
        "readonly": "responsible_type != 'other'"
      }
    },
    {
      "name": "delay_count"
    },
    {
      "name": "delay_unit",
      "string": "Unit"
    },
    {
      "name": "delay_from"
    },
    {
      "name": "next_activity_ids",
      "widget": "many2many_tags",
      "attrs": {
        "invisible": "not next_activity_ids",
        "optional": "hide",
        "options": "{                                                 'no_quick_create': True,                                                 'edit_tags': True,                                             }"
      }
    },
    {
      "name": "icon"
    },
    {
      "name": "activity_type_id"
    },
    {
      "name": "summary"
    },
    {
      "name": "delay_count"
    },
    {
      "name": "delay_unit"
    },
    {
      "name": "delay_from"
    },
    {
      "name": "next_activity_ids",
      "widget": "many2many_tags",
      "attrs": {
        "invisible": "not next_activity_ids"
      }
    },
    {
      "name": "responsible_type"
    },
    {
      "name": "responsible_id",
      "widget": "many2one_avatar_user",
      "attrs": {
        "class": "ms-auto",
        "readonly": "1"
      }
    }
  ]
}

export function renderMailActivityPlanForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.activity.plan' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.activity.plan/<método> (≈ call_kw)
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
