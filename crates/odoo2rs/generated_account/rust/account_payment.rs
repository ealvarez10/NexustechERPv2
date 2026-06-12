//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.payment`

use nexus_orm::prelude::*;

pub struct AccountPaymentFragment;

#[async_trait]
impl ModelFragment for AccountPaymentFragment {
    fn model_name(&self) -> &str {
        "account.payment"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Payments".into();
        def.order = "date desc, name desc".into();
        def.add_field(FieldDef::char("name").string("Number").computed("_compute_name", &["move_id.name", "state"]).stored());
        def.add_field(FieldDef::date("date").required());
        def.add_field(FieldDef::many2one("move_id", "account.move").string("Journal Entry"));
        def.add_field(FieldDef::many2one("journal_id", "account.journal").required().computed("_compute_journal_id", &["company_id", "partner_id"]).stored());
        def.add_field(FieldDef::many2one("company_id", "res.company").required().computed("_compute_company_id", &["journal_id"]).stored());
        def.add_field(FieldDef::selection("state", &[("draft", "Draft"), ("in_process", "In Process"), ("paid", "Paid"), ("canceled", "Canceled"), ("rejected", "Rejected")]).required().computed("_compute_state", &["reconciled_invoice_ids.payment_state", "move_id.line_ids.amount_residual"]).stored().default_val("draft"));
        def.add_field(FieldDef::boolean("is_reconciled").string("Is Reconciled").computed("_compute_reconciliation_status", &["move_id.line_ids.amount_residual", "move_id.line_ids.amount_residual_currency", "move_id.line_ids.account_id", "state"]).stored());
        def.add_field(FieldDef::boolean("is_matched").string("Is Matched With a Bank Statement").computed("_compute_reconciliation_status", &["move_id.line_ids.amount_residual", "move_id.line_ids.amount_residual_currency", "move_id.line_ids.account_id", "state"]).stored());
        def.add_field(FieldDef::boolean("is_sent").string("Is Sent").readonly());
        def.add_field(FieldDef::many2many("available_partner_bank_ids", "res.partner.bank").computed("_compute_available_partner_bank_ids", &["partner_id", "company_id", "payment_type"]).stored());
        def.add_field(FieldDef::many2one("partner_bank_id", "res.partner.bank").string("Recipient Bank Account").computed("_compute_partner_bank_id", &["available_partner_bank_ids", "journal_id"]).stored());
        def.add_field(FieldDef::html("qr_code").string("QR Code URL").computed("_compute_qr_code", &["partner_bank_id", "amount", "memo", "currency_id", "journal_id", "move_id.state", "payment_method_line_id", "payment_type"]).stored());
        def.add_field(FieldDef::many2one("paired_internal_transfer_payment_id", "account.payment"));
        def.add_field(FieldDef::many2one("payment_method_line_id", "account.payment.method.line").string("Payment Method").computed("_compute_payment_method_line_id", &["available_payment_method_line_ids"]).stored());
        def.add_field(FieldDef::many2many("available_payment_method_line_ids", "account.payment.method.line").computed("_compute_payment_method_line_fields", &["payment_type", "journal_id", "currency_id"]).stored());
        // TODO(odoo2rs): campo 'payment_method_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::many2many("available_journal_ids", "account.journal").computed("_compute_available_journal_ids", &["payment_type"]).stored());
        def.add_field(FieldDef::monetary("amount"));
        def.add_field(FieldDef::selection("payment_type", &[("outbound", "Send"), ("inbound", "Receive")]).string("Payment Type").required().default_val("inbound"));
        def.add_field(FieldDef::selection("partner_type", &[("customer", "Customer"), ("supplier", "Vendor")]).required().default_val("customer"));
        def.add_field(FieldDef::char("memo").string("Memo"));
        def.add_field(FieldDef::char("payment_reference").string("Payment Reference"));
        def.add_field(FieldDef::many2one("currency_id", "res.currency").string("Currency").computed("_compute_currency_id", &["journal_id"]).stored());
        // TODO(odoo2rs): campo 'company_currency_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::many2one("partner_id", "res.partner").string("Customer/Vendor"));
        def.add_field(FieldDef::many2one("outstanding_account_id", "account.account").string("Outstanding Account").computed("_compute_outstanding_account_id", &["payment_method_line_id"]).stored());
        def.add_field(FieldDef::many2one("destination_account_id", "account.account").string("Destination Account").computed("_compute_destination_account_id", &["journal_id", "partner_id", "partner_type"]).stored());
        def.add_field(FieldDef::many2many("invoice_ids", "account.move").string("Invoices"));
        def.add_field(FieldDef::many2many("reconciled_invoice_ids", "account.move").string("Reconciled Invoices").computed("_compute_stat_buttons_from_reconciliation", &["move_id.line_ids.matched_debit_ids", "move_id.line_ids.matched_credit_ids"]).stored());
        def.add_field(FieldDef::integer("reconciled_invoices_count").string("# Reconciled Invoices").computed("_compute_stat_buttons_from_reconciliation", &["move_id.line_ids.matched_debit_ids", "move_id.line_ids.matched_credit_ids"]).stored());
        def.add_field(FieldDef::selection("reconciled_invoices_type", &[("credit_note", "Credit Note"), ("invoice", "Invoice")]).computed("_compute_stat_buttons_from_reconciliation", &["move_id.line_ids.matched_debit_ids", "move_id.line_ids.matched_credit_ids"]).stored());
        def.add_field(FieldDef::many2many("reconciled_bill_ids", "account.move").string("Reconciled Bills").computed("_compute_stat_buttons_from_reconciliation", &["move_id.line_ids.matched_debit_ids", "move_id.line_ids.matched_credit_ids"]).stored());
        def.add_field(FieldDef::integer("reconciled_bills_count").string("# Reconciled Bills").computed("_compute_stat_buttons_from_reconciliation", &["move_id.line_ids.matched_debit_ids", "move_id.line_ids.matched_credit_ids"]).stored());
        def.add_field(FieldDef::many2many("reconciled_statement_line_ids", "account.bank.statement.line").string("Reconciled Statement Lines").computed("_compute_stat_buttons_from_reconciliation", &["move_id.line_ids.matched_debit_ids", "move_id.line_ids.matched_credit_ids"]).stored());
        def.add_field(FieldDef::integer("reconciled_statement_lines_count").string("# Reconciled Statement Lines").computed("_compute_stat_buttons_from_reconciliation", &["move_id.line_ids.matched_debit_ids", "move_id.line_ids.matched_credit_ids"]).stored());
        def.add_field({ let mut f = FieldDef::char("payment_method_code"); f.related = Some("payment_method_line_id.code".into()); f });
        def.add_field(FieldDef::char("payment_receipt_title").computed("_compute_payment_receipt_title", &[]).stored());
        def.add_field({ let mut f = FieldDef::boolean("need_cancel_request"); f.related = Some("move_id.need_cancel_request".into()); f });
        def.add_field(FieldDef::boolean("show_partner_bank_account").computed("_compute_show_require_partner_bank", &["payment_method_code"]).stored());
        def.add_field(FieldDef::boolean("require_partner_bank_account").computed("_compute_show_require_partner_bank", &["payment_method_code"]).stored());
        def.add_field({ let mut f = FieldDef::char("country_code"); f.related = Some("company_id.account_fiscal_country_id.code".into()); f });
        def.add_field(FieldDef::monetary("amount_signed").computed("_compute_amount_signed", &["amount", "payment_type"]).stored());
        def.add_field(FieldDef::monetary("amount_company_currency_signed").computed("_compute_amount_company_currency_signed", &["move_id.amount_total_signed", "amount", "payment_type", "currency_id", "date", "company_id", "company_currency_id"]).stored());
        def.add_field(FieldDef::many2many("duplicate_payment_ids", "account.payment").computed("_compute_duplicate_payment_ids", &["partner_id", "amount", "date", "payment_type"]).stored());
        def.add_field(FieldDef::one2many("attachment_ids", "ir.attachment", "res_id").string("Attachments"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_get_valid_payment_account_types", "_seek_for_lines", "_get_valid_liquidity_accounts", "_valid_payment_states", "_get_aml_default_display_name_list", "_prepare_move_withholding_lines", "_prepare_move_liquidity_lines", "_prepare_move_counterpart_lines", "_prepare_move_lines_per_type", "_prepare_move_line_default_vals", "_compute_name", "_compute_journal_id", "_compute_company_id", "_compute_state", "_compute_reconciliation_status", "_get_method_codes_using_bank_account", "_get_method_codes_needing_bank_account", "action_open_business_doc", "_compute_show_require_partner_bank", "_compute_amount_company_currency_signed", "_compute_amount_signed", "_compute_available_partner_bank_ids", "_compute_partner_bank_id", "_compute_payment_method_line_id", "_compute_payment_method_line_fields", "_compute_available_journal_ids", "_get_payment_method_codes_to_exclude", "_compute_currency_id", "_compute_outstanding_account_id", "_compute_destination_account_id", "_compute_qr_code", "_compute_stat_buttons_from_reconciliation", "_compute_payment_receipt_title", "_compute_duplicate_payment_ids", "_search_reconciled_invoice_ids", "_fetch_duplicate_reference", "_inverse_memo", "_check_payment_method_line_id", "_check_move_id", "create", "_get_outstanding_account", "write", "unlink", "_compute_display_name", "copy_data", "_message_mail_after_hook", "_synchronize_to_moves", "_get_trigger_fields_to_synchronize", "_generate_journal_entry", "_generate_move_vals", "_get_payment_receipt_report_values", "mark_as_sent", "unmark_as_sent", "action_post", "action_validate", "action_reject", "action_cancel", "button_request_cancel", "action_draft", "button_open_invoices", "button_open_bills", "button_open_statement_lines", "button_open_journal_entry"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_get_valid_payment_account_types" => self._get_valid_payment_account_types(env, ctx, rs, args).await,
            "_seek_for_lines" => self._seek_for_lines(env, ctx, rs, args).await,
            "_get_valid_liquidity_accounts" => self._get_valid_liquidity_accounts(env, ctx, rs, args).await,
            "_valid_payment_states" => self._valid_payment_states(env, ctx, rs, args).await,
            "_get_aml_default_display_name_list" => self._get_aml_default_display_name_list(env, ctx, rs, args).await,
            "_prepare_move_withholding_lines" => self._prepare_move_withholding_lines(env, ctx, rs, args).await,
            "_prepare_move_liquidity_lines" => self._prepare_move_liquidity_lines(env, ctx, rs, args).await,
            "_prepare_move_counterpart_lines" => self._prepare_move_counterpart_lines(env, ctx, rs, args).await,
            "_prepare_move_lines_per_type" => self._prepare_move_lines_per_type(env, ctx, rs, args).await,
            "_prepare_move_line_default_vals" => self._prepare_move_line_default_vals(env, ctx, rs, args).await,
            "_compute_name" => self._compute_name(env, ctx, rs, args).await,
            "_compute_journal_id" => self._compute_journal_id(env, ctx, rs, args).await,
            "_compute_company_id" => self._compute_company_id(env, ctx, rs, args).await,
            "_compute_state" => self._compute_state(env, ctx, rs, args).await,
            "_compute_reconciliation_status" => self._compute_reconciliation_status(env, ctx, rs, args).await,
            "_get_method_codes_using_bank_account" => self._get_method_codes_using_bank_account(env, ctx, rs, args).await,
            "_get_method_codes_needing_bank_account" => self._get_method_codes_needing_bank_account(env, ctx, rs, args).await,
            "action_open_business_doc" => self.action_open_business_doc(env, ctx, rs, args).await,
            "_compute_show_require_partner_bank" => self._compute_show_require_partner_bank(env, ctx, rs, args).await,
            "_compute_amount_company_currency_signed" => self._compute_amount_company_currency_signed(env, ctx, rs, args).await,
            "_compute_amount_signed" => self._compute_amount_signed(env, ctx, rs, args).await,
            "_compute_available_partner_bank_ids" => self._compute_available_partner_bank_ids(env, ctx, rs, args).await,
            "_compute_partner_bank_id" => self._compute_partner_bank_id(env, ctx, rs, args).await,
            "_compute_payment_method_line_id" => self._compute_payment_method_line_id(env, ctx, rs, args).await,
            "_compute_payment_method_line_fields" => self._compute_payment_method_line_fields(env, ctx, rs, args).await,
            "_compute_available_journal_ids" => self._compute_available_journal_ids(env, ctx, rs, args).await,
            "_get_payment_method_codes_to_exclude" => self._get_payment_method_codes_to_exclude(env, ctx, rs, args).await,
            "_compute_currency_id" => self._compute_currency_id(env, ctx, rs, args).await,
            "_compute_outstanding_account_id" => self._compute_outstanding_account_id(env, ctx, rs, args).await,
            "_compute_destination_account_id" => self._compute_destination_account_id(env, ctx, rs, args).await,
            "_compute_qr_code" => self._compute_qr_code(env, ctx, rs, args).await,
            "_compute_stat_buttons_from_reconciliation" => self._compute_stat_buttons_from_reconciliation(env, ctx, rs, args).await,
            "_compute_payment_receipt_title" => self._compute_payment_receipt_title(env, ctx, rs, args).await,
            "_compute_duplicate_payment_ids" => self._compute_duplicate_payment_ids(env, ctx, rs, args).await,
            "_search_reconciled_invoice_ids" => self._search_reconciled_invoice_ids(env, ctx, rs, args).await,
            "_fetch_duplicate_reference" => self._fetch_duplicate_reference(env, ctx, rs, args).await,
            "_inverse_memo" => self._inverse_memo(env, ctx, rs, args).await,
            "_check_payment_method_line_id" => self._check_payment_method_line_id(env, ctx, rs, args).await,
            "_check_move_id" => self._check_move_id(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "_get_outstanding_account" => self._get_outstanding_account(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "_compute_display_name" => self._compute_display_name(env, ctx, rs, args).await,
            "copy_data" => self.copy_data(env, ctx, rs, args).await,
            "_message_mail_after_hook" => self._message_mail_after_hook(env, ctx, rs, args).await,
            "_synchronize_to_moves" => self._synchronize_to_moves(env, ctx, rs, args).await,
            "_get_trigger_fields_to_synchronize" => self._get_trigger_fields_to_synchronize(env, ctx, rs, args).await,
            "_generate_journal_entry" => self._generate_journal_entry(env, ctx, rs, args).await,
            "_generate_move_vals" => self._generate_move_vals(env, ctx, rs, args).await,
            "_get_payment_receipt_report_values" => self._get_payment_receipt_report_values(env, ctx, rs, args).await,
            "mark_as_sent" => self.mark_as_sent(env, ctx, rs, args).await,
            "unmark_as_sent" => self.unmark_as_sent(env, ctx, rs, args).await,
            "action_post" => self.action_post(env, ctx, rs, args).await,
            "action_validate" => self.action_validate(env, ctx, rs, args).await,
            "action_reject" => self.action_reject(env, ctx, rs, args).await,
            "action_cancel" => self.action_cancel(env, ctx, rs, args).await,
            "button_request_cancel" => self.button_request_cancel(env, ctx, rs, args).await,
            "action_draft" => self.action_draft(env, ctx, rs, args).await,
            "button_open_invoices" => self.button_open_invoices(env, ctx, rs, args).await,
            "button_open_bills" => self.button_open_bills(env, ctx, rs, args).await,
            "button_open_statement_lines" => self.button_open_statement_lines(env, ctx, rs, args).await,
            "button_open_journal_entry" => self.button_open_journal_entry(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountPaymentFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:210`). Decoradores: api.model.
    async fn _get_valid_payment_account_types(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._get_valid_payment_account_types".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:213`).
    async fn _seek_for_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._seek_for_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:243`).
    async fn _get_valid_liquidity_accounts(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._get_valid_liquidity_accounts".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:253`).
    async fn _valid_payment_states(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._valid_payment_states".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:259`).
    async fn _get_aml_default_display_name_list(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._get_aml_default_display_name_list".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:282`).
    async fn _prepare_move_withholding_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._prepare_move_withholding_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:286`).
    async fn _prepare_move_liquidity_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._prepare_move_liquidity_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:298`).
    async fn _prepare_move_counterpart_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._prepare_move_counterpart_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:310`).
    async fn _prepare_move_lines_per_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._prepare_move_lines_per_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:392`).
    async fn _prepare_move_line_default_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._prepare_move_line_default_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:414`). Decoradores: api.depends('move_id.name', 'state').
    async fn _compute_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:426`). Decoradores: api.depends('company_id', 'partner_id').
    async fn _compute_journal_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_journal_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:447`). Decoradores: api.depends('journal_id').
    async fn _compute_company_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_company_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:453`). Decoradores: api.depends('reconciled_invoice_ids.payment_state', 'move_id.line_ids.amount_residual').
    async fn _compute_state(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_state".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:469`). Decoradores: api.depends('move_id.line_ids.amount_residual', 'move_id.line_ids.amount_residual_currency', 'move_id.line_ids.account_id', 'state').
    async fn _compute_reconciliation_status(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_reconciliation_status".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:499`). Decoradores: api.model.
    async fn _get_method_codes_using_bank_account(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._get_method_codes_using_bank_account".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:503`). Decoradores: api.model.
    async fn _get_method_codes_needing_bank_account(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._get_method_codes_needing_bank_account".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:506`).
    async fn action_open_business_doc(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.action_open_business_doc".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:516`). Decoradores: api.depends('payment_method_code').
    async fn _compute_show_require_partner_bank(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_show_require_partner_bank".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:527`). Decoradores: api.depends('move_id.amount_total_signed', 'amount', 'payment_type', 'currency_id', 'date', 'company_id', 'company_currency_id').
    async fn _compute_amount_company_currency_signed(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_amount_company_currency_signed".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:541`). Decoradores: api.depends('amount', 'payment_type').
    async fn _compute_amount_signed(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_amount_signed".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:549`). Decoradores: api.depends('partner_id', 'company_id', 'payment_type').
    async fn _compute_available_partner_bank_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_available_partner_bank_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:558`). Decoradores: api.depends('available_partner_bank_ids', 'journal_id').
    async fn _compute_partner_bank_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_partner_bank_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:565`). Decoradores: api.depends('available_payment_method_line_ids').
    async fn _compute_payment_method_line_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_payment_method_line_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:585`). Decoradores: api.depends('payment_type', 'journal_id', 'currency_id').
    async fn _compute_payment_method_line_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_payment_method_line_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:593`). Decoradores: api.depends('payment_type').
    async fn _compute_available_journal_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_available_journal_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:609`).
    async fn _get_payment_method_codes_to_exclude(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._get_payment_method_codes_to_exclude".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:615`). Decoradores: api.depends('journal_id').
    async fn _compute_currency_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_currency_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:620`). Decoradores: api.depends('payment_method_line_id').
    async fn _compute_outstanding_account_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_outstanding_account_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:625`). Decoradores: api.depends('journal_id', 'partner_id', 'partner_type').
    async fn _compute_destination_account_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_destination_account_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:649`). Decoradores: api.depends('partner_bank_id', 'amount', 'memo', 'currency_id', 'journal_id', 'move_id.state', 'payment_method_line_id', 'payment_type').
    async fn _compute_qr_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_qr_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:676`). Decoradores: api.depends('move_id.line_ids.matched_debit_ids', 'move_id.line_ids.matched_credit_ids').
    async fn _compute_stat_buttons_from_reconciliation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_stat_buttons_from_reconciliation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:770`).
    async fn _compute_payment_receipt_title(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_payment_receipt_title".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:775`). Decoradores: api.depends('partner_id', 'amount', 'date', 'payment_type').
    async fn _compute_duplicate_payment_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_duplicate_payment_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:782`).
    async fn _search_reconciled_invoice_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._search_reconciled_invoice_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:788`).
    async fn _fetch_duplicate_reference(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._fetch_duplicate_reference".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:852`).
    async fn _inverse_memo(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._inverse_memo".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:863`). Decoradores: api.constrains('payment_method_line_id').
    async fn _check_payment_method_line_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._check_payment_method_line_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:874`). Decoradores: api.constrains('state', 'move_id').
    async fn _check_move_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._check_move_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:888`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:935`).
    async fn _get_outstanding_account(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._get_outstanding_account".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:946`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:956`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:966`). Decoradores: api.depends('move_id.name').
    async fn _compute_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._compute_display_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:970`).
    async fn copy_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.copy_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:981`).
    async fn _message_mail_after_hook(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._message_mail_after_hook".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:994`).
    async fn _synchronize_to_moves(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._synchronize_to_moves".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:1062`). Decoradores: api.model.
    async fn _get_trigger_fields_to_synchronize(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._get_trigger_fields_to_synchronize".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:1068`).
    async fn _generate_journal_entry(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._generate_journal_entry".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:1080`).
    async fn _generate_move_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._generate_move_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:1102`).
    async fn _get_payment_receipt_report_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment._get_payment_receipt_report_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:1119`).
    async fn mark_as_sent(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.mark_as_sent".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:1122`).
    async fn unmark_as_sent(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.unmark_as_sent".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:1125`).
    async fn action_post(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.action_post".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:1145`).
    async fn action_validate(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.action_validate".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:1148`).
    async fn action_reject(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.action_reject".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:1151`).
    async fn action_cancel(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.action_cancel".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:1157`).
    async fn button_request_cancel(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.button_request_cancel".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:1160`).
    async fn action_draft(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.action_draft".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:1164`).
    async fn button_open_invoices(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.button_open_invoices".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:1175`).
    async fn button_open_bills(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.button_open_bills".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:1199`).
    async fn button_open_statement_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.button_open_statement_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py:1223`).
    async fn button_open_journal_entry(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.button_open_journal_entry".into(),
        ))
    }

}
