//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_push_device.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.push.device`

use nexus_orm::prelude::*;

pub struct MailPushDeviceFragment;

#[async_trait]
impl ModelFragment for MailPushDeviceFragment {
    fn model_name(&self) -> &str {
        "mail.push.device"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Push Notification Device".into();
        def.add_field(FieldDef::many2one("partner_id", "res.partner").string("Partner").required());
        def.add_field(FieldDef::char("endpoint").string("Browser endpoint").required());
        def.add_field(FieldDef::char("keys").string("Browser keys").required());
        def.add_field(FieldDef::datetime("expiration_time").string("Expiration Token Date"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["get_web_push_vapid_public_key", "register_devices", "unregister_devices", "_verify_vapid_public_key"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "get_web_push_vapid_public_key" => self.get_web_push_vapid_public_key(env, ctx, rs, args).await,
            "register_devices" => self.register_devices(env, ctx, rs, args).await,
            "unregister_devices" => self.unregister_devices(env, ctx, rs, args).await,
            "_verify_vapid_public_key" => self._verify_vapid_public_key(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailPushDeviceFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_push_device.py:34`). Decoradores: api.model.
    async fn get_web_push_vapid_public_key(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.push.device.get_web_push_vapid_public_key".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_push_device.py:48`). Decoradores: api.model.
    async fn register_devices(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.push.device.register_devices".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_push_device.py:76`). Decoradores: api.model.
    async fn unregister_devices(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.push.device.unregister_devices".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_push_device.py:86`).
    async fn _verify_vapid_public_key(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.push.device._verify_vapid_public_key".into(),
        ))
    }

}
