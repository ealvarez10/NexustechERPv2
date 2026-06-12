// Generado por odoo2rs — vista kanban de account.payment.term (view_account_payment_term_kanban).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { kanbanViewHtml } from '../components/kanban_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_account_payment_term_kanban",
  "name": "account.payment.term.kanban",
  "model": "account.payment.term",
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
              "t-name": "card"
            },
            "children": [
              {
                "tag": "field",
                "attrs": {
                  "class": "fw-bolder fs-5",
                  "name": "name"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "note"
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
      "name": "name",
      "attrs": {
        "class": "fw-bolder fs-5"
      }
    },
    {
      "name": "note"
    }
  ]
}

export function renderAccountPaymentTermKanban(records = []) {
  return kanbanViewHtml({
    // TODO(odoo2rs): columnas desde la selection del campo state del modelo
    columns: [],
    records,
    stateField: 'state',
  })
}
