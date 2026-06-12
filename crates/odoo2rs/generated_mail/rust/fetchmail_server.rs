//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/fetchmail.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `fetchmail.server`

use nexus_orm::prelude::*;

pub struct FetchmailServerFragment;

#[async_trait]
impl ModelFragment for FetchmailServerFragment {
    fn model_name(&self) -> &str {
        "fetchmail.server"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Incoming Mail Server".into();
        def.order = "priority".into();
        def.add_field(FieldDef::char("name").string("Name").required());
        def.add_field(FieldDef::boolean("active").string("Active").default_val(true));
        def.add_field(FieldDef::selection("state", &[("draft", "Not Confirmed"), ("done", "Confirmed")]).string("Status").readonly().default_val("draft"));
        def.add_field(FieldDef::char("server").string("Server Name"));
        def.add_field(FieldDef::integer("port"));
        def.add_field(FieldDef::selection("server_type", &[("imap", "IMAP Server"), ("pop", "POP Server"), ("local", "Local Server")]).string("Server Type").required().default_val("imap"));
        def.add_field(FieldDef::text("server_type_info").string("Server Type Info").computed("_compute_server_type_info", &["server_type"]).stored());
        def.add_field(FieldDef::boolean("is_ssl").string("SSL/TLS"));
        def.add_field(FieldDef::boolean("attach").string("Keep Attachments").default_val(true));
        def.add_field(FieldDef::boolean("original").string("Keep Original"));
        def.add_field(FieldDef::datetime("date").string("Last Fetch Date").readonly());
        def.add_field(FieldDef::datetime("error_date").string("Last Error Date").readonly());
        def.add_field(FieldDef::text("error_message").string("Last Error Message").readonly());
        def.add_field(FieldDef::char("user").string("Username"));
        def.add_field(FieldDef::char("password"));
        def.add_field(FieldDef::many2one("object_id", "ir.model").string("Create a New Record"));
        def.add_field(FieldDef::integer("priority").string("Server Priority").default_val(5i64));
        def.add_field(FieldDef::one2many("message_ids", "mail.mail", "fetchmail_server_id").string("Messages").readonly());
        def.add_field(FieldDef::text("configuration").string("Configuration").readonly());
        def.add_field(FieldDef::char("script").readonly().default_val("/mail/static/scripts/odoo-mailgate.py"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_server_type_info", "onchange_server_type", "create", "write", "unlink", "set_draft", "_connect__", "_imap_login__", "button_confirm_login", "fetch_mail", "_fetch_mails", "_fetch_mail", "_get_connection_type", "_update_cron"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_server_type_info" => self._compute_server_type_info(env, ctx, rs, args).await,
            "onchange_server_type" => self.onchange_server_type(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "set_draft" => self.set_draft(env, ctx, rs, args).await,
            "_connect__" => self._connect__(env, ctx, rs, args).await,
            "_imap_login__" => self._imap_login__(env, ctx, rs, args).await,
            "button_confirm_login" => self.button_confirm_login(env, ctx, rs, args).await,
            "fetch_mail" => self.fetch_mail(env, ctx, rs, args).await,
            "_fetch_mails" => self._fetch_mails(env, ctx, rs, args).await,
            "_fetch_mail" => self._fetch_mail(env, ctx, rs, args).await,
            "_get_connection_type" => self._get_connection_type(env, ctx, rs, args).await,
            "_update_cron" => self._update_cron(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl FetchmailServerFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/fetchmail.py:131`). Decoradores: api.depends('server_type').
    async fn _compute_server_type_info(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): fetchmail.server._compute_server_type_info".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/fetchmail.py:139`). Decoradores: api.onchange('server_type', 'is_ssl', 'object_id').
    async fn onchange_server_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): fetchmail.server.onchange_server_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/fetchmail.py:160`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): fetchmail.server.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/fetchmail.py:165`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): fetchmail.server.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/fetchmail.py:170`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): fetchmail.server.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/fetchmail.py:175`).
    async fn set_draft(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): fetchmail.server.set_draft".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/fetchmail.py:179`).
    async fn _connect__(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): fetchmail.server._connect__".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/fetchmail.py:201`).
    async fn _imap_login__(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): fetchmail.server._imap_login__".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/fetchmail.py:211`).
    async fn button_confirm_login(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): fetchmail.server.button_confirm_login".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/fetchmail.py:237`).
    async fn fetch_mail(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): fetchmail.server.fetch_mail".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/fetchmail.py:245`). Decoradores: api.model.
    async fn _fetch_mails(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): fetchmail.server._fetch_mails".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/fetchmail.py:253`).
    async fn _fetch_mail(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): fetchmail.server._fetch_mail".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/fetchmail.py:333`).
    async fn _get_connection_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): fetchmail.server._get_connection_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/fetchmail.py:342`). Decoradores: api.model.
    async fn _update_cron(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): fetchmail.server._update_cron".into(),
        ))
    }

}
