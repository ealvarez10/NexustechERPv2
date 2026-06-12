//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread_cc.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.thread.cc`

use nexus_orm::prelude::*;

pub struct MailThreadCcFragment;

#[async_trait]
impl ModelFragment for MailThreadCcFragment {
    fn model_name(&self) -> &str {
        "mail.thread.cc"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Email CC management".into();
        def.add_field(FieldDef::char("email_cc").string("Email cc"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_mail_cc_sanitized_raw_dict", "message_new", "message_update", "_message_add_suggested_recipients"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_mail_cc_sanitized_raw_dict" => self._mail_cc_sanitized_raw_dict(env, ctx, rs, args).await,
            "message_new" => self.message_new(env, ctx, rs, args).await,
            "message_update" => self.message_update(env, ctx, rs, args).await,
            "_message_add_suggested_recipients" => self._message_add_suggested_recipients(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailThreadCcFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread_cc.py:14`).
    async fn _mail_cc_sanitized_raw_dict(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.cc._mail_cc_sanitized_raw_dict".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread_cc.py:24`). Decoradores: api.model.
    async fn message_new(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.cc.message_new".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread_cc.py:33`).
    async fn message_update(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.cc.message_update".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread_cc.py:46`).
    async fn _message_add_suggested_recipients(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.cc._message_add_suggested_recipients".into(),
        ))
    }

}
