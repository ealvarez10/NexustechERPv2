//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/merge_partner_automatic.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `base.partner.merge.automatic.wizard` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct BasePartnerMergeAutomaticWizardExtFragment;

#[async_trait]
impl ModelFragment for BasePartnerMergeAutomaticWizardExtFragment {
    fn model_name(&self) -> &str {
        "base.partner.merge.automatic.wizard"
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
        vec!["_update_reference_fields"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_update_reference_fields" => self._update_reference_fields(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl BasePartnerMergeAutomaticWizardExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/merge_partner_automatic.py:8`).
    async fn _update_reference_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base.partner.merge.automatic.wizard._update_reference_fields".into(),
        ))
    }

}
