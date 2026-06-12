//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.tax`

use nexus_orm::prelude::*;

pub struct AccountTaxFragment;

#[async_trait]
impl ModelFragment for AccountTaxFragment {
    fn model_name(&self) -> &str {
        "account.tax"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Tax".into();
        def.order = "sequence,id".into();
        def.add_field(FieldDef::char("name").string("Tax Name").required());
        def.add_field(FieldDef::selection("type_tax_use", &[]).string("Tax Type").required().default_val("sale"));
        def.add_field(FieldDef::selection("tax_scope", &[("service", "Services"), ("consu", "Goods")]).string("Tax Scope"));
        def.add_field(FieldDef::selection("amount_type", &[("group", "Group of Taxes"), ("fixed", "Fixed"), ("percent", "Percentage"), ("division", "Percentage Tax Included")]).string("Tax Computation").required().default_val("percent"));
        def.add_field(FieldDef::many2many("fiscal_position_ids", "account.fiscal.position"));
        def.add_field(FieldDef::many2many("original_tax_ids", "account.tax").string("Replaces"));
        def.add_field(FieldDef::many2many("replacing_tax_ids", "account.tax").string("Replaced by").readonly());
        def.add_field(FieldDef::boolean("display_alternative_taxes_field").computed("_compute_display_alternative_taxes_field", &["fiscal_position_ids"]).stored());
        def.add_field(FieldDef::boolean("is_domestic").computed("_compute_is_domestic", &["company_id", "company_id.domestic_fiscal_position_id", "fiscal_position_ids"]).stored());
        def.add_field(FieldDef::boolean("active").default_val(true));
        def.add_field(FieldDef::many2one("company_id", "res.company").string("Company").required().readonly());
        def.add_field(FieldDef::many2many("children_tax_ids", "account.tax").string("Children Taxes"));
        def.add_field(FieldDef::integer("sequence").required().default_val(1i64));
        def.add_field(FieldDef::float("amount").required().default_val(0f64));
        def.add_field(FieldDef::html("description").string("Description"));
        def.add_field(FieldDef::char("invoice_label").string("Label on Invoices"));
        def.add_field(FieldDef::char("tax_label").computed("_compute_tax_label", &["name", "invoice_label"]).stored());
        def.add_field(FieldDef::boolean("price_include").computed("_compute_price_include", &["price_include_override"]).stored());
        def.add_field({ let mut f = FieldDef::selection("company_price_include", &[]); f.related = Some("company_id.account_price_include".into()); f });
        def.add_field(FieldDef::selection("price_include_override", &[("tax_included", "Tax Included"), ("tax_excluded", "Tax Excluded")]).string("Included in Price"));
        def.add_field(FieldDef::boolean("include_base_amount").string("Affect Base of Subsequent Taxes").default_val(false));
        def.add_field(FieldDef::boolean("is_base_affected").string("Base Affected by Previous Taxes").default_val(true));
        def.add_field(FieldDef::boolean("analytic").string("Include in Analytic Cost"));
        def.add_field(FieldDef::many2one("tax_group_id", "account.tax.group").string("Tax Group").required().computed("_compute_tax_group_id", &["company_id", "country_id"]).stored());
        def.add_field({ let mut f = FieldDef::boolean("hide_tax_exigibility").string("Hide Use Cash Basis Option").readonly(); f.related = Some("company_id.tax_exigibility".into()); f });
        def.add_field(FieldDef::selection("tax_exigibility", &[("on_invoice", "Based on Invoice"), ("on_payment", "Based on Payment")]).string("Tax Exigibility").default_val("on_invoice"));
        def.add_field(FieldDef::many2one("cash_basis_transition_account_id", "account.account").string("Cash Basis Transition Account"));
        def.add_field(FieldDef::one2many("invoice_repartition_line_ids", "account.tax.repartition.line", "tax_id").string("Distribution for Invoices").computed("_compute_invoice_repartition_line_ids", &["company_id"]).stored());
        def.add_field(FieldDef::one2many("refund_repartition_line_ids", "account.tax.repartition.line", "tax_id").string("Distribution for Refund Invoices").computed("_compute_refund_repartition_line_ids", &["company_id"]).stored());
        def.add_field(FieldDef::one2many("repartition_line_ids", "account.tax.repartition.line", "tax_id").string("Distribution"));
        def.add_field(FieldDef::many2one("country_id", "res.country").string("Country").required().computed("_compute_country_id", &["company_id"]).stored());
        def.add_field({ let mut f = FieldDef::char("country_code").readonly(); f.related = Some("country_id.code".into()); f });
        def.add_field(FieldDef::boolean("is_used").string("Tax used").computed("_compute_is_used", &[]).stored());
        def.add_field(FieldDef::char("repartition_lines_str").string("Repartition Lines").computed("_compute_repartition_lines_str", &["repartition_line_ids.account_id", "repartition_line_ids.sequence", "repartition_line_ids.factor_percent", "repartition_line_ids.use_in_tax_closing", "repartition_line_ids.tag_ids"]).stored());
        def.add_field(FieldDef::html("invoice_legal_notes").string("Legal Notes"));
        def.add_field(FieldDef::boolean("has_negative_factor").computed("_compute_has_negative_factor", &["invoice_repartition_line_ids.factor", "invoice_repartition_line_ids.repartition_type"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_constrains_name", "validate_tax_group_id", "_constrains_cash_basis_transition_account", "name_search", "_compute_country_id", "_compute_tax_group_id", "_compute_price_include", "_search_price_include", "_hook_compute_is_used", "_compute_is_domestic", "_compute_display_alternative_taxes_field", "_compute_is_used", "_compute_repartition_lines_str", "_message_log_repartition_lines", "_message_log", "_compute_invoice_repartition_line_ids", "_compute_refund_repartition_line_ids", "_compute_has_negative_factor", "_parse_name_search", "_search", "_check_repartition_lines", "_validate_repartition_lines", "_check_children_scope", "_check_company_consistency", "_sanitize_vals", "create", "write", "copy_data", "_compute_display_name", "_compute_tax_label", "onchange_amount", "onchange_amount_type", "onchange_price_include", "_eval_taxes_computation_prepare_product_fields", "_eval_taxes_computation_prepare_product_default_values", "_eval_taxes_computation_prepare_product_values", "_eval_taxes_computation_turn_to_product_values", "_eval_taxes_computation_prepare_product_uom_fields", "_eval_taxes_computation_prepare_product_uom_default_values", "_eval_taxes_computation_prepare_product_uom_values", "_eval_taxes_computation_turn_to_product_uom_values", "_flatten_taxes_and_sort_them", "_batch_for_taxes_computation", "_propagate_extra_taxes_base", "_eval_tax_amount_fixed_amount", "_eval_tax_amount_price_included", "_eval_tax_amount_price_excluded", "_get_tax_details", "_adapt_price_unit_to_another_taxes", "_export_base_line_extra_tax_data", "_import_base_line_extra_tax_data", "_reverse_quantity_base_line_extra_tax_data", "_turn_base_line_is_refund_flag_off", "_turn_base_lines_is_refund_flag_off", "_get_base_line_field_value_from_record", "_prepare_base_line_for_taxes_computation", "_prepare_tax_line_for_taxes_computation", "_add_tax_details_in_base_line", "_add_tax_details_in_base_lines", "_normalize_target_factors", "_distribute_delta_amount_smoothly", "_round_tax_details_tax_amounts", "_round_tax_details_base_lines", "_round_tax_details_tax_amounts_from_tax_lines", "_round_base_lines_tax_details", "_prepare_base_line_grouping_key", "_prepare_base_line_tax_repartition_grouping_key", "_prepare_tax_line_repartition_grouping_key", "_add_accounting_data_to_base_line_tax_details", "_add_accounting_data_in_base_lines_tax_details", "_aggregate_base_line_tax_details", "_aggregate_base_lines_tax_details", "_aggregate_base_lines_aggregated_values", "_get_tax_totals_summary", "_exclude_tax_groups_from_tax_totals_summary", "_prepare_tax_lines", "_can_be_discounted", "_merge_tax_details", "_fix_base_lines_tax_details_on_manual_tax_amounts", "_split_tax_data", "_split_tax_details", "_split_base_line", "_compute_subset_base_lines_total", "_reduce_base_lines_with_grouping_function", "_apply_base_lines_manual_amounts_to_reach", "_reduce_base_lines_to_target_amount", "_partition_base_lines_taxes", "_prepare_discountable_base_lines", "_prepare_global_discount_lines", "_prepare_base_lines_for_down_payment", "_prepare_down_payment_lines", "_dispatch_taxes_into_new_base_lines", "_turn_removed_taxes_into_new_base_lines", "_dispatch_global_discount_lines", "_squash_global_discount_lines", "_dispatch_return_of_merchandise_lines", "_squash_return_of_merchandise_lines", "_get_delta_amount_to_reach_target", "_round_raw_total_excluded", "_add_and_round_raw_gross_total_excluded_and_discount", "_round_raw_gross_total_excluded_and_discount", "_round_raw_tax_amounts", "flatten_taxes_hierarchy", "get_tax_tags", "compute_all", "_filter_taxes_by_company", "_fix_tax_included_price", "_fix_tax_included_price_company", "_get_description_plaintext"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_constrains_name" => self._constrains_name(env, ctx, rs, args).await,
            "validate_tax_group_id" => self.validate_tax_group_id(env, ctx, rs, args).await,
            "_constrains_cash_basis_transition_account" => self._constrains_cash_basis_transition_account(env, ctx, rs, args).await,
            "name_search" => self.name_search(env, ctx, rs, args).await,
            "_compute_country_id" => self._compute_country_id(env, ctx, rs, args).await,
            "_compute_tax_group_id" => self._compute_tax_group_id(env, ctx, rs, args).await,
            "_compute_price_include" => self._compute_price_include(env, ctx, rs, args).await,
            "_search_price_include" => self._search_price_include(env, ctx, rs, args).await,
            "_hook_compute_is_used" => self._hook_compute_is_used(env, ctx, rs, args).await,
            "_compute_is_domestic" => self._compute_is_domestic(env, ctx, rs, args).await,
            "_compute_display_alternative_taxes_field" => self._compute_display_alternative_taxes_field(env, ctx, rs, args).await,
            "_compute_is_used" => self._compute_is_used(env, ctx, rs, args).await,
            "_compute_repartition_lines_str" => self._compute_repartition_lines_str(env, ctx, rs, args).await,
            "_message_log_repartition_lines" => self._message_log_repartition_lines(env, ctx, rs, args).await,
            "_message_log" => self._message_log(env, ctx, rs, args).await,
            "_compute_invoice_repartition_line_ids" => self._compute_invoice_repartition_line_ids(env, ctx, rs, args).await,
            "_compute_refund_repartition_line_ids" => self._compute_refund_repartition_line_ids(env, ctx, rs, args).await,
            "_compute_has_negative_factor" => self._compute_has_negative_factor(env, ctx, rs, args).await,
            "_parse_name_search" => self._parse_name_search(env, ctx, rs, args).await,
            "_search" => self._search(env, ctx, rs, args).await,
            "_check_repartition_lines" => self._check_repartition_lines(env, ctx, rs, args).await,
            "_validate_repartition_lines" => self._validate_repartition_lines(env, ctx, rs, args).await,
            "_check_children_scope" => self._check_children_scope(env, ctx, rs, args).await,
            "_check_company_consistency" => self._check_company_consistency(env, ctx, rs, args).await,
            "_sanitize_vals" => self._sanitize_vals(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "copy_data" => self.copy_data(env, ctx, rs, args).await,
            "_compute_display_name" => self._compute_display_name(env, ctx, rs, args).await,
            "_compute_tax_label" => self._compute_tax_label(env, ctx, rs, args).await,
            "onchange_amount" => self.onchange_amount(env, ctx, rs, args).await,
            "onchange_amount_type" => self.onchange_amount_type(env, ctx, rs, args).await,
            "onchange_price_include" => self.onchange_price_include(env, ctx, rs, args).await,
            "_eval_taxes_computation_prepare_product_fields" => self._eval_taxes_computation_prepare_product_fields(env, ctx, rs, args).await,
            "_eval_taxes_computation_prepare_product_default_values" => self._eval_taxes_computation_prepare_product_default_values(env, ctx, rs, args).await,
            "_eval_taxes_computation_prepare_product_values" => self._eval_taxes_computation_prepare_product_values(env, ctx, rs, args).await,
            "_eval_taxes_computation_turn_to_product_values" => self._eval_taxes_computation_turn_to_product_values(env, ctx, rs, args).await,
            "_eval_taxes_computation_prepare_product_uom_fields" => self._eval_taxes_computation_prepare_product_uom_fields(env, ctx, rs, args).await,
            "_eval_taxes_computation_prepare_product_uom_default_values" => self._eval_taxes_computation_prepare_product_uom_default_values(env, ctx, rs, args).await,
            "_eval_taxes_computation_prepare_product_uom_values" => self._eval_taxes_computation_prepare_product_uom_values(env, ctx, rs, args).await,
            "_eval_taxes_computation_turn_to_product_uom_values" => self._eval_taxes_computation_turn_to_product_uom_values(env, ctx, rs, args).await,
            "_flatten_taxes_and_sort_them" => self._flatten_taxes_and_sort_them(env, ctx, rs, args).await,
            "_batch_for_taxes_computation" => self._batch_for_taxes_computation(env, ctx, rs, args).await,
            "_propagate_extra_taxes_base" => self._propagate_extra_taxes_base(env, ctx, rs, args).await,
            "_eval_tax_amount_fixed_amount" => self._eval_tax_amount_fixed_amount(env, ctx, rs, args).await,
            "_eval_tax_amount_price_included" => self._eval_tax_amount_price_included(env, ctx, rs, args).await,
            "_eval_tax_amount_price_excluded" => self._eval_tax_amount_price_excluded(env, ctx, rs, args).await,
            "_get_tax_details" => self._get_tax_details(env, ctx, rs, args).await,
            "_adapt_price_unit_to_another_taxes" => self._adapt_price_unit_to_another_taxes(env, ctx, rs, args).await,
            "_export_base_line_extra_tax_data" => self._export_base_line_extra_tax_data(env, ctx, rs, args).await,
            "_import_base_line_extra_tax_data" => self._import_base_line_extra_tax_data(env, ctx, rs, args).await,
            "_reverse_quantity_base_line_extra_tax_data" => self._reverse_quantity_base_line_extra_tax_data(env, ctx, rs, args).await,
            "_turn_base_line_is_refund_flag_off" => self._turn_base_line_is_refund_flag_off(env, ctx, rs, args).await,
            "_turn_base_lines_is_refund_flag_off" => self._turn_base_lines_is_refund_flag_off(env, ctx, rs, args).await,
            "_get_base_line_field_value_from_record" => self._get_base_line_field_value_from_record(env, ctx, rs, args).await,
            "_prepare_base_line_for_taxes_computation" => self._prepare_base_line_for_taxes_computation(env, ctx, rs, args).await,
            "_prepare_tax_line_for_taxes_computation" => self._prepare_tax_line_for_taxes_computation(env, ctx, rs, args).await,
            "_add_tax_details_in_base_line" => self._add_tax_details_in_base_line(env, ctx, rs, args).await,
            "_add_tax_details_in_base_lines" => self._add_tax_details_in_base_lines(env, ctx, rs, args).await,
            "_normalize_target_factors" => self._normalize_target_factors(env, ctx, rs, args).await,
            "_distribute_delta_amount_smoothly" => self._distribute_delta_amount_smoothly(env, ctx, rs, args).await,
            "_round_tax_details_tax_amounts" => self._round_tax_details_tax_amounts(env, ctx, rs, args).await,
            "_round_tax_details_base_lines" => self._round_tax_details_base_lines(env, ctx, rs, args).await,
            "_round_tax_details_tax_amounts_from_tax_lines" => self._round_tax_details_tax_amounts_from_tax_lines(env, ctx, rs, args).await,
            "_round_base_lines_tax_details" => self._round_base_lines_tax_details(env, ctx, rs, args).await,
            "_prepare_base_line_grouping_key" => self._prepare_base_line_grouping_key(env, ctx, rs, args).await,
            "_prepare_base_line_tax_repartition_grouping_key" => self._prepare_base_line_tax_repartition_grouping_key(env, ctx, rs, args).await,
            "_prepare_tax_line_repartition_grouping_key" => self._prepare_tax_line_repartition_grouping_key(env, ctx, rs, args).await,
            "_add_accounting_data_to_base_line_tax_details" => self._add_accounting_data_to_base_line_tax_details(env, ctx, rs, args).await,
            "_add_accounting_data_in_base_lines_tax_details" => self._add_accounting_data_in_base_lines_tax_details(env, ctx, rs, args).await,
            "_aggregate_base_line_tax_details" => self._aggregate_base_line_tax_details(env, ctx, rs, args).await,
            "_aggregate_base_lines_tax_details" => self._aggregate_base_lines_tax_details(env, ctx, rs, args).await,
            "_aggregate_base_lines_aggregated_values" => self._aggregate_base_lines_aggregated_values(env, ctx, rs, args).await,
            "_get_tax_totals_summary" => self._get_tax_totals_summary(env, ctx, rs, args).await,
            "_exclude_tax_groups_from_tax_totals_summary" => self._exclude_tax_groups_from_tax_totals_summary(env, ctx, rs, args).await,
            "_prepare_tax_lines" => self._prepare_tax_lines(env, ctx, rs, args).await,
            "_can_be_discounted" => self._can_be_discounted(env, ctx, rs, args).await,
            "_merge_tax_details" => self._merge_tax_details(env, ctx, rs, args).await,
            "_fix_base_lines_tax_details_on_manual_tax_amounts" => self._fix_base_lines_tax_details_on_manual_tax_amounts(env, ctx, rs, args).await,
            "_split_tax_data" => self._split_tax_data(env, ctx, rs, args).await,
            "_split_tax_details" => self._split_tax_details(env, ctx, rs, args).await,
            "_split_base_line" => self._split_base_line(env, ctx, rs, args).await,
            "_compute_subset_base_lines_total" => self._compute_subset_base_lines_total(env, ctx, rs, args).await,
            "_reduce_base_lines_with_grouping_function" => self._reduce_base_lines_with_grouping_function(env, ctx, rs, args).await,
            "_apply_base_lines_manual_amounts_to_reach" => self._apply_base_lines_manual_amounts_to_reach(env, ctx, rs, args).await,
            "_reduce_base_lines_to_target_amount" => self._reduce_base_lines_to_target_amount(env, ctx, rs, args).await,
            "_partition_base_lines_taxes" => self._partition_base_lines_taxes(env, ctx, rs, args).await,
            "_prepare_discountable_base_lines" => self._prepare_discountable_base_lines(env, ctx, rs, args).await,
            "_prepare_global_discount_lines" => self._prepare_global_discount_lines(env, ctx, rs, args).await,
            "_prepare_base_lines_for_down_payment" => self._prepare_base_lines_for_down_payment(env, ctx, rs, args).await,
            "_prepare_down_payment_lines" => self._prepare_down_payment_lines(env, ctx, rs, args).await,
            "_dispatch_taxes_into_new_base_lines" => self._dispatch_taxes_into_new_base_lines(env, ctx, rs, args).await,
            "_turn_removed_taxes_into_new_base_lines" => self._turn_removed_taxes_into_new_base_lines(env, ctx, rs, args).await,
            "_dispatch_global_discount_lines" => self._dispatch_global_discount_lines(env, ctx, rs, args).await,
            "_squash_global_discount_lines" => self._squash_global_discount_lines(env, ctx, rs, args).await,
            "_dispatch_return_of_merchandise_lines" => self._dispatch_return_of_merchandise_lines(env, ctx, rs, args).await,
            "_squash_return_of_merchandise_lines" => self._squash_return_of_merchandise_lines(env, ctx, rs, args).await,
            "_get_delta_amount_to_reach_target" => self._get_delta_amount_to_reach_target(env, ctx, rs, args).await,
            "_round_raw_total_excluded" => self._round_raw_total_excluded(env, ctx, rs, args).await,
            "_add_and_round_raw_gross_total_excluded_and_discount" => self._add_and_round_raw_gross_total_excluded_and_discount(env, ctx, rs, args).await,
            "_round_raw_gross_total_excluded_and_discount" => self._round_raw_gross_total_excluded_and_discount(env, ctx, rs, args).await,
            "_round_raw_tax_amounts" => self._round_raw_tax_amounts(env, ctx, rs, args).await,
            "flatten_taxes_hierarchy" => self.flatten_taxes_hierarchy(env, ctx, rs, args).await,
            "get_tax_tags" => self.get_tax_tags(env, ctx, rs, args).await,
            "compute_all" => self.compute_all(env, ctx, rs, args).await,
            "_filter_taxes_by_company" => self._filter_taxes_by_company(env, ctx, rs, args).await,
            "_fix_tax_included_price" => self._fix_tax_included_price(env, ctx, rs, args).await,
            "_fix_tax_included_price_company" => self._fix_tax_included_price_company(env, ctx, rs, args).await,
            "_get_description_plaintext" => self._get_description_plaintext(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountTaxFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:212`). Decoradores: api.constrains('company_id', 'name', 'type_tax_use', 'tax_scope', 'country_id').
    async fn _constrains_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._constrains_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:241`). Decoradores: api.constrains('tax_group_id').
    async fn validate_tax_group_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax.validate_tax_group_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:247`). Decoradores: api.constrains('tax_exigibility', 'cash_basis_transition_account_id').
    async fn _constrains_cash_basis_transition_account(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._constrains_cash_basis_transition_account".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:258`). Decoradores: api.model, api.readonly.
    async fn name_search(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax.name_search".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:269`). Decoradores: api.depends('company_id').
    async fn _compute_country_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._compute_country_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:274`). Decoradores: api.depends('company_id', 'country_id').
    async fn _compute_tax_group_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._compute_tax_group_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:293`). Decoradores: api.depends('price_include_override').
    async fn _compute_price_include(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._compute_price_include".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:301`).
    async fn _search_price_include(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._search_price_include".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:312`).
    async fn _hook_compute_is_used(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._hook_compute_is_used".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:321`). Decoradores: api.depends('company_id', 'company_id.domestic_fiscal_position_id', 'fiscal_position_ids').
    async fn _compute_is_domestic(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._compute_is_domestic".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:326`). Decoradores: api.depends('fiscal_position_ids').
    async fn _compute_display_alternative_taxes_field(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._compute_display_alternative_taxes_field".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:336`).
    async fn _compute_is_used(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._compute_is_used".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:379`). Decoradores: api.depends('repartition_line_ids.account_id', 'repartition_line_ids.sequence', 'repartition_line_ids.factor_percent', 'repartition_line_ids.use_in_tax_closing', 'repartition_line_ids.tag_ids').
    async fn _compute_repartition_lines_str(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._compute_repartition_lines_str".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:402`).
    async fn _message_log_repartition_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._message_log_repartition_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:462`).
    async fn _message_log(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._message_log".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:482`). Decoradores: api.depends('company_id').
    async fn _compute_invoice_repartition_line_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._compute_invoice_repartition_line_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:491`). Decoradores: api.depends('company_id').
    async fn _compute_refund_repartition_line_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._compute_refund_repartition_line_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:500`). Decoradores: api.depends('invoice_repartition_line_ids.factor', 'invoice_repartition_line_ids.repartition_type').
    async fn _compute_has_negative_factor(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._compute_has_negative_factor".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:506`). Decoradores: staticmethod.
    async fn _parse_name_search(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._parse_name_search".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:530`). Decoradores: api.model.
    async fn _search(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._search".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:546`).
    async fn _check_repartition_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._check_repartition_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:554`). Decoradores: api.constrains('invoice_repartition_line_ids', 'refund_repartition_line_ids', 'repartition_line_ids').
    async fn _validate_repartition_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._validate_repartition_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:591`). Decoradores: api.constrains('children_tax_ids', 'type_tax_use').
    async fn _check_children_scope(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._check_children_scope".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:608`). Decoradores: api.constrains('company_id').
    async fn _check_company_consistency(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._check_company_consistency".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:618`).
    async fn _sanitize_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._sanitize_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:648`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:658`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:661`).
    async fn copy_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax.copy_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:671`). Decoradores: api.depends('type_tax_use', 'tax_scope'), api.depends_context('append_fields', 'formatted_display_name').
    async fn _compute_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._compute_display_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:693`). Decoradores: api.depends('name', 'invoice_label').
    async fn _compute_tax_label(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._compute_tax_label".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:698`). Decoradores: api.onchange('amount').
    async fn onchange_amount(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax.onchange_amount".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:703`). Decoradores: api.onchange('amount_type').
    async fn onchange_amount_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax.onchange_amount_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:710`). Decoradores: api.onchange('price_include').
    async fn onchange_price_include(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax.onchange_price_include".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:720`).
    async fn _eval_taxes_computation_prepare_product_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._eval_taxes_computation_prepare_product_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:734`). Decoradores: api.model.
    async fn _eval_taxes_computation_prepare_product_default_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._eval_taxes_computation_prepare_product_default_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:764`). Decoradores: api.model.
    async fn _eval_taxes_computation_prepare_product_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._eval_taxes_computation_prepare_product_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:783`).
    async fn _eval_taxes_computation_turn_to_product_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._eval_taxes_computation_turn_to_product_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:802`).
    async fn _eval_taxes_computation_prepare_product_uom_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._eval_taxes_computation_prepare_product_uom_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:816`). Decoradores: api.model.
    async fn _eval_taxes_computation_prepare_product_uom_default_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._eval_taxes_computation_prepare_product_uom_default_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:846`). Decoradores: api.model.
    async fn _eval_taxes_computation_prepare_product_uom_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._eval_taxes_computation_prepare_product_uom_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:865`).
    async fn _eval_taxes_computation_turn_to_product_uom_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._eval_taxes_computation_turn_to_product_uom_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:884`).
    async fn _flatten_taxes_and_sort_them(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._flatten_taxes_and_sort_them".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:912`).
    async fn _batch_for_taxes_computation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._batch_for_taxes_computation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:965`).
    async fn _propagate_extra_taxes_base(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._propagate_extra_taxes_base".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:1071`).
    async fn _eval_tax_amount_fixed_amount(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._eval_tax_amount_fixed_amount".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:1086`).
    async fn _eval_tax_amount_price_included(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._eval_tax_amount_price_included".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:1106`).
    async fn _eval_tax_amount_price_excluded(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._eval_tax_amount_price_excluded".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:1126`).
    async fn _get_tax_details(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._get_tax_details".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:1331`). Decoradores: api.model.
    async fn _adapt_price_unit_to_another_taxes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._adapt_price_unit_to_another_taxes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:1384`). Decoradores: api.model.
    async fn _export_base_line_extra_tax_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._export_base_line_extra_tax_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:1419`). Decoradores: api.model.
    async fn _import_base_line_extra_tax_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._import_base_line_extra_tax_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:1470`). Decoradores: api.model.
    async fn _reverse_quantity_base_line_extra_tax_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._reverse_quantity_base_line_extra_tax_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:1494`).
    async fn _turn_base_line_is_refund_flag_off(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._turn_base_line_is_refund_flag_off".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:1534`). Decoradores: api.model.
    async fn _turn_base_lines_is_refund_flag_off(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._turn_base_lines_is_refund_flag_off".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:1545`). Decoradores: api.model.
    async fn _get_base_line_field_value_from_record(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._get_base_line_field_value_from_record".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:1578`). Decoradores: api.model.
    async fn _prepare_base_line_for_taxes_computation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._prepare_base_line_for_taxes_computation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:1679`). Decoradores: api.model.
    async fn _prepare_tax_line_for_taxes_computation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._prepare_tax_line_for_taxes_computation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:1724`). Decoradores: api.model.
    async fn _add_tax_details_in_base_line(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._add_tax_details_in_base_line".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:1798`). Decoradores: api.model.
    async fn _add_tax_details_in_base_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._add_tax_details_in_base_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:1811`). Decoradores: api.model.
    async fn _normalize_target_factors(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._normalize_target_factors".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:1827`). Decoradores: api.model.
    async fn _distribute_delta_amount_smoothly(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._distribute_delta_amount_smoothly".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:1881`). Decoradores: api.model.
    async fn _round_tax_details_tax_amounts(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._round_tax_details_tax_amounts".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:1979`). Decoradores: api.model.
    async fn _round_tax_details_base_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._round_tax_details_base_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:2083`). Decoradores: api.model.
    async fn _round_tax_details_tax_amounts_from_tax_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._round_tax_details_tax_amounts_from_tax_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:2162`). Decoradores: api.model.
    async fn _round_base_lines_tax_details(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._round_base_lines_tax_details".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:2274`). Decoradores: api.model.
    async fn _prepare_base_line_grouping_key(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._prepare_base_line_grouping_key".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:2292`). Decoradores: api.model.
    async fn _prepare_base_line_tax_repartition_grouping_key(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._prepare_base_line_tax_repartition_grouping_key".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:2324`). Decoradores: api.model.
    async fn _prepare_tax_line_repartition_grouping_key(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._prepare_tax_line_repartition_grouping_key".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:2346`). Decoradores: api.model.
    async fn _add_accounting_data_to_base_line_tax_details(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._add_accounting_data_to_base_line_tax_details".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:2482`). Decoradores: api.model.
    async fn _add_accounting_data_in_base_lines_tax_details(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._add_accounting_data_in_base_lines_tax_details".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:2499`). Decoradores: api.model.
    async fn _aggregate_base_line_tax_details(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._aggregate_base_line_tax_details".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:2625`). Decoradores: api.model.
    async fn _aggregate_base_lines_tax_details(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._aggregate_base_lines_tax_details".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:2642`). Decoradores: api.model.
    async fn _aggregate_base_lines_aggregated_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._aggregate_base_lines_aggregated_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:2693`). Decoradores: api.model.
    async fn _get_tax_totals_summary(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._get_tax_totals_summary".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:2975`). Decoradores: api.model.
    async fn _exclude_tax_groups_from_tax_totals_summary(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._exclude_tax_groups_from_tax_totals_summary".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:3016`). Decoradores: api.model.
    async fn _prepare_tax_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._prepare_tax_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:3115`).
    async fn _can_be_discounted(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._can_be_discounted".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:3127`). Decoradores: api.model.
    async fn _merge_tax_details(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._merge_tax_details".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:3181`). Decoradores: api.model.
    async fn _fix_base_lines_tax_details_on_manual_tax_amounts(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._fix_base_lines_tax_details_on_manual_tax_amounts".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:3224`). Decoradores: api.model.
    async fn _split_tax_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._split_tax_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:3280`). Decoradores: api.model.
    async fn _split_tax_details(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._split_tax_details".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:3354`). Decoradores: api.model.
    async fn _split_base_line(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._split_base_line".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:3388`). Decoradores: api.model.
    async fn _compute_subset_base_lines_total(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._compute_subset_base_lines_total".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:3440`). Decoradores: api.model.
    async fn _reduce_base_lines_with_grouping_function(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._reduce_base_lines_with_grouping_function".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:3530`). Decoradores: api.model.
    async fn _apply_base_lines_manual_amounts_to_reach(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._apply_base_lines_manual_amounts_to_reach".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:3634`). Decoradores: api.model.
    async fn _reduce_base_lines_to_target_amount(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._reduce_base_lines_to_target_amount".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:3863`). Decoradores: api.model.
    async fn _partition_base_lines_taxes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._partition_base_lines_taxes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:3894`). Decoradores: api.model.
    async fn _prepare_discountable_base_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._prepare_discountable_base_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:3918`). Decoradores: api.model.
    async fn _prepare_global_discount_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._prepare_global_discount_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:3962`). Decoradores: api.model.
    async fn _prepare_base_lines_for_down_payment(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._prepare_base_lines_for_down_payment".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:3984`). Decoradores: api.model.
    async fn _prepare_down_payment_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._prepare_down_payment_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:4028`). Decoradores: api.model.
    async fn _dispatch_taxes_into_new_base_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._dispatch_taxes_into_new_base_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:4186`). Decoradores: api.model.
    async fn _turn_removed_taxes_into_new_base_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._turn_removed_taxes_into_new_base_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:4209`). Decoradores: api.model.
    async fn _dispatch_global_discount_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._dispatch_global_discount_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:4276`). Decoradores: api.model.
    async fn _squash_global_discount_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._squash_global_discount_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:4298`). Decoradores: api.model.
    async fn _dispatch_return_of_merchandise_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._dispatch_return_of_merchandise_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:4397`). Decoradores: api.model.
    async fn _squash_return_of_merchandise_lines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._squash_return_of_merchandise_lines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:4424`). Decoradores: api.model.
    async fn _get_delta_amount_to_reach_target(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._get_delta_amount_to_reach_target".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:4467`). Decoradores: api.model.
    async fn _round_raw_total_excluded(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._round_raw_total_excluded".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:4539`). Decoradores: api.model.
    async fn _add_and_round_raw_gross_total_excluded_and_discount(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._add_and_round_raw_gross_total_excluded_and_discount".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:4659`). Decoradores: api.model.
    async fn _round_raw_gross_total_excluded_and_discount(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._round_raw_gross_total_excluded_and_discount".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:4738`). Decoradores: api.model.
    async fn _round_raw_tax_amounts(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._round_raw_tax_amounts".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:4839`).
    async fn flatten_taxes_hierarchy(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax.flatten_taxes_hierarchy".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:4842`).
    async fn get_tax_tags(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax.get_tax_tags".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:4848`).
    async fn compute_all(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax.compute_all".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:4966`).
    async fn _filter_taxes_by_company(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._filter_taxes_by_company".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:4979`). Decoradores: api.model.
    async fn _fix_tax_included_price(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._fix_tax_included_price".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:4990`). Decoradores: api.model.
    async fn _fix_tax_included_price_company(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._fix_tax_included_price_company".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:4997`).
    async fn _get_description_plaintext(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax._get_description_plaintext".into(),
        ))
    }

}
