// Generado por odoo2rs — vista tree de mail.link.preview (mail_link_preview_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "mail_link_preview_view_tree",
  "name": "mail.link.preview.list",
  "model": "mail.link.preview",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Link Previews"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "source_url"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "og_title"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "og_type"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "image_mimetype"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "id"
    },
    {
      "name": "source_url"
    },
    {
      "name": "og_title"
    },
    {
      "name": "og_type"
    },
    {
      "name": "image_mimetype"
    }
  ]
}

export function renderMailLinkPreviewTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
