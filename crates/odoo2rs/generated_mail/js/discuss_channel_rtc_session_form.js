// Generado por odoo2rs — vista form de discuss.channel.rtc.session (discuss_channel_rtc_session_view_form).
// NO EDITAR A MANO; regenerar con `odoo2rs gen-js`.

import { renderFormPage } from '../components/form_view.js'

export const DESCRIPTOR = {
  "xml_id": "discuss_channel_rtc_session_view_form",
  "name": "discuss.channel.rtc.session.form",
  "model": "discuss.channel.rtc.session",
  "type": "form",
  "arch": {
    "tag": "form",
    "attrs": {
      "string": "RTC Session"
    },
    "children": [
      {
        "tag": "sheet",
        "children": [
          {
            "tag": "div",
            "attrs": {
              "class": "oe_title"
            },
            "children": [
              {
                "tag": "h1",
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "channel_member_id"
                    }
                  }
                ]
              }
            ]
          },
          {
            "tag": "group",
            "children": [
              {
                "tag": "group",
                "attrs": {
                  "string": "Identity"
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
                  }
                ]
              },
              {
                "tag": "group",
                "attrs": {
                  "string": "State"
                },
                "children": [
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "is_screen_sharing_on"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "is_camera_on"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "is_muted"
                    }
                  },
                  {
                    "tag": "field",
                    "attrs": {
                      "name": "is_deaf"
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
      "name": "channel_member_id"
    },
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
      "name": "is_screen_sharing_on"
    },
    {
      "name": "is_camera_on"
    },
    {
      "name": "is_muted"
    },
    {
      "name": "is_deaf"
    }
  ]
}

export function renderDiscussChannelRtcSessionForm(record = {}) {
  renderFormPage({
    breadcrumb: [{ label: 'discuss.channel.rtc.session' }],
    title: record.name || record.display_name || 'Nuevo',
    currentStatus: record.state || '',
    // TODO(odoo2rs): statusSteps desde la selection del campo state del modelo
    statusSteps: [],
    statusButtons: DESCRIPTOR.buttons?.map(b => ({
      label: b.string || b.name,
      primary: (b.class || '').includes('btn-primary'),
      // TODO(odoo2rs): conectar a /api/v1/orm/discuss.channel.rtc.session/<método> (≈ call_kw)
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
