//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.move.line`

use nexus_orm::prelude::*;

pub struct AccountMoveLineFragment;

#[async_trait]
impl ModelFragment for AccountMoveLineFragment {
    fn model_name(&self) -> &str {
        "account.move.line"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Journal Item".into();
        def.order = "date desc, move_name desc, id".into();
        def.add_field(FieldDef::many2one("move_id", "account.move").string("Journal Entry").required().readonly());
        // TODO(odoo2rs): campo 'journal_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field({ let mut f = FieldDef::many2one("journal_group_id", "account.journal.group").string("Ledger"); f.store = false; f });
        // TODO(odoo2rs): campo 'company_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        // TODO(odoo2rs): campo 'company_currency_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field({ let mut f = FieldDef::char("move_name").string("Number"); f.related = Some("move_id.name".into()); f });
        def.add_field({ let mut f = FieldDef::selection("parent_state", &[]); f.related = Some("move_id.state".into()); f });
        def.add_field({ let mut f = FieldDef::date("date"); f.related = Some("move_id.date".into()); f });
        def.add_field({ let mut f = FieldDef::date("invoice_date"); f.related = Some("move_id.invoice_date".into()); f });
        def.add_field({ let mut f = FieldDef::char("ref"); f.related = Some("move_id.ref".into()); f });
        def.add_field(FieldDef::boolean("is_storno").string("Company Storno Accounting").computed("_compute_is_storno", &["move_id.is_storno", "price_unit", "quantity"]).stored());
        def.add_field(FieldDef::integer("sequence").computed("_compute_sequence", &["display_type"]).stored());
        def.add_field({ let mut f = FieldDef::selection("move_type", &[]); f.related = Some("move_id.move_type".into()); f });
        def.add_field(FieldDef::many2one("account_id", "account.account").string("Account").computed("_compute_account_id", &[]).stored());
        def.add_field({ let mut f = FieldDef::char("account_name"); f.related = Some("account_id.name".into()); f });
        def.add_field({ let mut f = FieldDef::char("account_code"); f.related = Some("account_id.code".into()); f });
        def.add_field({ let mut f = FieldDef::many2one("search_account_id", "account.account"); f.store = false; f });
        def.add_field(FieldDef::char("name").string("Label").computed("_compute_name", &["product_id", "move_id.ref", "move_id.payment_reference"]).stored());
        def.add_field(FieldDef::monetary("debit").string("Debit").computed("_compute_debit_credit", &["balance"]).stored());
        def.add_field(FieldDef::monetary("credit").string("Credit").computed("_compute_debit_credit", &["balance"]).stored());
        def.add_field(FieldDef::monetary("balance").string("Balance").computed("_compute_balance", &["move_id"]).stored());
        def.add_field(FieldDef::monetary("cumulated_balance").string("Cumulated Balance").computed("_compute_cumulated_balance", &[]).stored());
        def.add_field(FieldDef::float("currency_rate").computed("_compute_currency_rate", &["currency_id", "company_id", "move_id.invoice_currency_rate", "move_id.date"]).stored());
        def.add_field(FieldDef::monetary("amount_currency").string("Amount in Currency").computed("_compute_amount_currency", &["currency_rate", "balance"]).stored());
        def.add_field(FieldDef::many2one("currency_id", "res.currency").string("Currency").required().computed("_compute_currency_id", &["move_id.currency_id"]).stored());
        def.add_field(FieldDef::boolean("is_same_currency").computed("_compute_same_currency", &["currency_id", "company_currency_id"]).stored());
        def.add_field(FieldDef::many2one("partner_id", "res.partner").string("Partner").computed("_compute_partner_id", &[]).stored());
        def.add_field(FieldDef::boolean("is_imported"));
        def.add_field(FieldDef::many2one("reconcile_model_id", "account.reconcile.model").string("Reconciliation Model").readonly());
        def.add_field({ let mut f = FieldDef::many2one("payment_id", "account.payment").string("Originator Payment"); f.related = Some("move_id.origin_payment_id".into()); f });
        def.add_field({ let mut f = FieldDef::many2one("statement_line_id", "account.bank.statement.line").string("Originator Statement Line"); f.related = Some("move_id.statement_line_id".into()); f });
        // TODO(odoo2rs): campo 'statement_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        // TODO(odoo2rs): campo 'commercial_partner_country' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::many2many("tax_ids", "account.tax").string("Taxes").computed("_compute_tax_ids", &["product_id", "product_uom_id"]).stored());
        def.add_field(FieldDef::many2one("group_tax_id", "account.tax").string("Originator Group of Taxes"));
        def.add_field({ let mut f = FieldDef::many2one("tax_line_id", "account.tax").string("Originator Tax"); f.related = Some("tax_repartition_line_id.tax_id".into()); f });
        // TODO(odoo2rs): campo 'tax_group_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::monetary("tax_base_amount").string("Base Amount").readonly());
        def.add_field(FieldDef::many2one("tax_repartition_line_id", "account.tax.repartition.line").string("Originator Tax Distribution Line").readonly());
        def.add_field(FieldDef::many2many("tax_tag_ids", "account.account.tag").string("Tags"));
        def.add_field(FieldDef::json("extra_tax_data"));
        def.add_field(FieldDef::monetary("amount_residual").string("Residual Amount").computed("_compute_amount_residual", &["debit", "credit", "amount_currency", "account_id", "currency_id", "company_id", "matched_debit_ids", "matched_credit_ids"]).stored());
        def.add_field(FieldDef::monetary("amount_residual_currency").string("Residual Amount in Currency").computed("_compute_amount_residual", &["debit", "credit", "amount_currency", "account_id", "currency_id", "company_id", "matched_debit_ids", "matched_credit_ids"]).stored());
        def.add_field(FieldDef::boolean("reconciled").computed("_compute_amount_residual", &["debit", "credit", "amount_currency", "account_id", "currency_id", "company_id", "matched_debit_ids", "matched_credit_ids"]).stored());
        def.add_field(FieldDef::many2one("full_reconcile_id", "account.full.reconcile").string("Matching").readonly());
        def.add_field(FieldDef::one2many("matched_debit_ids", "account.partial.reconcile", "credit_move_id").string("Matched Debits").readonly());
        def.add_field(FieldDef::one2many("matched_credit_ids", "account.partial.reconcile", "debit_move_id").string("Matched Credits").readonly());
        def.add_field(FieldDef::many2many("reconciled_lines_ids", "account.move.line").computed("_compute_reconciled_lines_ids", &["matched_debit_ids", "matched_credit_ids"]).stored());
        def.add_field(FieldDef::many2many("reconciled_lines_excluding_exchange_diff_ids", "account.move.line").computed("_compute_reconciled_lines_excluding_exchange_diff_ids", &["matched_debit_ids", "matched_credit_ids"]).stored());
        def.add_field(FieldDef::char("matching_number").string("Matching #"));
        def.add_field({ let mut f = FieldDef::boolean("is_account_reconcile").string("Account Reconcile"); f.related = Some("account_id.reconcile".into()); f });
        def.add_field({ let mut f = FieldDef::selection("account_type", &[]).string("Internal Type"); f.related = Some("account_id.account_type".into()); f });
        def.add_field({ let mut f = FieldDef::selection("account_internal_group", &[]); f.related = Some("account_id.internal_group".into()); f });
        // TODO(odoo2rs): campo 'account_root_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        // TODO(odoo2rs): campo 'product_category_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::selection("display_type", &[("product", "Product"), ("cogs", "Cost of Goods Sold"), ("tax", "Tax"), ("discount", "Discount"), ("rounding", "Rounding"), ("payment_term", "Payment Term"), ("line_section", "Section"), ("line_subsection", "Subsection"), ("line_note", "Note"), ("epd", "Early Payment Discount"), ("non_deductible_product_total", "Non Deductible Products Total"), ("non_deductible_product", "Non Deductible Products"), ("non_deductible_tax", "Non Deductible Tax")]).required().computed("_compute_display_type", &["move_id"]).stored());
        def.add_field(FieldDef::boolean("collapse_composition").string("Hide Composition"));
        def.add_field(FieldDef::boolean("collapse_prices").string("Hide Prices"));
        def.add_field(FieldDef::many2one("parent_id", "account.move.line").string("Parent Section Line").computed("_compute_parent_id", &[]).stored());
        def.add_field(FieldDef::many2one("product_id", "product.product").string("Product"));
        def.add_field(FieldDef::many2many("allowed_uom_ids", "uom.uom").computed("_compute_allowed_uom_ids", &["product_id", "product_id.uom_id", "product_id.uom_ids"]).stored());
        def.add_field(FieldDef::many2one("product_uom_id", "uom.uom").string("Unit").computed("_compute_product_uom_id", &["product_id"]).stored());
        def.add_field(FieldDef::float("quantity").string("Quantity").computed("_compute_quantity", &["display_type"]).stored());
        def.add_field(FieldDef::date("date_maturity").string("Due Date"));
        def.add_field(FieldDef::float("price_unit").string("Unit Price").computed("_compute_price_unit", &["product_id", "product_uom_id"]).stored());
        def.add_field(FieldDef::monetary("price_subtotal").string("Subtotal").computed("_compute_totals", &["quantity", "discount", "price_unit", "tax_ids", "currency_id"]).stored());
        def.add_field(FieldDef::monetary("price_total").string("Total").computed("_compute_totals", &["quantity", "discount", "price_unit", "tax_ids", "currency_id"]).stored());
        def.add_field(FieldDef::float("discount").string("Discount (%)").default_val(0f64));
        def.add_field({ let mut f = FieldDef::selection("tax_calculation_rounding_method", &[]).string("Tax calculation rounding method").readonly(); f.related = Some("company_id.tax_calculation_rounding_method".into()); f });
        def.add_field(FieldDef::float("deductible_amount").string("Deductibility").default_val(100i64));
        def.add_field(FieldDef::new("term_key", FieldType::Binary).computed("_compute_term_key", &["date_maturity"]).stored());
        def.add_field(FieldDef::new("epd_key", FieldType::Binary).computed("_compute_epd_key", &["tax_ids", "account_id", "company_id"]).stored());
        def.add_field(FieldDef::new("epd_needed", FieldType::Binary).computed("_compute_epd_needed", &["move_id.needed_terms", "account_id", "analytic_distribution", "tax_ids", "tax_tag_ids", "company_id", "price_subtotal"]).stored());
        def.add_field(FieldDef::boolean("epd_dirty").computed("_compute_epd_needed", &["move_id.needed_terms", "account_id", "analytic_distribution", "tax_ids", "tax_tag_ids", "company_id", "price_subtotal"]).stored());
        def.add_field(FieldDef::new("discount_allocation_key", FieldType::Binary).computed("_compute_discount_allocation_key", &["account_id", "company_id"]).stored());
        def.add_field(FieldDef::new("discount_allocation_needed", FieldType::Binary).computed("_compute_discount_allocation_needed", &["account_id", "company_id", "discount", "price_unit", "quantity", "currency_rate", "analytic_distribution"]).stored());
        def.add_field(FieldDef::boolean("discount_allocation_dirty").computed("_compute_discount_allocation_needed", &["account_id", "company_id", "discount", "price_unit", "quantity", "currency_rate", "analytic_distribution"]).stored());
        def.add_field(FieldDef::one2many("analytic_line_ids", "account.analytic.line", "move_line_id").string("Analytic lines"));
        def.add_field(FieldDef::json("analytic_distribution"));
        def.add_field(FieldDef::boolean("has_invalid_analytics").computed("_compute_has_invalid_analytics", &["account_id", "company_id", "move_id", "product_id", "display_type", "analytic_distribution"]).stored());
        def.add_field(FieldDef::date("discount_date").string("Discount Date").readonly());
        def.add_field(FieldDef::monetary("discount_amount_currency").string("Discount amount in Currency"));
        def.add_field(FieldDef::monetary("discount_balance").string("Discount Balance"));
        def.add_field(FieldDef::date("payment_date").string("Next Payment Date").computed("_compute_payment_date", &["discount_date", "date_maturity"]).stored());
        def.add_field(FieldDef::boolean("is_refund").computed("_compute_is_refund", &["move_id.move_type", "balance", "tax_repartition_line_id", "tax_ids"]).stored());
        def.add_field(FieldDef::boolean("no_followup").string("No Follow-Up").computed("_compute_no_followup", &["journal_id.type"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["get_views", "_compute_display_type", "_compute_partner_id", "_compute_currency_id", "_compute_name", "_compute_account_id", "_search_account_id", "_compute_is_storno", "_compute_balance", "_compute_debit_credit", "_compute_currency_rate", "_compute_same_currency", "_compute_amount_currency", "_compute_cumulated_balance", "_compute_amount_residual", "_compute_allowed_uom_ids", "_compute_product_uom_id", "_compute_quantity", "_compute_sequence", "_compute_totals", "_compute_price_unit", "_compute_tax_ids", "_get_computed_taxes", "_compute_discount_allocation_key", "_compute_discount_allocation_needed", "_compute_epd_key", "_compute_epd_needed", "_compute_is_refund", "_compute_term_key", "_compute_analytic_distribution", "_get_analytic_distribution_arguments", "_compute_payment_date", "_compute_reconciled_lines_ids", "_compute_reconciled_lines_excluding_exchange_diff_ids", "_compute_parent_id", "_compute_no_followup", "_inverse_no_followup", "_search_payment_date", "action_payment_items_register_payment", "action_register_payment", "_search_journal_group_id", "_inverse_partner_id", "_inverse_product_id", "_inverse_amount_currency", "_inverse_debit", "_inverse_credit", "_inverse_analytic_distribution", "_inverse_account_id", "_inverse_reconciled_lines_ids", "_check_constrains_account_id_journal_id", "_check_off_balance", "_check_payable_receivable", "_affect_tax_report", "_check_tax_lock_date", "_check_reconciliation", "_check_caba_non_caba_shared_tags", "_constrains_matching_number", "_constrains_deductible_amount", "check_field_access_rights", "_get_default_read_fields", "read", "search_read", "invalidate_model", "invalidate_recordset", "search_fetch", "default_get", "_sanitize_vals", "_prepare_create_values", "_sync_invoice", "create", "write", "_parse_flush_fnames", "flush_recordset", "flush_model", "_valid_field_parameter", "_unlink_except_posted", "_prevent_automatic_line_deletion", "_except_hashed_entry_lines", "unlink", "_format_aml_name", "_compute_display_name", "_compute_has_invalid_analytics", "copy_data", "_field_to_sql", "_search_panel_domain_image", "_get_reconciliation_aml_field_value", "_prepare_move_line_residual_amounts", "_prepare_reconciliation_single_partial", "_prepare_reconciliation_amls", "_prepare_reconciliation_plan", "_check_amls_exigibility_for_reconciliation", "_optimize_reconciliation_plan", "_reconcile_pre_hook", "_reconcile_post_hook", "_reconcile_plan", "_reconcile_plan_with_sync", "_get_exchange_journal", "_get_exchange_account", "_prepare_exchange_difference_move_vals", "_create_exchange_difference_moves", "reconcile", "remove_move_reconcile", "action_unreconcile_match_entries", "_reconcile_marked", "_get_matched_move_ids", "_validate_analytic_distribution", "_create_analytic_lines", "_prepare_analytic_lines", "_prepare_analytic_distribution_line", "_related_analytic_distribution", "_update_analytic_distribution", "_round_analytic_distribution_line", "_get_installments_data", "_get_integrity_hash_fields", "_reconciled_lines", "_reconciled_by_number", "_filter_reconciled_by_number", "_all_reconciled_lines", "_get_attachment_domains", "_get_attachment_by_record", "_get_tax_exigible_domain", "_get_invoiced_qty_per_product", "_get_lock_date_protected_fields", "get_import_templates", "_prepare_edi_vals_to_export", "_get_journal_items_full_name", "_check_edi_line_tax_required", "_get_aml_values", "_filter_aml_lot_valuation", "_get_child_lines", "get_section_subtotal", "get_column_to_exclude_for_colspan_calculation", "get_parent_section_line", "_get_section_lines", "_is_line_in_section", "open_reconcile_view", "action_open_business_doc", "action_automatic_entry", "action_add_from_catalog", "_get_product_catalog_lines_data", "_conditional_add_to_compute", "_copy_data_extend_business_fields", "_get_downpayment_lines"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "get_views" => self.get_views(env, ctx, rs, args).await,
            "_compute_display_type" => self._compute_display_type(env, ctx, rs, args).await,
            "_compute_partner_id" => self._compute_partner_id(env, ctx, rs, args).await,
            "_compute_currency_id" => self._compute_currency_id(env, ctx, rs, args).await,
            "_compute_name" => self._compute_name(env, ctx, rs, args).await,
            "_compute_account_id" => self._compute_account_id(env, ctx, rs, args).await,
            "_search_account_id" => self._search_account_id(env, ctx, rs, args).await,
            "_compute_is_storno" => self._compute_is_storno(env, ctx, rs, args).await,
            "_compute_balance" => self._compute_balance(env, ctx, rs, args).await,
            "_compute_debit_credit" => self._compute_debit_credit(env, ctx, rs, args).await,
            "_compute_currency_rate" => self._compute_currency_rate(env, ctx, rs, args).await,
            "_compute_same_currency" => self._compute_same_currency(env, ctx, rs, args).await,
            "_compute_amount_currency" => self._compute_amount_currency(env, ctx, rs, args).await,
            "_compute_cumulated_balance" => self._compute_cumulated_balance(env, ctx, rs, args).await,
            "_compute_amount_residual" => self._compute_amount_residual(env, ctx, rs, args).await,
            "_compute_allowed_uom_ids" => self._compute_allowed_uom_ids(env, ctx, rs, args).await,
            "_compute_product_uom_id" => self._compute_product_uom_id(env, ctx, rs, args).await,
            "_compute_quantity" => self._compute_quantity(env, ctx, rs, args).await,
            "_compute_sequence" => self._compute_sequence(env, ctx, rs, args).await,
            "_compute_totals" => self._compute_totals(env, ctx, rs, args).await,
            "_compute_price_unit" => self._compute_price_unit(env, ctx, rs, args).await,
            "_compute_tax_ids" => self._compute_tax_ids(env, ctx, rs, args).await,
            "_get_computed_taxes" => self._get_computed_taxes(env, ctx, rs, args).await,
            "_compute_discount_allocation_key" => self._compute_discount_allocation_key(env, ctx, rs, args).await,
            "_compute_discount_allocation_needed" => self._compute_discount_allocation_needed(env, ctx, rs, args).await,
            "_compute_epd_key" => self._compute_epd_key(env, ctx, rs, args).await,
            "_compute_epd_needed" => self._compute_epd_needed(env, ctx, rs, args).await,
            "_compute_is_refund" => self._compute_is_refund(env, ctx, rs, args).await,
            "_compute_term_key" => self._compute_term_key(env, ctx, rs, args).await,
            "_compute_analytic_distribution" => self._compute_analytic_distribution(env, ctx, rs, args).await,
            "_get_analytic_distribution_arguments" => self._get_analytic_distribution_arguments(env, ctx, rs, args).await,
            "_compute_payment_date" => self._compute_payment_date(env, ctx, rs, args).await,
            "_compute_reconciled_lines_ids" => self._compute_reconciled_lines_ids(env, ctx, rs, args).await,
            "_compute_reconciled_lines_excluding_exchange_diff_ids" => self._compute_reconciled_lines_excluding_exchange_diff_ids(env, ctx, rs, args).await,
            "_compute_parent_id" => self._compute_parent_id(env, ctx, rs, args).await,
            "_compute_no_followup" => self._compute_no_followup(env, ctx, rs, args).await,
            "_inverse_no_followup" => self._inverse_no_followup(env, ctx, rs, args).await,
            "_search_payment_date" => self._search_payment_date(env, ctx, rs, args).await,
            "action_payment_items_register_payment" => self.action_payment_items_register_payment(env, ctx, rs, args).await,
            "action_register_payment" => self.action_register_payment(env, ctx, rs, args).await,
            "_search_journal_group_id" => self._search_journal_group_id(env, ctx, rs, args).await,
            "_inverse_partner_id" => self._inverse_partner_id(env, ctx, rs, args).await,
            "_inverse_product_id" => self._inverse_product_id(env, ctx, rs, args).await,
            "_inverse_amount_currency" => self._inverse_amount_currency(env, ctx, rs, args).await,
            "_inverse_debit" => self._inverse_debit(env, ctx, rs, args).await,
            "_inverse_credit" => self._inverse_credit(env, ctx, rs, args).await,
            "_inverse_analytic_distribution" => self._inverse_analytic_distribution(env, ctx, rs, args).await,
            "_inverse_account_id" => self._inverse_account_id(env, ctx, rs, args).await,
            "_inverse_reconciled_lines_ids" => self._inverse_reconciled_lines_ids(env, ctx, rs, args).await,
            "_check_constrains_account_id_journal_id" => self._check_constrains_account_id_journal_id(env, ctx, rs, args).await,
            "_check_off_balance" => self._check_off_balance(env, ctx, rs, args).await,
            "_check_payable_receivable" => self._check_payable_receivable(env, ctx, rs, args).await,
            "_affect_tax_report" => self._affect_tax_report(env, ctx, rs, args).await,
            "_check_tax_lock_date" => self._check_tax_lock_date(env, ctx, rs, args).await,
            "_check_reconciliation" => self._check_reconciliation(env, ctx, rs, args).await,
            "_check_caba_non_caba_shared_tags" => self._check_caba_non_caba_shared_tags(env, ctx, rs, args).await,
            "_constrains_matching_number" => self._constrains_matching_number(env, ctx, rs, args).await,
            "_constrains_deductible_amount" => self._constrains_deductible_amount(env, ctx, rs, args).await,
            "check_field_access_rights" => self.check_field_access_rights(env, ctx, rs, args).await,
            "_get_default_read_fields" => self._get_default_read_fields(env, ctx, rs, args).await,
            "read" => self.read(env, ctx, rs, args).await,
            "search_read" => self.search_read(env, ctx, rs, args).await,
            "invalidate_model" => self.invalidate_model(env, ctx, rs, args).await,
            "invalidate_recordset" => self.invalidate_recordset(env, ctx, rs, args).await,
            "search_fetch" => self.search_fetch(env, ctx, rs, args).await,
            "default_get" => self.default_get(env, ctx, rs, args).await,
            "_sanitize_vals" => self._sanitize_vals(env, ctx, rs, args).await,
            "_prepare_create_values" => self._prepare_create_values(env, ctx, rs, args).await,
            "_sync_invoice" => self._sync_invoice(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "_parse_flush_fnames" => self._parse_flush_fnames(env, ctx, rs, args).await,
            "flush_recordset" => self.flush_recordset(env, ctx, rs, args).await,
            "flush_model" => self.flush_model(env, ctx, rs, args).await,
            "_valid_field_parameter" => self._valid_field_parameter(env, ctx, rs, args).await,
            "_unlink_except_posted" => self._unlink_except_posted(env, ctx, rs, args).await,
            "_prevent_automatic_line_deletion" => self._prevent_automatic_line_deletion(env, ctx, rs, args).await,
            "_except_hashed_entry_lines" => self._except_hashed_entry_lines(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "_format_aml_name" => self._format_aml_name(env, ctx, rs, args).await,
            "_compute_display_name" => self._compute_display_name(env, ctx, rs, args).await,
            "_compute_has_invalid_analytics" => self._compute_has_invalid_analytics(env, ctx, rs, args).await,
            "copy_data" => self.copy_data(env, ctx, rs, args).await,
            "_field_to_sql" => self._field_to_sql(env, ctx, rs, args).await,
            "_search_panel_domain_image" => self._search_panel_domain_image(env, ctx, rs, args).await,
            "_get_reconciliation_aml_field_value" => self._get_reconciliation_aml_field_value(env, ctx, rs, args).await,
            "_prepare_move_line_residual_amounts" => self._prepare_move_line_residual_amounts(env, ctx, rs, args).await,
            "_prepare_reconciliation_single_partial" => self._prepare_reconciliation_single_partial(env, ctx, rs, args).await,
            "_prepare_reconciliation_amls" => self._prepare_reconciliation_amls(env, ctx, rs, args).await,
            "_prepare_reconciliation_plan" => self._prepare_reconciliation_plan(env, ctx, rs, args).await,
            "_check_amls_exigibility_for_reconciliation" => self._check_amls_exigibility_for_reconciliation(env, ctx, rs, args).await,
            "_optimize_reconciliation_plan" => self._optimize_reconciliation_plan(env, ctx, rs, args).await,
            "_reconcile_pre_hook" => self._reconcile_pre_hook(env, ctx, rs, args).await,
            "_reconcile_post_hook" => self._reconcile_post_hook(env, ctx, rs, args).await,
            "_reconcile_plan" => self._reconcile_plan(env, ctx, rs, args).await,
            "_reconcile_plan_with_sync" => self._reconcile_plan_with_sync(env, ctx, rs, args).await,
            "_get_exchange_journal" => self._get_exchange_journal(env, ctx, rs, args).await,
            "_get_exchange_account" => self._get_exchange_account(env, ctx, rs, args).await,
            "_prepare_exchange_difference_move_vals" => self._prepare_exchange_difference_move_vals(env, ctx, rs, args).await,
            "_create_exchange_difference_moves" => self._create_exchange_difference_moves(env, ctx, rs, args).await,
            "reconcile" => self.reconcile(env, ctx, rs, args).await,
            "remove_move_reconcile" => self.remove_move_reconcile(env, ctx, rs, args).await,
            "action_unreconcile_match_entries" => self.action_unreconcile_match_entries(env, ctx, rs, args).await,
            "_reconcile_marked" => self._reconcile_marked(env, ctx, rs, args).await,
            "_get_matched_move_ids" => self._get_matched_move_ids(env, ctx, rs, args).await,
            "_validate_analytic_distribution" => self._validate_analytic_distribution(env, ctx, rs, args).await,
            "_create_analytic_lines" => self._create_analytic_lines(env, ctx, rs, args).await,
            "_prepare_analytic_lines" => self._prepare_analytic_lines(env, ctx, rs, args).await,
            "_prepare_analytic_distribution_line" => self._prepare_analytic_distribution_line(env, ctx, rs, args).await,
            "_related_analytic_distribution" => self._related_analytic_distribution(env, ctx, rs, args).await,
            "_update_analytic_distribution" => self._update_analytic_distribution(env, ctx, rs, args).await,
            "_round_analytic_distribution_line" => self._round_analytic_distribution_line(env, ctx, rs, args).await,
            "_get_installments_data" => self._get_installments_data(env, ctx, rs, args).await,
            "_get_integrity_hash_fields" => self._get_integrity_hash_fields(env, ctx, rs, args).await,
            "_reconciled_lines" => self._reconciled_lines(env, ctx, rs, args).await,
            "_reconciled_by_number" => self._reconciled_by_number(env, ctx, rs, args).await,
            "_filter_reconciled_by_number" => self._filter_reconciled_by_number(env, ctx, rs, args).await,
            "_all_reconciled_lines" => self._all_reconciled_lines(env, ctx, rs, args).await,
            "_get_attachment_domains" => self._get_attachment_domains(env, ctx, rs, args).await,
            "_get_attachment_by_record" => self._get_attachment_by_record(env, ctx, rs, args).await,
            "_get_tax_exigible_domain" => self._get_tax_exigible_domain(env, ctx, rs, args).await,
            "_get_invoiced_qty_per_product" => self._get_invoiced_qty_per_product(env, ctx, rs, args).await,
            "_get_lock_date_protected_fields" => self._get_lock_date_protected_fields(env, ctx, rs, args).await,
            "get_import_templates" => self.get_import_templates(env, ctx, rs, args).await,
            "_prepare_edi_vals_to_export" => self._prepare_edi_vals_to_export(env, ctx, rs, args).await,
            "_get_journal_items_full_name" => self._get_journal_items_full_name(env, ctx, rs, args).await,
            "_check_edi_line_tax_required" => self._check_edi_line_tax_required(env, ctx, rs, args).await,
            "_get_aml_values" => self._get_aml_values(env, ctx, rs, args).await,
            "_filter_aml_lot_valuation" => self._filter_aml_lot_valuation(env, ctx, rs, args).await,
            "_get_child_lines" => self._get_child_lines(env, ctx, rs, args).await,
            "get_section_subtotal" => self.get_section_subtotal(env, ctx, rs, args).await,
            "get_column_to_exclude_for_colspan_calculation" => self.get_column_to_exclude_for_colspan_calculation(env, ctx, rs, args).await,
            "get_parent_section_line" => self.get_parent_section_line(env, ctx, rs, args).await,
            "_get_section_lines" => self._get_section_lines(env, ctx, rs, args).await,
            "_is_line_in_section" => self._is_line_in_section(env, ctx, rs, args).await,
            "open_reconcile_view" => self.open_reconcile_view(env, ctx, rs, args).await,
            "action_open_business_doc" => self.action_open_business_doc(env, ctx, rs, args).await,
            "action_automatic_entry" => self.action_automatic_entry(env, ctx, rs, args).await,
            "action_add_from_catalog" => self.action_add_from_catalog(env, ctx, rs, args).await,
            "_get_product_catalog_lines_data" => self._get_product_catalog_lines_data(env, ctx, rs, args).await,
            "_conditional_add_to_compute" => self._conditional_add_to_compute(env, ctx, rs, args).await,
            "_copy_data_extend_business_fields" => self._copy_data_extend_business_fields(env, ctx, rs, args).await,
            "_get_downpayment_lines" => self._get_downpayment_lines(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountMoveLineFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:491`). Decoradores: api.model.
    async fn get_views(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.get_views".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:504`). Decoradores: api.depends('move_id').
    async fn _compute_display_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_display_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:516`).
    async fn _compute_partner_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_partner_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:521`). Decoradores: api.depends('move_id.currency_id').
    async fn _compute_currency_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_currency_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:531`). Decoradores: api.depends('product_id', 'move_id.ref', 'move_id.payment_reference').
    async fn _compute_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:577`).
    async fn _compute_account_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_account_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:661`). Decoradores: api.model.
    async fn _search_account_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._search_account_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:686`). Decoradores: api.depends('move_id.is_storno', 'price_unit', 'quantity').
    async fn _compute_is_storno(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_is_storno".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:698`). Decoradores: api.depends('move_id').
    async fn _compute_balance(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_balance".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:717`). Decoradores: api.depends('balance').
    async fn _compute_debit_credit(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_debit_credit".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:727`). Decoradores: api.depends('currency_id', 'company_id', 'move_id.invoice_currency_rate', 'move_id.date').
    async fn _compute_currency_rate(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_currency_rate".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:742`). Decoradores: api.depends('currency_id', 'company_currency_id').
    async fn _compute_same_currency(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_same_currency".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:747`). Decoradores: api.depends('currency_rate', 'balance').
    async fn _compute_amount_currency(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_amount_currency".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:755`). Decoradores: api.depends_context('order_cumulated_balance', 'domain_cumulated_balance').
    async fn _compute_cumulated_balance(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_cumulated_balance".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:777`). Decoradores: api.depends('debit', 'credit', 'amount_currency', 'account_id', 'currency_id', 'company_id', 'matched_debit_ids', 'matched_credit_ids').
    async fn _compute_amount_residual(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_amount_residual".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:845`). Decoradores: api.depends('product_id', 'product_id.uom_id', 'product_id.uom_ids').
    async fn _compute_allowed_uom_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_allowed_uom_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:850`). Decoradores: api.depends('product_id').
    async fn _compute_product_uom_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_product_uom_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:860`). Decoradores: api.depends('display_type').
    async fn _compute_quantity(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_quantity".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:868`). Decoradores: api.depends('display_type').
    async fn _compute_sequence(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_sequence".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:878`). Decoradores: api.depends('quantity', 'discount', 'price_unit', 'tax_ids', 'currency_id').
    async fn _compute_totals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_totals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:897`). Decoradores: api.depends('product_id', 'product_uom_id').
    async fn _compute_price_unit(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_price_unit".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:917`). Decoradores: api.depends('product_id', 'product_uom_id').
    async fn _compute_tax_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_tax_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:925`).
    async fn _get_computed_taxes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_computed_taxes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:954`). Decoradores: api.depends('account_id', 'company_id').
    async fn _compute_discount_allocation_key(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_discount_allocation_key".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:966`). Decoradores: api.depends('account_id', 'company_id', 'discount', 'price_unit', 'quantity', 'currency_rate', 'analytic_distribution').
    async fn _compute_discount_allocation_needed(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_discount_allocation_needed".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1018`). Decoradores: api.depends('tax_ids', 'account_id', 'company_id').
    async fn _compute_epd_key(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_epd_key".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1033`). Decoradores: api.depends('move_id.needed_terms', 'account_id', 'analytic_distribution', 'tax_ids', 'tax_tag_ids', 'company_id', 'price_subtotal').
    async fn _compute_epd_needed(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_epd_needed".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1159`). Decoradores: api.depends('move_id.move_type', 'balance', 'tax_repartition_line_id', 'tax_ids').
    async fn _compute_is_refund(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_is_refund".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1183`). Decoradores: api.depends('date_maturity').
    async fn _compute_term_key(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_term_key".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1195`). Decoradores: api.depends('account_id', 'partner_id', 'product_id').
    async fn _compute_analytic_distribution(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_analytic_distribution".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1209`).
    async fn _get_analytic_distribution_arguments(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_analytic_distribution_arguments".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1228`). Decoradores: api.depends('discount_date', 'date_maturity').
    async fn _compute_payment_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_payment_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1233`). Decoradores: api.depends('matched_debit_ids', 'matched_credit_ids').
    async fn _compute_reconciled_lines_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_reconciled_lines_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1238`). Decoradores: api.depends('matched_debit_ids', 'matched_credit_ids').
    async fn _compute_reconciled_lines_excluding_exchange_diff_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_reconciled_lines_excluding_exchange_diff_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1247`).
    async fn _compute_parent_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_parent_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1274`). Decoradores: api.depends('journal_id.type').
    async fn _compute_no_followup(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_no_followup".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1278`).
    async fn _inverse_no_followup(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._inverse_no_followup".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1286`).
    async fn _search_payment_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._search_payment_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1302`).
    async fn action_payment_items_register_payment(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.action_payment_items_register_payment".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1305`).
    async fn action_register_payment(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.action_register_payment".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1329`).
    async fn _search_journal_group_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._search_journal_group_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1337`). Decoradores: api.onchange('partner_id').
    async fn _inverse_partner_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._inverse_partner_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1343`). Decoradores: api.onchange('product_id').
    async fn _inverse_product_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._inverse_product_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1351`). Decoradores: api.onchange('amount_currency', 'currency_id').
    async fn _inverse_amount_currency(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._inverse_amount_currency".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1363`). Decoradores: api.onchange('debit').
    async fn _inverse_debit(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._inverse_debit".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1371`). Decoradores: api.onchange('credit').
    async fn _inverse_credit(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._inverse_credit".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1378`).
    async fn _inverse_analytic_distribution(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._inverse_analytic_distribution".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1398`). Decoradores: api.onchange('account_id').
    async fn _inverse_account_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._inverse_account_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1405`).
    async fn _inverse_reconciled_lines_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._inverse_reconciled_lines_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1415`).
    async fn _check_constrains_account_id_journal_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._check_constrains_account_id_journal_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1434`). Decoradores: api.constrains('account_id', 'tax_ids', 'tax_line_id', 'reconciled').
    async fn _check_off_balance(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._check_off_balance".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1445`). Decoradores: api.constrains('account_id', 'display_type').
    async fn _check_payable_receivable(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._check_payable_receivable".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1459`).
    async fn _affect_tax_report(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._affect_tax_report".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1463`).
    async fn _check_tax_lock_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._check_tax_lock_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1482`).
    async fn _check_reconciliation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._check_reconciliation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1490`). Decoradores: api.constrains('tax_ids', 'tax_repartition_line_id').
    async fn _check_caba_non_caba_shared_tags(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._check_caba_non_caba_shared_tags".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1531`). Decoradores: api.constrains('matching_number', 'matched_debit_ids', 'matched_credit_ids').
    async fn _constrains_matching_number(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._constrains_matching_number".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1550`). Decoradores: api.constrains('deductible_amount').
    async fn _constrains_deductible_amount(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._constrains_deductible_amount".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1563`). Decoradores: api.model, api.deprecated('Override of a deprecated method').
    async fn check_field_access_rights(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.check_field_access_rights".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1571`). Decoradores: api.model.
    async fn _get_default_read_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_default_read_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1575`).
    async fn read(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.read".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1580`). Decoradores: api.model.
    async fn search_read(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.search_read".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1584`).
    async fn invalidate_model(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.invalidate_model".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1594`).
    async fn invalidate_recordset(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.invalidate_recordset".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1604`). Decoradores: api.model.
    async fn search_fetch(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.search_fetch".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1619`). Decoradores: api.model.
    async fn default_get(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.default_get".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1630`).
    async fn _sanitize_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._sanitize_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1651`).
    async fn _prepare_create_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._prepare_create_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1671`). Decoradores: contextmanager.
    async fn _sync_invoice(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._sync_invoice".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1718`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1753`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1878`).
    async fn _parse_flush_fnames(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._parse_flush_fnames".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1884`).
    async fn flush_recordset(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.flush_recordset".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1887`).
    async fn flush_model(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.flush_model".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1890`).
    async fn _valid_field_parameter(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._valid_field_parameter".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1895`). Decoradores: api.ondelete().
    async fn _unlink_except_posted(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._unlink_except_posted".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1901`). Decoradores: api.ondelete().
    async fn _prevent_automatic_line_deletion(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._prevent_automatic_line_deletion".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1915`). Decoradores: api.ondelete().
    async fn _except_hashed_entry_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._except_hashed_entry_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1923`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1958`). Decoradores: api.model.
    async fn _format_aml_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._format_aml_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1978`). Decoradores: api.depends('move_id', 'ref', 'product_id').
    async fn _compute_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_display_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:1983`). Decoradores: api.depends('account_id', 'company_id', 'move_id', 'product_id', 'display_type', 'analytic_distribution').
    async fn _compute_has_invalid_analytics(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._compute_has_invalid_analytics".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:2007`).
    async fn copy_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.copy_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:2025`).
    async fn _field_to_sql(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._field_to_sql".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:2042`).
    async fn _search_panel_domain_image(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._search_panel_domain_image".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:2079`).
    async fn _get_reconciliation_aml_field_value(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_reconciliation_aml_field_value".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:2087`). Decoradores: api.model.
    async fn _prepare_move_line_residual_amounts(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._prepare_move_line_residual_amounts".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:2165`). Decoradores: api.model.
    async fn _prepare_reconciliation_single_partial(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._prepare_reconciliation_single_partial".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:2474`). Decoradores: api.model.
    async fn _prepare_reconciliation_amls(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._prepare_reconciliation_amls".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:2541`). Decoradores: api.model.
    async fn _prepare_reconciliation_plan(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._prepare_reconciliation_plan".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:2580`).
    async fn _check_amls_exigibility_for_reconciliation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._check_amls_exigibility_for_reconciliation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:2618`). Decoradores: api.model.
    async fn _optimize_reconciliation_plan(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._optimize_reconciliation_plan".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:2712`).
    async fn _reconcile_pre_hook(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._reconcile_pre_hook".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:2719`).
    async fn _reconcile_post_hook(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._reconcile_post_hook".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:2726`). Decoradores: api.model.
    async fn _reconcile_plan(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._reconcile_plan".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:2750`).
    async fn _reconcile_plan_with_sync(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._reconcile_plan_with_sync".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:2920`).
    async fn _get_exchange_journal(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_exchange_journal".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:2923`).
    async fn _get_exchange_account(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_exchange_account".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:2928`).
    async fn _prepare_exchange_difference_move_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._prepare_exchange_difference_move_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3017`). Decoradores: api.model.
    async fn _create_exchange_difference_moves(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._create_exchange_difference_moves".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3071`).
    async fn reconcile(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.reconcile".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3075`).
    async fn remove_move_reconcile(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.remove_move_reconcile".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3079`).
    async fn action_unreconcile_match_entries(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.action_unreconcile_match_entries".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3086`).
    async fn _reconcile_marked(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._reconcile_marked".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3109`).
    async fn _get_matched_move_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_matched_move_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3117`).
    async fn _validate_analytic_distribution(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._validate_analytic_distribution".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3150`).
    async fn _create_analytic_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._create_analytic_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3163`).
    async fn _prepare_analytic_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._prepare_analytic_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3178`).
    async fn _prepare_analytic_distribution_line(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._prepare_analytic_distribution_line".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3212`).
    async fn _related_analytic_distribution(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._related_analytic_distribution".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3216`).
    async fn _update_analytic_distribution(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._update_analytic_distribution".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3226`).
    async fn _round_analytic_distribution_line(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._round_analytic_distribution_line".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3256`).
    async fn _get_installments_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_installments_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3323`).
    async fn _get_integrity_hash_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_integrity_hash_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3332`).
    async fn _reconciled_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._reconciled_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3339`).
    async fn _reconciled_by_number(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._reconciled_by_number".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3350`).
    async fn _filter_reconciled_by_number(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._filter_reconciled_by_number".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3360`).
    async fn _all_reconciled_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._all_reconciled_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3364`).
    async fn _get_attachment_domains(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_attachment_domains".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3377`). Decoradores: api.model.
    async fn _get_attachment_by_record(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_attachment_by_record".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3385`). Decoradores: api.model.
    async fn _get_tax_exigible_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_tax_exigible_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3404`).
    async fn _get_invoiced_qty_per_product(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_invoiced_qty_per_product".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3414`).
    async fn _get_lock_date_protected_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_lock_date_protected_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3427`). Decoradores: api.model.
    async fn get_import_templates(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.get_import_templates".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3433`).
    async fn _prepare_edi_vals_to_export(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._prepare_edi_vals_to_export".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3462`).
    async fn _get_journal_items_full_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_journal_items_full_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3465`).
    async fn _check_edi_line_tax_required(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._check_edi_line_tax_required".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3468`).
    async fn _get_aml_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_aml_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3486`).
    async fn _filter_aml_lot_valuation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._filter_aml_lot_valuation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3493`).
    async fn _get_child_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_child_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3573`).
    async fn get_section_subtotal(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.get_section_subtotal".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3578`).
    async fn get_column_to_exclude_for_colspan_calculation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.get_column_to_exclude_for_colspan_calculation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3581`).
    async fn get_parent_section_line(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.get_parent_section_line".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3587`).
    async fn _get_section_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_section_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3591`).
    async fn _is_line_in_section(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._is_line_in_section".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3607`).
    async fn open_reconcile_view(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.open_reconcile_view".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3613`).
    async fn action_open_business_doc(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.action_open_business_doc".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3616`).
    async fn action_automatic_entry(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.action_automatic_entry".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3629`).
    async fn action_add_from_catalog(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line.action_add_from_catalog".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3637`).
    async fn _get_product_catalog_lines_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_product_catalog_lines_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3680`).
    async fn _conditional_add_to_compute(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._conditional_add_to_compute".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3693`).
    async fn _copy_data_extend_business_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._copy_data_extend_business_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line.py:3696`).
    async fn _get_downpayment_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_downpayment_lines".into(),
        ))
    }

}
