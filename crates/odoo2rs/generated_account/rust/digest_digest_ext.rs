//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/digest.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `digest.digest` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct DigestDigestExtFragment;

#[async_trait]
impl ModelFragment for DigestDigestExtFragment {
    fn model_name(&self) -> &str {
        "digest.digest"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::boolean("kpi_account_total_revenue").string("Revenue"));
        def.add_field(FieldDef::monetary("kpi_account_total_revenue_value").computed("_compute_kpi_account_total_revenue_value", &[]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_kpi_account_total_revenue_value", "_compute_kpis_actions"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_kpi_account_total_revenue_value" => self._compute_kpi_account_total_revenue_value(env, ctx, rs, args).await,
            "_compute_kpis_actions" => self._compute_kpis_actions(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl DigestDigestExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/digest.py:14`).
    async fn _compute_kpi_account_total_revenue_value(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): digest.digest._compute_kpi_account_total_revenue_value".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/digest.py:36`).
    async fn _compute_kpis_actions(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): digest.digest._compute_kpis_actions".into(),
        ))
    }

}
