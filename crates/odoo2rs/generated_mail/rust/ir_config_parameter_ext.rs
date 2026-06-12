//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_config_parameter.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `ir.config_parameter` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct IrConfigParameterExtFragment;

#[async_trait]
impl ModelFragment for IrConfigParameterExtFragment {
    fn model_name(&self) -> &str {
        "ir.config_parameter"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
    }

    fn methods(&self) -> Vec<&str> {
        vec!["set_param"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "set_param" => self.set_param(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl IrConfigParameterExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_config_parameter.py:88`). Decoradores: api.model.
    async fn set_param(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.config_parameter.set_param".into(),
        ))
    }

}
