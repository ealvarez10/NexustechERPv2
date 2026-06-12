//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.payment.term.line`

use nexus_orm::prelude::*;

pub struct AccountPaymentTermLineFragment;

#[async_trait]
impl ModelFragment for AccountPaymentTermLineFragment {
    fn model_name(&self) -> &str {
        "account.payment.term.line"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Payment Terms Line".into();
        def.order = "id".into();
        def.add_field(FieldDef::selection("value", &[("percent", "Percent"), ("fixed", "Fixed")]).required().default_val("percent"));
        def.add_field(FieldDef::float("value_amount").string("Due").computed("_compute_value_amount", &["payment_id"]).stored());
        def.add_field(FieldDef::selection("delay_type", &[("days_after", "Days after invoice date"), ("days_after_end_of_month", "Days after end of month"), ("days_after_end_of_next_month", "Days after end of next month"), ("days_end_of_month_on_the", "Days end of month on the")]).required().default_val("days_after"));
        def.add_field(FieldDef::boolean("display_days_next_month").computed("_compute_display_days_next_month", &["delay_type"]).stored());
        def.add_field(FieldDef::char("days_next_month").string("Days on the next month").default_val("10"));
        def.add_field(FieldDef::integer("nb_days").string("Days").computed("_compute_days", &["payment_id"]).stored());
        def.add_field(FieldDef::many2one("payment_id", "account.payment.term").string("Payment Terms").required());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_get_due_date", "_check_valid_char_value", "_compute_display_days_next_month", "_check_percent", "_compute_days", "_compute_value_amount"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_get_due_date" => self._get_due_date(env, ctx, rs, args).await,
            "_check_valid_char_value" => self._check_valid_char_value(env, ctx, rs, args).await,
            "_compute_display_days_next_month" => self._compute_display_days_next_month(env, ctx, rs, args).await,
            "_check_percent" => self._check_percent(env, ctx, rs, args).await,
            "_compute_days" => self._compute_days(env, ctx, rs, args).await,
            "_compute_value_amount" => self._compute_value_amount(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountPaymentTermLineFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:310`).
    async fn _get_due_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term.line._get_due_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:330`). Decoradores: api.constrains('days_next_month').
    async fn _check_valid_char_value(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term.line._check_valid_char_value".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:339`). Decoradores: api.depends('delay_type').
    async fn _compute_display_days_next_month(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term.line._compute_display_days_next_month".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:344`). Decoradores: api.constrains('value', 'value_amount').
    async fn _check_percent(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term.line._check_percent".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:350`). Decoradores: api.depends('payment_id').
    async fn _compute_days(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term.line._compute_days".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:359`). Decoradores: api.depends('payment_id').
    async fn _compute_value_amount(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term.line._compute_value_amount".into(),
        ))
    }

}
