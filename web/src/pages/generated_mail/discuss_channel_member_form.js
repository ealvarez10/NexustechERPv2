// Generado por odoo2rs — vista form de discuss.channel.member (discuss_channel_member_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "discuss_channel_member_view_form",
  "name": "discuss.channel.member.form",
  "model": "discuss.channel.member",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "Channel Member"
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
                  "name": "channel_id",
                  "readonly": "id"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "partner_id",
                  "readonly": "id or guest_id",
                  "required": "not guest_id"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "guest_id",
                  "readonly": "id or partner_id",
                  "required": "not partner_id"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "custom_channel_name"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "fetched_message_id"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "seen_message_id"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "new_message_separator"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "message_unread_counter"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "custom_notifications"
                }
              },
              {
                "tag": "field",
                "attrs": {
                  "name": "mute_until_dt"
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
                  "name": "last_interest_dt"
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
                  "name": "rtc_inviting_session_id"
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
      "name": "channel_id",
      "attrs": {
        "readonly": "id"
      }
    },
    {
      "name": "partner_id",
      "attrs": {
        "readonly": "id or guest_id",
        "required": "not guest_id"
      }
    },
    {
      "name": "guest_id",
      "attrs": {
        "readonly": "id or partner_id",
        "required": "not partner_id"
      }
    },
    {
      "name": "custom_channel_name"
    },
    {
      "name": "fetched_message_id"
    },
    {
      "name": "seen_message_id"
    },
    {
      "name": "new_message_separator"
    },
    {
      "name": "message_unread_counter"
    },
    {
      "name": "custom_notifications"
    },
    {
      "name": "mute_until_dt"
    },
    {
      "name": "is_pinned"
    },
    {
      "name": "last_interest_dt"
    },
    {
      "name": "last_seen_dt"
    },
    {
      "name": "rtc_inviting_session_id"
    }
  ]
}

export function renderDiscussChannelMemberForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'discuss.channel.member' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/discuss.channel.member/<método> (≈ call_kw)
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
