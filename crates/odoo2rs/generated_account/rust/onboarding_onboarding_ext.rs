//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/onboarding_onboarding.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `onboarding.onboarding` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct OnboardingOnboardingExtFragment;

#[async_trait]
impl ModelFragment for OnboardingOnboardingExtFragment {
    fn model_name(&self) -> &str {
        "onboarding.onboarding"
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
        vec!["action_close_panel_account_invoice", "_prepare_rendering_values", "action_close_panel_account_dashboard"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "action_close_panel_account_invoice" => self.action_close_panel_account_invoice(env, ctx, rs, args).await,
            "_prepare_rendering_values" => self._prepare_rendering_values(env, ctx, rs, args).await,
            "action_close_panel_account_dashboard" => self.action_close_panel_account_dashboard(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl OnboardingOnboardingExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/onboarding_onboarding.py:11`). Decoradores: api.model.
    async fn action_close_panel_account_invoice(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): onboarding.onboarding.action_close_panel_account_invoice".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/onboarding_onboarding.py:14`).
    async fn _prepare_rendering_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): onboarding.onboarding._prepare_rendering_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/onboarding_onboarding.py:28`). Decoradores: api.model.
    async fn action_close_panel_account_dashboard(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): onboarding.onboarding.action_close_panel_account_dashboard".into(),
        ))
    }

}
