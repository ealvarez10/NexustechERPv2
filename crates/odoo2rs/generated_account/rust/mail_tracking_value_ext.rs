//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_tracking_value.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.tracking.value` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct MailTrackingValueExtFragment;

#[async_trait]
impl ModelFragment for MailTrackingValueExtFragment {
    fn model_name(&self) -> &str {
        "mail.tracking.value"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_except_audit_log", "write"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_except_audit_log" => self._except_audit_log(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailTrackingValueExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_tracking_value.py:10`). Decoradores: api.ondelete().
    async fn _except_audit_log(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.tracking.value._except_audit_log".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_tracking_value.py:13`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.tracking.value.write".into(),
        ))
    }

}
