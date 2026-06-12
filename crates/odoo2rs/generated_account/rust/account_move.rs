//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.move`

use nexus_orm::prelude::*;

pub struct AccountMoveFragment;

#[async_trait]
impl ModelFragment for AccountMoveFragment {
    fn model_name(&self) -> &str {
        "account.move"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Journal Entry".into();
        def.order = "date desc, name desc, invoice_date desc, id desc".into();
        def.add_field(FieldDef::char("name").string("Number").computed("_compute_name", &["posted_before", "state", "journal_id", "date", "move_type", "origin_payment_id"]).stored());
        def.add_field(FieldDef::char("name_placeholder").computed("_compute_name_placeholder", &["date", "journal_id", "move_type", "name", "posted_before", "sequence_number", "sequence_prefix", "state"]).stored());
        def.add_field(FieldDef::char("ref").string("Reference"));
        def.add_field(FieldDef::date("date").string("Date").required().computed("_compute_date", &["invoice_date", "company_id", "move_type", "taxable_supply_date"]).stored());
        def.add_field(FieldDef::selection("state", &[("draft", "Draft"), ("posted", "Posted"), ("cancel", "Cancelled")]).string("Status").required().readonly().default_val("draft"));
        def.add_field(FieldDef::selection("move_type", &[("entry", "Journal Entry"), ("out_invoice", "Customer Invoice"), ("out_refund", "Customer Credit Note"), ("in_invoice", "Vendor Bill"), ("in_refund", "Vendor Credit Note"), ("out_receipt", "Sales Receipt"), ("in_receipt", "Purchase Receipt")]).string("Type").required().readonly().default_val("entry"));
        def.add_field(FieldDef::boolean("is_storno").computed("_compute_is_storno", &["move_type"]).stored());
        def.add_field(FieldDef::many2one("journal_id", "account.journal").string("Journal").required().computed("_compute_journal_id", &["move_type", "origin_payment_id", "statement_line_id"]).stored());
        def.add_field({ let mut f = FieldDef::many2one("journal_group_id", "account.journal.group").string("Ledger"); f.store = false; f });
        def.add_field(FieldDef::many2one("company_id", "res.company").string("Company").computed("_compute_company_id", &["journal_id"]).stored());
        def.add_field(FieldDef::one2many("line_ids", "account.move.line", "move_id").string("Journal Items"));
        def.add_field(FieldDef::one2many("journal_line_ids", "account.move.line", "move_id").string("Journal Items (DEPRECATED)"));
        def.add_field(FieldDef::one2many("exchange_diff_partial_ids", "account.partial.reconcile", "exchange_move_id").string("Related reconciliation"));
        def.add_field(FieldDef::many2one("origin_payment_id", "account.payment").string("Payment"));
        def.add_field(FieldDef::many2many("matched_payment_ids", "account.payment").string("Matched Payments"));
        def.add_field(FieldDef::many2many("reconciled_payment_ids", "account.payment").string("Reconciled Payments").computed("_compute_reconciled_payment_ids", &["line_ids.matched_debit_ids", "line_ids.matched_credit_ids", "matched_payment_ids", "matched_payment_ids.state"]).stored());
        def.add_field(FieldDef::integer("payment_count").computed("_compute_payment_count", &["reconciled_payment_ids"]).stored());
        def.add_field(FieldDef::many2one("statement_line_id", "account.bank.statement.line").string("Statement Line"));
        // TODO(odoo2rs): campo 'statement_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::many2many("adjusting_entry_origin_move_ids", "account.move").string("Adjusting Entry Origin Moves"));
        def.add_field(FieldDef::char("adjusting_entry_origin_label").computed("_compute_adjusting_entry_origin_label", &["adjusting_entry_origin_move_ids"]).stored());
        def.add_field(FieldDef::integer("adjusting_entry_origin_moves_count").string("Adjusting Entry Origin Moves Count").computed("_compute_adjusting_entry_origin_moves_count", &["adjusting_entry_origin_move_ids"]).stored());
        def.add_field(FieldDef::many2many("adjusting_entries_move_ids", "account.move").string("Created Adjusting Entries"));
        def.add_field(FieldDef::integer("adjusting_entries_count").string("Adjusting Entries Count").computed("_compute_adjusting_entries_count", &["adjusting_entries_move_ids"]).stored());
        def.add_field(FieldDef::many2one("tax_cash_basis_rec_id", "account.partial.reconcile").string("Tax Cash Basis Entry of"));
        def.add_field(FieldDef::many2one("tax_cash_basis_origin_move_id", "account.move").string("Cash Basis Origin").readonly());
        def.add_field(FieldDef::one2many("tax_cash_basis_created_move_ids", "account.move", "tax_cash_basis_origin_move_id").string("Cash Basis Entries"));
        def.add_field(FieldDef::boolean("always_tax_exigible").computed("_compute_always_tax_exigible", &["line_ids.account_id.account_type"]).stored());
        def.add_field(FieldDef::selection("auto_post", &[("no", "No"), ("at_date", "At Date"), ("monthly", "Monthly"), ("quarterly", "Quarterly"), ("yearly", "Yearly")]).string("Auto-post").required().default_val("no"));
        def.add_field(FieldDef::date("auto_post_until").string("Auto-post until").computed("_compute_auto_post_until", &["auto_post"]).stored());
        def.add_field(FieldDef::many2one("auto_post_origin_id", "account.move").string("First recurring entry").readonly());
        def.add_field(FieldDef::boolean("hide_post_button").readonly().computed("_compute_hide_post_button", &["date", "auto_post"]).stored());
        def.add_field(FieldDef::boolean("checked").string("Reviewed").computed("_compute_checked", &["state", "journal_id.type"]).stored());
        def.add_field(FieldDef::boolean("posted_before"));
        def.add_field(FieldDef::many2many("suitable_journal_ids", "account.journal").computed("_compute_suitable_journal_ids", &["company_id", "invoice_filter_type_domain"]).stored());
        def.add_field(FieldDef::char("highest_name").computed("_compute_highest_name", &["journal_id", "date"]).stored());
        def.add_field(FieldDef::boolean("made_sequence_gap").computed("_compute_made_sequence_gap", &["journal_id", "sequence_number", "sequence_prefix", "state"]).stored());
        def.add_field({ let mut f = FieldDef::boolean("show_name_warning"); f.store = false; f });
        def.add_field(FieldDef::char("type_name").string("Type Name").computed("_compute_type_name", &["move_type"]).stored());
        def.add_field({ let mut f = FieldDef::char("country_code").readonly(); f.related = Some("company_id.account_fiscal_country_id.code".into()); f });
        def.add_field({ let mut f = FieldDef::json("account_fiscal_country_group_codes"); f.related = Some("company_id.account_fiscal_country_group_codes".into()); f });
        def.add_field({ let mut f = FieldDef::selection("company_price_include", &[]).readonly(); f.related = Some("company_id.account_price_include".into()); f });
        def.add_field(FieldDef::one2many("attachment_ids", "ir.attachment", "res_id").string("Attachments"));
        def.add_field(FieldDef::one2many("audit_trail_message_ids", "mail.message", "res_id").string("Audit Trail Messages"));
        def.add_field(FieldDef::boolean("no_followup").string("No Follow-Up").computed("_compute_no_followup", &["line_ids.no_followup"]).stored());
        def.add_field({ let mut f = FieldDef::boolean("restrict_mode_hash_table"); f.related = Some("journal_id.restrict_mode_hash_table".into()); f });
        def.add_field(FieldDef::integer("secure_sequence_number").string("Inalterability No Gap Sequence #").readonly());
        def.add_field(FieldDef::char("inalterable_hash").string("Inalterability Hash").readonly());
        def.add_field(FieldDef::boolean("secured").computed("_compute_secured", &["inalterable_hash"]).stored());
        def.add_field(FieldDef::one2many("invoice_line_ids", "account.move.line", "move_id").string("Invoice lines"));
        def.add_field(FieldDef::date("invoice_date").string("Invoice/Bill Date"));
        def.add_field(FieldDef::date("invoice_date_due").string("Due Date").computed("_compute_invoice_date_due", &["needed_terms"]).stored());
        def.add_field(FieldDef::date("delivery_date").string("Delivery Date").computed("_compute_delivery_date", &[]).stored());
        def.add_field(FieldDef::boolean("show_delivery_date").computed("_compute_show_delivery_date", &["delivery_date"]).stored());
        def.add_field(FieldDef::date("taxable_supply_date").string("Taxable Supply Date").computed("_compute_taxable_supply_date", &[]).stored());
        def.add_field(FieldDef::boolean("show_taxable_supply_date").computed("_compute_show_taxable_supply_date", &[]).stored());
        def.add_field(FieldDef::char("taxable_supply_date_placeholder").computed("_compute_taxable_supply_date_placeholder", &[]).stored());
        def.add_field(FieldDef::many2one("invoice_payment_term_id", "account.payment.term").string("Payment Terms").computed("_compute_invoice_payment_term_id", &["partner_id"]).stored());
        def.add_field(FieldDef::new("needed_terms", FieldType::Binary).computed("_compute_needed_terms", &["invoice_payment_term_id", "invoice_date", "currency_id", "amount_total_in_currency_signed", "invoice_date_due"]).stored());
        def.add_field(FieldDef::boolean("needed_terms_dirty").computed("_compute_needed_terms", &["invoice_payment_term_id", "invoice_date", "currency_id", "amount_total_in_currency_signed", "invoice_date_due"]).stored());
        def.add_field({ let mut f = FieldDef::selection("tax_calculation_rounding_method", &[]).string("Tax calculation rounding method").readonly(); f.related = Some("company_id.tax_calculation_rounding_method".into()); f });
        def.add_field(FieldDef::boolean("show_journal").computed("_compute_show_journal", &["suitable_journal_ids"]).stored());
        def.add_field(FieldDef::many2one("partner_id", "res.partner").string("Partner"));
        def.add_field(FieldDef::many2one("commercial_partner_id", "res.partner").string("Commercial Entity").readonly().computed("_compute_commercial_partner_id", &["partner_id"]).stored());
        def.add_field(FieldDef::many2one("partner_shipping_id", "res.partner").string("Delivery Address").computed("_compute_partner_shipping_id", &["partner_id"]).stored());
        def.add_field(FieldDef::many2one("partner_bank_id", "res.partner.bank").string("Recipient Bank").computed("_compute_partner_bank_id", &["bank_partner_id", "currency_id", "preferred_payment_method_line_id"]).stored());
        def.add_field(FieldDef::many2one("fiscal_position_id", "account.fiscal.position").string("Fiscal Position").computed("_compute_fiscal_position_id", &["partner_id", "partner_shipping_id", "company_id", "move_type"]).stored());
        def.add_field(FieldDef::char("payment_reference").string("Payment Reference").computed("_compute_payment_reference", &[]).stored());
        def.add_field(FieldDef::boolean("display_qr_code").string("Display QR-code").computed("_compute_display_qr_code", &["company_id"]).stored());
        def.add_field(FieldDef::boolean("display_link_qr_code").string("Display Link QR-code").computed("_compute_display_link_qr_code", &["company_id"]).stored());
        def.add_field(FieldDef::selection("qr_code_method", &[]).string("Payment QR-code"));
        def.add_field(FieldDef::new("invoice_outstanding_credits_debits_widget", FieldType::Binary).computed("_compute_payments_widget_to_reconcile_info", &[]).stored());
        def.add_field(FieldDef::boolean("invoice_has_outstanding").computed("_compute_invoice_has_outstanding", &["invoice_outstanding_credits_debits_widget"]).stored());
        def.add_field(FieldDef::new("invoice_payments_widget", FieldType::Binary).computed("_compute_payments_widget_reconciled_info", &["move_type", "line_ids.amount_residual"]).stored());
        def.add_field(FieldDef::many2one("preferred_payment_method_line_id", "account.payment.method.line").string("Preferred Payment Method Line").computed("_compute_preferred_payment_method_line_id", &["partner_id", "company_id"]).stored());
        // TODO(odoo2rs): campo 'company_currency_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::many2one("currency_id", "res.currency").string("Currency").required().computed("_compute_currency_id", &["journal_id", "statement_line_id"]).stored());
        def.add_field(FieldDef::float("expected_currency_rate").computed("_compute_expected_currency_rate", &["currency_id", "company_currency_id", "company_id", "invoice_date", "taxable_supply_date"]).stored());
        def.add_field(FieldDef::float("invoice_currency_rate").string("Currency Rate").computed("_compute_invoice_currency_rate", &["currency_id", "company_currency_id", "company_id", "invoice_date", "taxable_supply_date"]).stored());
        def.add_field(FieldDef::integer("direction_sign").computed("_compute_direction_sign", &["move_type"]).stored());
        def.add_field(FieldDef::monetary("amount_untaxed").string("Untaxed Amount").readonly().computed("_compute_amount", &["line_ids.matched_debit_ids.debit_move_id.move_id.origin_payment_id.is_matched", "line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual", "line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual_currency", "line_ids.matched_credit_ids.credit_move_id.move_id.origin_payment_id.is_matched", "line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual", "line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual_currency", "line_ids.balance", "line_ids.currency_id", "line_ids.amount_currency", "line_ids.amount_residual", "line_ids.amount_residual_currency", "line_ids.payment_id.state", "line_ids.full_reconcile_id", "state"]).stored());
        def.add_field(FieldDef::monetary("amount_tax").string("Tax").readonly().computed("_compute_amount", &["line_ids.matched_debit_ids.debit_move_id.move_id.origin_payment_id.is_matched", "line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual", "line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual_currency", "line_ids.matched_credit_ids.credit_move_id.move_id.origin_payment_id.is_matched", "line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual", "line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual_currency", "line_ids.balance", "line_ids.currency_id", "line_ids.amount_currency", "line_ids.amount_residual", "line_ids.amount_residual_currency", "line_ids.payment_id.state", "line_ids.full_reconcile_id", "state"]).stored());
        def.add_field(FieldDef::monetary("amount_total").string("Total").readonly().computed("_compute_amount", &["line_ids.matched_debit_ids.debit_move_id.move_id.origin_payment_id.is_matched", "line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual", "line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual_currency", "line_ids.matched_credit_ids.credit_move_id.move_id.origin_payment_id.is_matched", "line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual", "line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual_currency", "line_ids.balance", "line_ids.currency_id", "line_ids.amount_currency", "line_ids.amount_residual", "line_ids.amount_residual_currency", "line_ids.payment_id.state", "line_ids.full_reconcile_id", "state"]).stored());
        def.add_field(FieldDef::monetary("amount_residual").string("Amount Due").computed("_compute_amount", &["line_ids.matched_debit_ids.debit_move_id.move_id.origin_payment_id.is_matched", "line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual", "line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual_currency", "line_ids.matched_credit_ids.credit_move_id.move_id.origin_payment_id.is_matched", "line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual", "line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual_currency", "line_ids.balance", "line_ids.currency_id", "line_ids.amount_currency", "line_ids.amount_residual", "line_ids.amount_residual_currency", "line_ids.payment_id.state", "line_ids.full_reconcile_id", "state"]).stored());
        def.add_field(FieldDef::monetary("amount_untaxed_signed").string("Untaxed Amount Signed").readonly().computed("_compute_amount", &["line_ids.matched_debit_ids.debit_move_id.move_id.origin_payment_id.is_matched", "line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual", "line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual_currency", "line_ids.matched_credit_ids.credit_move_id.move_id.origin_payment_id.is_matched", "line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual", "line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual_currency", "line_ids.balance", "line_ids.currency_id", "line_ids.amount_currency", "line_ids.amount_residual", "line_ids.amount_residual_currency", "line_ids.payment_id.state", "line_ids.full_reconcile_id", "state"]).stored());
        def.add_field(FieldDef::monetary("amount_untaxed_in_currency_signed").string("Untaxed Amount Signed Currency").readonly().computed("_compute_amount", &["line_ids.matched_debit_ids.debit_move_id.move_id.origin_payment_id.is_matched", "line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual", "line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual_currency", "line_ids.matched_credit_ids.credit_move_id.move_id.origin_payment_id.is_matched", "line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual", "line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual_currency", "line_ids.balance", "line_ids.currency_id", "line_ids.amount_currency", "line_ids.amount_residual", "line_ids.amount_residual_currency", "line_ids.payment_id.state", "line_ids.full_reconcile_id", "state"]).stored());
        def.add_field(FieldDef::monetary("amount_tax_signed").string("Tax Signed").readonly().computed("_compute_amount", &["line_ids.matched_debit_ids.debit_move_id.move_id.origin_payment_id.is_matched", "line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual", "line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual_currency", "line_ids.matched_credit_ids.credit_move_id.move_id.origin_payment_id.is_matched", "line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual", "line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual_currency", "line_ids.balance", "line_ids.currency_id", "line_ids.amount_currency", "line_ids.amount_residual", "line_ids.amount_residual_currency", "line_ids.payment_id.state", "line_ids.full_reconcile_id", "state"]).stored());
        def.add_field(FieldDef::monetary("amount_total_signed").string("Total Signed").readonly().computed("_compute_amount", &["line_ids.matched_debit_ids.debit_move_id.move_id.origin_payment_id.is_matched", "line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual", "line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual_currency", "line_ids.matched_credit_ids.credit_move_id.move_id.origin_payment_id.is_matched", "line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual", "line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual_currency", "line_ids.balance", "line_ids.currency_id", "line_ids.amount_currency", "line_ids.amount_residual", "line_ids.amount_residual_currency", "line_ids.payment_id.state", "line_ids.full_reconcile_id", "state"]).stored());
        def.add_field(FieldDef::monetary("amount_total_in_currency_signed").string("Total in Currency Signed").readonly().computed("_compute_amount", &["line_ids.matched_debit_ids.debit_move_id.move_id.origin_payment_id.is_matched", "line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual", "line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual_currency", "line_ids.matched_credit_ids.credit_move_id.move_id.origin_payment_id.is_matched", "line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual", "line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual_currency", "line_ids.balance", "line_ids.currency_id", "line_ids.amount_currency", "line_ids.amount_residual", "line_ids.amount_residual_currency", "line_ids.payment_id.state", "line_ids.full_reconcile_id", "state"]).stored());
        def.add_field(FieldDef::monetary("amount_residual_signed").string("Amount Due Signed").computed("_compute_amount", &["line_ids.matched_debit_ids.debit_move_id.move_id.origin_payment_id.is_matched", "line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual", "line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual_currency", "line_ids.matched_credit_ids.credit_move_id.move_id.origin_payment_id.is_matched", "line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual", "line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual_currency", "line_ids.balance", "line_ids.currency_id", "line_ids.amount_currency", "line_ids.amount_residual", "line_ids.amount_residual_currency", "line_ids.payment_id.state", "line_ids.full_reconcile_id", "state"]).stored());
        def.add_field(FieldDef::new("tax_totals", FieldType::Binary).string("Invoice Totals").computed("_compute_tax_totals", &["invoice_line_ids.currency_rate", "invoice_line_ids.tax_base_amount", "invoice_line_ids.tax_line_id", "invoice_line_ids.price_total", "invoice_line_ids.price_subtotal", "invoice_payment_term_id", "partner_id", "currency_id"]).stored());
        def.add_field(FieldDef::selection("payment_state", &[]).string("Payment Status").readonly().computed("_compute_payment_state", &["amount_residual", "move_type", "state", "company_id", "reconciled_payment_ids.state"]).stored());
        def.add_field(FieldDef::selection("status_in_payment", &[]).computed("_compute_status_in_payment", &["payment_state", "state", "is_move_sent"]).stored());
        def.add_field(FieldDef::char("amount_total_words").string("Amount total in words").computed("_compute_amount_total_words", &["amount_total", "currency_id"]).stored());
        def.add_field(FieldDef::many2one("reversed_entry_id", "account.move").string("Reversal of").readonly());
        def.add_field(FieldDef::one2many("reversal_move_ids", "account.move", "reversed_entry_id"));
        def.add_field({ let mut f = FieldDef::many2one("invoice_vendor_bill_id", "account.move").string("Vendor Bill"); f.store = false; f });
        def.add_field(FieldDef::char("invoice_source_email").string("Source Email"));
        def.add_field(FieldDef::char("invoice_partner_display_name").computed("_compute_invoice_partner_display_info", &["partner_id", "invoice_source_email", "partner_id.display_name"]).stored());
        def.add_field(FieldDef::boolean("is_manually_modified"));
        def.add_field(FieldDef::boolean("quick_edit_mode").computed("_compute_quick_edit_mode", &["journal_id.type", "company_id"]).stored());
        def.add_field(FieldDef::monetary("quick_edit_total_amount").string("Total (Tax inc.)"));
        def.add_field(FieldDef::json("quick_encoding_vals").computed("_compute_quick_encoding_vals", &["quick_edit_total_amount", "invoice_line_ids.price_total", "tax_totals"]).stored());
        def.add_field(FieldDef::html("narration").string("Terms and Conditions").computed("_compute_narration", &["move_type", "partner_id", "partner_id.lang", "company_id"]).stored());
        def.add_field(FieldDef::boolean("is_move_sent").readonly());
        def.add_field(FieldDef::boolean("is_being_sent").computed("_compute_is_being_sent", &["sending_data"]).stored());
        def.add_field(FieldDef::selection("move_sent_values", &[("sent", "Sent"), ("not_sent", "Not Sent")]).string("Sent").computed("compute_move_sent_values", &["is_move_sent"]).stored());
        def.add_field(FieldDef::many2one("invoice_user_id", "res.users").string("Salesperson").computed("_compute_invoice_default_sale_person", &["move_type", "partner_id"]).stored());
        // TODO(odoo2rs): campo 'user_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::char("invoice_origin").string("Origin").readonly());
        def.add_field(FieldDef::many2one("invoice_incoterm_id", "account.incoterms").string("Incoterm").computed("_compute_incoterm", &["company_id", "move_type"]).stored());
        def.add_field(FieldDef::char("incoterm_location").string("Incoterm Location").computed("_compute_incoterm_location", &[]).stored());
        def.add_field(FieldDef::many2one("invoice_cash_rounding_id", "account.cash.rounding").string("Cash Rounding Method"));
        def.add_field(FieldDef::json("sending_data"));
        def.add_field(FieldDef::many2one("invoice_pdf_report_id", "ir.attachment").string("PDF Attachment"));
        def.add_field(FieldDef::new("invoice_pdf_report_file", FieldType::Binary).string("PDF File"));
        def.add_field(FieldDef::char("invoice_incoterm_placeholder").computed("_compute_invoice_incoterm_placeholder", &["company_id.incoterm_id"]).stored());
        def.add_field(FieldDef::char("invoice_filter_type_domain").computed("_compute_invoice_filter_type_domain", &["move_type"]).stored());
        def.add_field(FieldDef::many2one("bank_partner_id", "res.partner").computed("_compute_bank_partner_id", &["commercial_partner_id", "company_id", "move_type"]).stored());
        def.add_field(FieldDef::char("tax_lock_date_message").computed("_compute_tax_lock_date_message", &["date", "line_ids.debit", "line_ids.credit", "line_ids.tax_line_id", "line_ids.tax_ids", "line_ids.tax_tag_ids", "invoice_line_ids.debit", "invoice_line_ids.credit", "invoice_line_ids.tax_line_id", "invoice_line_ids.tax_ids", "invoice_line_ids.tax_tag_ids"]).stored());
        def.add_field(FieldDef::boolean("display_inactive_currency_warning").computed("_compute_display_inactive_currency_warning", &["currency_id"]).stored());
        def.add_field(FieldDef::many2one("tax_country_id", "res.country").computed("_compute_tax_country_id", &["company_id.account_fiscal_country_id", "fiscal_position_id", "fiscal_position_id.country_id", "fiscal_position_id.foreign_vat"]).stored());
        def.add_field(FieldDef::char("tax_country_code").computed("_compute_tax_country_code", &["tax_country_id"]).stored());
        def.add_field(FieldDef::boolean("has_reconciled_entries").computed("_compute_has_reconciled_entries", &["line_ids"]).stored());
        def.add_field(FieldDef::boolean("show_reset_to_draft_button").computed("_compute_show_reset_to_draft_button", &["restrict_mode_hash_table", "state", "inalterable_hash"]).stored());
        def.add_field(FieldDef::text("partner_credit_warning").computed("_compute_partner_credit_warning", &["company_id", "partner_id", "tax_totals", "currency_id"]).stored());
        def.add_field(FieldDef::many2many("duplicated_ref_ids", "account.move").computed("_compute_duplicated_ref_ids", &["ref", "move_type", "partner_id", "invoice_date", "tax_totals"]).stored());
        def.add_field(FieldDef::boolean("is_draft_duplicated_ref_ids").computed("_compute_is_draft_duplicated_ref_ids", &["duplicated_ref_ids"]).stored());
        def.add_field(FieldDef::boolean("need_cancel_request").computed("_compute_need_cancel_request", &["country_code"]).stored());
        def.add_field({ let mut f = FieldDef::boolean("show_update_fpos").string("Has Fiscal Position Changed"); f.store = false; f });
        def.add_field(FieldDef::new("payment_term_details", FieldType::Binary).computed("_compute_payment_term_details", &["show_payment_term_details"]).stored());
        def.add_field(FieldDef::boolean("show_payment_term_details").computed("_compute_show_payment_term_details", &["move_type", "payment_state", "invoice_payment_term_id"]).stored());
        def.add_field(FieldDef::boolean("show_discount_details").computed("_compute_show_payment_term_details", &["move_type", "payment_state", "invoice_payment_term_id"]).stored());
        def.add_field(FieldDef::text("abnormal_amount_warning").computed("_compute_abnormal_warnings", &["partner_id", "invoice_date", "amount_total"]).stored());
        def.add_field(FieldDef::text("abnormal_date_warning").computed("_compute_abnormal_warnings", &["partner_id", "invoice_date", "amount_total"]).stored());
        def.add_field(FieldDef::json("alerts").computed("_compute_alerts", &["state", "invoice_line_ids", "tax_lock_date_message", "auto_post", "auto_post_until", "is_being_sent", "partner_credit_warning", "abnormal_amount_warning", "abnormal_date_warning"]).stored());
        def.add_field(FieldDef::html("taxes_legal_notes").string("Taxes Legal Notes").computed("_compute_taxes_legal_notes", &["line_ids.tax_ids"]).stored());
        def.add_field(FieldDef::date("next_payment_date").string("Next Payment Date").computed("_compute_next_payment_date", &["line_ids.payment_date", "line_ids.reconciled"]).stored());
        def.add_field(FieldDef::boolean("display_send_button").computed("_compute_display_send_button", &["move_type", "state"]).stored());
        def.add_field(FieldDef::boolean("highlight_send_button").computed("_compute_highlight_send_button", &["is_being_sent", "invoice_pdf_report_id"]).stored());
        def.add_field(FieldDef::boolean("is_sale_installed").computed("_compute_is_sale_installed", &[]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_sequence_monthly_regex", "_sequence_yearly_regex", "_sequence_year_range_regex", "_sequence_fixed_regex", "_sequence_year_range_monthly_regex", "_auto_init", "_compute_invoice_default_sale_person", "_compute_is_being_sent", "compute_move_sent_values", "_search_move_sent_values", "_compute_payment_reference", "_get_accounting_date_source", "_compute_date", "_compute_auto_post_until", "_compute_hide_post_button", "_compute_company_id", "_compute_journal_id", "_get_valid_journal_types", "_search_default_journal", "_compute_is_storno", "_compute_suitable_journal_ids", "_compute_name", "_compute_name_placeholder", "_compute_highest_name", "_compute_made_sequence_gap", "_compute_type_name", "_compute_secured", "_search_secured", "_compute_always_tax_exigible", "_compute_commercial_partner_id", "_compute_partner_shipping_id", "_compute_fiscal_position_id", "_compute_partner_bank_id", "_compute_invoice_payment_term_id", "_compute_invoice_date_due", "_compute_delivery_date", "_compute_show_delivery_date", "_compute_taxable_supply_date", "_compute_show_taxable_supply_date", "_compute_taxable_supply_date_placeholder", "_compute_currency_id", "_get_invoice_currency_rate_date", "_get_expected_currency_rate_at", "_compute_expected_currency_rate", "_compute_invoice_currency_rate", "_compute_direction_sign", "_compute_amount", "_compute_payment_state", "_compute_status_in_payment", "_field_to_sql", "_compute_payment_count", "_compute_adjusting_entries_count", "_compute_adjusting_entry_origin_moves_count", "_compute_adjusting_entry_origin_label", "_compute_needed_terms", "_compute_show_journal", "_compute_payments_widget_to_reconcile_info", "_compute_invoice_has_outstanding", "_compute_preferred_payment_method_line_id", "_compute_payments_widget_reconciled_info", "_get_product_base_line_currency_rate", "_prepare_product_base_line_for_taxes_computation", "_prepare_epd_base_line_for_taxes_computation", "_prepare_epd_base_lines_for_taxes_computation_from_base_lines", "_prepare_cash_rounding_base_line_for_taxes_computation", "_prepare_tax_line_for_taxes_computation", "_prepare_non_deductible_base_line_for_taxes_computation", "_prepare_non_deductible_base_lines_for_taxes_computation_from_base_lines", "_get_rounded_base_and_tax_lines", "_compute_tax_totals", "_compute_payment_term_details", "_compute_show_payment_term_details", "_need_cancel_request", "_compute_need_cancel_request", "_compute_invoice_partner_display_info", "_compute_invoice_filter_type_domain", "_compute_bank_partner_id", "_compute_tax_lock_date_message", "_compute_display_inactive_currency_warning", "_compute_tax_country_id", "_compute_tax_country_code", "_compute_has_reconciled_entries", "_compute_show_reset_to_draft_button", "_compute_access_url", "_compute_narration", "_get_partner_credit_warning_exclude_amount", "_compute_partner_credit_warning", "_build_credit_warning_message", "_compute_quick_edit_mode", "_compute_quick_encoding_vals", "_compute_duplicated_ref_ids", "_fetch_duplicate_reference", "_compute_is_draft_duplicated_ref_ids", "_compute_display_qr_code", "_compute_display_link_qr_code", "_compute_amount_total_words", "_compute_incoterm", "_compute_linked_attachment_id", "_compute_incoterm_location", "_compute_invoice_incoterm_placeholder", "_compute_abnormal_warnings", "_compute_alerts", "_compute_taxes_legal_notes", "_compute_next_payment_date", "_compute_display_send_button", "_compute_highlight_send_button", "_compute_is_sale_installed", "_compute_reconciled_payment_ids", "_search_next_payment_date", "_compute_checked", "_compute_no_followup", "_inverse_no_followup", "_get_alerts", "_search_journal_group_id", "_search_reconciled_payment_ids", "_inverse_delivery_date", "_inverse_tax_totals", "_inverse_amount_total", "_inverse_partner_id", "_inverse_company_id", "_inverse_currency_id", "_inverse_journal_id", "_inverse_payment_reference", "_inverse_invoice_payment_term_id", "_inverse_name", "_onchange_date", "_onchange_invoice_vendor_bill", "_onchange_fpos_id_show_update_fpos", "_onchange_partner_id", "_onchange_name_warning", "_onchange_journal_id", "_onchange_invoice_cash_rounding_id", "_check_balanced", "_get_unbalanced_moves", "_check_fiscal_lock_dates", "_require_bill_date_for_autopost", "_check_journal_move_type", "_validate_taxes_country", "_check_invoice_currency_rate", "action_add_from_catalog", "_get_action_add_from_catalog_extra_context", "_get_product_catalog_domain", "_default_order_line_values", "_get_product_catalog_order_data", "_get_product_price_and_data", "_get_product_catalog_record_lines", "_update_order_line_info", "_is_readonly", "_get_parent_field_on_child_model", "_is_line_valid_for_section_line_count", "_is_eligible_for_early_payment_discount", "_early_payment_discount_move_types", "_synchronize_business_models", "_recompute_cash_rounding_lines", "_get_automatic_balancing_account", "_sync_unbalanced_lines", "_sync_rounding_lines", "_sync_dynamic_line_needed_values", "_sync_tax_lines", "_sync_non_deductible_base_lines", "_sync_dynamic_line", "_sync_invoice", "_get_sync_stack", "_sync_dynamic_lines", "check_field_access_rights", "_get_default_read_fields", "read", "search_read", "copy_data", "copy", "_get_copy_message_content", "_sanitize_vals", "_stolen_move", "_get_protected_vals", "create", "write", "check_move_sequence_chain", "_get_unlink_logger_message", "_unlink_forbid_parts_of_chain", "_unlink_account_audit_trail_except_once_post", "unlink", "_compute_display_name", "onchange", "_collect_tax_cash_basis_values", "_must_check_constrains_date_sequence", "_get_last_sequence_domain", "_get_starting_sequence", "_get_sequence_date_range", "_get_invoice_reference_euro_invoice", "_get_invoice_reference_euro_partner", "_get_invoice_reference_number_invoice", "_get_invoice_reference_number_partner", "_get_invoice_reference_odoo_invoice", "_get_invoice_reference_odoo_partner", "_get_invoice_computed_reference", "_get_frequent_account_and_taxes", "_get_quick_edit_suggestions", "_quick_edit_mode_suggest_invoice_date", "_onchange_quick_edit_total_amount", "_onchange_quick_edit_line_ids", "_check_total_amount", "_get_integrity_hash_fields", "_get_integrity_hash_fields_and_subfields", "_get_move_hash_domain", "_is_move_restricted", "_hash_moves", "_get_chain_info", "_get_chains_to_hash", "_calculate_hashes", "_apply_delta_recurring_entries", "_copy_recurring_entries", "_get_fields_to_copy_recurring_entries", "_extend_with_attachments", "_get_edi_creation", "_disable_discount_precision", "_reason_cannot_decode_has_invoice_lines", "_prepare_tax_lines_for_taxes_computation", "_prepare_invoice_aggregated_taxes", "_get_invoice_counterpart_amls_for_early_payment_discount_per_payment_term_line", "_get_invoice_counterpart_amls_for_early_payment_discount", "_affect_tax_report", "_get_move_display_name", "_get_reconciled_amls", "_get_reconciled_payments", "_get_reconciled_statement_lines", "_get_reconciled_invoices", "_get_all_reconciled_invoice_partials", "_get_reconciled_invoices_partials", "_reconcile_reversed_moves", "_reverse_moves", "_can_be_unlinked", "_is_protected_by_audit_trail", "_unlink_or_reverse", "_post", "_set_next_made_sequence_gap", "_find_and_set_purchase_orders", "_link_bill_origin_to_purchase_orders", "_autopost_bill", "_show_autopost_bills_wizard", "open_payments", "open_reconcile_view", "action_open_business_doc", "action_update_fpos_values", "open_created_caba_entries", "open_adjusting_entries", "open_adjusting_entry_origin_moves", "action_switch_move_type", "get_currency_rate", "refresh_invoice_currency_rate", "action_register_payment", "action_force_register_payment", "action_duplicate", "action_send_and_print", "action_invoice_sent", "action_invoice_download_pdf", "action_move_download_all", "action_print_pdf", "preview_invoice", "action_reverse", "action_post", "_get_moves_requiring_confirmation", "action_validate_moves_with_confirmation", "js_assign_outstanding_line", "js_remove_outstanding_partial", "button_set_checked", "check_selected_moves", "set_moves_checked", "button_draft", "_get_fields_to_detach", "_detach_attachments", "_check_draftable", "button_hash", "button_request_cancel", "button_cancel", "action_toggle_block_payment", "action_activate_currency", "action_delete_duplicates", "_get_mail_template", "_notify_get_recipients_groups", "_get_report_base_filename", "_autopost_draft_entries", "_cron_account_move_send", "_get_available_action_reports", "_is_action_report_available", "_get_suitable_journal_ids", "_get_invoice_filter_type_domain", "get_invoice_types", "is_invoice", "is_entry", "is_receipt", "get_sale_types", "is_sale_document", "get_purchase_types", "is_purchase_document", "get_inbound_types", "is_inbound", "get_outbound_types", "is_outbound", "_get_action_with_base_document_layout_configurator", "_get_installments_data", "_get_invoice_next_payment_values", "_get_invoice_portal_extra_values", "_get_accounting_date", "_get_violated_lock_dates", "_get_lock_date_message", "_move_dict_to_preview_vals", "_generate_qr_code", "_generate_portal_payment_qr", "_get_portal_payment_link", "_generate_and_send", "_get_invoice_pdf_proforma", "_get_invoice_legal_documents", "_get_invoice_legal_documents_all", "_get_invoice_report_filename", "_get_invoice_proforma_pdf_report_filename", "_prepare_edi_vals_to_export", "_get_discount_allocation_account", "_get_available_invoice_template_pdf_report_ids", "_is_user_able_to_review", "_field_will_change", "_cleanup_write_orm_values", "_disable_recursion", "_mailing_get_default_domain", "_routing_check_route", "message_new", "_attachment_fields_to_clear", "_message_post_after_hook", "_creation_subtype", "_track_subtype", "_creation_message", "_notify_by_email_prepare_rendering_context", "_get_mail_thread_data_attachments", "_conditional_add_to_compute", "_action_invoice_ready_to_be_sent", "_is_ready_to_be_sent", "_can_force_cancel", "_send_only_when_ready", "_invoice_paid_hook", "_get_lines_onchange_currency", "_get_invoice_in_payment_state", "_get_name_invoice_report", "_is_downpayment", "_refunds_origin_required", "_set_reversed_entry", "get_invoice_localisation_fields_required_to_invoice", "get_extra_print_items", "_get_move_zip_export_docs", "_get_move_lines_to_report", "_can_commit", "get_import_templates"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_sequence_monthly_regex" => self._sequence_monthly_regex(env, ctx, rs, args).await,
            "_sequence_yearly_regex" => self._sequence_yearly_regex(env, ctx, rs, args).await,
            "_sequence_year_range_regex" => self._sequence_year_range_regex(env, ctx, rs, args).await,
            "_sequence_fixed_regex" => self._sequence_fixed_regex(env, ctx, rs, args).await,
            "_sequence_year_range_monthly_regex" => self._sequence_year_range_monthly_regex(env, ctx, rs, args).await,
            "_auto_init" => self._auto_init(env, ctx, rs, args).await,
            "_compute_invoice_default_sale_person" => self._compute_invoice_default_sale_person(env, ctx, rs, args).await,
            "_compute_is_being_sent" => self._compute_is_being_sent(env, ctx, rs, args).await,
            "compute_move_sent_values" => self.compute_move_sent_values(env, ctx, rs, args).await,
            "_search_move_sent_values" => self._search_move_sent_values(env, ctx, rs, args).await,
            "_compute_payment_reference" => self._compute_payment_reference(env, ctx, rs, args).await,
            "_get_accounting_date_source" => self._get_accounting_date_source(env, ctx, rs, args).await,
            "_compute_date" => self._compute_date(env, ctx, rs, args).await,
            "_compute_auto_post_until" => self._compute_auto_post_until(env, ctx, rs, args).await,
            "_compute_hide_post_button" => self._compute_hide_post_button(env, ctx, rs, args).await,
            "_compute_company_id" => self._compute_company_id(env, ctx, rs, args).await,
            "_compute_journal_id" => self._compute_journal_id(env, ctx, rs, args).await,
            "_get_valid_journal_types" => self._get_valid_journal_types(env, ctx, rs, args).await,
            "_search_default_journal" => self._search_default_journal(env, ctx, rs, args).await,
            "_compute_is_storno" => self._compute_is_storno(env, ctx, rs, args).await,
            "_compute_suitable_journal_ids" => self._compute_suitable_journal_ids(env, ctx, rs, args).await,
            "_compute_name" => self._compute_name(env, ctx, rs, args).await,
            "_compute_name_placeholder" => self._compute_name_placeholder(env, ctx, rs, args).await,
            "_compute_highest_name" => self._compute_highest_name(env, ctx, rs, args).await,
            "_compute_made_sequence_gap" => self._compute_made_sequence_gap(env, ctx, rs, args).await,
            "_compute_type_name" => self._compute_type_name(env, ctx, rs, args).await,
            "_compute_secured" => self._compute_secured(env, ctx, rs, args).await,
            "_search_secured" => self._search_secured(env, ctx, rs, args).await,
            "_compute_always_tax_exigible" => self._compute_always_tax_exigible(env, ctx, rs, args).await,
            "_compute_commercial_partner_id" => self._compute_commercial_partner_id(env, ctx, rs, args).await,
            "_compute_partner_shipping_id" => self._compute_partner_shipping_id(env, ctx, rs, args).await,
            "_compute_fiscal_position_id" => self._compute_fiscal_position_id(env, ctx, rs, args).await,
            "_compute_partner_bank_id" => self._compute_partner_bank_id(env, ctx, rs, args).await,
            "_compute_invoice_payment_term_id" => self._compute_invoice_payment_term_id(env, ctx, rs, args).await,
            "_compute_invoice_date_due" => self._compute_invoice_date_due(env, ctx, rs, args).await,
            "_compute_delivery_date" => self._compute_delivery_date(env, ctx, rs, args).await,
            "_compute_show_delivery_date" => self._compute_show_delivery_date(env, ctx, rs, args).await,
            "_compute_taxable_supply_date" => self._compute_taxable_supply_date(env, ctx, rs, args).await,
            "_compute_show_taxable_supply_date" => self._compute_show_taxable_supply_date(env, ctx, rs, args).await,
            "_compute_taxable_supply_date_placeholder" => self._compute_taxable_supply_date_placeholder(env, ctx, rs, args).await,
            "_compute_currency_id" => self._compute_currency_id(env, ctx, rs, args).await,
            "_get_invoice_currency_rate_date" => self._get_invoice_currency_rate_date(env, ctx, rs, args).await,
            "_get_expected_currency_rate_at" => self._get_expected_currency_rate_at(env, ctx, rs, args).await,
            "_compute_expected_currency_rate" => self._compute_expected_currency_rate(env, ctx, rs, args).await,
            "_compute_invoice_currency_rate" => self._compute_invoice_currency_rate(env, ctx, rs, args).await,
            "_compute_direction_sign" => self._compute_direction_sign(env, ctx, rs, args).await,
            "_compute_amount" => self._compute_amount(env, ctx, rs, args).await,
            "_compute_payment_state" => self._compute_payment_state(env, ctx, rs, args).await,
            "_compute_status_in_payment" => self._compute_status_in_payment(env, ctx, rs, args).await,
            "_field_to_sql" => self._field_to_sql(env, ctx, rs, args).await,
            "_compute_payment_count" => self._compute_payment_count(env, ctx, rs, args).await,
            "_compute_adjusting_entries_count" => self._compute_adjusting_entries_count(env, ctx, rs, args).await,
            "_compute_adjusting_entry_origin_moves_count" => self._compute_adjusting_entry_origin_moves_count(env, ctx, rs, args).await,
            "_compute_adjusting_entry_origin_label" => self._compute_adjusting_entry_origin_label(env, ctx, rs, args).await,
            "_compute_needed_terms" => self._compute_needed_terms(env, ctx, rs, args).await,
            "_compute_show_journal" => self._compute_show_journal(env, ctx, rs, args).await,
            "_compute_payments_widget_to_reconcile_info" => self._compute_payments_widget_to_reconcile_info(env, ctx, rs, args).await,
            "_compute_invoice_has_outstanding" => self._compute_invoice_has_outstanding(env, ctx, rs, args).await,
            "_compute_preferred_payment_method_line_id" => self._compute_preferred_payment_method_line_id(env, ctx, rs, args).await,
            "_compute_payments_widget_reconciled_info" => self._compute_payments_widget_reconciled_info(env, ctx, rs, args).await,
            "_get_product_base_line_currency_rate" => self._get_product_base_line_currency_rate(env, ctx, rs, args).await,
            "_prepare_product_base_line_for_taxes_computation" => self._prepare_product_base_line_for_taxes_computation(env, ctx, rs, args).await,
            "_prepare_epd_base_line_for_taxes_computation" => self._prepare_epd_base_line_for_taxes_computation(env, ctx, rs, args).await,
            "_prepare_epd_base_lines_for_taxes_computation_from_base_lines" => self._prepare_epd_base_lines_for_taxes_computation_from_base_lines(env, ctx, rs, args).await,
            "_prepare_cash_rounding_base_line_for_taxes_computation" => self._prepare_cash_rounding_base_line_for_taxes_computation(env, ctx, rs, args).await,
            "_prepare_tax_line_for_taxes_computation" => self._prepare_tax_line_for_taxes_computation(env, ctx, rs, args).await,
            "_prepare_non_deductible_base_line_for_taxes_computation" => self._prepare_non_deductible_base_line_for_taxes_computation(env, ctx, rs, args).await,
            "_prepare_non_deductible_base_lines_for_taxes_computation_from_base_lines" => self._prepare_non_deductible_base_lines_for_taxes_computation_from_base_lines(env, ctx, rs, args).await,
            "_get_rounded_base_and_tax_lines" => self._get_rounded_base_and_tax_lines(env, ctx, rs, args).await,
            "_compute_tax_totals" => self._compute_tax_totals(env, ctx, rs, args).await,
            "_compute_payment_term_details" => self._compute_payment_term_details(env, ctx, rs, args).await,
            "_compute_show_payment_term_details" => self._compute_show_payment_term_details(env, ctx, rs, args).await,
            "_need_cancel_request" => self._need_cancel_request(env, ctx, rs, args).await,
            "_compute_need_cancel_request" => self._compute_need_cancel_request(env, ctx, rs, args).await,
            "_compute_invoice_partner_display_info" => self._compute_invoice_partner_display_info(env, ctx, rs, args).await,
            "_compute_invoice_filter_type_domain" => self._compute_invoice_filter_type_domain(env, ctx, rs, args).await,
            "_compute_bank_partner_id" => self._compute_bank_partner_id(env, ctx, rs, args).await,
            "_compute_tax_lock_date_message" => self._compute_tax_lock_date_message(env, ctx, rs, args).await,
            "_compute_display_inactive_currency_warning" => self._compute_display_inactive_currency_warning(env, ctx, rs, args).await,
            "_compute_tax_country_id" => self._compute_tax_country_id(env, ctx, rs, args).await,
            "_compute_tax_country_code" => self._compute_tax_country_code(env, ctx, rs, args).await,
            "_compute_has_reconciled_entries" => self._compute_has_reconciled_entries(env, ctx, rs, args).await,
            "_compute_show_reset_to_draft_button" => self._compute_show_reset_to_draft_button(env, ctx, rs, args).await,
            "_compute_access_url" => self._compute_access_url(env, ctx, rs, args).await,
            "_compute_narration" => self._compute_narration(env, ctx, rs, args).await,
            "_get_partner_credit_warning_exclude_amount" => self._get_partner_credit_warning_exclude_amount(env, ctx, rs, args).await,
            "_compute_partner_credit_warning" => self._compute_partner_credit_warning(env, ctx, rs, args).await,
            "_build_credit_warning_message" => self._build_credit_warning_message(env, ctx, rs, args).await,
            "_compute_quick_edit_mode" => self._compute_quick_edit_mode(env, ctx, rs, args).await,
            "_compute_quick_encoding_vals" => self._compute_quick_encoding_vals(env, ctx, rs, args).await,
            "_compute_duplicated_ref_ids" => self._compute_duplicated_ref_ids(env, ctx, rs, args).await,
            "_fetch_duplicate_reference" => self._fetch_duplicate_reference(env, ctx, rs, args).await,
            "_compute_is_draft_duplicated_ref_ids" => self._compute_is_draft_duplicated_ref_ids(env, ctx, rs, args).await,
            "_compute_display_qr_code" => self._compute_display_qr_code(env, ctx, rs, args).await,
            "_compute_display_link_qr_code" => self._compute_display_link_qr_code(env, ctx, rs, args).await,
            "_compute_amount_total_words" => self._compute_amount_total_words(env, ctx, rs, args).await,
            "_compute_incoterm" => self._compute_incoterm(env, ctx, rs, args).await,
            "_compute_linked_attachment_id" => self._compute_linked_attachment_id(env, ctx, rs, args).await,
            "_compute_incoterm_location" => self._compute_incoterm_location(env, ctx, rs, args).await,
            "_compute_invoice_incoterm_placeholder" => self._compute_invoice_incoterm_placeholder(env, ctx, rs, args).await,
            "_compute_abnormal_warnings" => self._compute_abnormal_warnings(env, ctx, rs, args).await,
            "_compute_alerts" => self._compute_alerts(env, ctx, rs, args).await,
            "_compute_taxes_legal_notes" => self._compute_taxes_legal_notes(env, ctx, rs, args).await,
            "_compute_next_payment_date" => self._compute_next_payment_date(env, ctx, rs, args).await,
            "_compute_display_send_button" => self._compute_display_send_button(env, ctx, rs, args).await,
            "_compute_highlight_send_button" => self._compute_highlight_send_button(env, ctx, rs, args).await,
            "_compute_is_sale_installed" => self._compute_is_sale_installed(env, ctx, rs, args).await,
            "_compute_reconciled_payment_ids" => self._compute_reconciled_payment_ids(env, ctx, rs, args).await,
            "_search_next_payment_date" => self._search_next_payment_date(env, ctx, rs, args).await,
            "_compute_checked" => self._compute_checked(env, ctx, rs, args).await,
            "_compute_no_followup" => self._compute_no_followup(env, ctx, rs, args).await,
            "_inverse_no_followup" => self._inverse_no_followup(env, ctx, rs, args).await,
            "_get_alerts" => self._get_alerts(env, ctx, rs, args).await,
            "_search_journal_group_id" => self._search_journal_group_id(env, ctx, rs, args).await,
            "_search_reconciled_payment_ids" => self._search_reconciled_payment_ids(env, ctx, rs, args).await,
            "_inverse_delivery_date" => self._inverse_delivery_date(env, ctx, rs, args).await,
            "_inverse_tax_totals" => self._inverse_tax_totals(env, ctx, rs, args).await,
            "_inverse_amount_total" => self._inverse_amount_total(env, ctx, rs, args).await,
            "_inverse_partner_id" => self._inverse_partner_id(env, ctx, rs, args).await,
            "_inverse_company_id" => self._inverse_company_id(env, ctx, rs, args).await,
            "_inverse_currency_id" => self._inverse_currency_id(env, ctx, rs, args).await,
            "_inverse_journal_id" => self._inverse_journal_id(env, ctx, rs, args).await,
            "_inverse_payment_reference" => self._inverse_payment_reference(env, ctx, rs, args).await,
            "_inverse_invoice_payment_term_id" => self._inverse_invoice_payment_term_id(env, ctx, rs, args).await,
            "_inverse_name" => self._inverse_name(env, ctx, rs, args).await,
            "_onchange_date" => self._onchange_date(env, ctx, rs, args).await,
            "_onchange_invoice_vendor_bill" => self._onchange_invoice_vendor_bill(env, ctx, rs, args).await,
            "_onchange_fpos_id_show_update_fpos" => self._onchange_fpos_id_show_update_fpos(env, ctx, rs, args).await,
            "_onchange_partner_id" => self._onchange_partner_id(env, ctx, rs, args).await,
            "_onchange_name_warning" => self._onchange_name_warning(env, ctx, rs, args).await,
            "_onchange_journal_id" => self._onchange_journal_id(env, ctx, rs, args).await,
            "_onchange_invoice_cash_rounding_id" => self._onchange_invoice_cash_rounding_id(env, ctx, rs, args).await,
            "_check_balanced" => self._check_balanced(env, ctx, rs, args).await,
            "_get_unbalanced_moves" => self._get_unbalanced_moves(env, ctx, rs, args).await,
            "_check_fiscal_lock_dates" => self._check_fiscal_lock_dates(env, ctx, rs, args).await,
            "_require_bill_date_for_autopost" => self._require_bill_date_for_autopost(env, ctx, rs, args).await,
            "_check_journal_move_type" => self._check_journal_move_type(env, ctx, rs, args).await,
            "_validate_taxes_country" => self._validate_taxes_country(env, ctx, rs, args).await,
            "_check_invoice_currency_rate" => self._check_invoice_currency_rate(env, ctx, rs, args).await,
            "action_add_from_catalog" => self.action_add_from_catalog(env, ctx, rs, args).await,
            "_get_action_add_from_catalog_extra_context" => self._get_action_add_from_catalog_extra_context(env, ctx, rs, args).await,
            "_get_product_catalog_domain" => self._get_product_catalog_domain(env, ctx, rs, args).await,
            "_default_order_line_values" => self._default_order_line_values(env, ctx, rs, args).await,
            "_get_product_catalog_order_data" => self._get_product_catalog_order_data(env, ctx, rs, args).await,
            "_get_product_price_and_data" => self._get_product_price_and_data(env, ctx, rs, args).await,
            "_get_product_catalog_record_lines" => self._get_product_catalog_record_lines(env, ctx, rs, args).await,
            "_update_order_line_info" => self._update_order_line_info(env, ctx, rs, args).await,
            "_is_readonly" => self._is_readonly(env, ctx, rs, args).await,
            "_get_parent_field_on_child_model" => self._get_parent_field_on_child_model(env, ctx, rs, args).await,
            "_is_line_valid_for_section_line_count" => self._is_line_valid_for_section_line_count(env, ctx, rs, args).await,
            "_is_eligible_for_early_payment_discount" => self._is_eligible_for_early_payment_discount(env, ctx, rs, args).await,
            "_early_payment_discount_move_types" => self._early_payment_discount_move_types(env, ctx, rs, args).await,
            "_synchronize_business_models" => self._synchronize_business_models(env, ctx, rs, args).await,
            "_recompute_cash_rounding_lines" => self._recompute_cash_rounding_lines(env, ctx, rs, args).await,
            "_get_automatic_balancing_account" => self._get_automatic_balancing_account(env, ctx, rs, args).await,
            "_sync_unbalanced_lines" => self._sync_unbalanced_lines(env, ctx, rs, args).await,
            "_sync_rounding_lines" => self._sync_rounding_lines(env, ctx, rs, args).await,
            "_sync_dynamic_line_needed_values" => self._sync_dynamic_line_needed_values(env, ctx, rs, args).await,
            "_sync_tax_lines" => self._sync_tax_lines(env, ctx, rs, args).await,
            "_sync_non_deductible_base_lines" => self._sync_non_deductible_base_lines(env, ctx, rs, args).await,
            "_sync_dynamic_line" => self._sync_dynamic_line(env, ctx, rs, args).await,
            "_sync_invoice" => self._sync_invoice(env, ctx, rs, args).await,
            "_get_sync_stack" => self._get_sync_stack(env, ctx, rs, args).await,
            "_sync_dynamic_lines" => self._sync_dynamic_lines(env, ctx, rs, args).await,
            "check_field_access_rights" => self.check_field_access_rights(env, ctx, rs, args).await,
            "_get_default_read_fields" => self._get_default_read_fields(env, ctx, rs, args).await,
            "read" => self.read(env, ctx, rs, args).await,
            "search_read" => self.search_read(env, ctx, rs, args).await,
            "copy_data" => self.copy_data(env, ctx, rs, args).await,
            "copy" => self.copy(env, ctx, rs, args).await,
            "_get_copy_message_content" => self._get_copy_message_content(env, ctx, rs, args).await,
            "_sanitize_vals" => self._sanitize_vals(env, ctx, rs, args).await,
            "_stolen_move" => self._stolen_move(env, ctx, rs, args).await,
            "_get_protected_vals" => self._get_protected_vals(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "check_move_sequence_chain" => self.check_move_sequence_chain(env, ctx, rs, args).await,
            "_get_unlink_logger_message" => self._get_unlink_logger_message(env, ctx, rs, args).await,
            "_unlink_forbid_parts_of_chain" => self._unlink_forbid_parts_of_chain(env, ctx, rs, args).await,
            "_unlink_account_audit_trail_except_once_post" => self._unlink_account_audit_trail_except_once_post(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "_compute_display_name" => self._compute_display_name(env, ctx, rs, args).await,
            "onchange" => self.onchange(env, ctx, rs, args).await,
            "_collect_tax_cash_basis_values" => self._collect_tax_cash_basis_values(env, ctx, rs, args).await,
            "_must_check_constrains_date_sequence" => self._must_check_constrains_date_sequence(env, ctx, rs, args).await,
            "_get_last_sequence_domain" => self._get_last_sequence_domain(env, ctx, rs, args).await,
            "_get_starting_sequence" => self._get_starting_sequence(env, ctx, rs, args).await,
            "_get_sequence_date_range" => self._get_sequence_date_range(env, ctx, rs, args).await,
            "_get_invoice_reference_euro_invoice" => self._get_invoice_reference_euro_invoice(env, ctx, rs, args).await,
            "_get_invoice_reference_euro_partner" => self._get_invoice_reference_euro_partner(env, ctx, rs, args).await,
            "_get_invoice_reference_number_invoice" => self._get_invoice_reference_number_invoice(env, ctx, rs, args).await,
            "_get_invoice_reference_number_partner" => self._get_invoice_reference_number_partner(env, ctx, rs, args).await,
            "_get_invoice_reference_odoo_invoice" => self._get_invoice_reference_odoo_invoice(env, ctx, rs, args).await,
            "_get_invoice_reference_odoo_partner" => self._get_invoice_reference_odoo_partner(env, ctx, rs, args).await,
            "_get_invoice_computed_reference" => self._get_invoice_computed_reference(env, ctx, rs, args).await,
            "_get_frequent_account_and_taxes" => self._get_frequent_account_and_taxes(env, ctx, rs, args).await,
            "_get_quick_edit_suggestions" => self._get_quick_edit_suggestions(env, ctx, rs, args).await,
            "_quick_edit_mode_suggest_invoice_date" => self._quick_edit_mode_suggest_invoice_date(env, ctx, rs, args).await,
            "_onchange_quick_edit_total_amount" => self._onchange_quick_edit_total_amount(env, ctx, rs, args).await,
            "_onchange_quick_edit_line_ids" => self._onchange_quick_edit_line_ids(env, ctx, rs, args).await,
            "_check_total_amount" => self._check_total_amount(env, ctx, rs, args).await,
            "_get_integrity_hash_fields" => self._get_integrity_hash_fields(env, ctx, rs, args).await,
            "_get_integrity_hash_fields_and_subfields" => self._get_integrity_hash_fields_and_subfields(env, ctx, rs, args).await,
            "_get_move_hash_domain" => self._get_move_hash_domain(env, ctx, rs, args).await,
            "_is_move_restricted" => self._is_move_restricted(env, ctx, rs, args).await,
            "_hash_moves" => self._hash_moves(env, ctx, rs, args).await,
            "_get_chain_info" => self._get_chain_info(env, ctx, rs, args).await,
            "_get_chains_to_hash" => self._get_chains_to_hash(env, ctx, rs, args).await,
            "_calculate_hashes" => self._calculate_hashes(env, ctx, rs, args).await,
            "_apply_delta_recurring_entries" => self._apply_delta_recurring_entries(env, ctx, rs, args).await,
            "_copy_recurring_entries" => self._copy_recurring_entries(env, ctx, rs, args).await,
            "_get_fields_to_copy_recurring_entries" => self._get_fields_to_copy_recurring_entries(env, ctx, rs, args).await,
            "_extend_with_attachments" => self._extend_with_attachments(env, ctx, rs, args).await,
            "_get_edi_creation" => self._get_edi_creation(env, ctx, rs, args).await,
            "_disable_discount_precision" => self._disable_discount_precision(env, ctx, rs, args).await,
            "_reason_cannot_decode_has_invoice_lines" => self._reason_cannot_decode_has_invoice_lines(env, ctx, rs, args).await,
            "_prepare_tax_lines_for_taxes_computation" => self._prepare_tax_lines_for_taxes_computation(env, ctx, rs, args).await,
            "_prepare_invoice_aggregated_taxes" => self._prepare_invoice_aggregated_taxes(env, ctx, rs, args).await,
            "_get_invoice_counterpart_amls_for_early_payment_discount_per_payment_term_line" => self._get_invoice_counterpart_amls_for_early_payment_discount_per_payment_term_line(env, ctx, rs, args).await,
            "_get_invoice_counterpart_amls_for_early_payment_discount" => self._get_invoice_counterpart_amls_for_early_payment_discount(env, ctx, rs, args).await,
            "_affect_tax_report" => self._affect_tax_report(env, ctx, rs, args).await,
            "_get_move_display_name" => self._get_move_display_name(env, ctx, rs, args).await,
            "_get_reconciled_amls" => self._get_reconciled_amls(env, ctx, rs, args).await,
            "_get_reconciled_payments" => self._get_reconciled_payments(env, ctx, rs, args).await,
            "_get_reconciled_statement_lines" => self._get_reconciled_statement_lines(env, ctx, rs, args).await,
            "_get_reconciled_invoices" => self._get_reconciled_invoices(env, ctx, rs, args).await,
            "_get_all_reconciled_invoice_partials" => self._get_all_reconciled_invoice_partials(env, ctx, rs, args).await,
            "_get_reconciled_invoices_partials" => self._get_reconciled_invoices_partials(env, ctx, rs, args).await,
            "_reconcile_reversed_moves" => self._reconcile_reversed_moves(env, ctx, rs, args).await,
            "_reverse_moves" => self._reverse_moves(env, ctx, rs, args).await,
            "_can_be_unlinked" => self._can_be_unlinked(env, ctx, rs, args).await,
            "_is_protected_by_audit_trail" => self._is_protected_by_audit_trail(env, ctx, rs, args).await,
            "_unlink_or_reverse" => self._unlink_or_reverse(env, ctx, rs, args).await,
            "_post" => self._post(env, ctx, rs, args).await,
            "_set_next_made_sequence_gap" => self._set_next_made_sequence_gap(env, ctx, rs, args).await,
            "_find_and_set_purchase_orders" => self._find_and_set_purchase_orders(env, ctx, rs, args).await,
            "_link_bill_origin_to_purchase_orders" => self._link_bill_origin_to_purchase_orders(env, ctx, rs, args).await,
            "_autopost_bill" => self._autopost_bill(env, ctx, rs, args).await,
            "_show_autopost_bills_wizard" => self._show_autopost_bills_wizard(env, ctx, rs, args).await,
            "open_payments" => self.open_payments(env, ctx, rs, args).await,
            "open_reconcile_view" => self.open_reconcile_view(env, ctx, rs, args).await,
            "action_open_business_doc" => self.action_open_business_doc(env, ctx, rs, args).await,
            "action_update_fpos_values" => self.action_update_fpos_values(env, ctx, rs, args).await,
            "open_created_caba_entries" => self.open_created_caba_entries(env, ctx, rs, args).await,
            "open_adjusting_entries" => self.open_adjusting_entries(env, ctx, rs, args).await,
            "open_adjusting_entry_origin_moves" => self.open_adjusting_entry_origin_moves(env, ctx, rs, args).await,
            "action_switch_move_type" => self.action_switch_move_type(env, ctx, rs, args).await,
            "get_currency_rate" => self.get_currency_rate(env, ctx, rs, args).await,
            "refresh_invoice_currency_rate" => self.refresh_invoice_currency_rate(env, ctx, rs, args).await,
            "action_register_payment" => self.action_register_payment(env, ctx, rs, args).await,
            "action_force_register_payment" => self.action_force_register_payment(env, ctx, rs, args).await,
            "action_duplicate" => self.action_duplicate(env, ctx, rs, args).await,
            "action_send_and_print" => self.action_send_and_print(env, ctx, rs, args).await,
            "action_invoice_sent" => self.action_invoice_sent(env, ctx, rs, args).await,
            "action_invoice_download_pdf" => self.action_invoice_download_pdf(env, ctx, rs, args).await,
            "action_move_download_all" => self.action_move_download_all(env, ctx, rs, args).await,
            "action_print_pdf" => self.action_print_pdf(env, ctx, rs, args).await,
            "preview_invoice" => self.preview_invoice(env, ctx, rs, args).await,
            "action_reverse" => self.action_reverse(env, ctx, rs, args).await,
            "action_post" => self.action_post(env, ctx, rs, args).await,
            "_get_moves_requiring_confirmation" => self._get_moves_requiring_confirmation(env, ctx, rs, args).await,
            "action_validate_moves_with_confirmation" => self.action_validate_moves_with_confirmation(env, ctx, rs, args).await,
            "js_assign_outstanding_line" => self.js_assign_outstanding_line(env, ctx, rs, args).await,
            "js_remove_outstanding_partial" => self.js_remove_outstanding_partial(env, ctx, rs, args).await,
            "button_set_checked" => self.button_set_checked(env, ctx, rs, args).await,
            "check_selected_moves" => self.check_selected_moves(env, ctx, rs, args).await,
            "set_moves_checked" => self.set_moves_checked(env, ctx, rs, args).await,
            "button_draft" => self.button_draft(env, ctx, rs, args).await,
            "_get_fields_to_detach" => self._get_fields_to_detach(env, ctx, rs, args).await,
            "_detach_attachments" => self._detach_attachments(env, ctx, rs, args).await,
            "_check_draftable" => self._check_draftable(env, ctx, rs, args).await,
            "button_hash" => self.button_hash(env, ctx, rs, args).await,
            "button_request_cancel" => self.button_request_cancel(env, ctx, rs, args).await,
            "button_cancel" => self.button_cancel(env, ctx, rs, args).await,
            "action_toggle_block_payment" => self.action_toggle_block_payment(env, ctx, rs, args).await,
            "action_activate_currency" => self.action_activate_currency(env, ctx, rs, args).await,
            "action_delete_duplicates" => self.action_delete_duplicates(env, ctx, rs, args).await,
            "_get_mail_template" => self._get_mail_template(env, ctx, rs, args).await,
            "_notify_get_recipients_groups" => self._notify_get_recipients_groups(env, ctx, rs, args).await,
            "_get_report_base_filename" => self._get_report_base_filename(env, ctx, rs, args).await,
            "_autopost_draft_entries" => self._autopost_draft_entries(env, ctx, rs, args).await,
            "_cron_account_move_send" => self._cron_account_move_send(env, ctx, rs, args).await,
            "_get_available_action_reports" => self._get_available_action_reports(env, ctx, rs, args).await,
            "_is_action_report_available" => self._is_action_report_available(env, ctx, rs, args).await,
            "_get_suitable_journal_ids" => self._get_suitable_journal_ids(env, ctx, rs, args).await,
            "_get_invoice_filter_type_domain" => self._get_invoice_filter_type_domain(env, ctx, rs, args).await,
            "get_invoice_types" => self.get_invoice_types(env, ctx, rs, args).await,
            "is_invoice" => self.is_invoice(env, ctx, rs, args).await,
            "is_entry" => self.is_entry(env, ctx, rs, args).await,
            "is_receipt" => self.is_receipt(env, ctx, rs, args).await,
            "get_sale_types" => self.get_sale_types(env, ctx, rs, args).await,
            "is_sale_document" => self.is_sale_document(env, ctx, rs, args).await,
            "get_purchase_types" => self.get_purchase_types(env, ctx, rs, args).await,
            "is_purchase_document" => self.is_purchase_document(env, ctx, rs, args).await,
            "get_inbound_types" => self.get_inbound_types(env, ctx, rs, args).await,
            "is_inbound" => self.is_inbound(env, ctx, rs, args).await,
            "get_outbound_types" => self.get_outbound_types(env, ctx, rs, args).await,
            "is_outbound" => self.is_outbound(env, ctx, rs, args).await,
            "_get_action_with_base_document_layout_configurator" => self._get_action_with_base_document_layout_configurator(env, ctx, rs, args).await,
            "_get_installments_data" => self._get_installments_data(env, ctx, rs, args).await,
            "_get_invoice_next_payment_values" => self._get_invoice_next_payment_values(env, ctx, rs, args).await,
            "_get_invoice_portal_extra_values" => self._get_invoice_portal_extra_values(env, ctx, rs, args).await,
            "_get_accounting_date" => self._get_accounting_date(env, ctx, rs, args).await,
            "_get_violated_lock_dates" => self._get_violated_lock_dates(env, ctx, rs, args).await,
            "_get_lock_date_message" => self._get_lock_date_message(env, ctx, rs, args).await,
            "_move_dict_to_preview_vals" => self._move_dict_to_preview_vals(env, ctx, rs, args).await,
            "_generate_qr_code" => self._generate_qr_code(env, ctx, rs, args).await,
            "_generate_portal_payment_qr" => self._generate_portal_payment_qr(env, ctx, rs, args).await,
            "_get_portal_payment_link" => self._get_portal_payment_link(env, ctx, rs, args).await,
            "_generate_and_send" => self._generate_and_send(env, ctx, rs, args).await,
            "_get_invoice_pdf_proforma" => self._get_invoice_pdf_proforma(env, ctx, rs, args).await,
            "_get_invoice_legal_documents" => self._get_invoice_legal_documents(env, ctx, rs, args).await,
            "_get_invoice_legal_documents_all" => self._get_invoice_legal_documents_all(env, ctx, rs, args).await,
            "_get_invoice_report_filename" => self._get_invoice_report_filename(env, ctx, rs, args).await,
            "_get_invoice_proforma_pdf_report_filename" => self._get_invoice_proforma_pdf_report_filename(env, ctx, rs, args).await,
            "_prepare_edi_vals_to_export" => self._prepare_edi_vals_to_export(env, ctx, rs, args).await,
            "_get_discount_allocation_account" => self._get_discount_allocation_account(env, ctx, rs, args).await,
            "_get_available_invoice_template_pdf_report_ids" => self._get_available_invoice_template_pdf_report_ids(env, ctx, rs, args).await,
            "_is_user_able_to_review" => self._is_user_able_to_review(env, ctx, rs, args).await,
            "_field_will_change" => self._field_will_change(env, ctx, rs, args).await,
            "_cleanup_write_orm_values" => self._cleanup_write_orm_values(env, ctx, rs, args).await,
            "_disable_recursion" => self._disable_recursion(env, ctx, rs, args).await,
            "_mailing_get_default_domain" => self._mailing_get_default_domain(env, ctx, rs, args).await,
            "_routing_check_route" => self._routing_check_route(env, ctx, rs, args).await,
            "message_new" => self.message_new(env, ctx, rs, args).await,
            "_attachment_fields_to_clear" => self._attachment_fields_to_clear(env, ctx, rs, args).await,
            "_message_post_after_hook" => self._message_post_after_hook(env, ctx, rs, args).await,
            "_creation_subtype" => self._creation_subtype(env, ctx, rs, args).await,
            "_track_subtype" => self._track_subtype(env, ctx, rs, args).await,
            "_creation_message" => self._creation_message(env, ctx, rs, args).await,
            "_notify_by_email_prepare_rendering_context" => self._notify_by_email_prepare_rendering_context(env, ctx, rs, args).await,
            "_get_mail_thread_data_attachments" => self._get_mail_thread_data_attachments(env, ctx, rs, args).await,
            "_conditional_add_to_compute" => self._conditional_add_to_compute(env, ctx, rs, args).await,
            "_action_invoice_ready_to_be_sent" => self._action_invoice_ready_to_be_sent(env, ctx, rs, args).await,
            "_is_ready_to_be_sent" => self._is_ready_to_be_sent(env, ctx, rs, args).await,
            "_can_force_cancel" => self._can_force_cancel(env, ctx, rs, args).await,
            "_send_only_when_ready" => self._send_only_when_ready(env, ctx, rs, args).await,
            "_invoice_paid_hook" => self._invoice_paid_hook(env, ctx, rs, args).await,
            "_get_lines_onchange_currency" => self._get_lines_onchange_currency(env, ctx, rs, args).await,
            "_get_invoice_in_payment_state" => self._get_invoice_in_payment_state(env, ctx, rs, args).await,
            "_get_name_invoice_report" => self._get_name_invoice_report(env, ctx, rs, args).await,
            "_is_downpayment" => self._is_downpayment(env, ctx, rs, args).await,
            "_refunds_origin_required" => self._refunds_origin_required(env, ctx, rs, args).await,
            "_set_reversed_entry" => self._set_reversed_entry(env, ctx, rs, args).await,
            "get_invoice_localisation_fields_required_to_invoice" => self.get_invoice_localisation_fields_required_to_invoice(env, ctx, rs, args).await,
            "get_extra_print_items" => self.get_extra_print_items(env, ctx, rs, args).await,
            "_get_move_zip_export_docs" => self._get_move_zip_export_docs(env, ctx, rs, args).await,
            "_get_move_lines_to_report" => self._get_move_lines_to_report(env, ctx, rs, args).await,
            "_can_commit" => self._can_commit(env, ctx, rs, args).await,
            "get_import_templates" => self.get_import_templates(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountMoveFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:84`). Decoradores: property.
    async fn _sequence_monthly_regex(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._sequence_monthly_regex".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:88`). Decoradores: property.
    async fn _sequence_yearly_regex(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._sequence_yearly_regex".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:92`). Decoradores: property.
    async fn _sequence_year_range_regex(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._sequence_year_range_regex".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:96`). Decoradores: property.
    async fn _sequence_fixed_regex(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._sequence_fixed_regex".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:100`). Decoradores: property.
    async fn _sequence_year_range_monthly_regex(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._sequence_year_range_monthly_regex".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:792`).
    async fn _auto_init(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._auto_init".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:802`). Decoradores: api.depends('move_type', 'partner_id').
    async fn _compute_invoice_default_sale_person(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_invoice_default_sale_person".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:818`). Decoradores: api.depends('sending_data').
    async fn _compute_is_being_sent(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_is_being_sent".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:823`). Decoradores: api.depends('is_move_sent').
    async fn compute_move_sent_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.compute_move_sent_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:827`).
    async fn _search_move_sent_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._search_move_sent_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:832`).
    async fn _compute_payment_reference(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_payment_reference".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:841`).
    async fn _get_accounting_date_source(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_accounting_date_source".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:846`). Decoradores: api.depends('invoice_date', 'company_id', 'move_type', 'taxable_supply_date').
    async fn _compute_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:863`). Decoradores: api.depends('auto_post').
    async fn _compute_auto_post_until(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_auto_post_until".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:869`). Decoradores: api.depends('date', 'auto_post').
    async fn _compute_hide_post_button(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_hide_post_button".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:876`). Decoradores: api.depends('journal_id').
    async fn _compute_company_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_company_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:882`). Decoradores: api.depends('move_type', 'origin_payment_id', 'statement_line_id').
    async fn _compute_journal_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_journal_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:886`).
    async fn _get_valid_journal_types(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_valid_journal_types".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:895`).
    async fn _search_default_journal(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._search_default_journal".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:925`). Decoradores: api.depends('move_type').
    async fn _compute_is_storno(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_is_storno".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:931`). Decoradores: api.depends('company_id', 'invoice_filter_type_domain').
    async fn _compute_suitable_journal_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_suitable_journal_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:936`). Decoradores: api.depends('posted_before', 'state', 'journal_id', 'date', 'move_type', 'origin_payment_id').
    async fn _compute_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:955`). Decoradores: api.depends('date', 'journal_id', 'move_type', 'name', 'posted_before', 'sequence_number', 'sequence_prefix', 'state').
    async fn _compute_name_placeholder(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_name_placeholder".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:965`). Decoradores: api.depends('journal_id', 'date').
    async fn _compute_highest_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_highest_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:970`). Decoradores: api.depends('journal_id', 'sequence_number', 'sequence_prefix', 'state').
    async fn _compute_made_sequence_gap(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_made_sequence_gap".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:985`). Decoradores: api.depends_context('lang'), api.depends('move_type').
    async fn _compute_type_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_type_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:996`). Decoradores: api.depends('inalterable_hash').
    async fn _compute_secured(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_secured".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1000`).
    async fn _search_secured(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._search_secured".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1007`). Decoradores: api.depends('line_ids.account_id.account_type').
    async fn _compute_always_tax_exigible(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_always_tax_exigible".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1017`). Decoradores: api.depends('partner_id').
    async fn _compute_commercial_partner_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_commercial_partner_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1022`). Decoradores: api.depends('partner_id').
    async fn _compute_partner_shipping_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_partner_shipping_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1031`). Decoradores: api.depends('partner_id', 'partner_shipping_id', 'company_id', 'move_type').
    async fn _compute_fiscal_position_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_fiscal_position_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1047`). Decoradores: api.depends('bank_partner_id', 'currency_id', 'preferred_payment_method_line_id').
    async fn _compute_partner_bank_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_partner_bank_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1074`). Decoradores: api.depends('partner_id').
    async fn _compute_invoice_payment_term_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_invoice_payment_term_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1085`). Decoradores: api.depends('needed_terms').
    async fn _compute_invoice_date_due(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_invoice_date_due".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1093`).
    async fn _compute_delivery_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_delivery_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1097`). Decoradores: api.depends('delivery_date').
    async fn _compute_show_delivery_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_show_delivery_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1101`).
    async fn _compute_taxable_supply_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_taxable_supply_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1104`).
    async fn _compute_show_taxable_supply_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_show_taxable_supply_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1108`).
    async fn _compute_taxable_supply_date_placeholder(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_taxable_supply_date_placeholder".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1113`). Decoradores: api.depends('journal_id', 'statement_line_id').
    async fn _compute_currency_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_currency_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1123`).
    async fn _get_invoice_currency_rate_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_invoice_currency_rate_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1127`).
    async fn _get_expected_currency_rate_at(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_expected_currency_rate_at".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1137`). Decoradores: api.depends('currency_id', 'company_currency_id', 'company_id', 'invoice_date', 'taxable_supply_date').
    async fn _compute_expected_currency_rate(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_expected_currency_rate".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1145`). Decoradores: api.depends('currency_id', 'company_currency_id', 'company_id', 'invoice_date', 'taxable_supply_date').
    async fn _compute_invoice_currency_rate(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_invoice_currency_rate".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1151`). Decoradores: api.depends('move_type').
    async fn _compute_direction_sign(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_direction_sign".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1173`). Decoradores: api.depends('line_ids.matched_debit_ids.debit_move_id.move_id.origin_payment_id.is_matched', 'line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual', 'line_ids.matched_debit_ids.debit_move_id.move_id.line_ids.amount_residual_currency', 'line_ids.matched_credit_ids.credit_move_id.move_id.origin_payment_id.is_matched', 'line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual', 'line_ids.matched_credit_ids.credit_move_id.move_id.line_ids.amount_residual_currency', 'line_ids.balance', 'line_ids.currency_id', 'line_ids.amount_currency', 'line_ids.amount_residual', 'line_ids.amount_residual_currency', 'line_ids.payment_id.state', 'line_ids.full_reconcile_id', 'state').
    async fn _compute_amount(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_amount".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1227`). Decoradores: api.depends('amount_residual', 'move_type', 'state', 'company_id', 'reconciled_payment_ids.state').
    async fn _compute_payment_state(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_payment_state".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1322`). Decoradores: api.depends('payment_state', 'state', 'is_move_sent').
    async fn _compute_status_in_payment(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_status_in_payment".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1336`).
    async fn _field_to_sql(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._field_to_sql".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1355`). Decoradores: api.depends('reconciled_payment_ids').
    async fn _compute_payment_count(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_payment_count".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1360`). Decoradores: api.depends('adjusting_entries_move_ids').
    async fn _compute_adjusting_entries_count(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_adjusting_entries_count".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1365`). Decoradores: api.depends('adjusting_entry_origin_move_ids').
    async fn _compute_adjusting_entry_origin_moves_count(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_adjusting_entry_origin_moves_count".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1371`). Decoradores: api.depends_context('lang'), api.depends('adjusting_entry_origin_move_ids').
    async fn _compute_adjusting_entry_origin_label(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_adjusting_entry_origin_label".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1379`). Decoradores: api.depends('invoice_payment_term_id', 'invoice_date', 'currency_id', 'amount_total_in_currency_signed', 'invoice_date_due').
    async fn _compute_needed_terms(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_needed_terms".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1450`). Decoradores: api.depends('suitable_journal_ids').
    async fn _compute_show_journal(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_show_journal".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1454`).
    async fn _compute_payments_widget_to_reconcile_info(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_payments_widget_to_reconcile_info".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1515`). Decoradores: api.depends('invoice_outstanding_credits_debits_widget').
    async fn _compute_invoice_has_outstanding(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_invoice_has_outstanding".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1520`). Decoradores: api.depends('partner_id', 'company_id').
    async fn _compute_preferred_payment_method_line_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_preferred_payment_method_line_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1529`). Decoradores: api.depends('move_type', 'line_ids.amount_residual').
    async fn _compute_payments_widget_reconciled_info(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_payments_widget_reconciled_info".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1572`).
    async fn _get_product_base_line_currency_rate(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_product_base_line_currency_rate".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1577`).
    async fn _prepare_product_base_line_for_taxes_computation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._prepare_product_base_line_for_taxes_computation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1598`).
    async fn _prepare_epd_base_line_for_taxes_computation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._prepare_epd_base_line_for_taxes_computation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1620`).
    async fn _prepare_epd_base_lines_for_taxes_computation_from_base_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._prepare_epd_base_lines_for_taxes_computation_from_base_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1653`).
    async fn _prepare_cash_rounding_base_line_for_taxes_computation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._prepare_cash_rounding_base_line_for_taxes_computation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1675`).
    async fn _prepare_tax_line_for_taxes_computation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._prepare_tax_line_for_taxes_computation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1687`).
    async fn _prepare_non_deductible_base_line_for_taxes_computation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._prepare_non_deductible_base_line_for_taxes_computation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1708`).
    async fn _prepare_non_deductible_base_lines_for_taxes_computation_from_base_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._prepare_non_deductible_base_lines_for_taxes_computation_from_base_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1757`).
    async fn _get_rounded_base_and_tax_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_rounded_base_and_tax_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1815`). Decoradores: api.depends_context('lang'), api.depends('invoice_line_ids.currency_rate', 'invoice_line_ids.tax_base_amount', 'invoice_line_ids.tax_line_id', 'invoice_line_ids.price_total', 'invoice_line_ids.price_subtotal', 'invoice_payment_term_id', 'partner_id', 'currency_id').
    async fn _compute_tax_totals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_tax_totals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1839`). Decoradores: api.depends('show_payment_term_details').
    async fn _compute_payment_term_details(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_payment_term_details".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1856`). Decoradores: api.depends('move_type', 'payment_state', 'invoice_payment_term_id').
    async fn _compute_show_payment_term_details(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_show_payment_term_details".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1871`).
    async fn _need_cancel_request(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._need_cancel_request".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1881`). Decoradores: api.depends('country_code').
    async fn _compute_need_cancel_request(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_need_cancel_request".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1886`). Decoradores: api.depends('partner_id', 'invoice_source_email', 'partner_id.display_name').
    async fn _compute_invoice_partner_display_info(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_invoice_partner_display_info".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1897`). Decoradores: api.depends('move_type').
    async fn _compute_invoice_filter_type_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_invoice_filter_type_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1902`). Decoradores: api.depends('commercial_partner_id', 'company_id', 'move_type').
    async fn _compute_bank_partner_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_bank_partner_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1911`). Decoradores: api.depends('date', 'line_ids.debit', 'line_ids.credit', 'line_ids.tax_line_id', 'line_ids.tax_ids', 'line_ids.tax_tag_ids', 'invoice_line_ids.debit', 'invoice_line_ids.credit', 'invoice_line_ids.tax_line_id', 'invoice_line_ids.tax_ids', 'invoice_line_ids.tax_tag_ids').
    async fn _compute_tax_lock_date_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_tax_lock_date_message".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1918`). Decoradores: api.depends('currency_id').
    async fn _compute_display_inactive_currency_warning(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_display_inactive_currency_warning".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1923`). Decoradores: api.depends('company_id.account_fiscal_country_id', 'fiscal_position_id', 'fiscal_position_id.country_id', 'fiscal_position_id.foreign_vat').
    async fn _compute_tax_country_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_tax_country_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1932`). Decoradores: api.depends('tax_country_id').
    async fn _compute_tax_country_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_tax_country_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1937`). Decoradores: api.depends('line_ids').
    async fn _compute_has_reconciled_entries(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_has_reconciled_entries".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1942`). Decoradores: api.depends('restrict_mode_hash_table', 'state', 'inalterable_hash').
    async fn _compute_show_reset_to_draft_button(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_show_reset_to_draft_button".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1951`).
    async fn _compute_access_url(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_access_url".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1957`). Decoradores: api.depends('move_type', 'partner_id', 'partner_id.lang', 'company_id').
    async fn _compute_narration(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_narration".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1971`).
    async fn _get_partner_credit_warning_exclude_amount(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_partner_credit_warning_exclude_amount".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1977`). Decoradores: api.depends('company_id', 'partner_id', 'tax_totals', 'currency_id').
    async fn _compute_partner_credit_warning(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_partner_credit_warning".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:1993`).
    async fn _build_credit_warning_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._build_credit_warning_message".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2042`). Decoradores: api.depends('journal_id.type', 'company_id').
    async fn _compute_quick_edit_mode(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_quick_edit_mode".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2053`). Decoradores: api.depends('quick_edit_total_amount', 'invoice_line_ids.price_total', 'tax_totals').
    async fn _compute_quick_encoding_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_quick_encoding_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2058`). Decoradores: api.depends('ref', 'move_type', 'partner_id', 'invoice_date', 'tax_totals').
    async fn _compute_duplicated_ref_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_duplicated_ref_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2064`).
    async fn _fetch_duplicate_reference(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._fetch_duplicate_reference".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2166`). Decoradores: api.depends('duplicated_ref_ids').
    async fn _compute_is_draft_duplicated_ref_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_is_draft_duplicated_ref_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2171`). Decoradores: api.depends('company_id').
    async fn _compute_display_qr_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_display_qr_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2179`). Decoradores: api.depends('company_id').
    async fn _compute_display_link_qr_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_display_link_qr_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2187`). Decoradores: api.depends('amount_total', 'currency_id').
    async fn _compute_amount_total_words(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_amount_total_words".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2192`). Decoradores: api.depends('company_id', 'move_type').
    async fn _compute_incoterm(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_incoterm".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2197`).
    async fn _compute_linked_attachment_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_linked_attachment_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2211`).
    async fn _compute_incoterm_location(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_incoterm_location".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2215`). Decoradores: api.depends('company_id.incoterm_id').
    async fn _compute_invoice_incoterm_placeholder(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_invoice_incoterm_placeholder".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2220`). Decoradores: api.depends('partner_id', 'invoice_date', 'amount_total').
    async fn _compute_abnormal_warnings(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_abnormal_warnings".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2326`). Decoradores: api.depends('state', 'invoice_line_ids', 'tax_lock_date_message', 'auto_post', 'auto_post_until', 'is_being_sent', 'partner_credit_warning', 'abnormal_amount_warning', 'abnormal_date_warning').
    async fn _compute_alerts(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_alerts".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2331`). Decoradores: api.depends('line_ids.tax_ids').
    async fn _compute_taxes_legal_notes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_taxes_legal_notes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2340`). Decoradores: api.depends('line_ids.payment_date', 'line_ids.reconciled').
    async fn _compute_next_payment_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_next_payment_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2345`). Decoradores: api.depends('move_type', 'state').
    async fn _compute_display_send_button(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_display_send_button".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2350`). Decoradores: api.depends('is_being_sent', 'invoice_pdf_report_id').
    async fn _compute_highlight_send_button(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_highlight_send_button".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2354`).
    async fn _compute_is_sale_installed(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_is_sale_installed".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2358`). Decoradores: api.depends('line_ids.matched_debit_ids', 'line_ids.matched_credit_ids', 'matched_payment_ids', 'matched_payment_ids.state').
    async fn _compute_reconciled_payment_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_reconciled_payment_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2394`).
    async fn _search_next_payment_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._search_next_payment_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2400`). Decoradores: api.depends('state', 'journal_id.type').
    async fn _compute_checked(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_checked".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2405`). Decoradores: api.depends('line_ids.no_followup').
    async fn _compute_no_followup(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_no_followup".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2415`).
    async fn _inverse_no_followup(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._inverse_no_followup".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2426`).
    async fn _get_alerts(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_alerts".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2494`).
    async fn _search_journal_group_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._search_journal_group_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2503`).
    async fn _search_reconciled_payment_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._search_reconciled_payment_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2513`).
    async fn _inverse_delivery_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._inverse_delivery_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2516`).
    async fn _inverse_tax_totals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._inverse_tax_totals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2546`).
    async fn _inverse_amount_total(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._inverse_amount_total".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2567`). Decoradores: api.onchange('partner_id').
    async fn _inverse_partner_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._inverse_partner_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2576`). Decoradores: api.onchange('company_id').
    async fn _inverse_company_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._inverse_company_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2587`). Decoradores: api.onchange('currency_id').
    async fn _inverse_currency_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._inverse_currency_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2594`). Decoradores: api.onchange('journal_id').
    async fn _inverse_journal_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._inverse_journal_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2605`). Decoradores: api.onchange('payment_reference').
    async fn _inverse_payment_reference(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._inverse_payment_reference".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2611`). Decoradores: api.onchange('invoice_payment_term_id').
    async fn _inverse_invoice_payment_term_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._inverse_invoice_payment_term_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2616`).
    async fn _inverse_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._inverse_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2627`). Decoradores: api.onchange('date').
    async fn _onchange_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._onchange_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2632`). Decoradores: api.onchange('invoice_vendor_bill_id').
    async fn _onchange_invoice_vendor_bill(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._onchange_invoice_vendor_bill".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2646`). Decoradores: api.onchange('fiscal_position_id').
    async fn _onchange_fpos_id_show_update_fpos(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._onchange_fpos_id_show_update_fpos".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2650`). Decoradores: api.onchange('partner_id').
    async fn _onchange_partner_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._onchange_partner_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2662`). Decoradores: api.onchange('name', 'highest_name').
    async fn _onchange_name_warning(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._onchange_name_warning".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2730`). Decoradores: api.onchange('journal_id').
    async fn _onchange_journal_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._onchange_journal_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2736`). Decoradores: api.onchange('invoice_cash_rounding_id').
    async fn _onchange_invoice_cash_rounding_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._onchange_invoice_cash_rounding_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2749`). Decoradores: contextmanager.
    async fn _check_balanced(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._check_balanced".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2768`).
    async fn _get_unbalanced_moves(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_unbalanced_moves".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2790`).
    async fn _check_fiscal_lock_dates(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._check_fiscal_lock_dates".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2810`). Decoradores: api.constrains('auto_post', 'invoice_date').
    async fn _require_bill_date_for_autopost(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._require_bill_date_for_autopost".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2817`). Decoradores: api.constrains('journal_id', 'move_type').
    async fn _check_journal_move_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._check_journal_move_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2825`). Decoradores: api.constrains('line_ids', 'fiscal_position_id', 'company_id').
    async fn _validate_taxes_country(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._validate_taxes_country".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2840`). Decoradores: api.constrains('invoice_currency_rate').
    async fn _check_invoice_currency_rate(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._check_invoice_currency_rate".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2855`).
    async fn action_add_from_catalog(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.action_add_from_catalog".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2860`).
    async fn _get_action_add_from_catalog_extra_context(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_action_add_from_catalog_extra_context".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2870`).
    async fn _get_product_catalog_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_product_catalog_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2879`).
    async fn _default_order_line_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._default_order_line_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2884`).
    async fn _get_product_catalog_order_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_product_catalog_order_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2890`).
    async fn _get_product_price_and_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_product_price_and_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2918`).
    async fn _get_product_catalog_record_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_product_catalog_record_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2935`).
    async fn _update_order_line_info(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._update_order_line_info".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2972`).
    async fn _is_readonly(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._is_readonly".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2979`).
    async fn _get_parent_field_on_child_model(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_parent_field_on_child_model".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2982`).
    async fn _is_line_valid_for_section_line_count(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._is_line_valid_for_section_line_count".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:2998`).
    async fn _is_eligible_for_early_payment_discount(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._is_eligible_for_early_payment_discount".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3015`).
    async fn _early_payment_discount_move_types(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._early_payment_discount_move_types".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3022`).
    async fn _synchronize_business_models(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._synchronize_business_models".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3043`).
    async fn _recompute_cash_rounding_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._recompute_cash_rounding_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3162`).
    async fn _get_automatic_balancing_account(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_automatic_balancing_account".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3170`). Decoradores: contextmanager.
    async fn _sync_unbalanced_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._sync_unbalanced_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3213`). Decoradores: contextmanager.
    async fn _sync_rounding_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._sync_rounding_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3220`). Decoradores: api.model.
    async fn _sync_dynamic_line_needed_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._sync_dynamic_line_needed_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3252`). Decoradores: contextmanager.
    async fn _sync_tax_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._sync_tax_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3456`). Decoradores: contextmanager.
    async fn _sync_non_deductible_base_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._sync_non_deductible_base_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3551`). Decoradores: contextmanager.
    async fn _sync_dynamic_line(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._sync_dynamic_line".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3649`). Decoradores: contextmanager.
    async fn _sync_invoice(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._sync_invoice".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3669`).
    async fn _get_sync_stack(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_sync_stack".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3714`). Decoradores: contextmanager.
    async fn _sync_dynamic_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._sync_dynamic_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3739`). Decoradores: api.model, api.deprecated('Override of a deprecated method').
    async fn check_field_access_rights(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.check_field_access_rights".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3747`). Decoradores: api.model.
    async fn _get_default_read_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_default_read_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3751`).
    async fn read(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.read".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3756`). Decoradores: api.model.
    async fn search_read(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.search_read".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3760`).
    async fn copy_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.copy_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3781`).
    async fn copy(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.copy".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3793`).
    async fn _get_copy_message_content(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_copy_message_content".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3801`).
    async fn _sanitize_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._sanitize_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3823`).
    async fn _stolen_move(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._stolen_move".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3830`).
    async fn _get_protected_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_protected_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3839`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3857`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3961`).
    async fn check_move_sequence_chain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.check_move_sequence_chain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3964`).
    async fn _get_unlink_logger_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_unlink_logger_message".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:3990`). Decoradores: api.ondelete().
    async fn _unlink_forbid_parts_of_chain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._unlink_forbid_parts_of_chain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4010`). Decoradores: api.ondelete().
    async fn _unlink_account_audit_trail_except_once_post(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._unlink_account_audit_trail_except_once_post".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4020`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4033`). Decoradores: api.depends('partner_id', 'date', 'state', 'move_type'), api.depends_context('input_full_display_name').
    async fn _compute_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._compute_display_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4037`).
    async fn onchange(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.onchange".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4065`).
    async fn _collect_tax_cash_basis_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._collect_tax_cash_basis_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4138`).
    async fn _must_check_constrains_date_sequence(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._must_check_constrains_date_sequence".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4142`).
    async fn _get_last_sequence_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_last_sequence_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4214`).
    async fn _get_starting_sequence(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_starting_sequence".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4254`).
    async fn _get_sequence_date_range(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_sequence_date_range".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4286`).
    async fn _get_invoice_reference_euro_invoice(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_invoice_reference_euro_invoice".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4297`).
    async fn _get_invoice_reference_euro_partner(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_invoice_reference_euro_partner".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4315`).
    async fn _get_invoice_reference_number_invoice(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_invoice_reference_number_invoice".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4322`).
    async fn _get_invoice_reference_number_partner(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_invoice_reference_number_partner".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4331`).
    async fn _get_invoice_reference_odoo_invoice(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_invoice_reference_odoo_invoice".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4339`).
    async fn _get_invoice_reference_odoo_partner(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_invoice_reference_odoo_partner".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4349`).
    async fn _get_invoice_computed_reference(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_invoice_computed_reference".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4360`). Decoradores: api.model.
    async fn _get_frequent_account_and_taxes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_frequent_account_and_taxes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4403`).
    async fn _get_quick_edit_suggestions(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_quick_edit_suggestions".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4458`). Decoradores: api.onchange('quick_edit_mode', 'journal_id', 'company_id').
    async fn _quick_edit_mode_suggest_invoice_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._quick_edit_mode_suggest_invoice_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4473`). Decoradores: api.onchange('quick_edit_total_amount', 'partner_id').
    async fn _onchange_quick_edit_total_amount(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._onchange_quick_edit_total_amount".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4496`). Decoradores: api.onchange('invoice_line_ids').
    async fn _onchange_quick_edit_line_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._onchange_quick_edit_line_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4508`).
    async fn _check_total_amount(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._check_total_amount".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4533`).
    async fn _get_integrity_hash_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_integrity_hash_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4542`).
    async fn _get_integrity_hash_fields_and_subfields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_integrity_hash_fields_and_subfields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4546`). Decoradores: api.model.
    async fn _get_move_hash_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_move_hash_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4558`). Decoradores: api.model.
    async fn _is_move_restricted(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._is_move_restricted".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4566`).
    async fn _hash_moves(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._hash_moves".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4580`).
    async fn _get_chain_info(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_chain_info".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4668`).
    async fn _get_chains_to_hash(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_chains_to_hash".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4712`).
    async fn _calculate_hashes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._calculate_hashes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4751`). Decoradores: api.model.
    async fn _apply_delta_recurring_entries(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._apply_delta_recurring_entries".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4757`).
    async fn _copy_recurring_entries(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._copy_recurring_entries".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4769`).
    async fn _get_fields_to_copy_recurring_entries(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_fields_to_copy_recurring_entries".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4791`).
    async fn _extend_with_attachments(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._extend_with_attachments".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4824`). Decoradores: contextmanager.
    async fn _get_edi_creation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_edi_creation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4840`). Decoradores: contextmanager.
    async fn _disable_discount_precision(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._disable_discount_precision".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4850`).
    async fn _reason_cannot_decode_has_invoice_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._reason_cannot_decode_has_invoice_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4859`).
    async fn _prepare_tax_lines_for_taxes_computation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._prepare_tax_lines_for_taxes_computation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4864`).
    async fn _prepare_invoice_aggregated_taxes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._prepare_invoice_aggregated_taxes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:4971`).
    async fn _get_invoice_counterpart_amls_for_early_payment_discount_per_payment_term_line(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_invoice_counterpart_amls_for_early_payment_discount_per_payment_term_line".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5148`). Decoradores: api.model.
    async fn _get_invoice_counterpart_amls_for_early_payment_discount(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_invoice_counterpart_amls_for_early_payment_discount".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5225`).
    async fn _affect_tax_report(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._affect_tax_report".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5228`).
    async fn _get_move_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_move_display_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5263`).
    async fn _get_reconciled_amls(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_reconciled_amls".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5268`).
    async fn _get_reconciled_payments(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_reconciled_payments".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5272`).
    async fn _get_reconciled_statement_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_reconciled_statement_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5276`).
    async fn _get_reconciled_invoices(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_reconciled_invoices".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5280`).
    async fn _get_all_reconciled_invoice_partials(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_all_reconciled_invoice_partials".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5361`).
    async fn _get_reconciled_invoices_partials(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_reconciled_invoices_partials".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5381`).
    async fn _reconcile_reversed_moves(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._reconcile_reversed_moves".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5402`).
    async fn _reverse_moves(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._reverse_moves".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5448`).
    async fn _can_be_unlinked(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._can_be_unlinked".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5455`).
    async fn _is_protected_by_audit_trail(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._is_protected_by_audit_trail".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5458`).
    async fn _unlink_or_reverse(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._unlink_or_reverse".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5476`).
    async fn _post(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._post".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5703`).
    async fn _set_next_made_sequence_gap(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._set_next_made_sequence_gap".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5721`).
    async fn _find_and_set_purchase_orders(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._find_and_set_purchase_orders".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5725`).
    async fn _link_bill_origin_to_purchase_orders(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._link_bill_origin_to_purchase_orders".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5731`).
    async fn _autopost_bill(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._autopost_bill".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5747`).
    async fn _show_autopost_bills_wizard(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._show_autopost_bills_wizard".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5790`).
    async fn open_payments(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.open_payments".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5794`).
    async fn open_reconcile_view(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.open_reconcile_view".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5797`).
    async fn action_open_business_doc(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.action_open_business_doc".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5822`).
    async fn action_update_fpos_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.action_update_fpos_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5842`).
    async fn open_created_caba_entries(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.open_created_caba_entries".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5853`).
    async fn open_adjusting_entries(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.open_adjusting_entries".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5857`).
    async fn open_adjusting_entry_origin_moves(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.open_adjusting_entry_origin_moves".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5862`).
    async fn action_switch_move_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.action_switch_move_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5888`).
    async fn get_currency_rate(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.get_currency_rate".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5899`).
    async fn refresh_invoice_currency_rate(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.refresh_invoice_currency_rate".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5903`).
    async fn action_register_payment(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.action_register_payment".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5908`).
    async fn action_force_register_payment(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.action_force_register_payment".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5913`).
    async fn action_duplicate(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.action_duplicate".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5923`).
    async fn action_send_and_print(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.action_send_and_print".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5937`).
    async fn action_invoice_sent(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.action_invoice_sent".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5946`).
    async fn action_invoice_download_pdf(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.action_invoice_download_pdf".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5953`).
    async fn action_move_download_all(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.action_move_download_all".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5960`).
    async fn action_print_pdf(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.action_print_pdf".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5966`).
    async fn preview_invoice(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.preview_invoice".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5974`).
    async fn action_reverse(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.action_reverse".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:5982`).
    async fn action_post(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.action_post".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6005`).
    async fn _get_moves_requiring_confirmation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_moves_requiring_confirmation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6012`).
    async fn action_validate_moves_with_confirmation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.action_validate_moves_with_confirmation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6041`).
    async fn js_assign_outstanding_line(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.js_assign_outstanding_line".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6052`).
    async fn js_remove_outstanding_partial(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.js_remove_outstanding_partial".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6061`).
    async fn button_set_checked(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.button_set_checked".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6064`).
    async fn check_selected_moves(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.check_selected_moves".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6067`).
    async fn set_moves_checked(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.set_moves_checked".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6071`).
    async fn button_draft(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.button_draft".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6085`).
    async fn _get_fields_to_detach(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_fields_to_detach".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6092`).
    async fn _detach_attachments(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._detach_attachments".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6114`).
    async fn _check_draftable(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._check_draftable".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6141`).
    async fn button_hash(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.button_hash".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6144`).
    async fn button_request_cancel(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.button_request_cancel".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6150`).
    async fn button_cancel(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.button_cancel".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6164`).
    async fn action_toggle_block_payment(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.action_toggle_block_payment".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6174`).
    async fn action_activate_currency(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.action_activate_currency".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6177`).
    async fn action_delete_duplicates(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.action_delete_duplicates".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6181`).
    async fn _get_mail_template(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_mail_template".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6194`).
    async fn _notify_get_recipients_groups(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._notify_get_recipients_groups".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6219`).
    async fn _get_report_base_filename(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_report_base_filename".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6226`).
    async fn _autopost_draft_entries(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._autopost_draft_entries".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6261`). Decoradores: api.model.
    async fn _cron_account_move_send(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._cron_account_move_send".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6286`).
    async fn _get_available_action_reports(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_available_action_reports".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6300`).
    async fn _is_action_report_available(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._is_action_report_available".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6311`). Decoradores: api.model.
    async fn _get_suitable_journal_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_suitable_journal_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6320`). Decoradores: api.model.
    async fn _get_invoice_filter_type_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_invoice_filter_type_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6329`). Decoradores: api.model.
    async fn get_invoice_types(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.get_invoice_types".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6332`).
    async fn is_invoice(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.is_invoice".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6335`).
    async fn is_entry(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.is_entry".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6338`).
    async fn is_receipt(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.is_receipt".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6342`). Decoradores: api.model.
    async fn get_sale_types(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.get_sale_types".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6345`).
    async fn is_sale_document(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.is_sale_document".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6349`). Decoradores: api.model.
    async fn get_purchase_types(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.get_purchase_types".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6352`).
    async fn is_purchase_document(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.is_purchase_document".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6356`). Decoradores: api.model.
    async fn get_inbound_types(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.get_inbound_types".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6359`).
    async fn is_inbound(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.is_inbound".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6363`). Decoradores: api.model.
    async fn get_outbound_types(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.get_outbound_types".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6366`).
    async fn is_outbound(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.is_outbound".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6369`).
    async fn _get_action_with_base_document_layout_configurator(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_action_with_base_document_layout_configurator".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6382`).
    async fn _get_installments_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_installments_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6387`).
    async fn _get_invoice_next_payment_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_invoice_next_payment_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6471`).
    async fn _get_invoice_portal_extra_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_invoice_portal_extra_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6479`).
    async fn _get_accounting_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_accounting_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6518`).
    async fn _get_violated_lock_dates(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_violated_lock_dates".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6527`).
    async fn _get_lock_date_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_lock_date_message".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6546`). Decoradores: api.model.
    async fn _move_dict_to_preview_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._move_dict_to_preview_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6560`).
    async fn _generate_qr_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._generate_qr_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6602`).
    async fn _generate_portal_payment_qr(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._generate_portal_payment_qr".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6609`).
    async fn _get_portal_payment_link(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_portal_payment_link".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6616`).
    async fn _generate_and_send(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._generate_and_send".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6642`).
    async fn _get_invoice_pdf_proforma(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_invoice_pdf_proforma".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6657`).
    async fn _get_invoice_legal_documents(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_invoice_legal_documents".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6676`).
    async fn _get_invoice_legal_documents_all(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_invoice_legal_documents_all".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6696`).
    async fn _get_invoice_report_filename(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_invoice_report_filename".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6707`).
    async fn _get_invoice_proforma_pdf_report_filename(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_invoice_proforma_pdf_report_filename".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6712`).
    async fn _prepare_edi_vals_to_export(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._prepare_edi_vals_to_export".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6741`).
    async fn _get_discount_allocation_account(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_discount_allocation_account".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6748`).
    async fn _get_available_invoice_template_pdf_report_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_available_invoice_template_pdf_report_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6764`).
    async fn _is_user_able_to_review(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._is_user_able_to_review".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6773`). Decoradores: api.model.
    async fn _field_will_change(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._field_will_change".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6794`). Decoradores: api.model.
    async fn _cleanup_write_orm_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._cleanup_write_orm_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6802`). Decoradores: contextmanager.
    async fn _disable_recursion(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._disable_recursion".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6834`).
    async fn _mailing_get_default_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._mailing_get_default_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6838`). Decoradores: api.model.
    async fn _routing_check_route(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._routing_check_route".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6857`). Decoradores: api.model.
    async fn message_new(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.message_new".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6915`).
    async fn _attachment_fields_to_clear(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._attachment_fields_to_clear".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6918`).
    async fn _message_post_after_hook(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._message_post_after_hook".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:6998`).
    async fn _creation_subtype(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._creation_subtype".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7005`).
    async fn _track_subtype(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._track_subtype".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7021`).
    async fn _creation_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._creation_message".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7034`).
    async fn _notify_by_email_prepare_rendering_context(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._notify_by_email_prepare_rendering_context".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7058`).
    async fn _get_mail_thread_data_attachments(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_mail_thread_data_attachments".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7067`).
    async fn _conditional_add_to_compute(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._conditional_add_to_compute".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7081`).
    async fn _action_invoice_ready_to_be_sent(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._action_invoice_ready_to_be_sent".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7086`).
    async fn _is_ready_to_be_sent(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._is_ready_to_be_sent".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7094`).
    async fn _can_force_cancel(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._can_force_cancel".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7102`). Decoradores: contextmanager.
    async fn _send_only_when_ready(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._send_only_when_ready".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7112`).
    async fn _invoice_paid_hook(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._invoice_paid_hook".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7115`).
    async fn _get_lines_onchange_currency(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_lines_onchange_currency".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7120`). Decoradores: api.model.
    async fn _get_invoice_in_payment_state(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_invoice_in_payment_state".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7126`).
    async fn _get_name_invoice_report(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_name_invoice_report".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7132`).
    async fn _is_downpayment(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._is_downpayment".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7138`).
    async fn _refunds_origin_required(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._refunds_origin_required".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7141`).
    async fn _set_reversed_entry(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._set_reversed_entry".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7152`). Decoradores: api.model.
    async fn get_invoice_localisation_fields_required_to_invoice(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.get_invoice_localisation_fields_required_to_invoice".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7163`).
    async fn get_extra_print_items(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.get_extra_print_items".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7176`).
    async fn _get_move_zip_export_docs(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_move_zip_export_docs".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7192`).
    async fn _get_move_lines_to_report(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._get_move_lines_to_report".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7205`). Decoradores: staticmethod.
    async fn _can_commit(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move._can_commit".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move.py:7213`). Decoradores: api.model.
    async fn get_import_templates(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.get_import_templates".into(),
        ))
    }

}
