//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.report`

use nexus_orm::prelude::*;

pub struct AccountReportFragment;

#[async_trait]
impl ModelFragment for AccountReportFragment {
    fn model_name(&self) -> &str {
        "account.report"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Accounting Report".into();
        def.order = "sequence, id".into();
        def.add_field(FieldDef::char("name").string("Name").required());
        def.add_field(FieldDef::integer("sequence").string("Sequence"));
        def.add_field(FieldDef::boolean("active").string("Active").default_val(true));
        def.add_field(FieldDef::one2many("line_ids", "account.report.line", "report_id").string("Lines"));
        def.add_field(FieldDef::one2many("column_ids", "account.report.column", "report_id").string("Columns"));
        def.add_field(FieldDef::many2one("root_report_id", "account.report").string("Root Report"));
        def.add_field(FieldDef::one2many("variant_report_ids", "account.report", "root_report_id").string("Variants"));
        def.add_field(FieldDef::many2many("section_report_ids", "account.report").string("Sections"));
        def.add_field(FieldDef::many2many("section_main_report_ids", "account.report").string("Section Of"));
        def.add_field(FieldDef::boolean("use_sections").string("Composite Report").computed("_compute_use_sections", &["section_report_ids"]).stored());
        def.add_field(FieldDef::selection("chart_template", &[]).string("Chart of Accounts"));
        def.add_field(FieldDef::many2one("country_id", "res.country").string("Country"));
        def.add_field(FieldDef::boolean("only_tax_exigible").string("Only Tax Exigible Lines"));
        def.add_field(FieldDef::selection("availability_condition", &[("country", "Country Matches"), ("coa", "Chart of Accounts Matches"), ("always", "Always")]).string("Availability").computed("_compute_default_availability_condition", &["root_report_id", "country_id"]).stored());
        def.add_field(FieldDef::integer("load_more_limit").string("Load More Limit"));
        def.add_field(FieldDef::boolean("search_bar").string("Search Bar"));
        def.add_field(FieldDef::integer("prefix_groups_threshold").string("Prefix Groups Threshold").default_val(4000i64));
        def.add_field(FieldDef::selection("integer_rounding", &[("HALF-UP", "Nearest"), ("UP", "Up"), ("DOWN", "Down")]).string("Integer Rounding"));
        def.add_field(FieldDef::boolean("allow_foreign_vat").string("Allow Foreign VAT"));
        def.add_field(FieldDef::selection("default_opening_date_filter", &[("this_year", "This Year"), ("this_quarter", "This Quarter"), ("this_month", "This Month"), ("today", "Today"), ("previous_month", "Last Month"), ("previous_quarter", "Last Quarter"), ("previous_year", "Last Year"), ("this_return_period", "This Return Period"), ("previous_return_period", "Last Return Period")]).string("Default Opening"));
        def.add_field(FieldDef::selection("currency_translation", &[("current", "Use the most recent rate at the date of the report"), ("cta", "Use CTA")]).string("Currency Translation"));
        def.add_field(FieldDef::selection("filter_multi_company", &[("selector", "Use Company Selector"), ("tax_units", "Use Tax Units")]).string("Multi-Company"));
        def.add_field(FieldDef::boolean("filter_date_range").string("Date Range"));
        def.add_field(FieldDef::boolean("filter_show_draft").string("Draft Entries"));
        def.add_field(FieldDef::boolean("filter_unreconciled").string("Unreconciled Entries"));
        def.add_field(FieldDef::boolean("filter_unfold_all").string("Unfold All"));
        def.add_field(FieldDef::selection("filter_hide_0_lines", &[("by_default", "Enabled by Default"), ("optional", "Optional"), ("never", "Never")]).string("Hide lines at 0"));
        def.add_field(FieldDef::boolean("filter_period_comparison").string("Period Comparison"));
        def.add_field(FieldDef::boolean("filter_growth_comparison").string("Growth Comparison"));
        def.add_field(FieldDef::boolean("filter_journals").string("Journals"));
        def.add_field(FieldDef::boolean("filter_analytic").string("Analytic Filter"));
        def.add_field(FieldDef::selection("filter_hierarchy", &[("by_default", "Enabled by Default"), ("optional", "Optional"), ("never", "Never")]).string("Account Groups"));
        def.add_field(FieldDef::selection("filter_account_type", &[("both", "Payable and receivable"), ("payable", "Payable"), ("receivable", "Receivable"), ("disabled", "Disabled")]).string("Account Types"));
        def.add_field(FieldDef::boolean("filter_partner").string("Partners"));
        def.add_field(FieldDef::boolean("filter_aml_ir_filters").string("Favorite Filters"));
        def.add_field(FieldDef::boolean("filter_budgets").string("Budgets"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_report_option_filter", "_compute_default_availability_condition", "_compute_use_sections", "_validate_root_report_id", "_validate_parent_sequence", "_validate_section_report_ids", "_validate_availability_condition", "_onchange_availability_condition", "write", "copy_data", "copy", "_unlink_if_no_variant", "_get_copied_name", "_compute_display_name"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_report_option_filter" => self._compute_report_option_filter(env, ctx, rs, args).await,
            "_compute_default_availability_condition" => self._compute_default_availability_condition(env, ctx, rs, args).await,
            "_compute_use_sections" => self._compute_use_sections(env, ctx, rs, args).await,
            "_validate_root_report_id" => self._validate_root_report_id(env, ctx, rs, args).await,
            "_validate_parent_sequence" => self._validate_parent_sequence(env, ctx, rs, args).await,
            "_validate_section_report_ids" => self._validate_section_report_ids(env, ctx, rs, args).await,
            "_validate_availability_condition" => self._validate_availability_condition(env, ctx, rs, args).await,
            "_onchange_availability_condition" => self._onchange_availability_condition(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "copy_data" => self.copy_data(env, ctx, rs, args).await,
            "copy" => self.copy(env, ctx, rs, args).await,
            "_unlink_if_no_variant" => self._unlink_if_no_variant(env, ctx, rs, args).await,
            "_get_copied_name" => self._get_copied_name(env, ctx, rs, args).await,
            "_compute_display_name" => self._compute_display_name(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountReportFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:201`).
    async fn _compute_report_option_filter(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report._compute_report_option_filter".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:221`). Decoradores: api.depends('root_report_id', 'country_id').
    async fn _compute_default_availability_condition(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report._compute_default_availability_condition".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:229`). Decoradores: api.depends('section_report_ids').
    async fn _compute_use_sections(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report._compute_use_sections".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:234`). Decoradores: api.constrains('root_report_id').
    async fn _validate_root_report_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report._validate_root_report_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:240`). Decoradores: api.constrains('line_ids').
    async fn _validate_parent_sequence(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report._validate_parent_sequence".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:250`). Decoradores: api.constrains('section_report_ids').
    async fn _validate_section_report_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report._validate_section_report_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:256`). Decoradores: api.constrains('availability_condition', 'country_id').
    async fn _validate_availability_condition(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report._validate_availability_condition".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:262`). Decoradores: api.onchange('availability_condition').
    async fn _onchange_availability_condition(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report._onchange_availability_condition".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:266`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:290`).
    async fn copy_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.copy_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:294`).
    async fn copy(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.copy".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:324`). Decoradores: api.ondelete().
    async fn _unlink_if_no_variant(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report._unlink_if_no_variant".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:328`).
    async fn _get_copied_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report._get_copied_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:341`). Decoradores: api.depends('name', 'country_id').
    async fn _compute_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report._compute_display_name".into(),
        ))
    }

}
