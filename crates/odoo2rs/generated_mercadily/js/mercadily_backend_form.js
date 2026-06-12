// Generado por odoo2rs — vista form de mercadily.backend (mercadily_backend_form_view).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "mercadily_backend_form_view",
  "name": "mercadily.backend.form",
  "model": "mercadily.backend",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Backend Mercadily"
    },
    "children": [
      {
        "tag": "header",
        "children": [
          {
            "tag": "button",
            "attrs": {
              "class": "oe_highlight",
              "icon": "fa-plug",
              "name": "action_test_connection",
              "string": "Probar Conexión",
              "type": "object"
            }
          },
          {
            "tag": "button",
            "attrs": {
              "class": "oe_highlight",
              "icon": "fa-refresh",
              "name": "action_sync_all",
              "string": "Sincronizar Todo",
              "type": "object"
            }
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
                  "icon": "fa-handshake-o",
                  "name": "action_view_leads",
                  "type": "object"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "lead_count",
                      "string": "Leads",
                      "widget": "statinfo"
                    }
                  }
                ]
              },
              {
                "tag": "button",
                "attrs": {
                  "class": "oe_stat_button",
                  "icon": "fa-shopping-cart",
                  "name": "action_view_orders",
                  "type": "object"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "order_count",
                      "string": "Pedidos",
                      "widget": "statinfo"
                    }
                  }
                ]
              },
              {
                "tag": "button",
                "attrs": {
                  "class": "oe_stat_button",
                  "icon": "fa-users",
                  "name": "action_view_customers",
                  "type": "object"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "customer_count",
                      "string": "Clientes",
                      "widget": "statinfo"
                    }
                  }
                ]
              }
            ]
          },
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
                      "name": "name",
                      "placeholder": "Nombre de la Tienda"
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
                  "string": "Configuración API"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "api_url",
                      "placeholder": "https://tudominio.com",
                      "widget": "url"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "api_key",
                      "password": "True"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "groups": "base.group_multi_company",
                      "name": "company_id"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "invisible": "1",
                      "name": "active"
                    }
                  }
                ]
              },
              {
                "tag": "group",
                "attrs": {
                  "string": "Última Sincronización"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "last_lead_sync"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "last_order_sync"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "last_customer_sync"
                    }
                  }
                ]
              }
            ]
          },
          {
            "tag": "group",
            "attrs": {
              "string": "Sincronización Manual"
            },
            "children": [
              {
                "tag": "div",
                "attrs": {
                  "class": "text-muted"
                },
                "children": [
                  {
                    "tag": "p",
                    "text": "Puedes sincronizar tipos de datos específicos:"
                  }
                ]
              },
              {
                "tag": "div",
                "children": [
                  {
                    "tag": "button",
                    "attrs": {
                      "class": "btn-secondary me-2",
                      "icon": "fa-bullhorn",
                      "name": "action_sync_leads",
                      "string": "Sincronizar Leads",
                      "type": "object"
                    }
                  },
                  {
                    "tag": "button",
                    "attrs": {
                      "class": "btn-secondary me-2",
                      "icon": "fa-users",
                      "name": "action_sync_customers",
                      "string": "Sincronizar Clientes",
                      "type": "object"
                    }
                  },
                  {
                    "tag": "button",
                    "attrs": {
                      "class": "btn-secondary",
                      "icon": "fa-shopping-cart",
                      "name": "action_sync_orders",
                      "string": "Sincronizar Pedidos",
                      "type": "object"
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
                  "name": "sync_logs",
                  "string": "Logs de Sincronización"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "sync_log_ids",
                      "readonly": "1"
                    },
                    "children": [
                      {
                        "tag": "list",
                        "children": [
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "create_date"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "sync_type"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "created_count"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "updated_count"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "decoration-danger": "error_count > 0",
                              "name": "error_count"
                            }
                          },
                          {
                            "tag": "field",
                            "attrs": {
                              "name": "error_details",
                              "optional": "hide"
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
      },
      {
        "tag": "chatter"
      }
    ]
  },
  "fields": [
    {
      "name": "lead_count",
      "string": "Leads",
      "widget": "statinfo"
    },
    {
      "name": "order_count",
      "string": "Pedidos",
      "widget": "statinfo"
    },
    {
      "name": "customer_count",
      "string": "Clientes",
      "widget": "statinfo"
    },
    {
      "name": "name",
      "attrs": {
        "placeholder": "Nombre de la Tienda"
      }
    },
    {
      "name": "api_url",
      "widget": "url",
      "attrs": {
        "placeholder": "https://tudominio.com"
      }
    },
    {
      "name": "api_key",
      "attrs": {
        "password": "True"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "groups": "base.group_multi_company"
      }
    },
    {
      "name": "active",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "last_lead_sync"
    },
    {
      "name": "last_order_sync"
    },
    {
      "name": "last_customer_sync"
    },
    {
      "name": "sync_log_ids",
      "attrs": {
        "readonly": "1"
      }
    },
    {
      "name": "create_date"
    },
    {
      "name": "sync_type"
    },
    {
      "name": "created_count"
    },
    {
      "name": "updated_count"
    },
    {
      "name": "error_count",
      "attrs": {
        "decoration-danger": "error_count > 0"
      }
    },
    {
      "name": "error_details",
      "attrs": {
        "optional": "hide"
      }
    }
  ],
  "buttons": [
    {
      "name": "action_test_connection",
      "string": "Probar Conexión",
      "type": "object",
      "class": "oe_highlight"
    },
    {
      "name": "action_sync_all",
      "string": "Sincronizar Todo",
      "type": "object",
      "class": "oe_highlight"
    },
    {
      "name": "action_view_leads",
      "type": "object",
      "class": "oe_stat_button"
    },
    {
      "name": "action_view_orders",
      "type": "object",
      "class": "oe_stat_button"
    },
    {
      "name": "action_view_customers",
      "type": "object",
      "class": "oe_stat_button"
    },
    {
      "name": "action_sync_leads",
      "string": "Sincronizar Leads",
      "type": "object",
      "class": "btn-secondary me-2"
    },
    {
      "name": "action_sync_customers",
      "string": "Sincronizar Clientes",
      "type": "object",
      "class": "btn-secondary me-2"
    },
    {
      "name": "action_sync_orders",
      "string": "Sincronizar Pedidos",
      "type": "object",
      "class": "btn-secondary"
    }
  ]
}

export function renderMercadilyBackendForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mercadily.backend' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mercadily.backend/<método> (≈ call_kw)
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
