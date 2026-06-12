//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/onboarding_onboarding_step.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `onboarding.onboarding.step` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct OnboardingOnboardingStepExtFragment;

#[async_trait]
impl ModelFragment for OnboardingOnboardingStepExtFragment {
    fn model_name(&self) -> &str {
        "onboarding.onboarding.step"
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
        vec!["action_open_step_company_data", "action_open_step_base_document_layout", "action_validate_step_base_document_layout", "action_open_step_bank_account", "action_open_step_create_invoice", "action_open_step_fiscal_year", "action_open_step_chart_of_accounts", "action_open_step_sales_tax"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "action_open_step_company_data" => self.action_open_step_company_data(env, ctx, rs, args).await,
            "action_open_step_base_document_layout" => self.action_open_step_base_document_layout(env, ctx, rs, args).await,
            "action_validate_step_base_document_layout" => self.action_validate_step_base_document_layout(env, ctx, rs, args).await,
            "action_open_step_bank_account" => self.action_open_step_bank_account(env, ctx, rs, args).await,
            "action_open_step_create_invoice" => self.action_open_step_create_invoice(env, ctx, rs, args).await,
            "action_open_step_fiscal_year" => self.action_open_step_fiscal_year(env, ctx, rs, args).await,
            "action_open_step_chart_of_accounts" => self.action_open_step_chart_of_accounts(env, ctx, rs, args).await,
            "action_open_step_sales_tax" => self.action_open_step_sales_tax(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl OnboardingOnboardingStepExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/onboarding_onboarding_step.py:11`). Decoradores: api.model.
    async fn action_open_step_company_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): onboarding.onboarding.step.action_open_step_company_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/onboarding_onboarding_step.py:25`). Decoradores: api.model.
    async fn action_open_step_base_document_layout(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): onboarding.onboarding.step.action_open_step_base_document_layout".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/onboarding_onboarding_step.py:37`). Decoradores: api.model.
    async fn action_validate_step_base_document_layout(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): onboarding.onboarding.step.action_validate_step_base_document_layout".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/onboarding_onboarding_step.py:46`). Decoradores: api.model.
    async fn action_open_step_bank_account(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): onboarding.onboarding.step.action_open_step_bank_account".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/onboarding_onboarding_step.py:50`). Decoradores: api.model.
    async fn action_open_step_create_invoice(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): onboarding.onboarding.step.action_open_step_create_invoice".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/onboarding_onboarding_step.py:61`). Decoradores: api.model.
    async fn action_open_step_fiscal_year(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): onboarding.onboarding.step.action_open_step_fiscal_year".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/onboarding_onboarding_step.py:80`). Decoradores: api.model.
    async fn action_open_step_chart_of_accounts(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): onboarding.onboarding.step.action_open_step_chart_of_accounts".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/onboarding_onboarding_step.py:109`). Decoradores: api.model.
    async fn action_open_step_sales_tax(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): onboarding.onboarding.step.action_open_step_sales_tax".into(),
        ))
    }

}
