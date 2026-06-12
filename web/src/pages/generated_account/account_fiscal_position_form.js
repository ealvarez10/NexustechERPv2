// Generado por odoo2rs — vista form de account.fiscal.position (view_account_position_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_account_position_form",
  "name": "account.fiscal.position.form",
  "model": "account.fiscal.position",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Fiscal Position"
    },
    "children": [
      {
        "tag": "div",
        "attrs": {
          "class": "alert alert-info",
          "groups": "account.group_account_manager",
          "invisible": "not foreign_vat_header_mode",
          "role": "alert"
        },
        "children": [
          {
            "tag": "div",
            "attrs": {
              "invisible": "foreign_vat_header_mode not in ('templates_found', 'no_template')"
            },
            "children": [
              {
                "tag": "button",
                "attrs": {
                  "class": "oe_link p-0 align-baseline",
                  "name": "action_create_foreign_taxes",
                  "string": "here",
                  "type": "object"
                }
              }
            ],
            "text": "Click\n                            \n                            to create the taxes for this country."
          }
        ]
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
                  "class": "oe_stat_button",
                  "icon": "fa-bars",
                  "invisible": "not tax_ids",
                  "name": "action_open_related_taxes",
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
                        "text": "Taxes"
                      }
                    ]
                  }
                ]
              }
            ]
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
                      "invisible": "1",
                      "name": "active"
                    }
                  },
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
                      "name": "states_count"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "1",
                      "name": "company_country_id"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "1",
                      "name": "fiscal_country_codes"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "1",
                      "name": "foreign_vat_header_mode"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "name"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "groups": "base.group_multi_company",
                      "name": "company_id",
                      "options": "{'no_create': True}"
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
                      "name": "auto_apply"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "not auto_apply",
                      "name": "vat_required"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "foreign_vat"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "not auto_apply",
                      "name": "country_group_id"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "country_id",
                      "options": "{'no_open': True, 'no_create': True}",
                      "required": "foreign_vat"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "domain": "[('country_id', '=', country_id)]",
                      "invisible": "(not auto_apply and not foreign_vat) or not country_id or states_count == 0",
                      "name": "state_ids",
                      "widget": "many2many_tags"
                    }
                  },
                  {
                    "tag": "label",
                    "attrs": {
                      "for": "zip_from",
                      "invisible": "not auto_apply or not country_id",
                      "string": "Zip Range"
                    }
                  },
                  {
                    "tag": "div",
                    "attrs": {
                      "invisible": "not auto_apply or not country_id"
                    },
                    "children": [
                      {
                        "tag": "span",
                        "text": "From"
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "class": "oe_inline",
                          "name": "zip_from"
                        }
                      },
                      {
                        "tag": "div",
                        "attrs": {
                          "class": "oe_edit_only"
                        }
                      },
                      {
                        "tag": "span",
                        "text": "To"
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "class": "oe_inline",
                          "name": "zip_to"
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
                  "groups": "account.group_account_readonly",
                  "name": "account_mapping",
                  "string": "Account Mapping"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "account_ids",
                      "nolabel": "1",
                      "widget": "one2many"
                    },
                    "children": [
                      {
                        "tag": "list",
                        "attrs": {
                          "editable": "bottom",
                          "string": "Account Mapping"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "domain": "['|', ('company_ids', '=', parent.company_id), ('account_type', '!=', 'off_balance')]",
                              "name": "account_src_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "domain": "['|', ('company_ids', '=', parent.company_id), ('account_type', '!=', 'off_balance')]",
                              "name": "account_dest_id"
                            }
                          }
                        ]
                      },
                      {
                        "tag": "form",
                        "attrs": {
                          "string": "Account Mapping"
                        },
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "domain": "['|', ('company_ids', '=', parent.company_id), ('account_type', '!=', 'off_balance')]",
                              "name": "account_src_id"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "domain": "['|', ('company_ids', '=', parent.company_id), ('account_type', '!=', 'off_balance')]",
                              "name": "account_dest_id"
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
            "tag": "field",
            "attrs": {
              "class": "oe-bordered-editor",
              "name": "note",
              "placeholder": "Legal Notes..."
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "active",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "states_count",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "company_country_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "fiscal_country_codes",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "foreign_vat_header_mode",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "name"
    },
    {
      "name": "company_id",
      "attrs": {
        "groups": "base.group_multi_company",
        "options": "{'no_create': True}"
      }
    },
    {
      "name": "auto_apply"
    },
    {
      "name": "vat_required",
      "attrs": {
        "invisible": "not auto_apply"
      }
    },
    {
      "name": "foreign_vat"
    },
    {
      "name": "country_group_id",
      "attrs": {
        "invisible": "not auto_apply"
      }
    },
    {
      "name": "country_id",
      "attrs": {
        "options": "{'no_open': True, 'no_create': True}",
        "required": "foreign_vat"
      }
    },
    {
      "name": "state_ids",
      "widget": "many2many_tags",
      "attrs": {
        "domain": "[('country_id', '=', country_id)]",
        "invisible": "(not auto_apply and not foreign_vat) or not country_id or states_count == 0"
      }
    },
    {
      "name": "zip_from",
      "attrs": {
        "class": "oe_inline"
      }
    },
    {
      "name": "zip_to",
      "attrs": {
        "class": "oe_inline"
      }
    },
    {
      "name": "account_ids",
      "widget": "one2many",
      "attrs": {
        "nolabel": "1"
      }
    },
    {
      "name": "account_src_id",
      "attrs": {
        "domain": "['|', ('company_ids', '=', parent.company_id), ('account_type', '!=', 'off_balance')]"
      }
    },
    {
      "name": "account_dest_id",
      "attrs": {
        "domain": "['|', ('company_ids', '=', parent.company_id), ('account_type', '!=', 'off_balance')]"
      }
    },
    {
      "name": "account_src_id",
      "attrs": {
        "domain": "['|', ('company_ids', '=', parent.company_id), ('account_type', '!=', 'off_balance')]"
      }
    },
    {
      "name": "account_dest_id",
      "attrs": {
        "domain": "['|', ('company_ids', '=', parent.company_id), ('account_type', '!=', 'off_balance')]"
      }
    },
    {
      "name": "note",
      "attrs": {
        "class": "oe-bordered-editor",
        "placeholder": "Legal Notes..."
      }
    }
  ],
  "buttons": [
    {
      "name": "action_create_foreign_taxes",
      "string": "here",
      "type": "object",
      "class": "oe_link p-0 align-baseline"
    },
    {
      "name": "action_open_related_taxes",
      "type": "object",
      "class": "oe_stat_button"
    }
  ]
}

export function renderAccountFiscalPositionForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'account.fiscal.position' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/account.fiscal.position/<método> (≈ call_kw)
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
