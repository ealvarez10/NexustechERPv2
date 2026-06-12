// Generado por odoo2rs — vista form de fetchmail.server (view_email_server_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_email_server_form",
  "name": "fetchmail.server.form",
  "model": "fetchmail.server",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Incoming Mail Server"
    },
    "children": [
      {
        "tag": "header",
        "attrs": {
          "invisible": "server_type == 'local'"
        },
        "children": [
          {
            "tag": "button",
            "attrs": {
              "invisible": "state != 'draft'",
              "name": "button_confirm_login",
              "string": "Test & Confirm",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "invisible": "state != 'done'",
              "name": "fetch_mail",
              "string": "Fetch Now",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "invisible": "state != 'done'",
              "name": "set_draft",
              "string": "Reset Confirmation",
              "type": "object"
            }
          },
          {
            "tag": "field",
            "attrs": {
              "name": "state",
              "widget": "statusbar"
            }
          }
        ]
      },
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "field",
            "attrs": {
              "invisible": "1",
              "name": "active"
            }
          },
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
            "tag": "group",
            "children": [
              {
                "tag": "group",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "name"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "server_type",
                      "readonly": "state == 'done'",
                      "widget": "radio"
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
                      "invisible": "not date",
                      "name": "date"
                    }
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "text-muted fst-italic",
                      "colspan": "8",
                      "invisible": "not server_type_info",
                      "role": "alert"
                    },
                    "children": [
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "server_type_info"
                        }
                      }
                    ]
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
                  "name": "server_login_details",
                  "string": "Server & Login"
                },
                "children": [
                  {
                    "tag": "group",
                    "children": [
                      {
                        "tag": "group",
                        "attrs": {
                          "invisible": "server_type == 'local'",
                          "string": "Server Information"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "server",
                              "readonly": "state != 'draft'",
                              "required": "server_type != 'local'"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "port",
                              "options": "{'format': false}",
                              "readonly": "state != 'draft'",
                              "required": "1"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "is_ssl"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "invisible": "server_type == 'local'",
                          "string": "Login Information"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "user",
                              "readonly": "state != 'draft'",
                              "required": "server_type != 'local'"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "invisible": "server_type not in ('imap', 'pop')",
                              "name": "password",
                              "password": "True",
                              "readonly": "state != 'draft'",
                              "required": "server_type in ('imap', 'pop')"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "string": "Actions to Perform on Incoming Mails"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "object_id"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "invisible": "server_type != 'local'",
                          "string": "Configuration"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "configuration"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "script",
                              "widget": "url"
                            }
                          }
                        ]
                      }
                    ]
                  }
                ]
              },
              {
                "tag": "page",
                "attrs": {
                  "groups": "base.group_no_one",
                  "name": "advanced_options",
                  "string": "Advanced"
                },
                "children": [
                  {
                    "tag": "group",
                    "children": [
                      {
                        "tag": "group",
                        "attrs": {
                          "string": "Advanced Options"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "priority",
                              "readonly": "state != 'draft'"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "attach"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "original"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "group",
                        "attrs": {
                          "string": "Last Error"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "error_date"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "error_message"
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
      "name": "state",
      "widget": "statusbar"
    },
    {
      "name": "active",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "name"
    },
    {
      "name": "server_type",
      "widget": "radio",
      "attrs": {
        "readonly": "state == 'done'"
      }
    },
    {
      "name": "date",
      "attrs": {
        "invisible": "not date"
      }
    },
    {
      "name": "server_type_info"
    },
    {
      "name": "server",
      "attrs": {
        "readonly": "state != 'draft'",
        "required": "server_type != 'local'"
      }
    },
    {
      "name": "port",
      "attrs": {
        "options": "{'format': false}",
        "readonly": "state != 'draft'",
        "required": "1"
      }
    },
    {
      "name": "is_ssl"
    },
    {
      "name": "user",
      "attrs": {
        "readonly": "state != 'draft'",
        "required": "server_type != 'local'"
      }
    },
    {
      "name": "password",
      "attrs": {
        "invisible": "server_type not in ('imap', 'pop')",
        "password": "True",
        "readonly": "state != 'draft'",
        "required": "server_type in ('imap', 'pop')"
      }
    },
    {
      "name": "object_id"
    },
    {
      "name": "configuration"
    },
    {
      "name": "script",
      "widget": "url"
    },
    {
      "name": "priority",
      "attrs": {
        "readonly": "state != 'draft'"
      }
    },
    {
      "name": "attach"
    },
    {
      "name": "original"
    },
    {
      "name": "error_date"
    },
    {
      "name": "error_message"
    }
  ],
  "buttons": [
    {
      "name": "button_confirm_login",
      "string": "Test & Confirm",
      "type": "object"
    },
    {
      "name": "fetch_mail",
      "string": "Fetch Now",
      "type": "object"
    },
    {
      "name": "set_draft",
      "string": "Reset Confirmation",
      "type": "object"
    }
  ]
}

export function renderFetchmailServerForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'fetchmail.server' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/fetchmail.server/<método> (≈ call_kw)
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
