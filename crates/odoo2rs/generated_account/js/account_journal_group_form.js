// Generado por odoo2rs — vista form de account.journal.group (view_account_journal_group_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_account_journal_group_form",
  "name": "account.journal.group.form",
  "model": "account.journal.group",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Multi-ledger"
    },
    "children": [
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "group",
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
                  "name": "name",
                  "placeholder": "e.g. GAAP, IFRS, ..."
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "excluded_journal_ids",
                  "options": "{'no_create': True}",
                  "widget": "many2many_tags_journals"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "groups": "base.group_no_one",
                  "name": "sequence"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "groups": "base.group_multi_company",
                  "name": "company_id"
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
      "name": "company_id",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "name",
      "attrs": {
        "placeholder": "e.g. GAAP, IFRS, ..."
      }
    },
    {
      "name": "excluded_journal_ids",
      "widget": "many2many_tags_journals",
      "attrs": {
        "options": "{'no_create': True}"
      }
    },
    {
      "name": "sequence",
      "attrs": {
        "groups": "base.group_no_one"
      }
    },
    {
      "name": "company_id",
      "attrs": {
        "groups": "base.group_multi_company"
      }
    }
  ]
}

export function renderAccountJournalGroupForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'account.journal.group' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/account.journal.group/<método> (≈ call_kw)
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
