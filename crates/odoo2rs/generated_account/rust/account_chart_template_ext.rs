//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/template_generic_coa.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.chart.template` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct AccountChartTemplateExtFragment;

#[async_trait]
impl ModelFragment for AccountChartTemplateExtFragment {
    fn model_name(&self) -> &str {
        "account.chart.template"
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
        vec!["_get_generic_coa_template_data", "_get_generic_coa_res_company", "_get_generic_coa_account_account"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_get_generic_coa_template_data" => self._get_generic_coa_template_data(env, ctx, rs, args).await,
            "_get_generic_coa_res_company" => self._get_generic_coa_res_company(env, ctx, rs, args).await,
            "_get_generic_coa_account_account" => self._get_generic_coa_account_account(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountChartTemplateExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/template_generic_coa.py:9`). Decoradores: template('generic_coa').
    async fn _get_generic_coa_template_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.chart.template._get_generic_coa_template_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/template_generic_coa.py:26`). Decoradores: template('generic_coa', 'res.company').
    async fn _get_generic_coa_res_company(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.chart.template._get_generic_coa_res_company".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/template_generic_coa.py:57`). Decoradores: template('generic_coa', 'account.account').
    async fn _get_generic_coa_account_account(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.chart.template._get_generic_coa_account_account".into(),
        ))
    }

}
