//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_scheduled_message.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.scheduled.message`

use nexus_orm::prelude::*;

pub struct MailScheduledMessageFragment;

#[async_trait]
impl ModelFragment for MailScheduledMessageFragment {
    fn model_name(&self) -> &str {
        "mail.scheduled.message"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Scheduled Message".into();
        def.add_field(FieldDef::char("subject").string("Subject"));
        def.add_field(FieldDef::html("body").string("Contents"));
        def.add_field(FieldDef::datetime("scheduled_date").string("Scheduled Date").required());
        def.add_field(FieldDef::many2many("attachment_ids", "ir.attachment").string("Attachments"));
        def.add_field(FieldDef::selection("composition_comment_option", &[("reply_all", "Reply-All"), ("forward", "Forward")]).string("Comment Options"));
        def.add_field(FieldDef::char("model").string("Related Document Model").required());
        def.add_field(FieldDef::many2one("author_id", "res.partner").string("Author").required());
        def.add_field(FieldDef::many2many("partner_ids", "res.partner").string("Recipients"));
        def.add_field(FieldDef::boolean("is_note").string("Is a note").default_val(false));
        def.add_field(FieldDef::text("notification_parameters").string("Notification parameters"));
        def.add_field(FieldDef::json("send_context").string("Sending Context"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_check_model", "_check_scheduled_date", "create", "_search", "unlink", "write", "open_edit_form", "post_message", "_message_created_hook", "_post_message", "_check", "_notification_parameters_whitelist", "_post_messages_cron", "_to_store_defaults"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_check_model" => self._check_model(env, ctx, rs, args).await,
            "_check_scheduled_date" => self._check_scheduled_date(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "_search" => self._search(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "open_edit_form" => self.open_edit_form(env, ctx, rs, args).await,
            "post_message" => self.post_message(env, ctx, rs, args).await,
            "_message_created_hook" => self._message_created_hook(env, ctx, rs, args).await,
            "_post_message" => self._post_message(env, ctx, rs, args).await,
            "_check" => self._check(env, ctx, rs, args).await,
            "_notification_parameters_whitelist" => self._notification_parameters_whitelist(env, ctx, rs, args).await,
            "_post_messages_cron" => self._post_messages_cron(env, ctx, rs, args).await,
            "_to_store_defaults" => self._to_store_defaults(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailScheduledMessageFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_scheduled_message.py:60`). Decoradores: api.constrains('model').
    async fn _check_model(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.scheduled.message._check_model".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_scheduled_message.py:65`). Decoradores: api.constrains('scheduled_date').
    async fn _check_scheduled_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.scheduled.message._check_scheduled_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_scheduled_message.py:74`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.scheduled.message.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_scheduled_message.py:98`). Decoradores: api.model.
    async fn _search(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.scheduled.message._search".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_scheduled_message.py:132`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.scheduled.message.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_scheduled_message.py:136`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.scheduled.message.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_scheduled_message.py:151`).
    async fn open_edit_form(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.scheduled.message.open_edit_form".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_scheduled_message.py:163`).
    async fn post_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.scheduled.message.post_message".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_scheduled_message.py:170`).
    async fn _message_created_hook(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.scheduled.message._message_created_hook".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_scheduled_message.py:174`).
    async fn _post_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.scheduled.message._post_message".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_scheduled_message.py:235`). Decoradores: api.model.
    async fn _check(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.scheduled.message._check".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_scheduled_message.py:256`). Decoradores: api.model.
    async fn _notification_parameters_whitelist(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.scheduled.message._notification_parameters_whitelist".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_scheduled_message.py:275`). Decoradores: api.model.
    async fn _post_messages_cron(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.scheduled.message._post_messages_cron".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_scheduled_message.py:287`).
    async fn _to_store_defaults(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.scheduled.message._to_store_defaults".into(),
        ))
    }

}
