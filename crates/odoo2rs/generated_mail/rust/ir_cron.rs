//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_cron.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `ir.cron`

use nexus_orm::prelude::*;

pub struct IrCronFragment;

#[async_trait]
impl ModelFragment for IrCronFragment {
    fn model_name(&self) -> &str {
        "ir.cron"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        // TODO(odoo2rs): campo 'user_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::integer("interval_number"));
        def.add_field(FieldDef::selection("interval_type", &[]));
        def.add_field(FieldDef::integer("priority"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_notify_admin"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_notify_admin" => self._notify_admin(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl IrCronFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_cron.py:15`).
    async fn _notify_admin(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.cron._notify_admin".into(),
        ))
    }

}
