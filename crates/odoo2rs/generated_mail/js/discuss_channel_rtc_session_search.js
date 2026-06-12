// Generado por odoo2rs — vista search de discuss.channel.rtc.session (discuss_channel_rtc_session_view_search).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

export const DESCRIPTOR = {
  "xml_id": "discuss_channel_rtc_session_view_search",
  "name": "discuss.channel.rtc.session.search",
  "model": "discuss.channel.rtc.session",
  "type": "search",
  "arch": {
    "tag": "search",
    "attrs": {
      "string": "Search RTC session"
    },
    "children": [
      {
        "tag": "field",
        "attrs": {
          "name": "channel_member_id"
        }
      },
      {
        "tag": "filter",
        "attrs": {
          "context": "{'group_by':'channel_id'}",
          "domain": "[]",
          "name": "group_by_channel",
          "string": "Channel"
        }
      }
    ]
  },
  "fields": [
    {
      "name": "channel_member_id"
    }
  ]
}

export function renderDiscussChannelRtcSessionSearch(records = []) {
  const cols = DESCRIPTOR.fields
  return `
    <table class="o-list-table">
      <thead><tr>${cols.map(c => `<th>${c.string || c.name}</th>`).join('')}</tr></thead>
      <tbody>
        ${records.map(r => `<tr>${cols.map(c => `<td>${r[c.name] ?? ''}</td>`).join('')}</tr>`).join('')}
      </tbody>
    </table>`
}
