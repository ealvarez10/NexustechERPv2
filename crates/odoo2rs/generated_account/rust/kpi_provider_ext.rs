//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/kpi_provider.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `kpi.provider` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct KpiProviderExtFragment;

#[async_trait]
impl ModelFragment for KpiProviderExtFragment {
    fn model_name(&self) -> &str {
        "kpi.provider"
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
        vec!["get_account_kpi_summary", "get_kpi_summary"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "get_account_kpi_summary" => self.get_account_kpi_summary(env, ctx, rs, args).await,
            "get_kpi_summary" => self.get_kpi_summary(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl KpiProviderExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/kpi_provider.py:8`). Decoradores: api.model.
    async fn get_account_kpi_summary(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): kpi.provider.get_account_kpi_summary".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/kpi_provider.py:38`). Decoradores: api.model.
    async fn get_kpi_summary(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): kpi.provider.get_kpi_summary".into(),
        ))
    }

}
