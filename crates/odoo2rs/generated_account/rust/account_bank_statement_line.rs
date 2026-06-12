//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.bank.statement.line`

use nexus_orm::prelude::*;

pub struct AccountBankStatementLineFragment;

#[async_trait]
impl ModelFragment for AccountBankStatementLineFragment {
    fn model_name(&self) -> &str {
        "account.bank.statement.line"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Bank Statement Line".into();
        def.order = "internal_index desc".into();
        def.add_field(FieldDef::many2one("move_id", "account.move").string("Journal Entry").required().readonly());
        def.add_field({ let mut f = FieldDef::many2one("journal_id", "account.journal").required(); f.related = Some("move_id.journal_id".into()); f });
        def.add_field({ let mut f = FieldDef::many2one("company_id", "res.company").required(); f.related = Some("move_id.company_id".into()); f });
        def.add_field(FieldDef::many2one("statement_id", "account.bank.statement").string("Statement"));
        def.add_field(FieldDef::many2many("payment_ids", "account.payment").string("Auto-generated Payments"));
        def.add_field(FieldDef::integer("sequence").default_val(1i64));
        def.add_field(FieldDef::many2one("partner_id", "res.partner").string("Partner"));
        def.add_field(FieldDef::char("account_number").string("Bank Account Number"));
        def.add_field(FieldDef::char("partner_name"));
        def.add_field(FieldDef::char("transaction_type"));
        def.add_field(FieldDef::char("payment_ref").string("Label"));
        def.add_field(FieldDef::many2one("currency_id", "res.currency").string("Journal Currency").computed("_compute_currency_id", &["journal_id.currency_id"]).stored());
        def.add_field(FieldDef::monetary("amount"));
        def.add_field(FieldDef::monetary("running_balance").computed("_compute_running_balance", &[]).stored());
        def.add_field(FieldDef::many2one("foreign_currency_id", "res.currency").string("Foreign Currency"));
        def.add_field(FieldDef::monetary("amount_currency").string("Amount in Currency").computed("_compute_amount_currency", &["foreign_currency_id", "date", "amount", "company_id"]).stored());
        def.add_field(FieldDef::float("amount_residual").string("Residual Amount").computed("_compute_is_reconciled", &["journal_id", "currency_id", "amount", "foreign_currency_id", "amount_currency", "move_id.checked", "move_id.line_ids.account_id", "move_id.line_ids.amount_currency", "move_id.line_ids.amount_residual_currency", "move_id.line_ids.currency_id", "move_id.line_ids.matched_debit_ids", "move_id.line_ids.matched_credit_ids"]).stored());
        def.add_field({ let mut f = FieldDef::char("country_code"); f.related = Some("company_id.account_fiscal_country_id.code".into()); f });
        def.add_field(FieldDef::char("internal_index").string("Internal Reference").computed("_compute_internal_index", &["date", "sequence"]).stored());
        def.add_field(FieldDef::boolean("is_reconciled").string("Is Reconciled").computed("_compute_is_reconciled", &["journal_id", "currency_id", "amount", "foreign_currency_id", "amount_currency", "move_id.checked", "move_id.line_ids.account_id", "move_id.line_ids.amount_currency", "move_id.line_ids.amount_residual_currency", "move_id.line_ids.currency_id", "move_id.line_ids.matched_debit_ids", "move_id.line_ids.matched_credit_ids"]).stored());
        def.add_field({ let mut f = FieldDef::boolean("statement_complete"); f.related = Some("statement_id.is_complete".into()); f });
        def.add_field({ let mut f = FieldDef::boolean("statement_valid"); f.related = Some("statement_id.is_valid".into()); f });
        def.add_field({ let mut f = FieldDef::monetary("statement_balance_end_real"); f.related = Some("statement_id.balance_end_real".into()); f });
        def.add_field({ let mut f = FieldDef::char("statement_name").string("Statement Name"); f.related = Some("statement_id.name".into()); f });
        def.add_field(FieldDef::json("transaction_details").readonly());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_amount_currency", "_compute_currency_id", "_compute_running_balance", "_compute_internal_index", "_compute_is_reconciled", "_check_amounts_currencies", "default_get", "new", "create", "write", "unlink", "formatted_read_group", "action_undo_reconciliation", "_check_allow_unlink", "_find_or_create_bank_account", "_get_default_amls_matching_domain", "_get_default_journal", "_get_default_statement", "_get_accounting_amounts_and_currencies", "_prepare_counterpart_amounts_using_st_line_rate", "_prepare_move_line_default_vals", "_seek_for_lines", "_synchronize_from_moves", "_synchronize_to_moves"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_amount_currency" => self._compute_amount_currency(env, ctx, rs, args).await,
            "_compute_currency_id" => self._compute_currency_id(env, ctx, rs, args).await,
            "_compute_running_balance" => self._compute_running_balance(env, ctx, rs, args).await,
            "_compute_internal_index" => self._compute_internal_index(env, ctx, rs, args).await,
            "_compute_is_reconciled" => self._compute_is_reconciled(env, ctx, rs, args).await,
            "_check_amounts_currencies" => self._check_amounts_currencies(env, ctx, rs, args).await,
            "default_get" => self.default_get(env, ctx, rs, args).await,
            "new" => self.new(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "formatted_read_group" => self.formatted_read_group(env, ctx, rs, args).await,
            "action_undo_reconciliation" => self.action_undo_reconciliation(env, ctx, rs, args).await,
            "_check_allow_unlink" => self._check_allow_unlink(env, ctx, rs, args).await,
            "_find_or_create_bank_account" => self._find_or_create_bank_account(env, ctx, rs, args).await,
            "_get_default_amls_matching_domain" => self._get_default_amls_matching_domain(env, ctx, rs, args).await,
            "_get_default_journal" => self._get_default_journal(env, ctx, rs, args).await,
            "_get_default_statement" => self._get_default_statement(env, ctx, rs, args).await,
            "_get_accounting_amounts_and_currencies" => self._get_accounting_amounts_and_currencies(env, ctx, rs, args).await,
            "_prepare_counterpart_amounts_using_st_line_rate" => self._prepare_counterpart_amounts_using_st_line_rate(env, ctx, rs, args).await,
            "_prepare_move_line_default_vals" => self._prepare_move_line_default_vals(env, ctx, rs, args).await,
            "_seek_for_lines" => self._seek_for_lines(env, ctx, rs, args).await,
            "_synchronize_from_moves" => self._synchronize_from_moves(env, ctx, rs, args).await,
            "_synchronize_to_moves" => self._synchronize_to_moves(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountBankStatementLineFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:160`). Decoradores: api.depends('foreign_currency_id', 'date', 'amount', 'company_id').
    async fn _compute_amount_currency(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line._compute_amount_currency".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:174`). Decoradores: api.depends('journal_id.currency_id').
    async fn _compute_currency_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line._compute_currency_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:178`).
    async fn _compute_running_balance(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line._compute_running_balance".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:259`). Decoradores: api.depends('date', 'sequence').
    async fn _compute_internal_index(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line._compute_internal_index".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:287`). Decoradores: api.depends('journal_id', 'currency_id', 'amount', 'foreign_currency_id', 'amount_currency', 'move_id.checked', 'move_id.line_ids.account_id', 'move_id.line_ids.amount_currency', 'move_id.line_ids.amount_residual_currency', 'move_id.line_ids.currency_id', 'move_id.line_ids.matched_debit_ids', 'move_id.line_ids.matched_credit_ids').
    async fn _compute_is_reconciled(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line._compute_is_reconciled".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:321`). Decoradores: api.constrains('amount', 'amount_currency', 'currency_id', 'foreign_currency_id', 'journal_id').
    async fn _check_amounts_currencies(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line._check_amounts_currencies".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:340`). Decoradores: api.model.
    async fn default_get(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line.default_get".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:363`). Decoradores: api.model.
    async fn new(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line.new".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:367`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:423`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:430`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:440`). Decoradores: api.model.
    async fn formatted_read_group(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line.formatted_read_group".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:460`).
    async fn action_undo_reconciliation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line.action_undo_reconciliation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:479`). Decoradores: api.ondelete().
    async fn _check_allow_unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line._check_allow_unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:484`).
    async fn _find_or_create_bank_account(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line._find_or_create_bank_account".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:500`).
    async fn _get_default_amls_matching_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line._get_default_amls_matching_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:529`). Decoradores: api.model.
    async fn _get_default_journal(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line._get_default_journal".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:537`). Decoradores: api.model.
    async fn _get_default_statement(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line._get_default_statement".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:548`).
    async fn _get_accounting_amounts_and_currencies(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line._get_accounting_amounts_and_currencies".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:577`).
    async fn _prepare_counterpart_amounts_using_st_line_rate(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line._prepare_counterpart_amounts_using_st_line_rate".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:625`).
    async fn _prepare_move_line_default_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line._prepare_move_line_default_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:682`).
    async fn _seek_for_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line._seek_for_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:708`).
    async fn _synchronize_from_moves(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line._synchronize_from_moves".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement_line.py:793`).
    async fn _synchronize_to_moves(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.line._synchronize_to_moves".into(),
        ))
    }

}
