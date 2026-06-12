// Generado por odoo2rs — vista form de mail.alias.domain (mail_alias_domain_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "mail_alias_domain_view_form",
  "name": "mail.alias.domain.view.form",
  "model": "mail.alias.domain",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "duplicate": "false",
      "string": "Alias Domain"
    },
    "children": [
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "group",
            "children": [
              {
                "tag": "group",
                "children": [
                  {
                    "tag": "label",
                    "attrs": {
                      "for": "name"
                    }
                  },
                  {
                    "tag": "div",
                    "children": [
                      {
                        "tag": "span",
                        "attrs": {
                          "class": "me-1 oe_inline"
                        },
                        "text": "@"
                      },
                      {
                        "tag": "field",
                        "attrs": {
                          "class": "oe_inline",
                          "name": "name",
                          "placeholder": "e.g. \"mycompany.com\"",
                          "string": "Domain"
                        }
                      }
                    ]
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "groups": "base.group_multi_company",
                      "name": "company_ids",
                      "placeholder": "Visible to all",
                      "readonly": "True",
                      "string": "Used In",
                      "widget": "many2many_tags"
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
                      "groups": "base.group_no_one",
                      "name": "bounce_alias",
                      "placeholder": "e.g. \"bounce\""
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "groups": "base.group_no_one",
                      "name": "catchall_alias",
                      "placeholder": "e.g. \"catchall\""
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "groups": "base.group_no_one",
                      "name": "default_from",
                      "placeholder": "e.g. \"notifications\""
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
  "fields": [
    {
      "name": "name",
      "string": "Domain",
      "attrs": {
        "class": "oe_inline",
        "placeholder": "e.g. \"mycompany.com\""
      }
    },
    {
      "name": "company_ids",
      "string": "Used In",
      "widget": "many2many_tags",
      "attrs": {
        "groups": "base.group_multi_company",
        "placeholder": "Visible to all",
        "readonly": "True"
      }
    },
    {
      "name": "bounce_alias",
      "attrs": {
        "groups": "base.group_no_one",
        "placeholder": "e.g. \"bounce\""
      }
    },
    {
      "name": "catchall_alias",
      "attrs": {
        "groups": "base.group_no_one",
        "placeholder": "e.g. \"catchall\""
      }
    },
    {
      "name": "default_from",
      "attrs": {
        "groups": "base.group_no_one",
        "placeholder": "e.g. \"notifications\""
      }
    }
  ]
}

export function renderMailAliasDomainForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'mail.alias.domain' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/mail.alias.domain/<método> (≈ call_kw)
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
