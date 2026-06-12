// Generado por odoo2rs — vista kanban de account.journal (account_journal_view_kanban).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { kanbanViewHtml } from '../components/kanban_view.js'

export const DESCRIPTOR = {
  "xml_id": "account_journal_view_kanban",
  "name": "account.journal.kanban",
  "model": "account.journal",
  "type": "kanban",
  "arch": {
    "tag": "kanban",
    "attrs": {
      "class": "o_kanban_mobile"
    },
    "children": [
      {
        "tag": "templates",
        "children": [
          {
            "tag": "t",
            "attrs": {
              "class": "row g-0",
              "t-name": "card"
            },
            "children": [
              {
                "tag": "div",
                "attrs": {
                  "class": "col-6"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "fw-bolder",
                      "name": "name"
                    }
                  }
                ]
              },
              {
                "tag": "div",
                "attrs": {
                  "class": "col-6 "
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "class": "float-end",
                      "name": "type"
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
      "attrs": {
        "class": "fw-bolder"
      }
    },
    {
      "name": "type",
      "attrs": {
        "class": "float-end"
      }
    }
  ]
}

export function renderAccountJournalKanban(records = []) {
  return kanbanViewHtml({
    // TODO(odoo2rs): columnas desde la selection del campo state del modelo
    columns: [],
    records,
    stateField: 'state',
  })
}
