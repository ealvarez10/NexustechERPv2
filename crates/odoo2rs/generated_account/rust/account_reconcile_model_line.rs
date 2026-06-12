//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_reconcile_model.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.reconcile.model.line`

use nexus_orm::prelude::*;

pub struct AccountReconcileModelLineFragment;

#[async_trait]
impl ModelFragment for AccountReconcileModelLineFragment {
    fn model_name(&self) -> &str {
        "account.reconcile.model.line"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Rules for the reconciliation model".into();
        def.order = "sequence, id".into();
        def.add_field(FieldDef::many2one("model_id", "account.reconcile.model").readonly());
        // TODO(odoo2rs): campo 'company_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::integer("sequence").required().default_val(10i64));
        def.add_field(FieldDef::many2one("account_id", "account.account").string("Account"));
        def.add_field(FieldDef::many2one("partner_id", "res.partner").string("Partner"));
        def.add_field(FieldDef::char("label").string("Label"));
        def.add_field(FieldDef::selection("amount_type", &[("fixed", "Fixed"), ("percentage", "Percentage of balance"), ("percentage_st_line", "Percentage of statement line"), ("regex", "From label")]).required().default_val("percentage"));
        def.add_field(FieldDef::float("amount").string("Float Amount").computed("_compute_float_amount", &["amount_string"]).stored());
        def.add_field(FieldDef::char("amount_string").string("Amount").required().default_val("100"));
        def.add_field(FieldDef::many2many("tax_ids", "account.tax").string("Taxes"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_onchange_amount_type", "_compute_float_amount", "_validate_amount"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_onchange_amount_type" => self._onchange_amount_type(env, ctx, rs, args).await,
            "_compute_float_amount" => self._compute_float_amount(env, ctx, rs, args).await,
            "_validate_amount" => self._validate_amount(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountReconcileModelLineFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_reconcile_model.py:60`). Decoradores: api.onchange('amount_type').
    async fn _onchange_amount_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.reconcile.model.line._onchange_amount_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_reconcile_model.py:68`). Decoradores: api.depends('amount_string').
    async fn _compute_float_amount(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.reconcile.model.line._compute_float_amount".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_reconcile_model.py:76`). Decoradores: api.constrains('amount_string').
    async fn _validate_amount(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.reconcile.model.line._validate_amount".into(),
        ))
    }

}
