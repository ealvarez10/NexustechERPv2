//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_partial_reconcile.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.partial.reconcile`

use nexus_orm::prelude::*;

pub struct AccountPartialReconcileFragment;

#[async_trait]
impl ModelFragment for AccountPartialReconcileFragment {
    fn model_name(&self) -> &str {
        "account.partial.reconcile"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Partial Reconcile".into();
        def.add_field(FieldDef::many2one("debit_move_id", "account.move.line").required());
        def.add_field(FieldDef::many2one("credit_move_id", "account.move.line").required());
        def.add_field(FieldDef::many2one("full_reconcile_id", "account.full.reconcile").string("Full Reconcile"));
        def.add_field(FieldDef::many2one("exchange_move_id", "account.move"));
        def.add_field(FieldDef::json("draft_caba_move_vals").string("Values that created the draft cash-basis entry"));
        def.add_field({ let mut f = FieldDef::many2one("company_currency_id", "res.currency").string("Company Currency"); f.related = Some("company_id.currency_id".into()); f });
        def.add_field({ let mut f = FieldDef::many2one("debit_currency_id", "res.currency").string("Currency of the debit journal item."); f.related = Some("debit_move_id.currency_id".into()); f });
        def.add_field({ let mut f = FieldDef::many2one("credit_currency_id", "res.currency").string("Currency of the credit journal item."); f.related = Some("credit_move_id.currency_id".into()); f });
        def.add_field(FieldDef::monetary("amount"));
        def.add_field(FieldDef::monetary("debit_amount_currency"));
        def.add_field(FieldDef::monetary("credit_amount_currency"));
        def.add_field(FieldDef::many2one("company_id", "res.company").string("Company").computed("_compute_company_id", &["debit_move_id", "credit_move_id"]).stored());
        def.add_field(FieldDef::date("max_date").string("Max Date of Matched Lines").computed("_compute_max_date", &["debit_move_id.date", "credit_move_id.date"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_check_required_computed_currencies", "_compute_max_date", "_compute_company_id", "unlink", "create", "_get_to_update_payments", "_update_matching_number", "_collect_tax_cash_basis_values", "_prepare_cash_basis_base_line_vals", "_prepare_cash_basis_counterpart_base_line_vals", "_prepare_cash_basis_tax_line_vals", "_prepare_cash_basis_counterpart_tax_line_vals", "_get_cash_basis_base_line_grouping_key_from_vals", "_get_cash_basis_base_line_grouping_key_from_record", "_get_cash_basis_tax_line_grouping_key_from_vals", "_get_cash_basis_tax_line_grouping_key_from_record", "_create_tax_cash_basis_moves", "_get_draft_caba_move_vals", "_set_draft_caba_move_vals"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_check_required_computed_currencies" => self._check_required_computed_currencies(env, ctx, rs, args).await,
            "_compute_max_date" => self._compute_max_date(env, ctx, rs, args).await,
            "_compute_company_id" => self._compute_company_id(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "_get_to_update_payments" => self._get_to_update_payments(env, ctx, rs, args).await,
            "_update_matching_number" => self._update_matching_number(env, ctx, rs, args).await,
            "_collect_tax_cash_basis_values" => self._collect_tax_cash_basis_values(env, ctx, rs, args).await,
            "_prepare_cash_basis_base_line_vals" => self._prepare_cash_basis_base_line_vals(env, ctx, rs, args).await,
            "_prepare_cash_basis_counterpart_base_line_vals" => self._prepare_cash_basis_counterpart_base_line_vals(env, ctx, rs, args).await,
            "_prepare_cash_basis_tax_line_vals" => self._prepare_cash_basis_tax_line_vals(env, ctx, rs, args).await,
            "_prepare_cash_basis_counterpart_tax_line_vals" => self._prepare_cash_basis_counterpart_tax_line_vals(env, ctx, rs, args).await,
            "_get_cash_basis_base_line_grouping_key_from_vals" => self._get_cash_basis_base_line_grouping_key_from_vals(env, ctx, rs, args).await,
            "_get_cash_basis_base_line_grouping_key_from_record" => self._get_cash_basis_base_line_grouping_key_from_record(env, ctx, rs, args).await,
            "_get_cash_basis_tax_line_grouping_key_from_vals" => self._get_cash_basis_tax_line_grouping_key_from_vals(env, ctx, rs, args).await,
            "_get_cash_basis_tax_line_grouping_key_from_record" => self._get_cash_basis_tax_line_grouping_key_from_record(env, ctx, rs, args).await,
            "_create_tax_cash_basis_moves" => self._create_tax_cash_basis_moves(env, ctx, rs, args).await,
            "_get_draft_caba_move_vals" => self._get_draft_caba_move_vals(env, ctx, rs, args).await,
            "_set_draft_caba_move_vals" => self._set_draft_caba_move_vals(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountPartialReconcileFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_partial_reconcile.py:74`). Decoradores: api.constrains('debit_currency_id', 'credit_currency_id').
    async fn _check_required_computed_currencies(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.partial.reconcile._check_required_computed_currencies".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_partial_reconcile.py:84`). Decoradores: api.depends('debit_move_id.date', 'credit_move_id.date').
    async fn _compute_max_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.partial.reconcile._compute_max_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_partial_reconcile.py:92`). Decoradores: api.depends('debit_move_id', 'credit_move_id').
    async fn _compute_company_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.partial.reconcile._compute_company_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_partial_reconcile.py:104`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.partial.reconcile.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_partial_reconcile.py:149`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.partial.reconcile.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_partial_reconcile.py:155`).
    async fn _get_to_update_payments(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.partial.reconcile._get_to_update_payments".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_partial_reconcile.py:171`). Decoradores: api.model.
    async fn _update_matching_number(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.partial.reconcile._update_matching_number".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_partial_reconcile.py:221`).
    async fn _collect_tax_cash_basis_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.partial.reconcile._collect_tax_cash_basis_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_partial_reconcile.py:335`). Decoradores: api.model.
    async fn _prepare_cash_basis_base_line_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.partial.reconcile._prepare_cash_basis_base_line_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_partial_reconcile.py:367`). Decoradores: api.model.
    async fn _prepare_cash_basis_counterpart_base_line_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.partial.reconcile._prepare_cash_basis_counterpart_base_line_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_partial_reconcile.py:388`). Decoradores: api.model.
    async fn _prepare_cash_basis_tax_line_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.partial.reconcile._prepare_cash_basis_tax_line_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_partial_reconcile.py:419`). Decoradores: api.model.
    async fn _prepare_cash_basis_counterpart_tax_line_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.partial.reconcile._prepare_cash_basis_counterpart_tax_line_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_partial_reconcile.py:441`). Decoradores: api.model.
    async fn _get_cash_basis_base_line_grouping_key_from_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.partial.reconcile._get_cash_basis_base_line_grouping_key_from_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_partial_reconcile.py:457`). Decoradores: api.model.
    async fn _get_cash_basis_base_line_grouping_key_from_record(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.partial.reconcile._get_cash_basis_base_line_grouping_key_from_record".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_partial_reconcile.py:472`). Decoradores: api.model.
    async fn _get_cash_basis_tax_line_grouping_key_from_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.partial.reconcile._get_cash_basis_tax_line_grouping_key_from_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_partial_reconcile.py:489`). Decoradores: api.model.
    async fn _get_cash_basis_tax_line_grouping_key_from_record(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.partial.reconcile._get_cash_basis_tax_line_grouping_key_from_record".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_partial_reconcile.py:504`).
    async fn _create_tax_cash_basis_moves(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.partial.reconcile._create_tax_cash_basis_moves".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_partial_reconcile.py:684`).
    async fn _get_draft_caba_move_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.partial.reconcile._get_draft_caba_move_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_partial_reconcile.py:699`).
    async fn _set_draft_caba_move_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.partial.reconcile._set_draft_caba_move_vals".into(),
        ))
    }

}
