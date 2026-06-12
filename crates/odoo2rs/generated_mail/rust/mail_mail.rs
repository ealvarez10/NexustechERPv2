//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.mail`

use nexus_orm::prelude::*;

pub struct MailMailFragment;

#[async_trait]
impl ModelFragment for MailMailFragment {
    fn model_name(&self) -> &str {
        "mail.mail"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Outgoing Mails".into();
        def.order = "id desc".into();
        def.rec_name = "subject".into();
        def.add_field(FieldDef::many2one("mail_message_id", "mail.message").string("Message").required());
        def.add_field(FieldDef::integer("mail_message_id_int").computed("_compute_mail_message_id_int", &[]).stored());
        def.add_field({ let mut f = FieldDef::selection("message_type", &[]).default_val("email_outgoing"); f.related = Some("mail_message_id.message_type".into()); f });
        def.add_field(FieldDef::text("body_html").string("Text Contents"));
        def.add_field(FieldDef::html("body_content").string("Rich-text Contents").computed("_compute_body_content", &[]).stored());
        def.add_field(FieldDef::text("references").string("References").readonly());
        def.add_field(FieldDef::text("headers").string("Headers"));
        def.add_field(FieldDef::integer("restricted_attachment_count").string("Restricted attachments").computed("_compute_restricted_attachments", &["attachment_ids"]).stored());
        def.add_field(FieldDef::many2many("unrestricted_attachment_ids", "ir.attachment").string("Unrestricted Attachments").computed("_compute_restricted_attachments", &["attachment_ids"]).stored());
        def.add_field(FieldDef::boolean("is_notification").string("Notification Email"));
        def.add_field(FieldDef::text("email_to").string("To"));
        def.add_field(FieldDef::char("email_cc").string("Cc"));
        def.add_field(FieldDef::many2many("recipient_ids", "res.partner").string("To (Partners)"));
        def.add_field(FieldDef::selection("state", &[("outgoing", "Outgoing"), ("sent", "Sent"), ("received", "Received"), ("exception", "Delivery Failed"), ("cancel", "Cancelled")]).string("Status").readonly().default_val("outgoing"));
        def.add_field(FieldDef::selection("failure_type", &[("unknown", "Unknown error"), ("mail_spam", "Detected As Spam"), ("mail_email_invalid", "Invalid email address"), ("mail_email_missing", "Missing email"), ("mail_from_invalid", "Invalid from address"), ("mail_from_missing", "Missing from address"), ("mail_smtp", "Connection failed (outgoing mail server problem)"), ("mail_bl", "Blacklisted Address"), ("mail_optout", "Opted Out"), ("mail_dup", "Duplicated Email")]).string("Failure type"));
        def.add_field(FieldDef::text("failure_reason").string("Failure Reason").readonly());
        def.add_field(FieldDef::boolean("auto_delete").string("Auto Delete"));
        def.add_field(FieldDef::datetime("scheduled_date").string("Scheduled Send Date"));
        def.add_field(FieldDef::many2one("fetchmail_server_id", "fetchmail.server").string("Inbound Mail Server").readonly());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["default_get", "_check_mail_server_id", "_compute_body_content", "_compute_mail_message_id_int", "_compute_restricted_attachments", "_inverse_unrestricted_attachment_ids", "_search_body_content", "create", "write", "unlink", "action_retry", "action_open_document", "mark_outgoing", "cancel", "process_email_queue", "_postprocess_sent_message", "_parse_scheduled_datetime", "_estimate_email_size", "_filter_mail_mail_servers", "_prepare_outgoing_body", "_personalize_outgoing_body", "_prepare_outgoing_list", "_split_by_mail_configuration", "_split_by_delayed_batch", "send_after_commit", "send", "action_send_and_close", "_send", "_get_notification_values", "_get_notification_status"]
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
            "_check_mail_server_id" => self._check_mail_server_id(env, ctx, rs, args).await,
            "_compute_body_content" => self._compute_body_content(env, ctx, rs, args).await,
            "_compute_mail_message_id_int" => self._compute_mail_message_id_int(env, ctx, rs, args).await,
            "_compute_restricted_attachments" => self._compute_restricted_attachments(env, ctx, rs, args).await,
            "_inverse_unrestricted_attachment_ids" => self._inverse_unrestricted_attachment_ids(env, ctx, rs, args).await,
            "_search_body_content" => self._search_body_content(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "action_retry" => self.action_retry(env, ctx, rs, args).await,
            "action_open_document" => self.action_open_document(env, ctx, rs, args).await,
            "mark_outgoing" => self.mark_outgoing(env, ctx, rs, args).await,
            "cancel" => self.cancel(env, ctx, rs, args).await,
            "process_email_queue" => self.process_email_queue(env, ctx, rs, args).await,
            "_postprocess_sent_message" => self._postprocess_sent_message(env, ctx, rs, args).await,
            "_parse_scheduled_datetime" => self._parse_scheduled_datetime(env, ctx, rs, args).await,
            "_estimate_email_size" => self._estimate_email_size(env, ctx, rs, args).await,
            "_filter_mail_mail_servers" => self._filter_mail_mail_servers(env, ctx, rs, args).await,
            "_prepare_outgoing_body" => self._prepare_outgoing_body(env, ctx, rs, args).await,
            "_personalize_outgoing_body" => self._personalize_outgoing_body(env, ctx, rs, args).await,
            "_prepare_outgoing_list" => self._prepare_outgoing_list(env, ctx, rs, args).await,
            "_split_by_mail_configuration" => self._split_by_mail_configuration(env, ctx, rs, args).await,
            "_split_by_delayed_batch" => self._split_by_delayed_batch(env, ctx, rs, args).await,
            "send_after_commit" => self.send_after_commit(env, ctx, rs, args).await,
            "send" => self.send(env, ctx, rs, args).await,
            "action_send_and_close" => self.action_send_and_close(env, ctx, rs, args).await,
            "_send" => self._send(env, ctx, rs, args).await,
            "_get_notification_values" => self._get_notification_values(env, ctx, rs, args).await,
            "_get_notification_status" => self._get_notification_status(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailMailFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:36`). Decoradores: api.model.
    async fn default_get(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail.default_get".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:102`). Decoradores: api.constrains('mail_message_id', 'mail_server_id').
    async fn _check_mail_server_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail._check_mail_server_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:107`).
    async fn _compute_body_content(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail._compute_body_content".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:111`).
    async fn _compute_mail_message_id_int(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail._compute_mail_message_id_int".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:116`). Decoradores: api.depends('attachment_ids').
    async fn _compute_restricted_attachments(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail._compute_restricted_attachments".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:125`).
    async fn _inverse_unrestricted_attachment_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail._inverse_unrestricted_attachment_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:131`).
    async fn _search_body_content(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail._search_body_content".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:135`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:156`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:165`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:173`).
    async fn action_retry(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail.action_retry".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:176`).
    async fn action_open_document(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail.action_open_document".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:187`).
    async fn mark_outgoing(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail.mark_outgoing".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:190`).
    async fn cancel(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail.cancel".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:194`). Decoradores: api.model.
    async fn process_email_queue(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail.process_email_queue".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:244`).
    async fn _postprocess_sent_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail._postprocess_sent_message".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:286`).
    async fn _parse_scheduled_datetime(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail._parse_scheduled_datetime".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:329`). Decoradores: api.model.
    async fn _estimate_email_size(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail._estimate_email_size".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:343`).
    async fn _filter_mail_mail_servers(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail._filter_mail_mail_servers".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:351`).
    async fn _prepare_outgoing_body(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail._prepare_outgoing_body".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:359`).
    async fn _personalize_outgoing_body(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail._personalize_outgoing_body".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:380`).
    async fn _prepare_outgoing_list(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail._prepare_outgoing_list".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:529`).
    async fn _split_by_mail_configuration(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail._split_by_mail_configuration".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:580`).
    async fn _split_by_delayed_batch(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail._split_by_delayed_batch".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:668`).
    async fn send_after_commit(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail.send_after_commit".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:690`).
    async fn send(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail.send".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:751`).
    async fn action_send_and_close(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail.action_send_and_close".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:763`).
    async fn _send(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail._send".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:981`).
    async fn _get_notification_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail._get_notification_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_mail.py:999`).
    async fn _get_notification_status(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.mail._get_notification_status".into(),
        ))
    }

}
