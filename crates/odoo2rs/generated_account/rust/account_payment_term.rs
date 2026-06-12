//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.payment.term`

use nexus_orm::prelude::*;

pub struct AccountPaymentTermFragment;

#[async_trait]
impl ModelFragment for AccountPaymentTermFragment {
    fn model_name(&self) -> &str {
        "account.payment.term"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Payment Terms".into();
        def.order = "sequence, id".into();
        def.add_field(FieldDef::char("name").string("Payment Terms").required());
        def.add_field(FieldDef::boolean("active").default_val(true));
        def.add_field(FieldDef::html("note").string("Description on the Invoice"));
        def.add_field(FieldDef::one2many("line_ids", "account.payment.term.line", "payment_id").string("Terms"));
        def.add_field(FieldDef::many2one("company_id", "res.company").string("Company"));
        def.add_field(FieldDef::char("fiscal_country_codes").computed("_compute_fiscal_country_codes", &["company_id"]).stored());
        def.add_field(FieldDef::integer("sequence").required().default_val(10i64));
        def.add_field(FieldDef::many2one("currency_id", "res.currency").computed("_compute_currency_id", &["company_id"]).stored());
        def.add_field(FieldDef::boolean("display_on_invoice").string("Show installment dates").default_val(true));
        def.add_field({ let mut f = FieldDef::monetary("example_amount").readonly(); f.store = false; f });
        def.add_field({ let mut f = FieldDef::date("example_date").string("Date example"); f.store = false; f });
        def.add_field(FieldDef::boolean("example_invalid").computed("_compute_example_invalid", &["line_ids"]).stored());
        def.add_field(FieldDef::html("example_preview").computed("_compute_example_preview", &["currency_id", "example_amount", "example_date", "line_ids.value", "line_ids.value_amount", "line_ids.nb_days", "early_discount", "discount_percentage", "discount_days"]).stored());
        def.add_field(FieldDef::html("example_preview_discount").computed("_compute_example_preview", &["currency_id", "example_amount", "example_date", "line_ids.value", "line_ids.value_amount", "line_ids.nb_days", "early_discount", "discount_percentage", "discount_days"]).stored());
        def.add_field(FieldDef::float("discount_percentage").string("Discount %").default_val(2f64));
        def.add_field(FieldDef::integer("discount_days").string("Discount Days").default_val(10i64));
        def.add_field(FieldDef::selection("early_pay_discount_computation", &[("included", "On early payment"), ("excluded", "Never"), ("mixed", "Always (upon invoice)")]).string("Cash Discount Tax Reduction").computed("_compute_discount_computation", &["company_id"]).stored());
        def.add_field(FieldDef::boolean("early_discount").string("Early Discount"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_default_line_ids", "_default_example_date", "_compute_fiscal_country_codes", "_compute_currency_id", "_get_amount_due_after_discount", "_compute_discount_computation", "_compute_example_invalid", "_compute_example_preview", "_get_amount_by_date", "_check_lines", "_compute_terms", "_unlink_except_referenced_terms", "_get_last_discount_date", "_get_last_discount_date_formatted", "copy_data"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_default_line_ids" => self._default_line_ids(env, ctx, rs, args).await,
            "_default_example_date" => self._default_example_date(env, ctx, rs, args).await,
            "_compute_fiscal_country_codes" => self._compute_fiscal_country_codes(env, ctx, rs, args).await,
            "_compute_currency_id" => self._compute_currency_id(env, ctx, rs, args).await,
            "_get_amount_due_after_discount" => self._get_amount_due_after_discount(env, ctx, rs, args).await,
            "_compute_discount_computation" => self._compute_discount_computation(env, ctx, rs, args).await,
            "_compute_example_invalid" => self._compute_example_invalid(env, ctx, rs, args).await,
            "_compute_example_preview" => self._compute_example_preview(env, ctx, rs, args).await,
            "_get_amount_by_date" => self._get_amount_by_date(env, ctx, rs, args).await,
            "_check_lines" => self._check_lines(env, ctx, rs, args).await,
            "_compute_terms" => self._compute_terms(env, ctx, rs, args).await,
            "_unlink_except_referenced_terms" => self._unlink_except_referenced_terms(env, ctx, rs, args).await,
            "_get_last_discount_date" => self._get_last_discount_date(env, ctx, rs, args).await,
            "_get_last_discount_date_formatted" => self._get_last_discount_date_formatted(env, ctx, rs, args).await,
            "copy_data" => self.copy_data(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountPaymentTermFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:17`).
    async fn _default_line_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term._default_line_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:20`).
    async fn _default_example_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term._default_example_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:50`). Decoradores: api.depends('company_id'), api.depends_context('allowed_company_ids').
    async fn _compute_fiscal_country_codes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term._compute_fiscal_country_codes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:57`). Decoradores: api.depends_context('company'), api.depends('company_id').
    async fn _compute_currency_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term._compute_currency_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:61`).
    async fn _get_amount_due_after_discount(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term._get_amount_due_after_discount".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:82`). Decoradores: api.depends('company_id').
    async fn _compute_discount_computation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term._compute_discount_computation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:93`). Decoradores: api.depends('line_ids').
    async fn _compute_example_invalid(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term._compute_example_invalid".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:98`). Decoradores: api.depends('currency_id', 'example_amount', 'example_date', 'line_ids.value', 'line_ids.value_amount', 'line_ids.nb_days', 'early_discount', 'discount_percentage', 'discount_days').
    async fn _compute_example_preview(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term._compute_example_preview".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:137`). Decoradores: api.model.
    async fn _get_amount_by_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term._get_amount_by_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:157`). Decoradores: api.constrains('line_ids', 'early_discount').
    async fn _check_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term._check_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:171`).
    async fn _compute_terms(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term._compute_terms".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:259`). Decoradores: api.ondelete().
    async fn _unlink_except_referenced_terms(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term._unlink_except_referenced_terms".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:263`).
    async fn _get_last_discount_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term._get_last_discount_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:269`).
    async fn _get_last_discount_date_formatted(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term._get_last_discount_date_formatted".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_term.py:275`).
    async fn copy_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.term.copy_data".into(),
        ))
    }

}
