//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_cash_rounding.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.cash.rounding`

use nexus_orm::prelude::*;

pub struct AccountCashRoundingFragment;

#[async_trait]
impl ModelFragment for AccountCashRoundingFragment {
    fn model_name(&self) -> &str {
        "account.cash.rounding"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Account Cash Rounding".into();
        def.add_field(FieldDef::char("name").string("Name").required());
        def.add_field(FieldDef::float("rounding").string("Rounding Precision").required().default_val(0.01f64));
        def.add_field(FieldDef::selection("strategy", &[("biggest_tax", "Modify tax amount"), ("add_invoice_line", "Add a rounding line")]).string("Rounding Strategy").required().default_val("add_invoice_line"));
        def.add_field(FieldDef::many2one("profit_account_id", "account.account").string("Profit Account"));
        def.add_field(FieldDef::many2one("loss_account_id", "account.account").string("Loss Account"));
        def.add_field(FieldDef::selection("rounding_method", &[("UP", "Up"), ("DOWN", "Down"), ("HALF-UP", "Nearest")]).string("Rounding Method").required().default_val("HALF-UP"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["validate_rounding", "round", "compute_difference"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "validate_rounding" => self.validate_rounding(env, ctx, rs, args).await,
            "round" => self.round(env, ctx, rs, args).await,
            "compute_difference" => self.compute_difference(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountCashRoundingFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_cash_rounding.py:46`). Decoradores: api.constrains('rounding').
    async fn validate_rounding(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.cash.rounding.validate_rounding".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_cash_rounding.py:51`).
    async fn round(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.cash.rounding.round".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_cash_rounding.py:59`).
    async fn compute_difference(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.cash.rounding.compute_difference".into(),
        ))
    }

}
