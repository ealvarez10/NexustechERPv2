//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `discuss.channel`

use nexus_orm::prelude::*;

pub struct DiscussChannelFragment;

#[async_trait]
impl ModelFragment for DiscussChannelFragment {
    fn model_name(&self) -> &str {
        "discuss.channel"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Discussion Channel".into();
        def.add_field(FieldDef::char("name").string("Name").required());
        def.add_field(FieldDef::boolean("active").default_val(true));
        def.add_field(FieldDef::selection("channel_type", &[("chat", "Chat"), ("channel", "Channel"), ("group", "Group")]).string("Channel Type").required().readonly().default_val("channel"));
        def.add_field(FieldDef::boolean("is_editable").string("Is Editable").computed("_compute_is_editable", &["channel_type", "is_member", "group_public_id"]).stored());
        def.add_field(FieldDef::selection("default_display_mode", &[("video_full_screen", "Full screen video")]).string("Default Display Mode"));
        def.add_field(FieldDef::text("description").string("Description"));
        def.add_field(FieldDef::new("image_128", FieldType::Binary).string("Image"));
        def.add_field(FieldDef::new("avatar_128", FieldType::Binary).string("Avatar").computed("_compute_avatar_128", &["channel_type", "image_128", "uuid"]).stored());
        def.add_field(FieldDef::char("avatar_cache_key").computed("_compute_avatar_cache_key", &["avatar_128"]).stored());
        def.add_field(FieldDef::many2many("channel_partner_ids", "res.partner").string("Partners").computed("_compute_channel_partner_ids", &["channel_member_ids.partner_id"]).stored());
        def.add_field(FieldDef::one2many("channel_member_ids", "discuss.channel.member", "channel_id").string("Members"));
        def.add_field(FieldDef::many2one("parent_channel_id", "discuss.channel").readonly());
        def.add_field(FieldDef::one2many("sub_channel_ids", "discuss.channel", "parent_channel_id").string("Sub Channels").readonly());
        def.add_field(FieldDef::many2one("from_message_id", "mail.message").readonly());
        def.add_field(FieldDef::one2many("pinned_message_ids", "mail.message", "res_id").string("Pinned Messages"));
        def.add_field(FieldDef::char("sfu_channel_uuid"));
        def.add_field(FieldDef::char("sfu_server_url"));
        def.add_field(FieldDef::one2many("rtc_session_ids", "discuss.channel.rtc.session", "channel_id"));
        def.add_field(FieldDef::one2many("call_history_ids", "discuss.call.history", "channel_id"));
        def.add_field(FieldDef::boolean("is_member").string("Is Member").computed("_compute_is_member", &["channel_member_ids"]).stored());
        def.add_field(FieldDef::many2one("self_member_id", "discuss.channel.member").computed("_compute_self_member_id", &["channel_member_ids"]).stored());
        // TODO(odoo2rs): campo 'invited_member_ids' (one2many) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::integer("member_count").string("Member Count").computed("_compute_member_count", &["channel_member_ids"]).stored());
        def.add_field(FieldDef::integer("message_count").string("# Messages").readonly().computed("_compute_message_count", &["message_ids"]).stored());
        def.add_field(FieldDef::datetime("last_interest_dt").string("Last Interest"));
        def.add_field(FieldDef::many2many("group_ids", "res.groups").string("Auto Subscription"));
        def.add_field(FieldDef::char("uuid").string("UUID"));
        def.add_field(FieldDef::many2one("group_public_id", "res.groups").string("Authorized Group").computed("_compute_group_public_id", &["channel_type", "parent_channel_id.group_public_id"]).stored());
        def.add_field(FieldDef::char("invitation_url").string("Invitation URL").computed("_compute_invitation_url", &["uuid"]).stored());
        // TODO(odoo2rs): campo 'channel_name_member_ids' (one2many) no generable — falta comodel/inverse o tipo sin equivalente.
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_generate_random_token", "_constraint_from_message_id", "_constraint_parent_channel_id", "_constraint_partners_chat", "_constraint_group_id_channel", "_compute_display_name", "_compute_channel_name_member_ids", "_compute_is_editable", "_compute_avatar_128", "_compute_avatar_cache_key", "_generate_avatar", "_compute_channel_partner_ids", "_inverse_channel_partner_ids", "_search_channel_partner_ids", "_compute_is_member", "_search_is_member", "_compute_self_member_id", "_compute_invited_member_ids", "_compute_member_count", "_compute_message_count", "_compute_group_public_id", "_compute_invitation_url", "_get_allowed_channel_member_create_params", "create", "_unlink_except_all_employee_channel", "write", "_sync_field_names", "_subscribe_users_automatically", "_subscribe_users_automatically_get_members", "action_unfollow", "_action_unfollow", "add_members", "_add_members", "invite_by_email", "_get_call_notification_tag", "_rtc_cancel_invitations", "_notify_get_recipients", "_notify_get_recipients_groups", "_get_notify_valid_parameters", "_notify_thread", "_notify_by_web_push_prepare_payload", "_notify_thread_by_web_push", "_message_receive_bounce", "_get_allowed_message_params", "_get_allowed_message_partner_ids", "message_post", "_message_post_after_hook", "_message_update_content", "_check_can_update_message_content", "_create_attachments_for_post", "_message_subscribe", "_should_invite_members_to_join_call", "_get_access_action", "_broadcast", "set_message_pin", "_find_or_create_member_for_self", "_find_or_create_persona_for_channel", "_get_channels_as_member", "_to_store_defaults", "_to_store", "_get_or_create_chat", "channel_pin", "_allow_invite_by_email", "_types_allowing_seen_infos", "_types_allowing_unfollow", "_member_based_naming_channel_types", "_lazy_load_members_channel_types", "channel_fetched", "channel_set_custom_name", "channel_rename", "channel_change_description", "channel_join", "_create_channel", "_create_group", "_create_sub_channel", "get_mention_suggestions", "_get_last_messages", "_clean_empty_message", "_get_store_message_update_extra_fields", "execute_command_help", "_execute_command_help_message_extra", "execute_command_leave", "execute_command_who"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_generate_random_token" => self._generate_random_token(env, ctx, rs, args).await,
            "_constraint_from_message_id" => self._constraint_from_message_id(env, ctx, rs, args).await,
            "_constraint_parent_channel_id" => self._constraint_parent_channel_id(env, ctx, rs, args).await,
            "_constraint_partners_chat" => self._constraint_partners_chat(env, ctx, rs, args).await,
            "_constraint_group_id_channel" => self._constraint_group_id_channel(env, ctx, rs, args).await,
            "_compute_display_name" => self._compute_display_name(env, ctx, rs, args).await,
            "_compute_channel_name_member_ids" => self._compute_channel_name_member_ids(env, ctx, rs, args).await,
            "_compute_is_editable" => self._compute_is_editable(env, ctx, rs, args).await,
            "_compute_avatar_128" => self._compute_avatar_128(env, ctx, rs, args).await,
            "_compute_avatar_cache_key" => self._compute_avatar_cache_key(env, ctx, rs, args).await,
            "_generate_avatar" => self._generate_avatar(env, ctx, rs, args).await,
            "_compute_channel_partner_ids" => self._compute_channel_partner_ids(env, ctx, rs, args).await,
            "_inverse_channel_partner_ids" => self._inverse_channel_partner_ids(env, ctx, rs, args).await,
            "_search_channel_partner_ids" => self._search_channel_partner_ids(env, ctx, rs, args).await,
            "_compute_is_member" => self._compute_is_member(env, ctx, rs, args).await,
            "_search_is_member" => self._search_is_member(env, ctx, rs, args).await,
            "_compute_self_member_id" => self._compute_self_member_id(env, ctx, rs, args).await,
            "_compute_invited_member_ids" => self._compute_invited_member_ids(env, ctx, rs, args).await,
            "_compute_member_count" => self._compute_member_count(env, ctx, rs, args).await,
            "_compute_message_count" => self._compute_message_count(env, ctx, rs, args).await,
            "_compute_group_public_id" => self._compute_group_public_id(env, ctx, rs, args).await,
            "_compute_invitation_url" => self._compute_invitation_url(env, ctx, rs, args).await,
            "_get_allowed_channel_member_create_params" => self._get_allowed_channel_member_create_params(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "_unlink_except_all_employee_channel" => self._unlink_except_all_employee_channel(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "_sync_field_names" => self._sync_field_names(env, ctx, rs, args).await,
            "_subscribe_users_automatically" => self._subscribe_users_automatically(env, ctx, rs, args).await,
            "_subscribe_users_automatically_get_members" => self._subscribe_users_automatically_get_members(env, ctx, rs, args).await,
            "action_unfollow" => self.action_unfollow(env, ctx, rs, args).await,
            "_action_unfollow" => self._action_unfollow(env, ctx, rs, args).await,
            "add_members" => self.add_members(env, ctx, rs, args).await,
            "_add_members" => self._add_members(env, ctx, rs, args).await,
            "invite_by_email" => self.invite_by_email(env, ctx, rs, args).await,
            "_get_call_notification_tag" => self._get_call_notification_tag(env, ctx, rs, args).await,
            "_rtc_cancel_invitations" => self._rtc_cancel_invitations(env, ctx, rs, args).await,
            "_notify_get_recipients" => self._notify_get_recipients(env, ctx, rs, args).await,
            "_notify_get_recipients_groups" => self._notify_get_recipients_groups(env, ctx, rs, args).await,
            "_get_notify_valid_parameters" => self._get_notify_valid_parameters(env, ctx, rs, args).await,
            "_notify_thread" => self._notify_thread(env, ctx, rs, args).await,
            "_notify_by_web_push_prepare_payload" => self._notify_by_web_push_prepare_payload(env, ctx, rs, args).await,
            "_notify_thread_by_web_push" => self._notify_thread_by_web_push(env, ctx, rs, args).await,
            "_message_receive_bounce" => self._message_receive_bounce(env, ctx, rs, args).await,
            "_get_allowed_message_params" => self._get_allowed_message_params(env, ctx, rs, args).await,
            "_get_allowed_message_partner_ids" => self._get_allowed_message_partner_ids(env, ctx, rs, args).await,
            "message_post" => self.message_post(env, ctx, rs, args).await,
            "_message_post_after_hook" => self._message_post_after_hook(env, ctx, rs, args).await,
            "_message_update_content" => self._message_update_content(env, ctx, rs, args).await,
            "_check_can_update_message_content" => self._check_can_update_message_content(env, ctx, rs, args).await,
            "_create_attachments_for_post" => self._create_attachments_for_post(env, ctx, rs, args).await,
            "_message_subscribe" => self._message_subscribe(env, ctx, rs, args).await,
            "_should_invite_members_to_join_call" => self._should_invite_members_to_join_call(env, ctx, rs, args).await,
            "_get_access_action" => self._get_access_action(env, ctx, rs, args).await,
            "_broadcast" => self._broadcast(env, ctx, rs, args).await,
            "set_message_pin" => self.set_message_pin(env, ctx, rs, args).await,
            "_find_or_create_member_for_self" => self._find_or_create_member_for_self(env, ctx, rs, args).await,
            "_find_or_create_persona_for_channel" => self._find_or_create_persona_for_channel(env, ctx, rs, args).await,
            "_get_channels_as_member" => self._get_channels_as_member(env, ctx, rs, args).await,
            "_to_store_defaults" => self._to_store_defaults(env, ctx, rs, args).await,
            "_to_store" => self._to_store(env, ctx, rs, args).await,
            "_get_or_create_chat" => self._get_or_create_chat(env, ctx, rs, args).await,
            "channel_pin" => self.channel_pin(env, ctx, rs, args).await,
            "_allow_invite_by_email" => self._allow_invite_by_email(env, ctx, rs, args).await,
            "_types_allowing_seen_infos" => self._types_allowing_seen_infos(env, ctx, rs, args).await,
            "_types_allowing_unfollow" => self._types_allowing_unfollow(env, ctx, rs, args).await,
            "_member_based_naming_channel_types" => self._member_based_naming_channel_types(env, ctx, rs, args).await,
            "_lazy_load_members_channel_types" => self._lazy_load_members_channel_types(env, ctx, rs, args).await,
            "channel_fetched" => self.channel_fetched(env, ctx, rs, args).await,
            "channel_set_custom_name" => self.channel_set_custom_name(env, ctx, rs, args).await,
            "channel_rename" => self.channel_rename(env, ctx, rs, args).await,
            "channel_change_description" => self.channel_change_description(env, ctx, rs, args).await,
            "channel_join" => self.channel_join(env, ctx, rs, args).await,
            "_create_channel" => self._create_channel(env, ctx, rs, args).await,
            "_create_group" => self._create_group(env, ctx, rs, args).await,
            "_create_sub_channel" => self._create_sub_channel(env, ctx, rs, args).await,
            "get_mention_suggestions" => self.get_mention_suggestions(env, ctx, rs, args).await,
            "_get_last_messages" => self._get_last_messages(env, ctx, rs, args).await,
            "_clean_empty_message" => self._clean_empty_message(env, ctx, rs, args).await,
            "_get_store_message_update_extra_fields" => self._get_store_message_update_extra_fields(env, ctx, rs, args).await,
            "execute_command_help" => self.execute_command_help(env, ctx, rs, args).await,
            "_execute_command_help_message_extra" => self._execute_command_help_message_extra(env, ctx, rs, args).await,
            "execute_command_leave" => self.execute_command_leave(env, ctx, rs, args).await,
            "execute_command_who" => self.execute_command_who(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl DiscussChannelFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:60`). Decoradores: api.model.
    async fn _generate_random_token(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._generate_random_token".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:135`). Decoradores: api.constrains('from_message_id').
    async fn _constraint_from_message_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._constraint_from_message_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:152`). Decoradores: api.constrains('parent_channel_id').
    async fn _constraint_parent_channel_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._constraint_parent_channel_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:170`). Decoradores: api.constrains('channel_member_ids').
    async fn _constraint_partners_chat(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._constraint_partners_chat".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:177`). Decoradores: api.constrains('group_public_id', 'group_ids').
    async fn _constraint_group_id_channel(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._constraint_group_id_channel".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:186`). Decoradores: api.depends('channel_name_member_ids', 'name').
    async fn _compute_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._compute_display_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:202`). Decoradores: api.depends('channel_member_ids').
    async fn _compute_channel_name_member_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._compute_channel_name_member_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:230`). Decoradores: api.depends('channel_type', 'is_member', 'group_public_id'), api.depends_context('uid').
    async fn _compute_is_editable(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._compute_is_editable".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:235`). Decoradores: api.depends('channel_type', 'image_128', 'uuid').
    async fn _compute_avatar_128(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._compute_avatar_128".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:240`). Decoradores: api.depends('avatar_128').
    async fn _compute_avatar_cache_key(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._compute_avatar_cache_key".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:247`).
    async fn _generate_avatar(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._generate_avatar".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:256`). Decoradores: api.depends('channel_member_ids.partner_id').
    async fn _compute_channel_partner_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._compute_channel_partner_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:260`).
    async fn _inverse_channel_partner_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._inverse_channel_partner_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:278`).
    async fn _search_channel_partner_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._search_channel_partner_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:283`). Decoradores: api.depends_context('uid', 'guest'), api.depends('channel_member_ids').
    async fn _compute_is_member(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._compute_is_member".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:287`).
    async fn _search_is_member(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._search_is_member".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:308`). Decoradores: api.depends_context('uid', 'guest'), api.depends('channel_member_ids').
    async fn _compute_self_member_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._compute_self_member_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:319`). Decoradores: api.depends('channel_member_ids.rtc_inviting_session_id').
    async fn _compute_invited_member_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._compute_invited_member_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:332`). Decoradores: api.depends('channel_member_ids').
    async fn _compute_member_count(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._compute_member_count".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:339`). Decoradores: api.depends('message_ids').
    async fn _compute_message_count(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._compute_message_count".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:352`). Decoradores: api.depends('channel_type', 'parent_channel_id.group_public_id').
    async fn _compute_group_public_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._compute_group_public_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:362`). Decoradores: api.depends('uuid').
    async fn _compute_invitation_url(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._compute_invitation_url".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:371`). Decoradores: api.model.
    async fn _get_allowed_channel_member_create_params(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._get_allowed_channel_member_create_params".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:375`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:426`). Decoradores: api.ondelete().
    async fn _unlink_except_all_employee_channel(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._unlink_except_all_employee_channel".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:437`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:503`).
    async fn _sync_field_names(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._sync_field_names".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:525`).
    async fn _subscribe_users_automatically(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._subscribe_users_automatically".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:550`).
    async fn _subscribe_users_automatically_get_members(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._subscribe_users_automatically_get_members".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:558`).
    async fn action_unfollow(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.action_unfollow".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:561`).
    async fn _action_unfollow(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._action_unfollow".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:595`).
    async fn add_members(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.add_members".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:606`).
    async fn _add_members(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._add_members".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:687`).
    async fn invite_by_email(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.invite_by_email".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:769`).
    async fn _get_call_notification_tag(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._get_call_notification_tag".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:773`).
    async fn _rtc_cancel_invitations(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._rtc_cancel_invitations".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:818`).
    async fn _notify_get_recipients(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._notify_get_recipients".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:920`).
    async fn _notify_get_recipients_groups(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._notify_get_recipients_groups".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:933`).
    async fn _get_notify_valid_parameters(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._get_notify_valid_parameters".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:936`).
    async fn _notify_thread(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._notify_thread".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:947`).
    async fn _notify_by_web_push_prepare_payload(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._notify_by_web_push_prepare_payload".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:971`).
    async fn _notify_thread_by_web_push(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._notify_thread_by_web_push".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:977`).
    async fn _message_receive_bounce(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._message_receive_bounce".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:984`).
    async fn _get_allowed_message_params(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._get_allowed_message_params".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:987`).
    async fn _get_allowed_message_partner_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._get_allowed_message_partner_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1006`).
    async fn message_post(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.message_post".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1026`).
    async fn _message_post_after_hook(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._message_post_after_hook".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1048`).
    async fn _message_update_content(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._message_update_content".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1053`).
    async fn _check_can_update_message_content(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._check_can_update_message_content".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1058`).
    async fn _create_attachments_for_post(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._create_attachments_for_post".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1069`).
    async fn _message_subscribe(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._message_subscribe".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1073`).
    async fn _should_invite_members_to_join_call(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._should_invite_members_to_join_call".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1077`).
    async fn _get_access_action(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._get_access_action".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1098`).
    async fn _broadcast(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._broadcast".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1118`).
    async fn set_message_pin(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.set_message_pin".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1157`).
    async fn _find_or_create_member_for_self(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._find_or_create_member_for_self".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1170`).
    async fn _find_or_create_persona_for_channel(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._find_or_create_persona_for_channel".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1207`). Decoradores: api.model.
    async fn _get_channels_as_member(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._get_channels_as_member".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1218`).
    async fn _to_store_defaults(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._to_store_defaults".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1313`).
    async fn _to_store(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._to_store".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1319`). Decoradores: api.model.
    async fn _get_or_create_chat(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._get_or_create_chat".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1396`).
    async fn channel_pin(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.channel_pin".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1409`).
    async fn _allow_invite_by_email(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._allow_invite_by_email".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1414`).
    async fn _types_allowing_seen_infos(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._types_allowing_seen_infos".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1419`).
    async fn _types_allowing_unfollow(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._types_allowing_unfollow".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1424`).
    async fn _member_based_naming_channel_types(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._member_based_naming_channel_types".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1430`).
    async fn _lazy_load_members_channel_types(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._lazy_load_members_channel_types".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1434`).
    async fn channel_fetched(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.channel_fetched".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1471`).
    async fn channel_set_custom_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.channel_set_custom_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1479`).
    async fn channel_rename(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.channel_rename".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1485`).
    async fn channel_change_description(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.channel_change_description".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1489`).
    async fn channel_join(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.channel_join".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1496`). Decoradores: api.model.
    async fn _create_channel(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._create_channel".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1516`). Decoradores: api.model.
    async fn _create_group(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._create_group".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1535`).
    async fn _create_sub_channel(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._create_sub_channel".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1577`). Decoradores: api.readonly, api.model.
    async fn get_mention_suggestions(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.get_mention_suggestions".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1592`).
    async fn _get_last_messages(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._get_last_messages".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1617`).
    async fn _clean_empty_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._clean_empty_message".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1621`).
    async fn _get_store_message_update_extra_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._get_store_message_update_extra_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1628`).
    async fn execute_command_help(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.execute_command_help".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1652`).
    async fn _execute_command_help_message_extra(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel._execute_command_help_message_extra".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1666`).
    async fn execute_command_leave(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.execute_command_leave".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_channel.py:1672`).
    async fn execute_command_who(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.channel.execute_command_who".into(),
        ))
    }

}
