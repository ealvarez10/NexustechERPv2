//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_notification.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.notification`

use nexus_orm::prelude::*;

pub struct MailNotificationFragment;

#[async_trait]
impl ModelFragment for MailNotificationFragment {
    fn model_name(&self) -> &str {
        "mail.notification"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.table = "mail_notification".into();
        def.description = "Message Notifications".into();
        def.rec_name = "res_partner_id".into();
        def.add_field(FieldDef::many2one("author_id", "res.partner").string("Author"));
        def.add_field(FieldDef::many2one("mail_message_id", "mail.message").string("Message").required());
        def.add_field(FieldDef::many2one("mail_mail_id", "mail.mail").string("Mail"));
        def.add_field(FieldDef::many2one("res_partner_id", "res.partner").string("Recipient"));
        def.add_field(FieldDef::char("mail_email_address"));
        def.add_field(FieldDef::selection("notification_type", &[("inbox", "Inbox"), ("email", "Email")]).string("Notification Type").required().default_val("inbox"));
        def.add_field(FieldDef::selection("notification_status", &[("ready", "Ready to Send"), ("process", "Processing"), ("pending", "Sent"), ("sent", "Delivered"), ("bounce", "Bounced"), ("exception", "Exception"), ("canceled", "Cancelled")]).string("Status").default_val("ready"));
        def.add_field(FieldDef::boolean("is_read").string("Is Read"));
        def.add_field(FieldDef::datetime("read_date").string("Read Date"));
        def.add_field(FieldDef::selection("failure_type", &[("unknown", "Unknown error"), ("mail_bounce", "Bounce"), ("mail_spam", "Detected As Spam"), ("mail_email_invalid", "Invalid email address"), ("mail_email_missing", "Missing email address"), ("mail_from_invalid", "Invalid from address"), ("mail_from_missing", "Missing from address"), ("mail_smtp", "Connection failed (outgoing mail server problem)"), ("mail_bl", "Blacklisted Address"), ("mail_optout", "Opted Out"), ("mail_dup", "Duplicated Email")]).string("Failure type"));
        def.add_field(FieldDef::text("failure_reason").string("Failure reason"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["create", "write", "_gc_notifications", "format_failure_reason", "_filtered_for_web_client", "_to_store_defaults"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "_gc_notifications" => self._gc_notifications(env, ctx, rs, args).await,
            "format_failure_reason" => self.format_failure_reason(env, ctx, rs, args).await,
            "_filtered_for_web_client" => self._filtered_for_web_client(env, ctx, rs, args).await,
            "_to_store_defaults" => self._to_store_defaults(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailNotificationFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_notification.py:77`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.notification.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_notification.py:85`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.notification.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_notification.py:93`). Decoradores: api.model.
    async fn _gc_notifications(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.notification._gc_notifications".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_notification.py:108`).
    async fn format_failure_reason(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.notification.format_failure_reason".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_notification.py:121`).
    async fn _filtered_for_web_client(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.notification._filtered_for_web_client".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_notification.py:132`).
    async fn _to_store_defaults(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.notification._to_store_defaults".into(),
        ))
    }

}
