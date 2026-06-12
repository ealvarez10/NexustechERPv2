//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.report.expression`

use nexus_orm::prelude::*;

pub struct AccountReportExpressionFragment;

#[async_trait]
impl ModelFragment for AccountReportExpressionFragment {
    fn model_name(&self) -> &str {
        "account.report.expression"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Accounting Report Expression".into();
        def.rec_name = "report_line_name".into();
        def.add_field(FieldDef::many2one("report_line_id", "account.report.line").string("Report Line").required());
        def.add_field({ let mut f = FieldDef::char("report_line_name").string("Report Line Name"); f.related = Some("report_line_id.name".into()); f });
        def.add_field(FieldDef::char("label").string("Label").required());
        def.add_field(FieldDef::selection("engine", &[("domain", "Odoo Domain"), ("tax_tags", "Tax Tags"), ("aggregation", "Aggregate Other Formulas"), ("account_codes", "Prefix of Account Codes"), ("external", "External Value"), ("custom", "Custom Python Function")]).string("Computation Engine").required());
        def.add_field(FieldDef::char("formula").string("Formula").required());
        def.add_field(FieldDef::char("subformula").string("Subformula"));
        def.add_field(FieldDef::selection("date_scope", &[("from_beginning", "From the very start"), ("from_fiscalyear", "From the start of the fiscal year"), ("to_beginning_of_fiscalyear", "At the beginning of the fiscal year"), ("to_beginning_of_period", "At the beginning of the period"), ("strict_range", "Strictly on the given dates"), ("previous_return_period", "From previous return period")]).string("Date Scope").required().default_val("strict_range"));
        def.add_field(FieldDef::selection("figure_type", &[]).string("Figure Type"));
        def.add_field(FieldDef::boolean("green_on_positive").string("Is Growth Good when Positive").default_val(true));
        def.add_field(FieldDef::boolean("blank_if_zero").string("Blank if Zero"));
        def.add_field(FieldDef::boolean("auditable").string("Auditable").computed("_compute_auditable", &["engine"]).stored());
        def.add_field(FieldDef::char("carryover_target").string("Carry Over To"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_check_carryover_target", "_check_formula", "_compute_auditable", "_validate_engine", "_get_auditable_engines", "_strip_formula", "_create_tax_tags", "create", "write", "_unlink_archive_used_tags", "_compute_display_name", "_expand_aggregations", "_get_aggregation_terms_details", "_get_matching_tags", "_get_tags_create_vals", "_get_carryover_target_expression"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_check_carryover_target" => self._check_carryover_target(env, ctx, rs, args).await,
            "_check_formula" => self._check_formula(env, ctx, rs, args).await,
            "_compute_auditable" => self._compute_auditable(env, ctx, rs, args).await,
            "_validate_engine" => self._validate_engine(env, ctx, rs, args).await,
            "_get_auditable_engines" => self._get_auditable_engines(env, ctx, rs, args).await,
            "_strip_formula" => self._strip_formula(env, ctx, rs, args).await,
            "_create_tax_tags" => self._create_tax_tags(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "_unlink_archive_used_tags" => self._unlink_archive_used_tags(env, ctx, rs, args).await,
            "_compute_display_name" => self._compute_display_name(env, ctx, rs, args).await,
            "_expand_aggregations" => self._expand_aggregations(env, ctx, rs, args).await,
            "_get_aggregation_terms_details" => self._get_aggregation_terms_details(env, ctx, rs, args).await,
            "_get_matching_tags" => self._get_matching_tags(env, ctx, rs, args).await,
            "_get_tags_create_vals" => self._get_tags_create_vals(env, ctx, rs, args).await,
            "_get_carryover_target_expression" => self._get_carryover_target_expression(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountReportExpressionFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:636`). Decoradores: api.constrains('carryover_target', 'label').
    async fn _check_carryover_target(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.expression._check_carryover_target".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:644`). Decoradores: api.constrains('formula').
    async fn _check_formula(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.expression._check_formula".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:672`). Decoradores: api.depends('engine').
    async fn _compute_auditable(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.expression._compute_auditable".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:678`). Decoradores: api.constrains('engine', 'report_line_id').
    async fn _validate_engine(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.expression._validate_engine".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:688`).
    async fn _get_auditable_engines(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.expression._get_auditable_engines".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:691`).
    async fn _strip_formula(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.expression._strip_formula".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:695`).
    async fn _create_tax_tags(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.expression._create_tax_tags".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:702`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.expression.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:718`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.expression.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:766`). Decoradores: api.ondelete().
    async fn _unlink_archive_used_tags(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.expression._unlink_archive_used_tags".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:795`). Decoradores: api.depends('report_line_name', 'label').
    async fn _compute_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.expression._compute_display_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:800`).
    async fn _expand_aggregations(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.expression._expand_aggregations".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:860`).
    async fn _get_aggregation_terms_details(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.expression._get_aggregation_terms_details".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:885`).
    async fn _get_matching_tags(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.expression._get_matching_tags".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:900`). Decoradores: api.model.
    async fn _get_tags_create_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.expression._get_tags_create_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:911`).
    async fn _get_carryover_target_expression(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.expression._get_carryover_target_expression".into(),
        ))
    }

}
