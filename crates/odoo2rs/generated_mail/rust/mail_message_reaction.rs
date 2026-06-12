//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message_reaction.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.message.reaction`

use nexus_orm::prelude::*;

pub struct MailMessageReactionFragment;

#[async_trait]
impl ModelFragment for MailMessageReactionFragment {
    fn model_name(&self) -> &str {
        "mail.message.reaction"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Message Reaction".into();
        def.order = "id desc".into();
        def.add_field(FieldDef::many2one("message_id", "mail.message").string("Message").required().readonly());
        def.add_field(FieldDef::char("content").string("Content").required().readonly());
        def.add_field(FieldDef::many2one("partner_id", "res.partner").string("Reacting Partner").readonly());
        def.add_field(FieldDef::many2one("guest_id", "mail.guest").string("Reacting Guest").readonly());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_to_store"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_to_store" => self._to_store(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailMessageReactionFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message_reaction.py:27`).
    async fn _to_store(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.reaction._to_store".into(),
        ))
    }

}
