// Generado por odoo2rs — vista form de account.lock_exception (view_account_lock_exception_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_account_lock_exception_form",
  "name": "account.lock_exception.form",
  "model": "account.lock_exception",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Account Lock Exception"
    },
    "children": [
      {
        "tag": "header",
        "children": [
          {
            "tag": "field",
            "attrs": {
              "invisible": "True",
              "name": "active"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "invisible": "not active",
              "name": "state",
              "statusbar_visible": "active,expired",
              "widget": "statusbar"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "class": "btn-secondary",
              "invisible": "state != 'active'",
              "name": "action_revoke",
              "string": "Revoke",
              "type": "object"
            }
          }
        ]
      },
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "widget",
            "attrs": {
              "bg_color": "bg-danger",
              "invisible": "active",
              "name": "web_ribbon",
              "title": "Revoked"
            }
          },
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
                  "class": "oe_stat_button",
                  "icon": "fa-bars",
                  "name": "action_show_audit_trail_during_exception",
                  "type": "object"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "o_stat_info"
                    },
                    "children": [
                      {
                        "tag": "span",
                        "attrs": {
                          "class": "o_stat_text"
                        },
                        "text": "Audit"
                      }
                    ]
                  }
                ]
              }
            ]
          },
          {
            "tag": "div",
            "children": [
              {
                "tag": "div",
                "attrs": {
                  "class": "oe_title"
                },
                "children": [
                  {
                    "tag": "h2",
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "decoration-bf": "1",
                          "name": "display_name"
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
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "create_uid",
                          "readonly": "True"
                        }
                      },
                      {
                        "tag": "label",
                        "attrs": {
                          "for": "user_id",
                          "string": "Valid for"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "invisible": "not user_id",
                          "name": "user_id",
                          "nolabel": "1",
                          "readonly": "True"
                        }
                      },
                      {
                        "tag": "span",
                        "attrs": {
                          "invisible": "user_id"
                        },
                        "text": "everyone"
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "invisible": "not reason",
                          "name": "reason",
                          "readonly": "True"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "create_date",
                          "readonly": "True"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "end_datetime",
                          "readonly": "True"
                        }
                      }
                    ]
                  },
                  {
                    "tag": "group",
                    "children": [
                      {
                        "tag": "div",
                        "attrs": {
                          "class": "o_wrap_label",
                          "colspan": "2"
                        },
                        "children": [
                          {
                            "tag": "span",
                            "attrs": {
                              "class": "o_form_label"
                            },
                            "text": "Changed Lock Date:"
                          }
                        ]
                      },
                      {
                        "tag": "div",
                        "attrs": {
                          "class": "o_wrap_label"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "class": "o_form_label o_form_label_readonly",
                              "name": "lock_date_field",
                              "nolabel": "1",
                              "readonly": "True"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "div",
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "class": "oe_inline",
                              "name": "lock_date",
                              "nolabel": "1",
                              "readonly": "True"
                            }
                          },
                          {
                            "tag": "i",
                            "attrs": {
                              "class": "text-muted"
                            },
                            "children": [
                              {
                                "tag": "field",
                                "attrs": {
                                  "class": "oe_inline",
                                  "name": "company_lock_date",
                                  "readonly": "True"
                                }
                              }
                            ],
                            "text": "(from )"
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
      "name": "active",
      "attrs": {
        "invisible": "True"
      }
    },
    {
      "name": "state",
      "widget": "statusbar",
      "attrs": {
        "invisible": "not active",
        "statusbar_visible": "active,expired"
      }
    },
    {
      "name": "display_name",
      "attrs": {
        "decoration-bf": "1"
      }
    },
    {
      "name": "create_uid",
      "attrs": {
        "readonly": "True"
      }
    },
    {
      "name": "user_id",
      "attrs": {
        "invisible": "not user_id",
        "nolabel": "1",
        "readonly": "True"
      }
    },
    {
      "name": "reason",
      "attrs": {
        "invisible": "not reason",
        "readonly": "True"
      }
    },
    {
      "name": "create_date",
      "attrs": {
        "readonly": "True"
      }
    },
    {
      "name": "end_datetime",
      "attrs": {
        "readonly": "True"
      }
    },
    {
      "name": "lock_date_field",
      "attrs": {
        "class": "o_form_label o_form_label_readonly",
        "nolabel": "1",
        "readonly": "True"
      }
    },
    {
      "name": "lock_date",
      "attrs": {
        "class": "oe_inline",
        "nolabel": "1",
        "readonly": "True"
      }
    },
    {
      "name": "company_lock_date",
      "attrs": {
        "class": "oe_inline",
        "readonly": "True"
      }
    }
  ],
  "buttons": [
    {
      "name": "action_revoke",
      "string": "Revoke",
      "type": "object",
      "class": "btn-secondary"
    },
    {
      "name": "action_show_audit_trail_during_exception",
      "type": "object",
      "class": "oe_stat_button"
    }
  ]
}

export function renderAccountLockExceptionForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'account.lock_exception' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/account.lock_exception/<método> (≈ call_kw)
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
