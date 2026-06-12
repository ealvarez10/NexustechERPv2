//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.bank.statement`

use nexus_orm::prelude::*;

pub struct AccountBankStatementFragment;

#[async_trait]
impl ModelFragment for AccountBankStatementFragment {
    fn model_name(&self) -> &str {
        "account.bank.statement"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Bank Statement".into();
        def.order = "first_line_index desc".into();
        def.add_field(FieldDef::char("name").string("Reference").computed("_compute_name", &["create_date"]).stored());
        def.add_field(FieldDef::char("reference").string("External Reference"));
        def.add_field(FieldDef::date("date").computed("_compute_date", &["line_ids.internal_index", "line_ids.state"]).stored());
        def.add_field(FieldDef::char("first_line_index").computed("_compute_first_line_index", &["line_ids.internal_index", "line_ids.state"]).stored());
        def.add_field(FieldDef::monetary("balance_start").string("Starting Balance").computed("_compute_balance_start", &["create_date"]).stored());
        def.add_field(FieldDef::monetary("balance_end").string("Computed Balance").computed("_compute_balance_end", &["balance_start", "line_ids.amount", "line_ids.state"]).stored());
        def.add_field(FieldDef::monetary("balance_end_real").string("Ending Balance").computed("_compute_balance_end_real", &["balance_start"]).stored());
        def.add_field({ let mut f = FieldDef::many2one("company_id", "res.company"); f.related = Some("journal_id.company_id".into()); f });
        def.add_field(FieldDef::many2one("currency_id", "res.currency").computed("_compute_currency_id", &["journal_id.currency_id", "company_id.currency_id"]).stored());
        def.add_field(FieldDef::many2one("journal_id", "account.journal").computed("_compute_journal_id", &["line_ids.journal_id"]).stored());
        def.add_field(FieldDef::one2many("line_ids", "account.bank.statement.line", "statement_id").string("Statement lines"));
        def.add_field(FieldDef::boolean("is_complete").computed("_compute_is_complete", &["balance_end", "balance_end_real", "line_ids.amount", "line_ids.state"]).stored());
        def.add_field(FieldDef::boolean("is_valid").computed("_compute_is_valid", &["balance_end", "balance_end_real"]).stored());
        def.add_field({ let mut f = FieldDef::boolean("journal_has_invalid_statements"); f.related = Some("journal_id.has_invalid_statements".into()); f });
        def.add_field(FieldDef::text("problem_description").computed("_compute_problem_description", &["is_valid", "is_complete"]).stored());
        def.add_field(FieldDef::many2many("attachment_ids", "ir.attachment").string("Attachments"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_name", "_compute_first_line_index", "_compute_date", "_compute_balance_start", "_compute_balance_end", "_compute_balance_end_real", "_compute_currency_id", "_compute_journal_id", "_compute_is_complete", "_compute_is_valid", "_compute_problem_description", "_search_is_valid", "_get_statement_validity", "_get_invalid_statement_ids", "default_get", "_check_attachments", "create", "write"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_name" => self._compute_name(env, ctx, rs, args).await,
            "_compute_first_line_index" => self._compute_first_line_index(env, ctx, rs, args).await,
            "_compute_date" => self._compute_date(env, ctx, rs, args).await,
            "_compute_balance_start" => self._compute_balance_start(env, ctx, rs, args).await,
            "_compute_balance_end" => self._compute_balance_end(env, ctx, rs, args).await,
            "_compute_balance_end_real" => self._compute_balance_end_real(env, ctx, rs, args).await,
            "_compute_currency_id" => self._compute_currency_id(env, ctx, rs, args).await,
            "_compute_journal_id" => self._compute_journal_id(env, ctx, rs, args).await,
            "_compute_is_complete" => self._compute_is_complete(env, ctx, rs, args).await,
            "_compute_is_valid" => self._compute_is_valid(env, ctx, rs, args).await,
            "_compute_problem_description" => self._compute_problem_description(env, ctx, rs, args).await,
            "_search_is_valid" => self._search_is_valid(env, ctx, rs, args).await,
            "_get_statement_validity" => self._get_statement_validity(env, ctx, rs, args).await,
            "_get_invalid_statement_ids" => self._get_invalid_statement_ids(env, ctx, rs, args).await,
            "default_get" => self.default_get(env, ctx, rs, args).await,
            "_check_attachments" => self._check_attachments(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountBankStatementFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement.py:119`). Decoradores: api.depends('create_date').
    async fn _compute_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement._compute_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement.py:127`). Decoradores: api.depends('line_ids.internal_index', 'line_ids.state').
    async fn _compute_first_line_index(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement._compute_first_line_index".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement.py:134`). Decoradores: api.depends('line_ids.internal_index', 'line_ids.state').
    async fn _compute_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement._compute_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement.py:141`). Decoradores: api.depends('create_date').
    async fn _compute_balance_start(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement._compute_balance_start".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement.py:170`). Decoradores: api.depends('balance_start', 'line_ids.amount', 'line_ids.state').
    async fn _compute_balance_end(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement._compute_balance_end".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement.py:176`). Decoradores: api.depends('balance_start').
    async fn _compute_balance_end_real(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement._compute_balance_end_real".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement.py:181`). Decoradores: api.depends('journal_id.currency_id', 'company_id.currency_id').
    async fn _compute_currency_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement._compute_currency_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement.py:186`). Decoradores: api.depends('line_ids.journal_id').
    async fn _compute_journal_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement._compute_journal_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement.py:191`). Decoradores: api.depends('balance_end', 'balance_end_real', 'line_ids.amount', 'line_ids.state').
    async fn _compute_is_complete(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement._compute_is_complete".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement.py:197`). Decoradores: api.depends('balance_end', 'balance_end_real').
    async fn _compute_is_valid(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement._compute_is_valid".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement.py:210`). Decoradores: api.depends('is_valid', 'is_complete').
    async fn _compute_problem_description(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement._compute_problem_description".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement.py:219`).
    async fn _search_is_valid(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement._search_is_valid".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement.py:228`).
    async fn _get_statement_validity(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement._get_statement_validity".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement.py:242`).
    async fn _get_invalid_statement_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement._get_invalid_statement_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement.py:282`). Decoradores: api.model.
    async fn default_get(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.default_get".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement.py:340`). Decoradores: contextmanager.
    async fn _check_attachments(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement._check_attachments".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement.py:360`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_bank_statement.py:366`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.bank.statement.write".into(),
        ))
    }

}
