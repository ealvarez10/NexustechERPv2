//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_push.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.push`

use nexus_orm::prelude::*;

pub struct MailPushFragment;

#[async_trait]
impl ModelFragment for MailPushFragment {
    fn model_name(&self) -> &str {
        "mail.push"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Push Notifications".into();
        def.add_field(FieldDef::many2one("mail_push_device_id", "mail.push.device").string("devices").required());
        def.add_field(FieldDef::text("payload"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_push_notification_to_endpoint"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_push_notification_to_endpoint" => self._push_notification_to_endpoint(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailPushFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_push.py:22`). Decoradores: api.model.
    async fn _push_notification_to_endpoint(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.push._push_notification_to_endpoint".into(),
        ))
    }

}
