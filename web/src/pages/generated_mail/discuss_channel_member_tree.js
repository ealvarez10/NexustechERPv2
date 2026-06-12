// Generado por odoo2rs — vista tree de discuss.channel.member (discuss_channel_member_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "discuss_channel_member_view_tree",
  "name": "discuss.channel.member.list",
  "model": "discuss.channel.member",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "Channels"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "channel_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "partner_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "guest_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "is_pinned"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "last_seen_dt"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "last_interest_dt"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "channel_id"
    },
    {
      "name": "partner_id"
    },
    {
      "name": "guest_id"
    },
    {
      "name": "is_pinned"
    },
    {
      "name": "last_seen_dt"
    },
    {
      "name": "last_interest_dt"
    }
  ]
}

export function renderDiscussChannelMemberTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
