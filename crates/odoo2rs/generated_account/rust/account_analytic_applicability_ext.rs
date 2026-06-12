//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_plan.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.analytic.applicability` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct AccountAnalyticApplicabilityExtFragment;

#[async_trait]
impl ModelFragment for AccountAnalyticApplicabilityExtFragment {
    fn model_name(&self) -> &str {
        "account.analytic.applicability"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Analytic Plan's Applicabilities".into();
        def.add_field(FieldDef::selection("business_domain", &[]));
        def.add_field(FieldDef::char("account_prefix").string("Financial Accounts Prefixes"));
        def.add_field(FieldDef::many2one("product_categ_id", "product.category").string("Product Category"));
        def.add_field(FieldDef::boolean("display_account_prefix").computed("_compute_display_account_prefix", &["business_domain"]).stored());
        def.add_field(FieldDef::char("account_prefix_placeholder").computed("_compute_prefix_placeholder", &["account_prefix", "business_domain"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_prefix_placeholder", "_get_score", "_compute_display_account_prefix"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_prefix_placeholder" => self._compute_prefix_placeholder(env, ctx, rs, args).await,
            "_get_score" => self._get_score(env, ctx, rs, args).await,
            "_compute_display_account_prefix" => self._compute_display_account_prefix(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountAnalyticApplicabilityExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_plan.py:35`). Decoradores: api.depends('account_prefix', 'business_domain').
    async fn _compute_prefix_placeholder(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.analytic.applicability._compute_prefix_placeholder".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_plan.py:59`).
    async fn _get_score(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.analytic.applicability._get_score".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_plan.py:79`). Decoradores: api.depends('business_domain').
    async fn _compute_display_account_prefix(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.analytic.applicability._compute_display_account_prefix".into(),
        ))
    }

}
