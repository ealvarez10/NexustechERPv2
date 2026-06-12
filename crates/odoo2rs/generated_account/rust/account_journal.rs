//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.journal`

use nexus_orm::prelude::*;

pub struct AccountJournalFragment;

#[async_trait]
impl ModelFragment for AccountJournalFragment {
    fn model_name(&self) -> &str {
        "account.journal"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Journal".into();
        def.order = "sequence, type, code".into();
        def.add_field(FieldDef::char("name").string("Journal Name").required());
        def.add_field(FieldDef::char("name_placeholder").computed("_compute_name_placeholder", &["type"]).stored());
        def.add_field(FieldDef::char("code").string("Sequence Prefix").required().computed("_compute_code", &["type", "company_id"]).stored());
        def.add_field(FieldDef::boolean("active").default_val(true));
        def.add_field(FieldDef::selection("type", &[("sale", "Sales"), ("purchase", "Purchase"), ("cash", "Cash"), ("bank", "Bank"), ("credit", "Credit Card"), ("general", "Miscellaneous")]).required());
        def.add_field(FieldDef::boolean("is_self_billing").string("Self Billing"));
        def.add_field(FieldDef::char("default_account_type").string("Default Account Type").computed("_compute_default_account_type", &["type"]).stored());
        def.add_field(FieldDef::many2one("default_account_id", "account.account").string("Default Account"));
        def.add_field(FieldDef::many2one("suspense_account_id", "account.account").string("Suspense Account").computed("_compute_suspense_account_id", &["company_id", "type"]).stored());
        def.add_field(FieldDef::many2one("non_deductible_account_id", "account.account").string("Private Share Account"));
        def.add_field(FieldDef::boolean("restrict_mode_hash_table").string("Secure Posted Entries with Hash"));
        def.add_field(FieldDef::integer("sequence").default_val(10i64));
        def.add_field(FieldDef::selection("invoice_reference_type", &[("partner", "Based on Customer"), ("invoice", "Based on Invoice")]).string("Communication Type").required().default_val("invoice"));
        def.add_field(FieldDef::selection("invoice_reference_model", &[("odoo", "Full Reference (INV/2024/00001)"), ("euro", "European (RF83INV202400001)"), ("number", "Numbers only (202400001)")]).string("Communication Standard").required());
        def.add_field(FieldDef::many2one("currency_id", "res.currency").string("Currency"));
        def.add_field(FieldDef::many2one("company_id", "res.company").string("Company").required().readonly());
        def.add_field({ let mut f = FieldDef::char("country_code").readonly(); f.related = Some("company_id.account_fiscal_country_id.code".into()); f });
        def.add_field({ let mut f = FieldDef::json("account_fiscal_country_group_codes"); f.related = Some("company_id.account_fiscal_country_group_codes".into()); f });
        def.add_field(FieldDef::boolean("refund_sequence").string("Dedicated Credit Note Sequence").computed("_compute_refund_sequence", &["type"]).stored());
        def.add_field(FieldDef::boolean("payment_sequence").string("Dedicated Payment Sequence").computed("_compute_payment_sequence", &["type"]).stored());
        def.add_field(FieldDef::many2one("invoice_template_pdf_report_id", "ir.actions.report").string("Invoice report"));
        // TODO(odoo2rs): campo 'available_invoice_template_pdf_report_ids' (one2many) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field({ let mut f = FieldDef::boolean("display_invoice_template_pdf_report_id"); f.store = false; f });
        def.add_field(FieldDef::text("sequence_override_regex"));
        def.add_field(FieldDef::one2many("inbound_payment_method_line_ids", "account.payment.method.line", "journal_id").string("Inbound Payment Methods").computed("_compute_inbound_payment_method_line_ids", &["type", "currency_id"]).stored());
        def.add_field(FieldDef::one2many("outbound_payment_method_line_ids", "account.payment.method.line", "journal_id").string("Outbound Payment Methods").computed("_compute_outbound_payment_method_line_ids", &["type", "currency_id"]).stored());
        def.add_field(FieldDef::many2one("profit_account_id", "account.account").string("Profit Account"));
        def.add_field(FieldDef::many2one("loss_account_id", "account.account").string("Loss Account"));
        def.add_field({ let mut f = FieldDef::many2one("company_partner_id", "res.partner").string("Account Holder").readonly(); f.store = false; f });
        def.add_field(FieldDef::many2one("bank_account_id", "res.partner.bank").string("Bank Account"));
        def.add_field(FieldDef::selection("bank_statements_source", &[]).string("Bank Feeds").default_val("undefined"));
        def.add_field({ let mut f = FieldDef::char("bank_acc_number"); f.related = Some("bank_account_id.acc_number".into()); f });
        def.add_field({ let mut f = FieldDef::many2one("bank_id", "res.bank"); f.related = Some("bank_account_id.bank_id".into()); f });
        def.add_field(FieldDef::char("alias_name"));
        def.add_field(FieldDef::many2many("journal_group_ids", "account.journal.group").string("Ledger Group"));
        def.add_field(FieldDef::many2many("available_payment_method_ids", "account.payment.method").computed("_compute_available_payment_method_ids", &["outbound_payment_method_line_ids", "inbound_payment_method_line_ids"]).stored());
        def.add_field(FieldDef::char("selected_payment_method_codes").computed("_compute_selected_payment_method_codes", &["outbound_payment_method_line_ids", "inbound_payment_method_line_ids"]).stored());
        def.add_field(FieldDef::date("accounting_date").computed("_compute_accounting_date", &["company_id"]).stored());
        def.add_field(FieldDef::boolean("display_alias_fields").computed("_compute_display_alias_fields", &[]).stored());
        def.add_field(FieldDef::boolean("has_invalid_statements").computed("_compute_has_invalid_statements", &[]).stored());
        def.add_field(FieldDef::boolean("show_fetch_in_einvoices_button").string("Show E-Invoice Buttons").computed("_compute_show_fetch_in_einvoices_button", &["type"]).stored());
        def.add_field(FieldDef::boolean("show_refresh_out_einvoices_status_button").string("Show E-Invoice Status Buttons").computed("_compute_show_refresh_out_einvoices_status_button", &["type"]).stored());
        def.add_field(FieldDef::char("incoming_einvoice_notification_email").string("Send Copy To"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_default_display_invoice_template_pdf_report_id", "_default_inbound_payment_methods", "_default_outbound_payment_methods", "__get_bank_statements_available_sources", "_get_bank_statements_available_sources", "_default_invoice_reference_model", "_get_default_account_domain", "_compute_has_invalid_statements", "_compute_display_alias_fields", "_compute_code", "_get_journals_payment_method_information", "_compute_available_payment_method_ids", "_compute_default_account_type", "_compute_inbound_payment_method_line_ids", "_compute_outbound_payment_method_line_ids", "_compute_selected_payment_method_codes", "_compute_suspense_account_id", "_compute_accounting_date", "_compute_show_fetch_in_einvoices_button", "_compute_show_refresh_out_einvoices_status_button", "_onchange_type", "_compute_name_placeholder", "_check_bank_account", "_check_company_consistency", "_check_type_default_account_id_type", "_check_payment_method_line_ids_multiplicity", "_check_auto_post_draft_entries", "_check_incoming_einvoice_notification_email", "_onchange_incoming_einvoice_notification_email", "_compute_refund_sequence", "_compute_payment_sequence", "_compute_available_invoice_template_pdf_report_ids", "unlink", "copy_data", "write", "_alias_get_creation_values", "_alias_prepare_alias_name", "_ensure_unique_alias", "_get_next_journal_default_code", "_prepare_liquidity_account_vals", "_prepare_credit_account_vals", "_create_default_account", "_fill_missing_values", "create", "set_bank_account", "_compute_display_name", "action_configure_bank_journal", "_create_document_from_attachment", "create_document_from_attachment", "_get_journal_bank_account_balance", "_get_journal_inbound_outstanding_payment_accounts", "_get_journal_outbound_outstanding_payment_accounts", "_get_available_payment_method_lines", "_is_payment_method_available", "_process_reference_for_sale_order", "_get_journal_notification_unsubscribe_scope", "_unsubscribe_invoice_notification_email", "_notify_einvoices_received", "button_unsubscribe_from_invoice_notifications", "_notify_invoice_subscribers", "button_fetch_in_einvoices", "button_refresh_out_einvoices_status"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_default_display_invoice_template_pdf_report_id" => self._default_display_invoice_template_pdf_report_id(env, ctx, rs, args).await,
            "_default_inbound_payment_methods" => self._default_inbound_payment_methods(env, ctx, rs, args).await,
            "_default_outbound_payment_methods" => self._default_outbound_payment_methods(env, ctx, rs, args).await,
            "__get_bank_statements_available_sources" => self.__get_bank_statements_available_sources(env, ctx, rs, args).await,
            "_get_bank_statements_available_sources" => self._get_bank_statements_available_sources(env, ctx, rs, args).await,
            "_default_invoice_reference_model" => self._default_invoice_reference_model(env, ctx, rs, args).await,
            "_get_default_account_domain" => self._get_default_account_domain(env, ctx, rs, args).await,
            "_compute_has_invalid_statements" => self._compute_has_invalid_statements(env, ctx, rs, args).await,
            "_compute_display_alias_fields" => self._compute_display_alias_fields(env, ctx, rs, args).await,
            "_compute_code" => self._compute_code(env, ctx, rs, args).await,
            "_get_journals_payment_method_information" => self._get_journals_payment_method_information(env, ctx, rs, args).await,
            "_compute_available_payment_method_ids" => self._compute_available_payment_method_ids(env, ctx, rs, args).await,
            "_compute_default_account_type" => self._compute_default_account_type(env, ctx, rs, args).await,
            "_compute_inbound_payment_method_line_ids" => self._compute_inbound_payment_method_line_ids(env, ctx, rs, args).await,
            "_compute_outbound_payment_method_line_ids" => self._compute_outbound_payment_method_line_ids(env, ctx, rs, args).await,
            "_compute_selected_payment_method_codes" => self._compute_selected_payment_method_codes(env, ctx, rs, args).await,
            "_compute_suspense_account_id" => self._compute_suspense_account_id(env, ctx, rs, args).await,
            "_compute_accounting_date" => self._compute_accounting_date(env, ctx, rs, args).await,
            "_compute_show_fetch_in_einvoices_button" => self._compute_show_fetch_in_einvoices_button(env, ctx, rs, args).await,
            "_compute_show_refresh_out_einvoices_status_button" => self._compute_show_refresh_out_einvoices_status_button(env, ctx, rs, args).await,
            "_onchange_type" => self._onchange_type(env, ctx, rs, args).await,
            "_compute_name_placeholder" => self._compute_name_placeholder(env, ctx, rs, args).await,
            "_check_bank_account" => self._check_bank_account(env, ctx, rs, args).await,
            "_check_company_consistency" => self._check_company_consistency(env, ctx, rs, args).await,
            "_check_type_default_account_id_type" => self._check_type_default_account_id_type(env, ctx, rs, args).await,
            "_check_payment_method_line_ids_multiplicity" => self._check_payment_method_line_ids_multiplicity(env, ctx, rs, args).await,
            "_check_auto_post_draft_entries" => self._check_auto_post_draft_entries(env, ctx, rs, args).await,
            "_check_incoming_einvoice_notification_email" => self._check_incoming_einvoice_notification_email(env, ctx, rs, args).await,
            "_onchange_incoming_einvoice_notification_email" => self._onchange_incoming_einvoice_notification_email(env, ctx, rs, args).await,
            "_compute_refund_sequence" => self._compute_refund_sequence(env, ctx, rs, args).await,
            "_compute_payment_sequence" => self._compute_payment_sequence(env, ctx, rs, args).await,
            "_compute_available_invoice_template_pdf_report_ids" => self._compute_available_invoice_template_pdf_report_ids(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "copy_data" => self.copy_data(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "_alias_get_creation_values" => self._alias_get_creation_values(env, ctx, rs, args).await,
            "_alias_prepare_alias_name" => self._alias_prepare_alias_name(env, ctx, rs, args).await,
            "_ensure_unique_alias" => self._ensure_unique_alias(env, ctx, rs, args).await,
            "_get_next_journal_default_code" => self._get_next_journal_default_code(env, ctx, rs, args).await,
            "_prepare_liquidity_account_vals" => self._prepare_liquidity_account_vals(env, ctx, rs, args).await,
            "_prepare_credit_account_vals" => self._prepare_credit_account_vals(env, ctx, rs, args).await,
            "_create_default_account" => self._create_default_account(env, ctx, rs, args).await,
            "_fill_missing_values" => self._fill_missing_values(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "set_bank_account" => self.set_bank_account(env, ctx, rs, args).await,
            "_compute_display_name" => self._compute_display_name(env, ctx, rs, args).await,
            "action_configure_bank_journal" => self.action_configure_bank_journal(env, ctx, rs, args).await,
            "_create_document_from_attachment" => self._create_document_from_attachment(env, ctx, rs, args).await,
            "create_document_from_attachment" => self.create_document_from_attachment(env, ctx, rs, args).await,
            "_get_journal_bank_account_balance" => self._get_journal_bank_account_balance(env, ctx, rs, args).await,
            "_get_journal_inbound_outstanding_payment_accounts" => self._get_journal_inbound_outstanding_payment_accounts(env, ctx, rs, args).await,
            "_get_journal_outbound_outstanding_payment_accounts" => self._get_journal_outbound_outstanding_payment_accounts(env, ctx, rs, args).await,
            "_get_available_payment_method_lines" => self._get_available_payment_method_lines(env, ctx, rs, args).await,
            "_is_payment_method_available" => self._is_payment_method_available(env, ctx, rs, args).await,
            "_process_reference_for_sale_order" => self._process_reference_for_sale_order(env, ctx, rs, args).await,
            "_get_journal_notification_unsubscribe_scope" => self._get_journal_notification_unsubscribe_scope(env, ctx, rs, args).await,
            "_unsubscribe_invoice_notification_email" => self._unsubscribe_invoice_notification_email(env, ctx, rs, args).await,
            "_notify_einvoices_received" => self._notify_einvoices_received(env, ctx, rs, args).await,
            "button_unsubscribe_from_invoice_notifications" => self.button_unsubscribe_from_invoice_notifications(env, ctx, rs, args).await,
            "_notify_invoice_subscribers" => self._notify_invoice_subscribers(env, ctx, rs, args).await,
            "button_fetch_in_einvoices" => self.button_fetch_in_einvoices(env, ctx, rs, args).await,
            "button_refresh_out_einvoices_status" => self.button_refresh_out_einvoices_status(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountJournalFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:55`).
    async fn _default_display_invoice_template_pdf_report_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._default_display_invoice_template_pdf_report_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:59`).
    async fn _default_inbound_payment_methods(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._default_inbound_payment_methods".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:62`).
    async fn _default_outbound_payment_methods(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._default_outbound_payment_methods".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:65`).
    async fn __get_bank_statements_available_sources(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal.__get_bank_statements_available_sources".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:68`).
    async fn _get_bank_statements_available_sources(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._get_bank_statements_available_sources".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:71`).
    async fn _default_invoice_reference_model(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._default_invoice_reference_model".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:81`).
    async fn _get_default_account_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._get_default_account_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:298`).
    async fn _compute_has_invalid_statements(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._compute_has_invalid_statements".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:308`).
    async fn _compute_display_alias_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._compute_display_alias_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:312`). Decoradores: api.depends('type', 'company_id').
    async fn _compute_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._compute_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:323`).
    async fn _get_journals_payment_method_information(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._get_journals_payment_method_information".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:392`). Decoradores: api.depends('outbound_payment_method_line_ids', 'inbound_payment_method_line_ids').
    async fn _compute_available_payment_method_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._compute_available_payment_method_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:450`). Decoradores: api.depends('type').
    async fn _compute_default_account_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._compute_default_account_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:463`). Decoradores: api.depends('type', 'currency_id').
    async fn _compute_inbound_payment_method_line_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._compute_inbound_payment_method_line_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:485`). Decoradores: api.depends('type', 'currency_id').
    async fn _compute_outbound_payment_method_line_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._compute_outbound_payment_method_line_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:507`). Decoradores: api.depends('outbound_payment_method_line_ids', 'inbound_payment_method_line_ids').
    async fn _compute_selected_payment_method_codes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._compute_selected_payment_method_codes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:517`). Decoradores: api.depends('company_id', 'type').
    async fn _compute_suspense_account_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._compute_suspense_account_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:530`). Decoradores: api.depends('company_id'), api.depends_context('move_date', 'has_tax').
    async fn _compute_accounting_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._compute_accounting_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:538`). Decoradores: api.depends('type').
    async fn _compute_show_fetch_in_einvoices_button(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._compute_show_fetch_in_einvoices_button".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:543`). Decoradores: api.depends('type').
    async fn _compute_show_refresh_out_einvoices_status_button(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._compute_show_refresh_out_einvoices_status_button".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:548`). Decoradores: api.onchange('type').
    async fn _onchange_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._onchange_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:577`). Decoradores: api.depends('type').
    async fn _compute_name_placeholder(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._compute_name_placeholder".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:595`). Decoradores: api.constrains('type', 'bank_account_id').
    async fn _check_bank_account(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._check_bank_account".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:606`). Decoradores: api.constrains('company_id').
    async fn _check_company_consistency(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._check_company_consistency".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:615`). Decoradores: api.constrains('type', 'default_account_id').
    async fn _check_type_default_account_id_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._check_type_default_account_id_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:621`). Decoradores: api.constrains('inbound_payment_method_line_ids', 'outbound_payment_method_line_ids').
    async fn _check_payment_method_line_ids_multiplicity(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._check_payment_method_line_ids_multiplicity".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:686`). Decoradores: api.constrains('active').
    async fn _check_auto_post_draft_entries(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._check_auto_post_draft_entries".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:702`). Decoradores: api.constrains('type', 'incoming_einvoice_notification_email').
    async fn _check_incoming_einvoice_notification_email(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._check_incoming_einvoice_notification_email".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:707`). Decoradores: api.onchange('incoming_einvoice_notification_email').
    async fn _onchange_incoming_einvoice_notification_email(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._onchange_incoming_einvoice_notification_email".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:712`). Decoradores: api.depends('type').
    async fn _compute_refund_sequence(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._compute_refund_sequence".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:717`). Decoradores: api.depends('type').
    async fn _compute_payment_sequence(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._compute_payment_sequence".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:721`).
    async fn _compute_available_invoice_template_pdf_report_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._compute_available_invoice_template_pdf_report_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:725`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:736`).
    async fn copy_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal.copy_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:768`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:829`).
    async fn _alias_get_creation_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._alias_get_creation_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:844`). Decoradores: api.model.
    async fn _alias_prepare_alias_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._alias_prepare_alias_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:864`). Decoradores: api.model.
    async fn _ensure_unique_alias(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._ensure_unique_alias".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:884`). Decoradores: api.model.
    async fn _get_next_journal_default_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._get_next_journal_default_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:906`). Decoradores: api.model.
    async fn _prepare_liquidity_account_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._prepare_liquidity_account_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:916`). Decoradores: api.model.
    async fn _prepare_credit_account_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._prepare_credit_account_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:926`). Decoradores: api.model.
    async fn _create_default_account(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._create_default_account".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:963`). Decoradores: api.model.
    async fn _fill_missing_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._fill_missing_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:1023`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:1041`).
    async fn set_bank_account(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal.set_bank_account".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:1056`). Decoradores: api.depends('currency_id').
    async fn _compute_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._compute_display_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:1063`).
    async fn action_configure_bank_journal(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal.action_configure_bank_journal".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:1070`).
    async fn _create_document_from_attachment(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._create_document_from_attachment".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:1107`).
    async fn create_document_from_attachment(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal.create_document_from_attachment".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:1137`).
    async fn _get_journal_bank_account_balance(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._get_journal_bank_account_balance".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:1161`).
    async fn _get_journal_inbound_outstanding_payment_accounts(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._get_journal_inbound_outstanding_payment_accounts".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:1171`).
    async fn _get_journal_outbound_outstanding_payment_accounts(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._get_journal_outbound_outstanding_payment_accounts".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:1181`).
    async fn _get_available_payment_method_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._get_available_payment_method_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:1198`).
    async fn _is_payment_method_available(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._is_payment_method_available".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:1208`).
    async fn _process_reference_for_sale_order(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._process_reference_for_sale_order".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:1220`).
    async fn _get_journal_notification_unsubscribe_scope(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._get_journal_notification_unsubscribe_scope".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:1223`).
    async fn _unsubscribe_invoice_notification_email(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._unsubscribe_invoice_notification_email".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:1233`).
    async fn _notify_einvoices_received(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._notify_einvoices_received".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:1250`).
    async fn button_unsubscribe_from_invoice_notifications(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal.button_unsubscribe_from_invoice_notifications".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:1255`).
    async fn _notify_invoice_subscribers(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal._notify_invoice_subscribers".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:1286`).
    async fn button_fetch_in_einvoices(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal.button_fetch_in_einvoices".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py:1294`).
    async fn button_refresh_out_einvoices_status(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.journal.button_refresh_out_einvoices_status".into(),
        ))
    }

}
