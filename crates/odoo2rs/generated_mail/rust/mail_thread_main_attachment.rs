//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread_main_attachment.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.thread.main.attachment`

use nexus_orm::prelude::*;

pub struct MailThreadMainAttachmentFragment;

#[async_trait]
impl ModelFragment for MailThreadMainAttachmentFragment {
    fn model_name(&self) -> &str {
        "mail.thread.main.attachment"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Mail Main Attachment management".into();
        def.add_field(FieldDef::many2one("message_main_attachment_id", "ir.attachment").string("Main Attachment"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_message_post_after_hook", "_message_set_main_attachment_id", "_thread_to_store"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_message_post_after_hook" => self._message_post_after_hook(env, ctx, rs, args).await,
            "_message_set_main_attachment_id" => self._message_set_main_attachment_id(env, ctx, rs, args).await,
            "_thread_to_store" => self._thread_to_store(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailThreadMainAttachmentFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread_main_attachment.py:17`).
    async fn _message_post_after_hook(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.main.attachment._message_post_after_hook".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread_main_attachment.py:27`).
    async fn _message_set_main_attachment_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.main.attachment._message_set_main_attachment_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread_main_attachment.py:52`).
    async fn _thread_to_store(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.main.attachment._thread_to_store".into(),
        ))
    }

}
