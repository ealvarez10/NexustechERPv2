//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/ir_http.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `ir.http` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct IrHttpExtFragment;

#[async_trait]
impl ModelFragment for IrHttpExtFragment {
    fn model_name(&self) -> &str {
        "ir.http"
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
        vec!["lazy_session_info"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "lazy_session_info" => self.lazy_session_info(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl IrHttpExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/ir_http.py:8`). Decoradores: api.model.
    async fn lazy_session_info(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.http.lazy_session_info".into(),
        ))
    }

}
