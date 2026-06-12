// Generado por odoo2rs — vista kanban de account.payment.method.line (view_account_payment_method_line_kanban_mobile).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { kanbanViewHtml } from '../components/kanban_view.js'

export const DESCRIPTOR = {
  "xml_id": "view_account_payment_method_line_kanban_mobile",
  "name": "account.payment.method.line.kanban",
  "model": "account.payment.method.line",
  "type": "kanban",
  "arch": {
    "tag": "kanban",
    "attrs": {
      "class": "o_kanban_mobile"
    },
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
              "name": "display_name"
            }
          }
        ]
      }
    ]
  },
  "fields": [
    {
      "name": "display_name"
    }
  ]
}

export function renderAccountPaymentMethodLineKanban(records = []) {
  return kanbanViewHtml({
    // TODO(odoo2rs): columnas desde la selection del campo state del modelo
    columns: [],
    records,
    stateField: 'state',
  })
}
