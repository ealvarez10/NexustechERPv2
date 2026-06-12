// Generado por odoo2rs — vista tree de discuss.gif.favorite (discuss_gif_favorite_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "discuss_gif_favorite_view_tree",
  "name": "discuss.gif.favorite.list",
  "model": "discuss.gif.favorite",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "GIF favorite"
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
          "name": "tenor_gif_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "create_uid"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "id"
    },
    {
      "name": "tenor_gif_id"
    },
    {
      "name": "create_uid"
    }
  ]
}

export function renderDiscussGifFavoriteTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
