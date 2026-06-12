//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.message`

use nexus_orm::prelude::*;

pub struct MailMessageFragment;

#[async_trait]
impl ModelFragment for MailMessageFragment {
    fn model_name(&self) -> &str {
        "mail.message"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Message".into();
        def.order = "id desc".into();
        def.rec_name = "subject".into();
        def.add_field(FieldDef::char("subject").string("Subject"));
        def.add_field(FieldDef::datetime("date").string("Date"));
        def.add_field(FieldDef::html("body").string("Contents").default_val(""));
        def.add_field(FieldDef::char("preview").string("Preview").computed("_compute_preview", &["body"]).stored());
        def.add_field(FieldDef::many2many("linked_message_ids", "mail.message").computed("_compute_linked_message_ids", &["body"]).stored());
        def.add_field(FieldDef::one2many("message_link_preview_ids", "mail.message.link.preview", "message_id"));
        def.add_field(FieldDef::one2many("reaction_ids", "mail.message.reaction", "message_id").string("Reactions"));
        def.add_field(FieldDef::many2many("attachment_ids", "ir.attachment").string("Attachments"));
        def.add_field(FieldDef::many2one("parent_id", "mail.message").string("Parent Message"));
        def.add_field(FieldDef::one2many("child_ids", "mail.message", "parent_id").string("Child Messages"));
        def.add_field(FieldDef::char("model").string("Related Document Model"));
        def.add_field(FieldDef::char("record_name").string("Message Record Name").computed("_compute_record_name", &["model", "res_id"]));
        def.add_field(FieldDef::many2one("record_alias_domain_id", "mail.alias.domain").string("Alias Domain"));
        def.add_field(FieldDef::many2one("record_company_id", "res.company").string("Company"));
        def.add_field(FieldDef::selection("message_type", &[("email", "Incoming Email"), ("comment", "Comment"), ("email_outgoing", "Outgoing Email"), ("notification", "System notification"), ("auto_comment", "Automated Targeted Notification"), ("out_of_office", "Out-of-office Message"), ("user_notification", "User Specific Notification")]).string("Type").required().default_val("comment"));
        def.add_field(FieldDef::many2one("subtype_id", "mail.message.subtype").string("Subtype"));
        def.add_field(FieldDef::many2one("mail_activity_type_id", "mail.activity.type").string("Mail Activity Type"));
        def.add_field(FieldDef::boolean("is_internal").string("Employee Only"));
        def.add_field(FieldDef::char("email_from").string("From"));
        def.add_field(FieldDef::many2one("author_id", "res.partner").string("Author"));
        def.add_field({ let mut f = FieldDef::new("author_avatar", FieldType::Binary).string("Author's avatar"); f.related = Some("author_id.avatar_128".into()); f });
        def.add_field(FieldDef::many2one("author_guest_id", "mail.guest").string("Guest"));
        def.add_field(FieldDef::boolean("is_current_user_or_guest_author").computed("_compute_is_current_user_or_guest_author", &["author_id", "author_guest_id"]).stored());
        def.add_field(FieldDef::many2many("partner_ids", "res.partner").string("Recipients"));
        def.add_field(FieldDef::text("incoming_email_to").string("Emails To"));
        def.add_field(FieldDef::char("incoming_email_cc").string("Emails Cc"));
        def.add_field(FieldDef::char("outgoing_email_to").string("emails To"));
        def.add_field(FieldDef::many2many("notified_partner_ids", "res.partner").string("Partners with Need Action"));
        def.add_field(FieldDef::boolean("needaction").string("Need Action").computed("_compute_needaction", &[]).stored());
        def.add_field(FieldDef::boolean("has_error").string("Has error").computed("_compute_has_error", &[]).stored());
        def.add_field(FieldDef::one2many("notification_ids", "mail.notification", "mail_message_id").string("Notifications"));
        def.add_field(FieldDef::many2many("starred_partner_ids", "res.partner").string("Favorited By"));
        def.add_field(FieldDef::datetime("pinned_at").string("Pinned"));
        def.add_field(FieldDef::boolean("starred").string("Starred").computed("_compute_starred", &["starred_partner_ids"]).stored());
        def.add_field(FieldDef::one2many("tracking_value_ids", "mail.tracking.value", "mail_message_id").string("Tracking values"));
        def.add_field(FieldDef::boolean("reply_to_force_new").string("No threading for answers"));
        def.add_field(FieldDef::char("message_id").string("Message-Id").readonly());
        def.add_field(FieldDef::char("reply_to").string("Reply-To"));
        def.add_field(FieldDef::many2one("mail_server_id", "ir.mail_server").string("Outgoing mail server"));
        def.add_field(FieldDef::char("email_layout_xmlid").string("Layout"));
        def.add_field(FieldDef::boolean("email_add_signature").default_val(true));
        def.add_field(FieldDef::one2many("mail_ids", "mail.mail", "mail_message_id").string("Mails"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["default_get", "_compute_preview", "_compute_linked_message_ids", "_compute_record_name", "_compute_is_current_user_or_guest_author", "_compute_needaction", "_search_needaction", "_compute_has_error", "_search_has_error", "_compute_starred", "_search_starred", "_search", "_get_search_domain_share", "_filter_records_for_message_operation", "_find_allowed_doc_ids", "_check_access", "_get_forbidden_access", "_make_access_error", "_get_with_access", "create", "read", "copy_data", "fetch", "write", "unlink", "export_data", "action_open_document", "mark_all_as_read", "set_message_done", "unstar_all", "toggle_message_starred", "_message_fetch", "_get_tracking_values_domain", "_message_reaction", "_bus_send_reaction_group", "_reaction_group_to_store", "_field_store_repr", "_to_store_defaults", "_to_store", "_get_store_partner_name_fields", "_get_store_attachment_fields", "_get_store_linked_messages_fields", "_extras_to_store", "_message_notifications_to_store", "_notify_message_notification_update", "_bus_channel", "_filter_empty", "_is_empty", "_get_reply_to", "_get_message_id", "_is_thread_message", "_is_thread_message_visible", "_invalidate_documents", "_records_by_model_name", "_record_by_message"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "default_get" => self.default_get(env, ctx, rs, args).await,
            "_compute_preview" => self._compute_preview(env, ctx, rs, args).await,
            "_compute_linked_message_ids" => self._compute_linked_message_ids(env, ctx, rs, args).await,
            "_compute_record_name" => self._compute_record_name(env, ctx, rs, args).await,
            "_compute_is_current_user_or_guest_author" => self._compute_is_current_user_or_guest_author(env, ctx, rs, args).await,
            "_compute_needaction" => self._compute_needaction(env, ctx, rs, args).await,
            "_search_needaction" => self._search_needaction(env, ctx, rs, args).await,
            "_compute_has_error" => self._compute_has_error(env, ctx, rs, args).await,
            "_search_has_error" => self._search_has_error(env, ctx, rs, args).await,
            "_compute_starred" => self._compute_starred(env, ctx, rs, args).await,
            "_search_starred" => self._search_starred(env, ctx, rs, args).await,
            "_search" => self._search(env, ctx, rs, args).await,
            "_get_search_domain_share" => self._get_search_domain_share(env, ctx, rs, args).await,
            "_filter_records_for_message_operation" => self._filter_records_for_message_operation(env, ctx, rs, args).await,
            "_find_allowed_doc_ids" => self._find_allowed_doc_ids(env, ctx, rs, args).await,
            "_check_access" => self._check_access(env, ctx, rs, args).await,
            "_get_forbidden_access" => self._get_forbidden_access(env, ctx, rs, args).await,
            "_make_access_error" => self._make_access_error(env, ctx, rs, args).await,
            "_get_with_access" => self._get_with_access(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "read" => self.read(env, ctx, rs, args).await,
            "copy_data" => self.copy_data(env, ctx, rs, args).await,
            "fetch" => self.fetch(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "export_data" => self.export_data(env, ctx, rs, args).await,
            "action_open_document" => self.action_open_document(env, ctx, rs, args).await,
            "mark_all_as_read" => self.mark_all_as_read(env, ctx, rs, args).await,
            "set_message_done" => self.set_message_done(env, ctx, rs, args).await,
            "unstar_all" => self.unstar_all(env, ctx, rs, args).await,
            "toggle_message_starred" => self.toggle_message_starred(env, ctx, rs, args).await,
            "_message_fetch" => self._message_fetch(env, ctx, rs, args).await,
            "_get_tracking_values_domain" => self._get_tracking_values_domain(env, ctx, rs, args).await,
            "_message_reaction" => self._message_reaction(env, ctx, rs, args).await,
            "_bus_send_reaction_group" => self._bus_send_reaction_group(env, ctx, rs, args).await,
            "_reaction_group_to_store" => self._reaction_group_to_store(env, ctx, rs, args).await,
            "_field_store_repr" => self._field_store_repr(env, ctx, rs, args).await,
            "_to_store_defaults" => self._to_store_defaults(env, ctx, rs, args).await,
            "_to_store" => self._to_store(env, ctx, rs, args).await,
            "_get_store_partner_name_fields" => self._get_store_partner_name_fields(env, ctx, rs, args).await,
            "_get_store_attachment_fields" => self._get_store_attachment_fields(env, ctx, rs, args).await,
            "_get_store_linked_messages_fields" => self._get_store_linked_messages_fields(env, ctx, rs, args).await,
            "_extras_to_store" => self._extras_to_store(env, ctx, rs, args).await,
            "_message_notifications_to_store" => self._message_notifications_to_store(env, ctx, rs, args).await,
            "_notify_message_notification_update" => self._notify_message_notification_update(env, ctx, rs, args).await,
            "_bus_channel" => self._bus_channel(env, ctx, rs, args).await,
            "_filter_empty" => self._filter_empty(env, ctx, rs, args).await,
            "_is_empty" => self._is_empty(env, ctx, rs, args).await,
            "_get_reply_to" => self._get_reply_to(env, ctx, rs, args).await,
            "_get_message_id" => self._get_message_id(env, ctx, rs, args).await,
            "_is_thread_message" => self._is_thread_message(env, ctx, rs, args).await,
            "_is_thread_message_visible" => self._is_thread_message_visible(env, ctx, rs, args).await,
            "_invalidate_documents" => self._invalidate_documents(env, ctx, rs, args).await,
            "_records_by_model_name" => self._records_by_model_name(env, ctx, rs, args).await,
            "_record_by_message" => self._record_by_message(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailMessageFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:78`). Decoradores: api.model.
    async fn default_get(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.default_get".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:209`). Decoradores: api.depends('body').
    async fn _compute_preview(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._compute_preview".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:219`). Decoradores: api.depends_context('uid'), api.depends('body').
    async fn _compute_linked_message_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._compute_linked_message_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:245`). Decoradores: api.depends('model', 'res_id').
    async fn _compute_record_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._compute_record_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:257`). Decoradores: api.depends('author_id', 'author_guest_id'), api.depends_context('guest', 'uid').
    async fn _compute_is_current_user_or_guest_author(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._compute_is_current_user_or_guest_author".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:268`).
    async fn _compute_needaction(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._compute_needaction".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:278`). Decoradores: api.model.
    async fn _search_needaction(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._search_needaction".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:285`).
    async fn _compute_has_error(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._compute_has_error".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:292`).
    async fn _search_has_error(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._search_has_error".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:299`). Decoradores: api.depends('starred_partner_ids'), api.depends_context('uid').
    async fn _compute_starred(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._compute_starred".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:307`). Decoradores: api.model.
    async fn _search_starred(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._search_starred".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:317`). Decoradores: api.model.
    async fn _search(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._search".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:397`).
    async fn _get_search_domain_share(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._get_search_domain_share".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:400`).
    async fn _filter_records_for_message_operation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._filter_records_for_message_operation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:428`). Decoradores: api.model.
    async fn _find_allowed_doc_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._find_allowed_doc_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:453`).
    async fn _check_access(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._check_access".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:493`).
    async fn _get_forbidden_access(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._get_forbidden_access".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:643`).
    async fn _make_access_error(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._make_access_error".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:656`). Decoradores: api.model.
    async fn _get_with_access(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._get_with_access".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:686`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:789`).
    async fn read(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.read".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:795`).
    async fn copy_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.copy_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:801`).
    async fn fetch(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.fetch".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:810`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:826`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:849`).
    async fn export_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.export_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:859`).
    async fn action_open_document(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.action_open_document".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:875`). Decoradores: api.model.
    async fn mark_all_as_read(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.mark_all_as_read".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:898`).
    async fn set_message_done(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.set_message_done".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:919`). Decoradores: api.model.
    async fn unstar_all(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.unstar_all".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:928`).
    async fn toggle_message_starred(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.toggle_message_starred".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:947`). Decoradores: api.model.
    async fn _message_fetch(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._message_fetch".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:995`).
    async fn _get_tracking_values_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._get_tracking_values_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1028`).
    async fn _message_reaction(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._message_reaction".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1055`).
    async fn _bus_send_reaction_group(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._bus_send_reaction_group".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1060`).
    async fn _reaction_group_to_store(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._reaction_group_to_store".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1074`).
    async fn _field_store_repr(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._field_store_repr".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1095`).
    async fn _to_store_defaults(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._to_store_defaults".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1157`).
    async fn _to_store(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._to_store".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1278`).
    async fn _get_store_partner_name_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._get_store_partner_name_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1282`).
    async fn _get_store_attachment_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._get_store_attachment_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1288`).
    async fn _get_store_linked_messages_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._get_store_linked_messages_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1314`).
    async fn _extras_to_store(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._extras_to_store".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1317`).
    async fn _message_notifications_to_store(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._message_notifications_to_store".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1350`).
    async fn _notify_message_notification_update(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._notify_message_notification_update".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1380`).
    async fn _bus_channel(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._bus_channel".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1387`).
    async fn _filter_empty(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._filter_empty".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1391`).
    async fn _is_empty(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._is_empty".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1404`). Decoradores: api.model.
    async fn _get_reply_to(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._get_reply_to".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1419`). Decoradores: api.model.
    async fn _get_message_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._get_message_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1428`).
    async fn _is_thread_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._is_thread_message".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1435`).
    async fn _is_thread_message_visible(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._is_thread_message_visible".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1444`).
    async fn _invalidate_documents(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._invalidate_documents".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1454`).
    async fn _records_by_model_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._records_by_model_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message.py:1467`).
    async fn _record_by_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._record_by_message".into(),
        ))
    }

}
