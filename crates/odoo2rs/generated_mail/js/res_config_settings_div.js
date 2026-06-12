// Generado por odoo2rs — vista div de res.config.settings (res_config_settings_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "res_config_settings_view_form",
  "name": "res.config.settings.view.form.inherit.mail",
  "model": "res.config.settings",
  "type": "div",
  "arch": {
    "tag": "div",
    "attrs": {
      "id": "emails",
      "position": "replace"
    },
    "children": [
      {
        "tag": "block",
        "attrs": {
          "id": "emails",
          "title": "Emails"
        },
        "children": [
          {
            "tag": "setting",
            "attrs": {
              "documentation": "/applications/general/email_communication/email_servers.html",
              "help": "Configure your own email servers",
              "id": "email_servers_setting",
              "title": "Using your own email server is required to send/receive emails in Community and Enterprise versions. Online users already benefit from a ready-to-use email server (@mycompany.odoo.com)."
            },
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "name": "external_email_server_default"
                }
              },
              {
                "tag": "div",
                "attrs": {
                  "class": "content-group mb-3",
                  "invisible": "not external_email_server_default"
                },
                "children": [
                  {
                    "tag": "div",
                    "children": [
                      {
                        "tag": "div",
                        "attrs": {
                          "class": "mt8"
                        },
                        "children": [
                          {
                            "tag": "button",
                            "attrs": {
                              "class": "btn-link",
                              "icon": "oi-arrow-right",
                              "name": "%(action_email_server_tree)d",
                              "string": "Incoming Email Servers",
                              "type": "action"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "div",
                        "attrs": {
                          "class": "mt8"
                        },
                        "children": [
                          {
                            "tag": "button",
                            "attrs": {
                              "class": "btn-link",
                              "icon": "oi-arrow-right",
                              "name": "%(base.action_ir_mail_server_list)d",
                              "string": "Outgoing Email Servers",
                              "type": "action"
                            }
                          }
                        ]
                      }
                    ]
                  }
                ]
              }
            ]
          },
          {
            "tag": "setting",
            "attrs": {
              "company_dependent": "1",
              "documentation": "/applications/general/email_communication/email_domain.html#be-spf-compliant",
              "help": "Use different domains for your mail aliases",
              "id": "email-alias-setting",
              "string": "Alias Domain"
            },
            "children": [
              {
                "tag": "span",
                "attrs": {
                  "class": "me-1"
                },
                "text": "@"
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "alias_domain_id",
                  "options": "{'no_create_edit': True}",
                  "placeholder": "e.g. mycompany.com"
                }
              }
            ]
          },
          {
            "tag": "setting",
            "attrs": {
              "documentation": "https://console.developers.google.com/",
              "help": "Send and receive emails through your Gmail account.",
              "invisible": "not external_email_server_default",
              "string": "Use a Gmail Server"
            },
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "name": "module_google_gmail"
                }
              },
              {
                "tag": "div",
                "attrs": {
                  "class": "content-group",
                  "id": "msg_module_google_gmail",
                  "invisible": "not module_google_gmail"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "mt16 text-warning"
                    },
                    "children": [
                      {
                        "tag": "strong",
                        "text": "Save"
                      }
                    ],
                    "text": "this page and come back here to set up the feature."
                  }
                ]
              }
            ]
          },
          {
            "tag": "setting",
            "attrs": {
              "documentation": "https://docs.microsoft.com/en-us/azure/active-directory/develop/quickstart-register-app",
              "help": "Send and receive emails through your Outlook account.",
              "id": "email-outlook-setting",
              "invisible": "not external_email_server_default",
              "string": "Use an Outlook Server"
            },
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "name": "module_microsoft_outlook"
                }
              },
              {
                "tag": "div",
                "attrs": {
                  "class": "content-group",
                  "id": "msg_module_microsoft_outlook",
                  "invisible": "not module_microsoft_outlook"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "mt16 text-warning"
                    },
                    "children": [
                      {
                        "tag": "strong",
                        "text": "Save"
                      }
                    ],
                    "text": "this page and come back here to set up the feature."
                  }
                ]
              }
            ]
          },
          {
            "tag": "setting",
            "attrs": {
              "help": "Restrict mail templates edition and QWEB placeholders usage.",
              "id": "restrict_template_rendering_setting"
            },
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "name": "restrict_template_rendering"
                }
              }
            ]
          }
        ]
      },
      {
        "tag": "block",
        "attrs": {
          "id": "discuss",
          "title": "Discuss"
        },
        "children": [
          {
            "tag": "setting",
            "attrs": {
              "help": "Configure your activity types",
              "id": "activities_setting",
              "string": "Activities"
            },
            "children": [
              {
                "tag": "div",
                "attrs": {
                  "class": "content-group"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "mt8"
                    },
                    "children": [
                      {
                        "tag": "button",
                        "attrs": {
                          "class": "oe_link",
                          "icon": "oi-arrow-right",
                          "name": "%(mail.mail_activity_type_action)d",
                          "string": "Activity Types",
                          "type": "action"
                        }
                      }
                    ]
                  }
                ]
              }
            ]
          },
          {
            "tag": "setting",
            "attrs": {
              "documentation": "https://www.odoo.com/documentation/latest/applications/productivity/discuss/ice_servers.html",
              "help": "Set up your own server for small group calls using peer-to-peer connections",
              "string": "Custom ICE Server with Twilio"
            },
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "name": "use_twilio_rtc_servers"
                }
              },
              {
                "tag": "div",
                "attrs": {
                  "class": "content-group",
                  "invisible": "not use_twilio_rtc_servers"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "row mt16",
                      "id": "mail_twilio_sid"
                    },
                    "children": [
                      {
                        "tag": "label",
                        "attrs": {
                          "class": "col-lg-3",
                          "for": "twilio_account_sid"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "twilio_account_sid",
                          "placeholder": "e.g. ACd5543a0b450ar4c7t95f1b6e8a39t543",
                          "string": "Account SID"
                        }
                      }
                    ]
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "row mt16",
                      "id": "mail_twilio_auth_token"
                    },
                    "children": [
                      {
                        "tag": "label",
                        "attrs": {
                          "class": "col-lg-3",
                          "for": "twilio_account_token"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "twilio_account_token",
                          "placeholder": "e.g. 65ea4f9e948b693N5156F350256bd152",
                          "string": "Account Auth Token"
                        }
                      }
                    ]
                  }
                ]
              }
            ]
          },
          {
            "tag": "setting",
            "attrs": {
              "help": "Set up your own server for large group calls by routing connections centrally",
              "string": "Custom SFU Server"
            },
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "name": "use_sfu_server"
                }
              },
              {
                "tag": "div",
                "attrs": {
                  "class": "content-group",
                  "invisible": "not use_sfu_server"
                },
                "children": [
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "row mt16"
                    },
                    "children": [
                      {
                        "tag": "label",
                        "attrs": {
                          "class": "col-lg-3",
                          "for": "sfu_server_url",
                          "string": "URL"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "sfu_server_url"
                        }
                      }
                    ]
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "class": "row mt16"
                    },
                    "children": [
                      {
                        "tag": "label",
                        "attrs": {
                          "class": "col-lg-3",
                          "for": "sfu_server_key",
                          "string": "Key"
                        }
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "name": "sfu_server_key"
                        }
                      }
                    ]
                  }
                ]
              }
            ]
          },
          {
            "tag": "setting",
            "attrs": {
              "documentation": "https://www.odoo.com/documentation/latest/applications/productivity/discuss/ice_servers.html#define-a-list-of-custom-ice-servers",
              "help": "Use your own servers for calls to manage heavy traffic and ensure reliability if Twilio is unavailable",
              "string": "Custom ICE Servers"
            },
            "children": [
              {
                "tag": "div",
                "attrs": {
                  "class": "content-group"
                },
                "children": [
                  {
                    "tag": "button",
                    "attrs": {
                      "class": "btn-link",
                      "icon": "oi-arrow-right",
                      "name": "%(mail.action_ice_servers)d",
                      "string": "Configure ICE Servers",
                      "type": "action"
                    }
                  }
                ]
              }
            ]
          },
          {
            "tag": "setting",
            "attrs": {
              "documentation": "https://developers.google.com/tenor/guides/quickstart#setup",
              "help": "Add a Tenor GIF API key to enable GIFs support.",
              "id": "tenor_api_key",
              "string": "Tenor GIF API key"
            },
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "name": "tenor_api_key",
                  "placeholder": "Paste your API key"
                }
              }
            ]
          },
          {
            "tag": "setting",
            "attrs": {
              "documentation": "https://cloud.google.com/translate/docs/setup",
              "help": "Google Translate Integration",
              "id": "message_translation_setting",
              "string": "Message Translation"
            },
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "name": "google_translate_api_key",
                  "placeholder": "Paste your API key"
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
      "name": "external_email_server_default"
    },
    {
      "name": "alias_domain_id",
      "attrs": {
        "options": "{'no_create_edit': True}",
        "placeholder": "e.g. mycompany.com"
      }
    },
    {
      "name": "module_google_gmail"
    },
    {
      "name": "module_microsoft_outlook"
    },
    {
      "name": "restrict_template_rendering"
    },
    {
      "name": "use_twilio_rtc_servers"
    },
    {
      "name": "twilio_account_sid",
      "string": "Account SID",
      "attrs": {
        "placeholder": "e.g. ACd5543a0b450ar4c7t95f1b6e8a39t543"
      }
    },
    {
      "name": "twilio_account_token",
      "string": "Account Auth Token",
      "attrs": {
        "placeholder": "e.g. 65ea4f9e948b693N5156F350256bd152"
      }
    },
    {
      "name": "use_sfu_server"
    },
    {
      "name": "sfu_server_url"
    },
    {
      "name": "sfu_server_key"
    },
    {
      "name": "tenor_api_key",
      "attrs": {
        "placeholder": "Paste your API key"
      }
    },
    {
      "name": "google_translate_api_key",
      "attrs": {
        "placeholder": "Paste your API key"
      }
    }
  ],
  "buttons": [
    {
      "name": "%(action_email_server_tree)d",
      "string": "Incoming Email Servers",
      "type": "action",
      "class": "btn-link"
    },
    {
      "name": "%(base.action_ir_mail_server_list)d",
      "string": "Outgoing Email Servers",
      "type": "action",
      "class": "btn-link"
    },
    {
      "name": "%(mail.mail_activity_type_action)d",
      "string": "Activity Types",
      "type": "action",
      "class": "oe_link"
    },
    {
      "name": "%(mail.action_ice_servers)d",
      "string": "Configure ICE Servers",
      "type": "action",
      "class": "btn-link"
    }
  ]
}

export function renderResConfigSettingsDiv(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
