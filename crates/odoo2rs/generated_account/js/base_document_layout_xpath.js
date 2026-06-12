// Generado por odoo2rs — vista xpath de base.document.layout (view_base_document_layout).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "view_base_document_layout",
  "name": "Document Layout",
  "model": "base.document.layout",
  "type": "xpath",
  "arch": {
    "tag": "xpath",
    "attrs": {
      "expr": "//field[@name='paperformat_id']",
      "position": "after"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "invisible": "1",
          "name": "from_invoice"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "vat"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "account_number",
          "placeholder": "BE71096123456769",
          "required": "qr_code",
          "string": "Bank Account Number"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "invisible": "not from_invoice",
          "name": "qr_code",
          "string": "QR Code"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "from_invoice",
      "attrs": {
        "invisible": "1"
      }
    },
    {
      "name": "vat"
    },
    {
      "name": "account_number",
      "string": "Bank Account Number",
      "attrs": {
        "placeholder": "BE71096123456769",
        "required": "qr_code"
      }
    },
    {
      "name": "qr_code",
      "string": "QR Code",
      "attrs": {
        "invisible": "not from_invoice"
      }
    }
  ]
}

export function renderBaseDocumentLayoutXpath(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
