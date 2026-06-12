// Generado por odoo2rs — vista search de mail.template (view_email_template_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_email_template_search",
  "name": "email.template.search",
  "model": "mail.template",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Templates"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "filter_domain": "['|', '|', ('name','ilike',self), ('subject','ilike',self), ('email_to','ilike',self)]",
          "name": "name",
          "string": "Templates"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "lang"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "filter_domain": "[('model', '=', raw_value)]",
          "name": "model"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "model_id"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('user_id', '=', uid)]",
          "name": "my_templates",
          "string": "My Templates"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('template_category', '=', 'base_template')]",
          "name": "base_templates",
          "string": "Base Templates"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "domain": "[('template_category', '=', 'custom_template')]",
          "name": "custom_templates",
          "string": "Custom Templates"
        }
      },
      {
        "tag": "group",
        "children": [
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by':'mail_server_id'}",
              "domain": "[]",
              "name": "smtpserver",
              "string": "SMTP Server"
            }
          },
          {
            "tag": "filter",
            "attrs": {
              "context": "{'group_by':'model_id'}",
              "domain": "[]",
              "name": "group_by_model_id",
              "string": "Model"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "name",
      "string": "Templates",
      "attrs": {
        "filter_domain": "['|', '|', ('name','ilike',self), ('subject','ilike',self), ('email_to','ilike',self)]"
      }
    },
    {
      "name": "lang"
    },
    {
      "name": "model",
      "attrs": {
        "filter_domain": "[('model', '=', raw_value)]"
      }
    },
    {
      "name": "model_id"
    }
  ]
}

export function renderMailTemplateSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
