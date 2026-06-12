//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message_translation.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.message.translation`

use nexus_orm::prelude::*;

pub struct MailMessageTranslationFragment;

#[async_trait]
impl ModelFragment for MailMessageTranslationFragment {
    fn model_name(&self) -> &str {
        "mail.message.translation"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Message Translation".into();
        def.add_field(FieldDef::many2one("message_id", "mail.message").string("Message").required());
        def.add_field(FieldDef::char("source_lang").string("Source Language").required());
        def.add_field(FieldDef::char("target_lang").string("Target Language").required());
        def.add_field(FieldDef::html("body").string("Translation Body").required());
        def.add_field(FieldDef::datetime("create_date"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_gc_translations"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_gc_translations" => self._gc_translations(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailMessageTranslationFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message_translation.py:27`). Decoradores: api.autovacuum.
    async fn _gc_translations(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.translation._gc_translations".into(),
        ))
    }

}
