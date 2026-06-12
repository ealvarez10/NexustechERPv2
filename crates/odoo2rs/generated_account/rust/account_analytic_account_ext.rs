//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_account.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.analytic.account` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct AccountAnalyticAccountExtFragment;

#[async_trait]
impl ModelFragment for AccountAnalyticAccountExtFragment {
    fn model_name(&self) -> &str {
        "account.analytic.account"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::integer("invoice_count").string("Invoice Count").computed("_compute_invoice_count", &["line_ids"]).stored());
        def.add_field(FieldDef::integer("vendor_bill_count").string("Vendor Bill Count").computed("_compute_vendor_bill_count", &["line_ids"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_invoice_count", "_compute_vendor_bill_count", "action_view_invoice", "action_view_vendor_bill"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_invoice_count" => self._compute_invoice_count(env, ctx, rs, args).await,
            "_compute_vendor_bill_count" => self._compute_vendor_bill_count(env, ctx, rs, args).await,
            "action_view_invoice" => self.action_view_invoice(env, ctx, rs, args).await,
            "action_view_vendor_bill" => self.action_view_vendor_bill(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountAnalyticAccountExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_account.py:19`). Decoradores: api.depends('line_ids').
    async fn _compute_invoice_count(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.analytic.account._compute_invoice_count".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_account.py:35`). Decoradores: api.depends('line_ids').
    async fn _compute_vendor_bill_count(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.analytic.account._compute_vendor_bill_count".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_account.py:50`).
    async fn action_view_invoice(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.analytic.account.action_view_invoice".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_account.py:65`).
    async fn action_view_vendor_bill(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.analytic.account.action_view_vendor_bill".into(),
        ))
    }

}
