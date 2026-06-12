//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_distribution_model.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.analytic.distribution.model` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct AccountAnalyticDistributionModelExtFragment;

#[async_trait]
impl ModelFragment for AccountAnalyticDistributionModelExtFragment {
    fn model_name(&self) -> &str {
        "account.analytic.distribution.model"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::char("account_prefix").string("Accounts Prefix"));
        def.add_field(FieldDef::many2one("product_id", "product.product").string("Product"));
        def.add_field(FieldDef::many2one("product_categ_id", "product.category").string("Product Category"));
        def.add_field(FieldDef::char("prefix_placeholder").computed("_compute_prefix_placeholder", &["analytic_precision"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_get_default_search_domain_vals", "_get_applicable_models", "_create_domain", "_compute_prefix_placeholder"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_get_default_search_domain_vals" => self._get_default_search_domain_vals(env, ctx, rs, args).await,
            "_get_applicable_models" => self._get_applicable_models(env, ctx, rs, args).await,
            "_create_domain" => self._create_domain(env, ctx, rs, args).await,
            "_compute_prefix_placeholder" => self._compute_prefix_placeholder(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountAnalyticDistributionModelExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_distribution_model.py:28`).
    async fn _get_default_search_domain_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.analytic.distribution.model._get_default_search_domain_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_distribution_model.py:34`).
    async fn _get_applicable_models(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.analytic.distribution.model._get_applicable_models".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_distribution_model.py:46`).
    async fn _create_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.analytic.distribution.model._create_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_distribution_model.py:54`). Decoradores: api.depends('analytic_precision').
    async fn _compute_prefix_placeholder(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.analytic.distribution.model._compute_prefix_placeholder".into(),
        ))
    }

}
