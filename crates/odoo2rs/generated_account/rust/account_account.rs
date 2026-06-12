//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.account`

use nexus_orm::prelude::*;

pub struct AccountAccountFragment;

#[async_trait]
impl ModelFragment for AccountAccountFragment {
    fn model_name(&self) -> &str {
        "account.account"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Account".into();
        def.order = "code, placeholder_code".into();
        def.add_field(FieldDef::char("name").string("Account Name").required());
        def.add_field(FieldDef::text("description"));
        def.add_field(FieldDef::many2one("currency_id", "res.currency").string("Account Currency"));
        def.add_field(FieldDef::many2one("company_currency_id", "res.currency").computed("_compute_company_currency_id", &[]).stored());
        def.add_field(FieldDef::char("company_fiscal_country_code").computed("_compute_company_fiscal_country_code", &[]).stored());
        def.add_field(FieldDef::char("code").string("Code").computed("_compute_code", &["code_store"]).stored());
        def.add_field(FieldDef::char("code_store"));
        def.add_field(FieldDef::char("placeholder_code").string("Display code").computed("_compute_placeholder_code", &["code"]).stored());
        def.add_field(FieldDef::boolean("active").default_val(true));
        def.add_field(FieldDef::boolean("used").computed("_compute_used", &[]).stored());
        def.add_field(FieldDef::selection("account_type", &[("asset_receivable", "Receivable"), ("asset_cash", "Bank and Cash"), ("asset_current", "Current Assets"), ("asset_non_current", "Non-current Assets"), ("asset_prepayments", "Prepayments"), ("asset_fixed", "Fixed Assets"), ("liability_payable", "Payable"), ("liability_credit_card", "Credit Card"), ("liability_current", "Current Liabilities"), ("liability_non_current", "Non-current Liabilities"), ("equity", "Equity"), ("equity_unaffected", "Current Year Earnings"), ("income", "Income"), ("income_other", "Other Income"), ("expense", "Expenses"), ("expense_other", "Other Expenses"), ("expense_depreciation", "Depreciation"), ("expense_direct_cost", "Cost of Revenue"), ("off_balance", "Off-Balance Sheet")]).string("Type").required().computed("_compute_account_type", &["code"]).stored());
        def.add_field(FieldDef::boolean("include_initial_balance").string("Bring Accounts Balance Forward").computed("_compute_include_initial_balance", &["account_type"]).stored());
        def.add_field(FieldDef::selection("internal_group", &[("equity", "Equity"), ("asset", "Asset"), ("liability", "Liability"), ("income", "Income"), ("expense", "Expense"), ("off", "Off Balance")]).string("Internal Group").computed("_compute_internal_group", &["account_type"]).stored());
        def.add_field(FieldDef::boolean("reconcile").string("Allow Reconciliation").computed("_compute_reconcile", &["account_type"]).stored());
        def.add_field(FieldDef::many2many("tax_ids", "account.tax").string("Default Taxes"));
        def.add_field(FieldDef::text("note").string("Internal Notes"));
        def.add_field(FieldDef::many2many("company_ids", "res.company").string("Companies").required());
        def.add_field(FieldDef::one2many("code_mapping_ids", "account.code.mapping", "account_id"));
        def.add_field(FieldDef::many2many("tag_ids", "account.account.tag").string("Tags").computed("_compute_account_tags", &["code"]).stored());
        def.add_field(FieldDef::many2one("group_id", "account.group").computed("_compute_account_group", &["code"]).stored());
        def.add_field(FieldDef::many2one("root_id", "account.root").computed("_compute_account_root", &["code"]).stored());
        def.add_field(FieldDef::monetary("opening_debit").string("Opening Debit").computed("_compute_opening_debit_credit", &[]).stored());
        def.add_field(FieldDef::monetary("opening_credit").string("Opening Credit").computed("_compute_opening_debit_credit", &[]).stored());
        def.add_field(FieldDef::monetary("opening_balance").string("Opening Balance").computed("_compute_opening_debit_credit", &[]).stored());
        def.add_field(FieldDef::float("current_balance").computed("_compute_current_balance", &[]).stored());
        def.add_field(FieldDef::integer("related_taxes_amount").computed("_compute_related_taxes_amount", &[]).stored());
        def.add_field(FieldDef::boolean("non_trade").default_val(false));
        def.add_field({ let mut f = FieldDef::boolean("display_mapping_tab"); f.store = false; f });
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_check_reconcile", "_field_to_sql", "_constrains_reconcile", "_check_journal_consistency", "_check_company_consistency", "_check_account_type_sales_purchase_journal", "_check_account_code", "_check_account_is_bank_journal_bank_account", "_compute_code", "_search_code", "_inverse_code", "_compute_placeholder_code", "_search_placeholder_code", "_compute_account_root", "_search_account_root", "_search_panel_domain_image", "_compute_account_group", "_get_used_account_ids", "_search_used", "_compute_used", "_search_new_account_code", "_compute_current_balance", "_compute_related_taxes_amount", "_compute_company_currency_id", "_compute_company_fiscal_country_code", "_compute_opening_debit_credit", "_compute_account_type", "_compute_account_tags", "_get_closest_parent_account", "_compute_include_initial_balance", "_search_include_initial_balance", "_get_internal_group", "_compute_internal_group", "_search_internal_group", "_compute_reconcile", "_set_opening_debit", "_set_opening_credit", "_set_opening_balance", "_set_opening_debit_credit", "default_get", "_get_most_frequent_accounts_for_partner", "_get_most_frequent_account_for_partner", "_order_accounts_by_frequency_for_partner", "_order_to_sql", "name_search", "_search_display_name", "_onchange_account_type", "_split_code_name", "_onchange_name", "_compute_display_name", "copy_data", "copy_translations", "_load_precommit_update_opening_move", "_toggle_reconcile_to_true", "_toggle_reconcile_to_false", "name_create", "create", "write", "_ensure_code_is_unique", "_load_records_write", "_unlink_except_contains_journal_items", "_unlink_except_linked_to_fiscal_position", "_unlink_except_linked_to_tax_repartition_line", "action_open_related_taxes", "get_import_templates", "_merge_method", "action_unmerge", "_check_action_unmerge_possible", "_action_unmerge_get_user_confirmation", "_action_unmerge"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_check_reconcile" => self._check_reconcile(env, ctx, rs, args).await,
            "_field_to_sql" => self._field_to_sql(env, ctx, rs, args).await,
            "_constrains_reconcile" => self._constrains_reconcile(env, ctx, rs, args).await,
            "_check_journal_consistency" => self._check_journal_consistency(env, ctx, rs, args).await,
            "_check_company_consistency" => self._check_company_consistency(env, ctx, rs, args).await,
            "_check_account_type_sales_purchase_journal" => self._check_account_type_sales_purchase_journal(env, ctx, rs, args).await,
            "_check_account_code" => self._check_account_code(env, ctx, rs, args).await,
            "_check_account_is_bank_journal_bank_account" => self._check_account_is_bank_journal_bank_account(env, ctx, rs, args).await,
            "_compute_code" => self._compute_code(env, ctx, rs, args).await,
            "_search_code" => self._search_code(env, ctx, rs, args).await,
            "_inverse_code" => self._inverse_code(env, ctx, rs, args).await,
            "_compute_placeholder_code" => self._compute_placeholder_code(env, ctx, rs, args).await,
            "_search_placeholder_code" => self._search_placeholder_code(env, ctx, rs, args).await,
            "_compute_account_root" => self._compute_account_root(env, ctx, rs, args).await,
            "_search_account_root" => self._search_account_root(env, ctx, rs, args).await,
            "_search_panel_domain_image" => self._search_panel_domain_image(env, ctx, rs, args).await,
            "_compute_account_group" => self._compute_account_group(env, ctx, rs, args).await,
            "_get_used_account_ids" => self._get_used_account_ids(env, ctx, rs, args).await,
            "_search_used" => self._search_used(env, ctx, rs, args).await,
            "_compute_used" => self._compute_used(env, ctx, rs, args).await,
            "_search_new_account_code" => self._search_new_account_code(env, ctx, rs, args).await,
            "_compute_current_balance" => self._compute_current_balance(env, ctx, rs, args).await,
            "_compute_related_taxes_amount" => self._compute_related_taxes_amount(env, ctx, rs, args).await,
            "_compute_company_currency_id" => self._compute_company_currency_id(env, ctx, rs, args).await,
            "_compute_company_fiscal_country_code" => self._compute_company_fiscal_country_code(env, ctx, rs, args).await,
            "_compute_opening_debit_credit" => self._compute_opening_debit_credit(env, ctx, rs, args).await,
            "_compute_account_type" => self._compute_account_type(env, ctx, rs, args).await,
            "_compute_account_tags" => self._compute_account_tags(env, ctx, rs, args).await,
            "_get_closest_parent_account" => self._get_closest_parent_account(env, ctx, rs, args).await,
            "_compute_include_initial_balance" => self._compute_include_initial_balance(env, ctx, rs, args).await,
            "_search_include_initial_balance" => self._search_include_initial_balance(env, ctx, rs, args).await,
            "_get_internal_group" => self._get_internal_group(env, ctx, rs, args).await,
            "_compute_internal_group" => self._compute_internal_group(env, ctx, rs, args).await,
            "_search_internal_group" => self._search_internal_group(env, ctx, rs, args).await,
            "_compute_reconcile" => self._compute_reconcile(env, ctx, rs, args).await,
            "_set_opening_debit" => self._set_opening_debit(env, ctx, rs, args).await,
            "_set_opening_credit" => self._set_opening_credit(env, ctx, rs, args).await,
            "_set_opening_balance" => self._set_opening_balance(env, ctx, rs, args).await,
            "_set_opening_debit_credit" => self._set_opening_debit_credit(env, ctx, rs, args).await,
            "default_get" => self.default_get(env, ctx, rs, args).await,
            "_get_most_frequent_accounts_for_partner" => self._get_most_frequent_accounts_for_partner(env, ctx, rs, args).await,
            "_get_most_frequent_account_for_partner" => self._get_most_frequent_account_for_partner(env, ctx, rs, args).await,
            "_order_accounts_by_frequency_for_partner" => self._order_accounts_by_frequency_for_partner(env, ctx, rs, args).await,
            "_order_to_sql" => self._order_to_sql(env, ctx, rs, args).await,
            "name_search" => self.name_search(env, ctx, rs, args).await,
            "_search_display_name" => self._search_display_name(env, ctx, rs, args).await,
            "_onchange_account_type" => self._onchange_account_type(env, ctx, rs, args).await,
            "_split_code_name" => self._split_code_name(env, ctx, rs, args).await,
            "_onchange_name" => self._onchange_name(env, ctx, rs, args).await,
            "_compute_display_name" => self._compute_display_name(env, ctx, rs, args).await,
            "copy_data" => self.copy_data(env, ctx, rs, args).await,
            "copy_translations" => self.copy_translations(env, ctx, rs, args).await,
            "_load_precommit_update_opening_move" => self._load_precommit_update_opening_move(env, ctx, rs, args).await,
            "_toggle_reconcile_to_true" => self._toggle_reconcile_to_true(env, ctx, rs, args).await,
            "_toggle_reconcile_to_false" => self._toggle_reconcile_to_false(env, ctx, rs, args).await,
            "name_create" => self.name_create(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "_ensure_code_is_unique" => self._ensure_code_is_unique(env, ctx, rs, args).await,
            "_load_records_write" => self._load_records_write(env, ctx, rs, args).await,
            "_unlink_except_contains_journal_items" => self._unlink_except_contains_journal_items(env, ctx, rs, args).await,
            "_unlink_except_linked_to_fiscal_position" => self._unlink_except_linked_to_fiscal_position(env, ctx, rs, args).await,
            "_unlink_except_linked_to_tax_repartition_line" => self._unlink_except_linked_to_tax_repartition_line(env, ctx, rs, args).await,
            "action_open_related_taxes" => self.action_open_related_taxes(env, ctx, rs, args).await,
            "get_import_templates" => self.get_import_templates(env, ctx, rs, args).await,
            "_merge_method" => self._merge_method(env, ctx, rs, args).await,
            "action_unmerge" => self.action_unmerge(env, ctx, rs, args).await,
            "_check_action_unmerge_possible" => self._check_action_unmerge_possible(env, ctx, rs, args).await,
            "_action_unmerge_get_user_confirmation" => self._action_unmerge_get_user_confirmation(env, ctx, rs, args).await,
            "_action_unmerge" => self._action_unmerge(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountAccountFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:28`). Decoradores: api.constrains('account_type', 'reconcile').
    async fn _check_reconcile(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._check_reconcile".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:130`).
    async fn _field_to_sql(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._field_to_sql".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:188`). Decoradores: api.constrains('reconcile', 'account_type', 'tax_ids').
    async fn _constrains_reconcile(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._constrains_reconcile".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:197`). Decoradores: api.constrains('currency_id').
    async fn _check_journal_consistency(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._check_journal_consistency".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:270`). Decoradores: api.constrains('company_ids', 'account_type').
    async fn _check_company_consistency(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._check_company_consistency".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:291`). Decoradores: api.constrains('account_type').
    async fn _check_account_type_sales_purchase_journal(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._check_account_type_sales_purchase_journal".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:311`). Decoradores: api.constrains('code').
    async fn _check_account_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._check_account_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:319`). Decoradores: api.constrains('account_type').
    async fn _check_account_is_bank_journal_bank_account(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._check_account_is_bank_journal_bank_account".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:336`). Decoradores: api.depends_context('company'), api.depends('code_store').
    async fn _compute_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._compute_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:341`).
    async fn _search_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._search_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:344`).
    async fn _inverse_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._inverse_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:357`). Decoradores: api.depends_context('company'), api.depends('code').
    async fn _compute_placeholder_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._compute_placeholder_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:367`).
    async fn _search_placeholder_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._search_placeholder_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:380`). Decoradores: api.depends_context('company'), api.depends('code').
    async fn _compute_account_root(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._compute_account_root".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:384`).
    async fn _search_account_root(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._search_account_root".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:399`).
    async fn _search_panel_domain_image(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._search_panel_domain_image".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:418`). Decoradores: api.depends_context('company'), api.depends('code').
    async fn _compute_account_group(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._compute_account_group".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:448`).
    async fn _get_used_account_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._get_used_account_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:455`).
    async fn _search_used(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._search_used".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:460`).
    async fn _compute_used(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._compute_used".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:466`). Decoradores: api.model.
    async fn _search_new_account_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._search_new_account_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:547`). Decoradores: api.depends_context('company').
    async fn _compute_current_balance(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._compute_current_balance".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:559`). Decoradores: api.depends_context('company').
    async fn _compute_related_taxes_amount(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._compute_related_taxes_amount".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:567`). Decoradores: api.depends_context('company').
    async fn _compute_company_currency_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._compute_company_currency_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:571`). Decoradores: api.depends_context('company').
    async fn _compute_company_fiscal_country_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._compute_company_fiscal_country_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:575`). Decoradores: api.depends_context('company').
    async fn _compute_opening_debit_credit(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._compute_opening_debit_credit".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:604`). Decoradores: api.depends('code').
    async fn _compute_account_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._compute_account_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:609`). Decoradores: api.depends('code').
    async fn _compute_account_tags(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._compute_account_tags".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:613`).
    async fn _get_closest_parent_account(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._get_closest_parent_account".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:639`). Decoradores: api.depends('account_type').
    async fn _compute_include_initial_balance(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._compute_include_initial_balance".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:643`).
    async fn _search_include_initial_balance(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._search_include_initial_balance".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:648`).
    async fn _get_internal_group(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._get_internal_group".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:652`). Decoradores: api.depends('account_type').
    async fn _compute_internal_group(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._compute_internal_group".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:656`).
    async fn _search_internal_group(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._search_internal_group".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:665`). Decoradores: api.depends('account_type').
    async fn _compute_reconcile(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._compute_reconcile".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:675`).
    async fn _set_opening_debit(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._set_opening_debit".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:679`).
    async fn _set_opening_credit(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._set_opening_credit".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:683`).
    async fn _set_opening_balance(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._set_opening_balance".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:690`).
    async fn _set_opening_debit_credit(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._set_opening_debit_credit".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:707`). Decoradores: api.model.
    async fn default_get(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account.default_get".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:730`). Decoradores: api.model.
    async fn _get_most_frequent_accounts_for_partner(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._get_most_frequent_accounts_for_partner".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:776`). Decoradores: api.model.
    async fn _get_most_frequent_account_for_partner(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._get_most_frequent_account_for_partner".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:781`). Decoradores: api.model.
    async fn _order_accounts_by_frequency_for_partner(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._order_accounts_by_frequency_for_partner".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:784`).
    async fn _order_to_sql(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._order_to_sql".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:807`). Decoradores: api.model, api.readonly.
    async fn name_search(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account.name_search".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:836`). Decoradores: api.model.
    async fn _search_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._search_display_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:852`). Decoradores: api.onchange('account_type').
    async fn _onchange_account_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._onchange_account_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:856`).
    async fn _split_code_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._split_code_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:862`). Decoradores: api.onchange('name').
    async fn _onchange_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._onchange_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:870`). Decoradores: api.depends_context('company', 'formatted_display_name'), api.depends('code').
    async fn _compute_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._compute_display_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:890`).
    async fn copy_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account.copy_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:914`).
    async fn copy_translations(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account.copy_translations".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:924`). Decoradores: api.model.
    async fn _load_precommit_update_opening_move(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._load_precommit_update_opening_move".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:942`).
    async fn _toggle_reconcile_to_true(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._toggle_reconcile_to_true".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:960`).
    async fn _toggle_reconcile_to_false(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._toggle_reconcile_to_false".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:987`). Decoradores: api.model.
    async fn name_create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account.name_create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:999`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1040`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1066`).
    async fn _ensure_code_is_unique(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._ensure_code_is_unique".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1118`).
    async fn _load_records_write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._load_records_write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1125`). Decoradores: api.ondelete().
    async fn _unlink_except_contains_journal_items(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._unlink_except_contains_journal_items".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1130`). Decoradores: api.ondelete().
    async fn _unlink_except_linked_to_fiscal_position(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._unlink_except_linked_to_fiscal_position".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1135`). Decoradores: api.ondelete().
    async fn _unlink_except_linked_to_tax_repartition_line(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._unlink_except_linked_to_tax_repartition_line".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1139`).
    async fn action_open_related_taxes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account.action_open_related_taxes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1152`). Decoradores: api.model.
    async fn get_import_templates(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account.get_import_templates".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1158`).
    async fn _merge_method(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._merge_method".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1161`).
    async fn action_unmerge(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account.action_unmerge".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1178`).
    async fn _check_action_unmerge_possible(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._check_action_unmerge_possible".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1195`).
    async fn _action_unmerge_get_user_confirmation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._action_unmerge_get_user_confirmation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1211`).
    async fn _action_unmerge(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account._action_unmerge".into(),
        ))
    }

}
