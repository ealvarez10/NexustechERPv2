//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_config_settings.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `res.config.settings` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct ResConfigSettingsExtFragment;

#[async_trait]
impl ModelFragment for ResConfigSettingsExtFragment {
    fn model_name(&self) -> &str {
        "res.config.settings"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::boolean("has_accounting_entries").computed("_compute_has_chart_of_accounts", &["company_id"]).stored());
        def.add_field({ let mut f = FieldDef::many2one("currency_id", "res.currency").string("Currency").required(); f.related = Some("company_id.currency_id".into()); f });
        def.add_field({ let mut f = FieldDef::many2one("currency_exchange_journal_id", "account.journal").string("Currency Exchange Journal"); f.related = Some("company_id.currency_exchange_journal_id".into()); f });
        def.add_field({ let mut f = FieldDef::many2one("income_currency_exchange_account_id", "account.account").string("Gain Exchange Rate Account"); f.related = Some("company_id.income_currency_exchange_account_id".into()); f });
        def.add_field({ let mut f = FieldDef::many2one("expense_currency_exchange_account_id", "account.account").string("Loss Exchange Rate Account"); f.related = Some("company_id.expense_currency_exchange_account_id".into()); f });
        def.add_field(FieldDef::boolean("has_chart_of_accounts").string("Company has a chart of accounts").computed("_compute_has_chart_of_accounts", &["company_id"]).stored());
        def.add_field(FieldDef::selection("chart_template", &[]));
        def.add_field({ let mut f = FieldDef::many2one("sale_tax_id", "account.tax").string("Default Sale Tax"); f.related = Some("company_id.account_sale_tax_id".into()); f });
        def.add_field({ let mut f = FieldDef::many2one("purchase_tax_id", "account.tax").string("Default Purchase Tax"); f.related = Some("company_id.account_purchase_tax_id".into()); f });
        def.add_field({ let mut f = FieldDef::selection("account_price_include", &[]).string("Default Sales Price Include").required(); f.related = Some("company_id.account_price_include".into()); f });
        def.add_field({ let mut f = FieldDef::selection("tax_calculation_rounding_method", &[]).string("Tax calculation rounding method"); f.related = Some("company_id.tax_calculation_rounding_method".into()); f });
        def.add_field({ let mut f = FieldDef::many2one("account_journal_suspense_account_id", "account.account").string("Bank Suspense"); f.related = Some("company_id.account_journal_suspense_account_id".into()); f });
        def.add_field({ let mut f = FieldDef::many2one("transfer_account_id", "account.account").string("Internal Transfer"); f.related = Some("company_id.transfer_account_id".into()); f });
        def.add_field(FieldDef::boolean("module_account_accountant").string("Accounting"));
        def.add_field(FieldDef::boolean("group_cash_rounding").string("Cash Rounding"));
        def.add_field(FieldDef::boolean("show_sale_receipts").string("Sale Receipt"));
        def.add_field(FieldDef::boolean("module_account_budget").string("Budget Management"));
        def.add_field(FieldDef::boolean("module_account_payment").string("Invoice Online Payment"));
        def.add_field(FieldDef::boolean("module_account_reports").string("Dynamic Reports"));
        def.add_field(FieldDef::boolean("module_account_check_printing").string("Allow check printing and deposits"));
        def.add_field(FieldDef::boolean("module_account_batch_payment").string("Use batch payments"));
        def.add_field(FieldDef::boolean("module_account_iso20022").string("SEPA Credit Transfer / ISO20022"));
        def.add_field(FieldDef::boolean("module_account_sepa_direct_debit").string("Use SEPA Direct Debit"));
        def.add_field(FieldDef::boolean("module_account_bank_statement_import_qif").string("Import .qif files"));
        def.add_field(FieldDef::boolean("module_currency_rate_live").string("Automatic Currency Rates"));
        def.add_field(FieldDef::boolean("module_account_intrastat").string("Intrastat"));
        def.add_field(FieldDef::boolean("module_product_margin").string("Allow Product Margin"));
        def.add_field(FieldDef::boolean("module_account_extract").string("Document Digitization"));
        def.add_field(FieldDef::boolean("module_account_invoice_extract").string("Invoice Digitization").computed("_compute_module_account_invoice_extract", &["module_account_extract"]).stored());
        def.add_field(FieldDef::boolean("module_account_bank_statement_extract").string("Bank Statement Digitization").computed("_compute_module_account_bank_statement_extract", &["module_account_extract"]).stored());
        def.add_field(FieldDef::boolean("module_snailmail_account").string("Snailmail"));
        def.add_field(FieldDef::boolean("module_account_peppol").string("PEPPOL Invoicing"));
        def.add_field({ let mut f = FieldDef::boolean("tax_exigibility").string("Cash Basis"); f.related = Some("company_id.tax_exigibility".into()); f });
        def.add_field({ let mut f = FieldDef::many2one("tax_cash_basis_journal_id", "account.journal").string("Tax Cash Basis Journal"); f.related = Some("company_id.tax_cash_basis_journal_id".into()); f });
        def.add_field({ let mut f = FieldDef::many2one("account_cash_basis_base_account_id", "account.account").string("Base Tax Received Account"); f.related = Some("company_id.account_cash_basis_base_account_id".into()); f });
        // TODO(odoo2rs): campo 'account_fiscal_country_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field({ let mut f = FieldDef::boolean("qr_code").string("Display SEPA QR-code"); f.related = Some("company_id.qr_code".into()); f });
        def.add_field({ let mut f = FieldDef::boolean("link_qr_code").string("Display Link QR-code"); f.related = Some("company_id.link_qr_code".into()); f });
        def.add_field({ let mut f = FieldDef::many2one("incoterm_id", "account.incoterms").string("Default incoterm"); f.related = Some("company_id.incoterm_id".into()); f });
        def.add_field({ let mut f = FieldDef::html("invoice_terms").string("Terms & Conditions"); f.related = Some("company_id.invoice_terms".into()); f });
        def.add_field({ let mut f = FieldDef::html("invoice_terms_html").string("Terms & Conditions as a Web page"); f.related = Some("company_id.invoice_terms_html".into()); f });
        def.add_field({ let mut f = FieldDef::selection("terms_type", &[]); f.related = Some("company_id.terms_type".into()); f });
        def.add_field({ let mut f = FieldDef::boolean("display_invoice_amount_total_words").string("Total amount of invoice in letters"); f.related = Some("company_id.display_invoice_amount_total_words".into()); f });
        def.add_field({ let mut f = FieldDef::boolean("display_invoice_tax_company_currency").string("Taxes in company currency"); f.related = Some("company_id.display_invoice_tax_company_currency".into()); f });
        def.add_field(FieldDef::boolean("preview_ready").string("Display preview button").computed("_compute_terms_preview", &["terms_type"]).stored());
        def.add_field(FieldDef::boolean("use_invoice_terms").string("Default Terms & Conditions"));
        def.add_field({ let mut f = FieldDef::boolean("account_use_credit_limit").string("Sales Credit Limit"); f.related = Some("company_id.account_use_credit_limit".into()); f });
        def.add_field(FieldDef::monetary("account_default_credit_limit").string("Default Credit Limit").computed("_compute_account_default_credit_limit", &["company_id"]).stored());
        def.add_field({ let mut f = FieldDef::char("country_code").readonly(); f.related = Some("company_id.account_fiscal_country_id.code".into()); f });
        def.add_field({ let mut f = FieldDef::boolean("account_storno").string("Storno accounting"); f.related = Some("company_id.account_storno".into()); f });
        def.add_field({ let mut f = FieldDef::boolean("display_account_storno"); f.related = Some("company_id.display_account_storno".into()); f });
        def.add_field(FieldDef::boolean("group_sale_delivery_address").string("Customer Addresses"));
        def.add_field({ let mut f = FieldDef::selection("quick_edit_mode", &[]).string("Quick encoding"); f.related = Some("company_id.quick_edit_mode".into()); f });
        def.add_field({ let mut f = FieldDef::many2one("account_journal_early_pay_discount_loss_account_id", "account.account").string("Early Discount Loss"); f.related = Some("company_id.account_journal_early_pay_discount_loss_account_id".into()); f });
        def.add_field({ let mut f = FieldDef::many2one("account_journal_early_pay_discount_gain_account_id", "account.account").string("Early Discount Gain"); f.related = Some("company_id.account_journal_early_pay_discount_gain_account_id".into()); f });
        def.add_field({ let mut f = FieldDef::many2one("account_discount_income_allocation_id", "account.account").string("Vendor Bills Discounts Account"); f.related = Some("company_id.account_discount_income_allocation_id".into()); f });
        def.add_field({ let mut f = FieldDef::many2one("account_discount_expense_allocation_id", "account.account").string("Customer Invoices Discounts Account"); f.related = Some("company_id.account_discount_expense_allocation_id".into()); f });
        def.add_field(FieldDef::boolean("is_account_peppol_eligible").string("PEPPOL eligible").computed("_compute_is_account_peppol_eligible", &["country_code"]).stored());
        def.add_field({ let mut f = FieldDef::boolean("restrictive_audit_trail").string("Restricted Audit Trail"); f.related = Some("company_id.restrictive_audit_trail".into()); f });
        def.add_field({ let mut f = FieldDef::boolean("force_restrictive_audit_trail").string("Forced Audit Trail"); f.related = Some("company_id.force_restrictive_audit_trail".into()); f });
        def.add_field({ let mut f = FieldDef::boolean("autopost_bills"); f.related = Some("company_id.autopost_bills".into()); f });
        // TODO(odoo2rs): campo 'income_account_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        // TODO(odoo2rs): campo 'expense_account_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_is_account_peppol_eligible", "set_values", "reload_template", "_compute_account_default_credit_limit", "_inverse_account_default_credit_limit", "_compute_has_chart_of_accounts", "_compute_module_account_invoice_extract", "_compute_module_account_bank_statement_extract", "onchange_analytic_accounting", "onchange_module_account_budget", "_onchange_tax_exigibility", "_compute_terms_preview", "action_update_terms", "action_eu_oss_tax_mapping"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_is_account_peppol_eligible" => self._compute_is_account_peppol_eligible(env, ctx, rs, args).await,
            "set_values" => self.set_values(env, ctx, rs, args).await,
            "reload_template" => self.reload_template(env, ctx, rs, args).await,
            "_compute_account_default_credit_limit" => self._compute_account_default_credit_limit(env, ctx, rs, args).await,
            "_inverse_account_default_credit_limit" => self._inverse_account_default_credit_limit(env, ctx, rs, args).await,
            "_compute_has_chart_of_accounts" => self._compute_has_chart_of_accounts(env, ctx, rs, args).await,
            "_compute_module_account_invoice_extract" => self._compute_module_account_invoice_extract(env, ctx, rs, args).await,
            "_compute_module_account_bank_statement_extract" => self._compute_module_account_bank_statement_extract(env, ctx, rs, args).await,
            "onchange_analytic_accounting" => self.onchange_analytic_accounting(env, ctx, rs, args).await,
            "onchange_module_account_budget" => self.onchange_module_account_budget(env, ctx, rs, args).await,
            "_onchange_tax_exigibility" => self._onchange_tax_exigibility(env, ctx, rs, args).await,
            "_compute_terms_preview" => self._compute_terms_preview(env, ctx, rs, args).await,
            "action_update_terms" => self.action_update_terms(env, ctx, rs, args).await,
            "action_eu_oss_tax_mapping" => self.action_eu_oss_tax_mapping(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl ResConfigSettingsExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_config_settings.py:212`). Decoradores: api.depends('country_code').
    async fn _compute_is_account_peppol_eligible(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.config.settings._compute_is_account_peppol_eligible".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_config_settings.py:218`).
    async fn set_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.config.settings.set_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_config_settings.py:226`).
    async fn reload_template(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.config.settings.reload_template".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_config_settings.py:230`). Decoradores: api.depends('company_id').
    async fn _compute_account_default_credit_limit(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.config.settings._compute_account_default_credit_limit".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_config_settings.py:235`).
    async fn _inverse_account_default_credit_limit(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.config.settings._inverse_account_default_credit_limit".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_config_settings.py:245`). Decoradores: api.depends('company_id').
    async fn _compute_has_chart_of_accounts(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.config.settings._compute_has_chart_of_accounts".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_config_settings.py:250`). Decoradores: api.depends('module_account_extract').
    async fn _compute_module_account_invoice_extract(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.config.settings._compute_module_account_invoice_extract".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_config_settings.py:255`). Decoradores: api.depends('module_account_extract').
    async fn _compute_module_account_bank_statement_extract(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.config.settings._compute_module_account_bank_statement_extract".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_config_settings.py:260`). Decoradores: api.onchange('group_analytic_accounting').
    async fn onchange_analytic_accounting(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.config.settings.onchange_analytic_accounting".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_config_settings.py:265`). Decoradores: api.onchange('module_account_budget').
    async fn onchange_module_account_budget(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.config.settings.onchange_module_account_budget".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_config_settings.py:270`). Decoradores: api.onchange('tax_exigibility').
    async fn _onchange_tax_exigibility(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.config.settings._onchange_tax_exigibility".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_config_settings.py:286`). Decoradores: api.depends('terms_type').
    async fn _compute_terms_preview(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.config.settings._compute_terms_preview".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_config_settings.py:292`).
    async fn action_update_terms(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.config.settings.action_update_terms".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_config_settings.py:306`).
    async fn action_eu_oss_tax_mapping(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.config.settings.action_eu_oss_tax_mapping".into(),
        ))
    }

}
