//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `discuss.channel.member`

use nexus_orm::prelude::*;

pub struct DiscussChannelMemberFragment;

#[async_trait]
impl ModelFragment for DiscussChannelMemberFragment {
    fn model_name(&self) -> &str {
        "discuss.channel.member"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Channel Member".into();
        def.add_field(FieldDef::many2one("partner_id", "res.partner").string("Partner"));
        def.add_field(FieldDef::many2one("guest_id", "mail.guest").string("Guest"));
        def.add_field(FieldDef::boolean("is_self").computed("_compute_is_self", &[]).stored());
        def.add_field(FieldDef::many2one("channel_id", "discuss.channel").string("Channel").required());
        def.add_field(FieldDef::char("custom_channel_name").string("Custom channel name"));
        def.add_field(FieldDef::many2one("fetched_message_id", "mail.message").string("Last Fetched"));
        def.add_field(FieldDef::many2one("seen_message_id", "mail.message").string("Last Seen"));
        def.add_field(FieldDef::integer("new_message_separator").required().default_val(0i64));
        def.add_field(FieldDef::integer("message_unread_counter").string("Unread Messages Counter").computed("_compute_message_unread", &["channel_id.message_ids", "new_message_separator"]).stored());
        def.add_field(FieldDef::selection("custom_notifications", &[("all", "All Messages"), ("mentions", "Mentions Only"), ("no_notif", "Nothing")]).string("Customized Notifications"));
        def.add_field(FieldDef::datetime("mute_until_dt").string("Mute notifications until"));
        def.add_field(FieldDef::boolean("is_pinned").string("Is pinned on the interface").computed("_compute_is_pinned", &["last_interest_dt", "unpin_dt", "channel_id.last_interest_dt"]).stored());
        def.add_field(FieldDef::datetime("unpin_dt").string("Unpin date"));
        def.add_field(FieldDef::datetime("last_interest_dt").string("Last Interest"));
        def.add_field(FieldDef::datetime("last_seen_dt").string("Last seen date"));
        def.add_field(FieldDef::one2many("rtc_session_ids", "discuss.channel.rtc.session", "channel_member_id").string("RTC Sessions"));
        def.add_field(FieldDef::many2one("rtc_inviting_session_id", "discuss.channel.rtc.session").string("Ringing session"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_gc_unpin_outdated_sub_channels", "_contrains_no_public_member", "_compute_is_self", "_search_is_self", "_search_is_pinned", "_compute_message_unread", "_compute_display_name", "_compute_is_pinned", "create", "write", "_sync_field_names", "unlink", "_bus_channel", "_notify_typing", "_notify_mute", "_cleanup_expired_mutes", "_to_store_persona", "_to_store_defaults", "_get_store_partner_fields", "_get_store_guest_fields", "_rtc_join_call", "_join_sfu", "_get_rtc_server_info", "_rtc_leave_call", "_rtc_sync_sessions", "_get_rtc_invite_members_domain", "_rtc_invite_members", "_mark_as_read", "_set_last_seen_message", "_set_new_message_separator", "_get_html_link_title", "_get_html_link"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_gc_unpin_outdated_sub_channels" => self._gc_unpin_outdated_sub_channels(env, ctx, rs, args).await,
            "_contrains_no_public_member" => self._contrains_no_public_member(env, ctx, rs, args).await,
            "_compute_is_self" => self._compute_is_self(env, ctx, rs, args).await,
            "_search_is_self" => self._search_is_self(env, ctx, rs, args).await,
            "_search_is_pinned" => self._search_is_pinned(env, ctx, rs, args).await,
            "_compute_message_unread" => self._compute_message_unread(env, ctx, rs, args).await,
            "_compute_display_name" => self._compute_display_name(env, ctx, rs, args).await,
            "_compute_is_pinned" => self._compute_is_pinned(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "_sync_field_names" => self._sync_field_names(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "_bus_channel" => self._bus_channel(env, ctx, rs, args).await,
            "_notify_typing" => self._notify_typing(env, ctx, rs, args).await,
            "_notify_mute" => self._notify_mute(env, ctx, rs, args).await,
            "_cleanup_expired_mutes" => self._cleanup_expired_mutes(env, ctx, rs, args).await,
            "_to_store_persona" => self._to_store_persona(env, ctx, rs, args).await,
            "_to_store_defaults" => self._to_store_defaults(env, ctx, rs, args).await,
            "_get_store_partner_fields" => self._get_store_partner_fields(env, ctx, rs, args).await,
            "_get_store_guest_fields" => self._get_store_guest_fields(env, ctx, rs, args).await,
            "_rtc_join_call" => self._rtc_join_call(env, ctx, rs, args).await,
            "_join_sfu" => self._join_sfu(env, ctx, rs, args).await,
            "_get_rtc_server_info" => self._get_rtc_server_info(env, ctx, rs, args).await,
            "_rtc_leave_call" => self._rtc_leave_call(env, ctx, rs, args).await,
            "_rtc_sync_sessions" => self._rtc_sync_sessions(env, ctx, rs, args).await,
            "_get_rtc_invite_members_domain" => self._get_rtc_invite_members_domain(env, ctx, rs, args).await,
            "_rtc_invite_members" => self._rtc_invite_members(env, ctx, rs, args).await,
            "_mark_as_read" => self._mark_as_read(env, ctx, rs, args).await,
            "_set_last_seen_message" => self._set_last_seen_message(env, ctx, rs, args).await,
            "_set_new_message_separator" => self._set_new_message_separator(env, ctx, rs, args).await,
            "_get_html_link_title" => self._get_html_link_title(env, ctx, rs, args).await,
            "_get_html_link" => self._get_html_link(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl DiscussChannelMemberFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:63`). Decoradores: api.autovacuum.
    async fn _gc_unpin_outdated_sub_channels(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._gc_unpin_outdated_sub_channels".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:103`). Decoradores: api.constrains('partner_id').
    async fn _contrains_no_public_member(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._contrains_no_public_member".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:109`). Decoradores: api.depends_context('uid', 'guest').
    async fn _compute_is_self(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._compute_is_self".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:120`).
    async fn _search_is_self(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._search_is_self".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:128`).
    async fn _search_is_pinned(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._search_is_pinned".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:153`). Decoradores: api.depends('channel_id.message_ids', 'new_message_separator').
    async fn _compute_message_unread(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._compute_message_unread".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:176`). Decoradores: api.depends('partner_id.name', 'guest_id.name', 'channel_id.display_name').
    async fn _compute_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._compute_display_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:185`). Decoradores: api.depends('last_interest_dt', 'unpin_dt', 'channel_id.last_interest_dt').
    async fn _compute_is_pinned(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._compute_is_pinned".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:207`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:246`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:290`). Decoradores: api.model.
    async fn _sync_field_names(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._sync_field_names".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:307`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:338`).
    async fn _bus_channel(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._bus_channel".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:341`).
    async fn _notify_typing(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._notify_typing".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:351`).
    async fn _notify_mute(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._notify_mute".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:357`). Decoradores: api.model.
    async fn _cleanup_expired_mutes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._cleanup_expired_mutes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:365`).
    async fn _to_store_persona(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._to_store_persona".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:389`).
    async fn _to_store_defaults(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._to_store_defaults".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:399`).
    async fn _get_store_partner_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._get_store_partner_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:403`).
    async fn _get_store_guest_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._get_store_guest_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:411`).
    async fn _rtc_join_call(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._rtc_join_call".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:445`).
    async fn _join_sfu(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._join_sfu".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:485`).
    async fn _get_rtc_server_info(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._get_rtc_server_info".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:499`).
    async fn _rtc_leave_call(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._rtc_leave_call".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:509`).
    async fn _rtc_sync_sessions(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._rtc_sync_sessions".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:525`).
    async fn _get_rtc_invite_members_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._get_rtc_invite_members_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:546`).
    async fn _rtc_invite_members(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._rtc_invite_members".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:614`).
    async fn _mark_as_read(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._mark_as_read".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:633`).
    async fn _set_last_seen_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._set_last_seen_message".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:662`).
    async fn _set_new_message_separator(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._set_new_message_separator".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:683`).
    async fn _get_html_link_title(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._get_html_link_title".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel_member.py:686`).
    async fn _get_html_link(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.member._get_html_link".into(),
        ))
    }

}
