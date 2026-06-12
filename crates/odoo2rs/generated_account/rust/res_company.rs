//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `res.company`

use nexus_orm::prelude::*;

pub struct ResCompanyFragment;

#[async_trait]
impl ModelFragment for ResCompanyFragment {
    fn model_name(&self) -> &str {
        "res.company"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::integer("fiscalyear_last_day").required().default_val(31i64));
        def.add_field(FieldDef::selection("fiscalyear_last_month", &[]).required().default_val("12"));
        def.add_field(FieldDef::date("fiscalyear_lock_date").string("Global Lock Date"));
        def.add_field(FieldDef::date("tax_lock_date").string("Tax Return Lock Date"));
        def.add_field(FieldDef::date("sale_lock_date").string("Sales Lock Date"));
        def.add_field(FieldDef::date("purchase_lock_date").string("Purchase Lock date"));
        def.add_field(FieldDef::date("hard_lock_date").string("Hard Lock Date"));
        def.add_field(FieldDef::date("user_fiscalyear_lock_date").computed("_compute_user_fiscalyear_lock_date", &["fiscalyear_lock_date"]).stored());
        def.add_field(FieldDef::date("user_tax_lock_date").computed("_compute_user_tax_lock_date", &["tax_lock_date"]).stored());
        def.add_field(FieldDef::date("user_sale_lock_date").computed("_compute_user_sale_lock_date", &["sale_lock_date"]).stored());
        def.add_field(FieldDef::date("user_purchase_lock_date").computed("_compute_user_purchase_lock_date", &["purchase_lock_date"]).stored());
        def.add_field(FieldDef::date("user_hard_lock_date").computed("_compute_user_hard_lock_date", &["hard_lock_date"]).stored());
        def.add_field(FieldDef::many2one("transfer_account_id", "account.account").string("Inter-Banks Transfer Account"));
        def.add_field(FieldDef::boolean("expects_chart_of_accounts").string("Expects a Chart of Accounts").default_val(true));
        def.add_field(FieldDef::selection("chart_template", &[]));
        def.add_field(FieldDef::char("bank_account_code_prefix").string("Prefix of the bank accounts"));
        def.add_field(FieldDef::char("cash_account_code_prefix").string("Prefix of the cash accounts"));
        def.add_field(FieldDef::many2one("default_cash_difference_income_account_id", "account.account").string("Cash Difference Income"));
        def.add_field(FieldDef::many2one("default_cash_difference_expense_account_id", "account.account").string("Cash Difference Expense"));
        def.add_field(FieldDef::many2one("account_journal_suspense_account_id", "account.account").string("Journal Suspense Account"));
        def.add_field(FieldDef::many2one("account_journal_early_pay_discount_gain_account_id", "account.account").string("Cash Discount Write-Off Gain Account"));
        def.add_field(FieldDef::many2one("account_journal_early_pay_discount_loss_account_id", "account.account").string("Cash Discount Write-Off Loss Account"));
        def.add_field(FieldDef::char("transfer_account_code_prefix").string("Prefix of the transfer accounts"));
        def.add_field(FieldDef::many2one("account_sale_tax_id", "account.tax").string("Default Sale Tax"));
        def.add_field(FieldDef::many2one("account_purchase_tax_id", "account.tax").string("Default Purchase Tax"));
        def.add_field(FieldDef::many2one("account_purchase_receipt_fiscal_position_id", "account.fiscal.position").string("Default Purchase Receipt Fiscal Position"));
        def.add_field(FieldDef::selection("tax_calculation_rounding_method", &[("round_globally", "Round per Tax"), ("round_per_line", "Round per Line")]).string("Tax Calculation Rounding Method").default_val("round_globally"));
        def.add_field(FieldDef::many2one("currency_exchange_journal_id", "account.journal").string("Exchange Gain or Loss Journal"));
        def.add_field(FieldDef::many2one("income_currency_exchange_account_id", "account.account").string("Gain Exchange Rate Account"));
        def.add_field(FieldDef::many2one("expense_currency_exchange_account_id", "account.account").string("Loss Exchange Rate Account"));
        def.add_field(FieldDef::boolean("anglo_saxon_accounting").string("Use anglo-saxon accounting"));
        def.add_field(FieldDef::one2many("bank_journal_ids", "account.journal", "company_id").string("Bank Journals"));
        def.add_field(FieldDef::many2one("incoterm_id", "account.incoterms").string("Default incoterm"));
        def.add_field(FieldDef::boolean("qr_code").string("Display QR-code on invoices"));
        def.add_field(FieldDef::boolean("link_qr_code").string("Display Link QR-code"));
        def.add_field(FieldDef::boolean("display_invoice_amount_total_words").string("Total amount of invoice in letters"));
        def.add_field(FieldDef::boolean("display_invoice_tax_company_currency").string("Taxes in company currency").default_val(true));
        def.add_field(FieldDef::boolean("account_use_credit_limit").string("Sales Credit Limit"));
        def.add_field(FieldDef::many2one("batch_payment_sequence_id", "ir.sequence").readonly());
        def.add_field(FieldDef::many2one("account_opening_move_id", "account.move").string("Opening Journal Entry"));
        def.add_field({ let mut f = FieldDef::many2one("account_opening_journal_id", "account.journal").string("Opening Journal"); f.related = Some("account_opening_move_id.journal_id".into()); f });
        def.add_field(FieldDef::date("account_opening_date").string("Opening Entry"));
        def.add_field(FieldDef::html("invoice_terms").string("Default Terms and Conditions"));
        def.add_field(FieldDef::selection("terms_type", &[("plain", "Add a Note"), ("html", "Add a link to a Web Page")]).string("Terms & Conditions format").default_val("plain"));
        def.add_field(FieldDef::html("invoice_terms_html").string("Default Terms and Conditions as a Web page").computed("_compute_invoice_terms_html", &["terms_type"]).stored());
        def.add_field(FieldDef::many2one("account_default_pos_receivable_account_id", "account.account").string("Default PoS Receivable Account"));
        def.add_field(FieldDef::many2one("expense_accrual_account_id", "account.account"));
        def.add_field(FieldDef::many2one("revenue_accrual_account_id", "account.account"));
        def.add_field(FieldDef::many2one("automatic_entry_default_journal_id", "account.journal"));
        def.add_field(FieldDef::many2one("domestic_fiscal_position_id", "account.fiscal.position").computed("_compute_domestic_fiscal_position_id", &["fiscal_position_ids", "fiscal_position_ids.sequence", "fiscal_position_ids.country_id", "fiscal_position_ids.country_group_id"]).stored());
        def.add_field(FieldDef::many2one("account_fiscal_country_id", "res.country").string("Fiscal Country").computed("compute_account_tax_fiscal_country", &["country_id"]).stored());
        def.add_field(FieldDef::json("account_fiscal_country_group_codes").computed("_compute_account_fiscal_country_group_codes", &["account_fiscal_country_id"]).stored());
        def.add_field(FieldDef::many2many("account_enabled_tax_country_ids", "res.country").string("l10n-used countries").computed("_compute_account_enabled_tax_country_ids", &["account_fiscal_country_id"]).stored());
        def.add_field(FieldDef::boolean("tax_exigibility").string("Use Cash Basis"));
        def.add_field(FieldDef::many2one("tax_cash_basis_journal_id", "account.journal").string("Cash Basis Journal"));
        def.add_field(FieldDef::many2one("account_cash_basis_base_account_id", "account.account").string("Base Tax Received Account"));
        def.add_field(FieldDef::boolean("account_storno").string("Storno accounting").computed("_compute_account_storno", &["account_fiscal_country_id"]).stored());
        def.add_field(FieldDef::boolean("display_account_storno").computed("_compute_display_account_storno", &["account_fiscal_country_id"]).stored());
        def.add_field(FieldDef::one2many("fiscal_position_ids", "account.fiscal.position", "company_id"));
        def.add_field(FieldDef::many2many("multi_vat_foreign_country_ids", "res.country").string("Foreign VAT countries").computed("_compute_multi_vat_foreign_country", &["fiscal_position_ids.foreign_vat"]).stored());
        def.add_field(FieldDef::selection("quick_edit_mode", &[("out_invoices", "Customer Invoices"), ("in_invoices", "Vendor Bills"), ("out_and_in_invoices", "Customer Invoices and Vendor Bills")]).string("Quick encoding"));
        def.add_field(FieldDef::many2one("account_discount_income_allocation_id", "account.account").string("Separate account for income discount"));
        def.add_field(FieldDef::many2one("account_discount_expense_allocation_id", "account.account").string("Separate account for expense discount"));
        def.add_field(FieldDef::boolean("restrictive_audit_trail").string("Restrictive Audit Trail"));
        def.add_field(FieldDef::boolean("force_restrictive_audit_trail").string("Force Audit Trail").computed("_compute_force_restrictive_audit_trail", &[]).stored());
        def.add_field(FieldDef::boolean("autopost_bills").string("Auto-validate bills").default_val(true));
        def.add_field(FieldDef::selection("account_price_include", &[("tax_included", "Tax Included"), ("tax_excluded", "Tax Excluded")]).string("Default Sales Price Include").required().default_val("tax_excluded"));
        def.add_field(FieldDef::char("company_vat_placeholder").computed("_compute_company_vat_placeholder", &["country_id", "account_fiscal_country_id"]).stored());
        def.add_field(FieldDef::char("company_registry_placeholder").computed("_compute_company_registry_placeholder", &["country_id", "account_fiscal_country_id"]).stored());
        def.add_field(FieldDef::many2one("income_account_id", "account.account").string("Income Account"));
        def.add_field(FieldDef::many2one("expense_account_id", "account.account").string("Expense Account"));
        def.add_field(FieldDef::many2one("price_difference_account_id", "account.account").string("Price Difference Account"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["get_next_batch_payment_communication", "_get_company_root_delegated_field_names", "_check_audit_trail_restriction", "_check_set_account_price_include", "_check_fiscalyear_last_day", "_compute_force_restrictive_audit_trail", "_compute_domestic_fiscal_position_id", "_compute_account_fiscal_country_group_codes", "_compute_multi_vat_foreign_country", "compute_account_tax_fiscal_country", "_compute_account_enabled_tax_country_ids", "_compute_invoice_terms_html", "_compute_user_fiscalyear_lock_date", "_compute_user_tax_lock_date", "_compute_user_sale_lock_date", "_compute_user_purchase_lock_date", "_compute_user_hard_lock_date", "_compute_account_storno", "_compute_display_account_storno", "_initiate_account_onboardings", "create", "get_new_account_code", "reflect_code_prefix_change", "_get_unreconciled_statement_lines_redirect_action", "_get_unreconciled_statement_lines_domain", "_validate_locks", "_get_user_lock_date", "_get_user_fiscal_lock_date", "_get_violated_soft_lock_date", "_get_lock_date_violations", "_format_lock_dates", "_get_violated_lock_dates", "write", "setting_init_bank_account_action", "setting_init_credit_card_account_action", "_get_default_opening_move_values", "opening_move_posted", "get_unaffected_earnings_account", "_update_opening_move", "action_save_onboarding_sale_tax", "action_save_onboarding_company_data", "get_chart_of_accounts_or_fail", "install_l10n_modules", "_existing_accounting", "_chart_template_selection", "_action_check_hash_integrity", "_check_hash_integrity", "_with_locked_records", "compute_fiscalyear_dates", "_compute_company_vat_placeholder", "_compute_company_registry_placeholder", "_set_category_defaults", "_check_tax_return_configuration"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "get_next_batch_payment_communication" => self.get_next_batch_payment_communication(env, ctx, rs, args).await,
            "_get_company_root_delegated_field_names" => self._get_company_root_delegated_field_names(env, ctx, rs, args).await,
            "_check_audit_trail_restriction" => self._check_audit_trail_restriction(env, ctx, rs, args).await,
            "_check_set_account_price_include" => self._check_set_account_price_include(env, ctx, rs, args).await,
            "_check_fiscalyear_last_day" => self._check_fiscalyear_last_day(env, ctx, rs, args).await,
            "_compute_force_restrictive_audit_trail" => self._compute_force_restrictive_audit_trail(env, ctx, rs, args).await,
            "_compute_domestic_fiscal_position_id" => self._compute_domestic_fiscal_position_id(env, ctx, rs, args).await,
            "_compute_account_fiscal_country_group_codes" => self._compute_account_fiscal_country_group_codes(env, ctx, rs, args).await,
            "_compute_multi_vat_foreign_country" => self._compute_multi_vat_foreign_country(env, ctx, rs, args).await,
            "compute_account_tax_fiscal_country" => self.compute_account_tax_fiscal_country(env, ctx, rs, args).await,
            "_compute_account_enabled_tax_country_ids" => self._compute_account_enabled_tax_country_ids(env, ctx, rs, args).await,
            "_compute_invoice_terms_html" => self._compute_invoice_terms_html(env, ctx, rs, args).await,
            "_compute_user_fiscalyear_lock_date" => self._compute_user_fiscalyear_lock_date(env, ctx, rs, args).await,
            "_compute_user_tax_lock_date" => self._compute_user_tax_lock_date(env, ctx, rs, args).await,
            "_compute_user_sale_lock_date" => self._compute_user_sale_lock_date(env, ctx, rs, args).await,
            "_compute_user_purchase_lock_date" => self._compute_user_purchase_lock_date(env, ctx, rs, args).await,
            "_compute_user_hard_lock_date" => self._compute_user_hard_lock_date(env, ctx, rs, args).await,
            "_compute_account_storno" => self._compute_account_storno(env, ctx, rs, args).await,
            "_compute_display_account_storno" => self._compute_display_account_storno(env, ctx, rs, args).await,
            "_initiate_account_onboardings" => self._initiate_account_onboardings(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "get_new_account_code" => self.get_new_account_code(env, ctx, rs, args).await,
            "reflect_code_prefix_change" => self.reflect_code_prefix_change(env, ctx, rs, args).await,
            "_get_unreconciled_statement_lines_redirect_action" => self._get_unreconciled_statement_lines_redirect_action(env, ctx, rs, args).await,
            "_get_unreconciled_statement_lines_domain" => self._get_unreconciled_statement_lines_domain(env, ctx, rs, args).await,
            "_validate_locks" => self._validate_locks(env, ctx, rs, args).await,
            "_get_user_lock_date" => self._get_user_lock_date(env, ctx, rs, args).await,
            "_get_user_fiscal_lock_date" => self._get_user_fiscal_lock_date(env, ctx, rs, args).await,
            "_get_violated_soft_lock_date" => self._get_violated_soft_lock_date(env, ctx, rs, args).await,
            "_get_lock_date_violations" => self._get_lock_date_violations(env, ctx, rs, args).await,
            "_format_lock_dates" => self._format_lock_dates(env, ctx, rs, args).await,
            "_get_violated_lock_dates" => self._get_violated_lock_dates(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "setting_init_bank_account_action" => self.setting_init_bank_account_action(env, ctx, rs, args).await,
            "setting_init_credit_card_account_action" => self.setting_init_credit_card_account_action(env, ctx, rs, args).await,
            "_get_default_opening_move_values" => self._get_default_opening_move_values(env, ctx, rs, args).await,
            "opening_move_posted" => self.opening_move_posted(env, ctx, rs, args).await,
            "get_unaffected_earnings_account" => self.get_unaffected_earnings_account(env, ctx, rs, args).await,
            "_update_opening_move" => self._update_opening_move(env, ctx, rs, args).await,
            "action_save_onboarding_sale_tax" => self.action_save_onboarding_sale_tax(env, ctx, rs, args).await,
            "action_save_onboarding_company_data" => self.action_save_onboarding_company_data(env, ctx, rs, args).await,
            "get_chart_of_accounts_or_fail" => self.get_chart_of_accounts_or_fail(env, ctx, rs, args).await,
            "install_l10n_modules" => self.install_l10n_modules(env, ctx, rs, args).await,
            "_existing_accounting" => self._existing_accounting(env, ctx, rs, args).await,
            "_chart_template_selection" => self._chart_template_selection(env, ctx, rs, args).await,
            "_action_check_hash_integrity" => self._action_check_hash_integrity(env, ctx, rs, args).await,
            "_check_hash_integrity" => self._check_hash_integrity(env, ctx, rs, args).await,
            "_with_locked_records" => self._with_locked_records(env, ctx, rs, args).await,
            "compute_fiscalyear_dates" => self.compute_fiscalyear_dates(env, ctx, rs, args).await,
            "_compute_company_vat_placeholder" => self._compute_company_vat_placeholder(env, ctx, rs, args).await,
            "_compute_company_registry_placeholder" => self._compute_company_registry_placeholder(env, ctx, rs, args).await,
            "_set_category_defaults" => self._set_category_defaults(env, ctx, rs, args).await,
            "_check_tax_return_configuration" => self._check_tax_return_configuration(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl ResCompanyFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:312`).
    async fn get_next_batch_payment_communication(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company.get_next_batch_payment_communication".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:320`).
    async fn _get_company_root_delegated_field_names(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._get_company_root_delegated_field_names".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:329`). Decoradores: api.constrains('restrictive_audit_trail').
    async fn _check_audit_trail_restriction(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._check_audit_trail_restriction".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:335`). Decoradores: api.constrains('account_price_include').
    async fn _check_set_account_price_include(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._check_set_account_price_include".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:340`). Decoradores: api.constrains('account_opening_move_id', 'fiscalyear_last_day', 'fiscalyear_last_month').
    async fn _check_fiscalyear_last_day(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._check_fiscalyear_last_day".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:356`).
    async fn _compute_force_restrictive_audit_trail(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._compute_force_restrictive_audit_trail".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:361`). Decoradores: api.depends('fiscal_position_ids', 'fiscal_position_ids.sequence', 'fiscal_position_ids.country_id', 'fiscal_position_ids.country_group_id').
    async fn _compute_domestic_fiscal_position_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._compute_domestic_fiscal_position_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:373`). Decoradores: api.depends('account_fiscal_country_id').
    async fn _compute_account_fiscal_country_group_codes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._compute_account_fiscal_country_group_codes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:380`). Decoradores: api.depends('fiscal_position_ids.foreign_vat').
    async fn _compute_multi_vat_foreign_country(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._compute_multi_vat_foreign_country".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:396`). Decoradores: api.depends('country_id').
    async fn compute_account_tax_fiscal_country(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company.compute_account_tax_fiscal_country".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:402`). Decoradores: api.depends('account_fiscal_country_id').
    async fn _compute_account_enabled_tax_country_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._compute_account_enabled_tax_country_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:415`). Decoradores: api.depends('terms_type').
    async fn _compute_invoice_terms_html(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._compute_invoice_terms_html".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:425`). Decoradores: api.depends('fiscalyear_lock_date'), api.depends_context('uid', 'ignore_exceptions').
    async fn _compute_user_fiscalyear_lock_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._compute_user_fiscalyear_lock_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:432`). Decoradores: api.depends('tax_lock_date'), api.depends_context('uid', 'ignore_exceptions').
    async fn _compute_user_tax_lock_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._compute_user_tax_lock_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:439`). Decoradores: api.depends('sale_lock_date'), api.depends_context('uid', 'ignore_exceptions').
    async fn _compute_user_sale_lock_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._compute_user_sale_lock_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:446`). Decoradores: api.depends('purchase_lock_date'), api.depends_context('uid', 'ignore_exceptions').
    async fn _compute_user_purchase_lock_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._compute_user_purchase_lock_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:452`). Decoradores: api.depends('hard_lock_date').
    async fn _compute_user_hard_lock_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._compute_user_hard_lock_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:460`). Decoradores: api.depends('account_fiscal_country_id').
    async fn _compute_account_storno(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._compute_account_storno".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:465`). Decoradores: api.depends('account_fiscal_country_id').
    async fn _compute_display_account_storno(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._compute_display_account_storno".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:469`).
    async fn _initiate_account_onboardings(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._initiate_account_onboardings".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:478`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:492`).
    async fn get_new_account_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company.get_new_account_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:496`).
    async fn reflect_code_prefix_change(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company.reflect_code_prefix_change".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:507`).
    async fn _get_unreconciled_statement_lines_redirect_action(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._get_unreconciled_statement_lines_redirect_action".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:533`).
    async fn _get_unreconciled_statement_lines_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._get_unreconciled_statement_lines_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:541`).
    async fn _validate_locks(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._validate_locks".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:596`).
    async fn _get_user_lock_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._get_user_lock_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:631`).
    async fn _get_user_fiscal_lock_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._get_user_fiscal_lock_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:645`).
    async fn _get_violated_soft_lock_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._get_violated_soft_lock_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:664`).
    async fn _get_lock_date_violations(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._get_lock_date_violations".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:702`). Decoradores: api.model.
    async fn _format_lock_dates(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._format_lock_dates".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:712`).
    async fn _get_violated_lock_dates(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._get_violated_lock_dates".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:730`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:764`). Decoradores: api.model.
    async fn setting_init_bank_account_action(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company.setting_init_bank_account_action".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:779`). Decoradores: api.model.
    async fn setting_init_credit_card_account_action(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company.setting_init_credit_card_account_action".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:794`). Decoradores: api.model.
    async fn _get_default_opening_move_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._get_default_opening_move_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:818`).
    async fn opening_move_posted(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company.opening_move_posted".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:822`).
    async fn get_unaffected_earnings_account(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company.get_unaffected_earnings_account".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:854`).
    async fn _update_opening_move(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._update_opening_move".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:941`).
    async fn action_save_onboarding_sale_tax(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company.action_save_onboarding_sale_tax".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:945`).
    async fn action_save_onboarding_company_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company.action_save_onboarding_company_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:952`).
    async fn get_chart_of_accounts_or_fail(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company.get_chart_of_accounts_or_fail".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:962`).
    async fn install_l10n_modules(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company.install_l10n_modules".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:981`).
    async fn _existing_accounting(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._existing_accounting".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:986`).
    async fn _chart_template_selection(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._chart_template_selection".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:990`). Decoradores: api.model.
    async fn _action_check_hash_integrity(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._action_check_hash_integrity".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:993`).
    async fn _check_hash_integrity(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._check_hash_integrity".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:1087`). Decoradores: api.model.
    async fn _with_locked_records(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._with_locked_records".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:1103`).
    async fn compute_fiscalyear_dates(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company.compute_fiscalyear_dates".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:1114`). Decoradores: api.depends('country_id', 'account_fiscal_country_id').
    async fn _compute_company_vat_placeholder(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._compute_company_vat_placeholder".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:1127`). Decoradores: api.depends('country_id', 'account_fiscal_country_id').
    async fn _compute_company_registry_placeholder(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._compute_company_registry_placeholder".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:1135`).
    async fn _set_category_defaults(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._set_category_defaults".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/company.py:1140`).
    async fn _check_tax_return_configuration(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._check_tax_return_configuration".into(),
        ))
    }

}
