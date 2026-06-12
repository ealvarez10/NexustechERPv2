// Generado por odoo2rs — vista tree de discuss.channel.rtc.session (discuss_channel_rtc_session_view_tree).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "discuss_channel_rtc_session_view_tree",
  "name": "discuss.channel.rtc.session.list",
  "model": "discuss.channel.rtc.session",
  "type": "tree",
  "arch": {
    "tag": "list",
    "attrs": {
      "string": "RTC Session"
    },
    "children": [
      {
        "tag": "header",
        "children": [
          {
            "tag": "button",
            "attrs": {
              "name": "action_disconnect",
              "string": "Disconnect",
              "type": "object"
            }
          }
        ]
      },
      {
        "tag": "field",
        "attrs": {
          "name": "id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "channel_member_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "channel_id"
        }
      },
      {
        "tag": "field",
        "attrs": {
          "name": "write_date"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "id"
    },
    {
      "name": "channel_member_id"
    },
    {
      "name": "channel_id"
    },
    {
      "name": "write_date"
    }
  ],
  "buttons": [
    {
      "name": "action_disconnect",
      "string": "Disconnect",
      "type": "object"
    }
  ]
}

export function renderDiscussChannelRtcSessionTree(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
