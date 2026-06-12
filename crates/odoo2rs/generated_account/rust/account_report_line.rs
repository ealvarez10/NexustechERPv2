//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.report.line`

use nexus_orm::prelude::*;

pub struct AccountReportLineFragment;

#[async_trait]
impl ModelFragment for AccountReportLineFragment {
    fn model_name(&self) -> &str {
        "account.report.line"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Accounting Report Line".into();
        def.order = "sequence, id".into();
        def.add_field(FieldDef::char("name").string("Name").required());
        def.add_field(FieldDef::one2many("expression_ids", "account.report.expression", "report_line_id").string("Expressions"));
        def.add_field(FieldDef::many2one("report_id", "account.report").string("Parent Report").required().computed("_compute_report_id", &["parent_id.report_id"]).stored());
        def.add_field(FieldDef::integer("hierarchy_level").string("Level").required().computed("_compute_hierarchy_level", &["parent_id.hierarchy_level"]).stored());
        def.add_field(FieldDef::many2one("parent_id", "account.report.line").string("Parent Line"));
        def.add_field(FieldDef::one2many("children_ids", "account.report.line", "parent_id").string("Child Lines"));
        def.add_field(FieldDef::char("groupby").string("Group By"));
        def.add_field(FieldDef::char("user_groupby").string("User Group By").computed("_compute_user_groupby", &["groupby", "expression_ids.engine"]).stored());
        def.add_field(FieldDef::integer("sequence").string("Sequence"));
        def.add_field(FieldDef::char("code").string("Code"));
        def.add_field(FieldDef::boolean("foldable").string("Foldable"));
        def.add_field(FieldDef::boolean("print_on_new_page").string("Print On New Page"));
        def.add_field(FieldDef::many2one("action_id", "ir.actions.actions").string("Action"));
        def.add_field(FieldDef::boolean("hide_if_zero").string("Hide if Zero"));
        def.add_field({ let mut f = FieldDef::char("domain_formula").string("Domain Formula Shortcut"); f.store = false; f });
        def.add_field({ let mut f = FieldDef::char("account_codes_formula").string("Account Codes Formula Shortcut"); f.store = false; f });
        def.add_field({ let mut f = FieldDef::char("aggregation_formula").string("Aggregation Formula Shortcut"); f.store = false; f });
        def.add_field({ let mut f = FieldDef::char("external_formula").string("External Formula Shortcut"); f.store = false; f });
        def.add_field(FieldDef::selection("horizontal_split_side", &[("left", "Left"), ("right", "Right")]).string("Horizontal Split Side").computed("_compute_horizontal_split_side", &["parent_id.horizontal_split_side"]).stored());
        def.add_field({ let mut f = FieldDef::char("tax_tags_formula").string("Tax Tags Formula Shortcut"); f.store = false; f });
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_hierarchy_level", "_compute_report_id", "_compute_horizontal_split_side", "_compute_user_groupby", "_validate_groupby_no_child", "_validate_groupby", "_check_parent_line", "_copy_hierarchy", "_get_copied_code", "_inverse_domain_formula", "_inverse_aggregation_formula", "_inverse_aggregation_tax_formula", "_inverse_account_codes_formula", "_inverse_external_formula", "_create_report_expression", "_unlink_child_expressions"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_hierarchy_level" => self._compute_hierarchy_level(env, ctx, rs, args).await,
            "_compute_report_id" => self._compute_report_id(env, ctx, rs, args).await,
            "_compute_horizontal_split_side" => self._compute_horizontal_split_side(env, ctx, rs, args).await,
            "_compute_user_groupby" => self._compute_user_groupby(env, ctx, rs, args).await,
            "_validate_groupby_no_child" => self._validate_groupby_no_child(env, ctx, rs, args).await,
            "_validate_groupby" => self._validate_groupby(env, ctx, rs, args).await,
            "_check_parent_line" => self._check_parent_line(env, ctx, rs, args).await,
            "_copy_hierarchy" => self._copy_hierarchy(env, ctx, rs, args).await,
            "_get_copied_code" => self._get_copied_code(env, ctx, rs, args).await,
            "_inverse_domain_formula" => self._inverse_domain_formula(env, ctx, rs, args).await,
            "_inverse_aggregation_formula" => self._inverse_aggregation_formula(env, ctx, rs, args).await,
            "_inverse_aggregation_tax_formula" => self._inverse_aggregation_tax_formula(env, ctx, rs, args).await,
            "_inverse_account_codes_formula" => self._inverse_account_codes_formula(env, ctx, rs, args).await,
            "_inverse_external_formula" => self._inverse_external_formula(env, ctx, rs, args).await,
            "_create_report_expression" => self._create_report_expression(env, ctx, rs, args).await,
            "_unlink_child_expressions" => self._unlink_child_expressions(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountReportLineFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:404`). Decoradores: api.depends('parent_id.hierarchy_level').
    async fn _compute_hierarchy_level(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.line._compute_hierarchy_level".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:413`). Decoradores: api.depends('parent_id.report_id').
    async fn _compute_report_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.line._compute_report_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:419`). Decoradores: api.depends('parent_id.horizontal_split_side').
    async fn _compute_horizontal_split_side(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.line._compute_horizontal_split_side".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:425`). Decoradores: api.depends('groupby', 'expression_ids.engine').
    async fn _compute_user_groupby(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.line._compute_user_groupby".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:435`). Decoradores: api.constrains('parent_id').
    async fn _validate_groupby_no_child(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.line._validate_groupby_no_child".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:441`). Decoradores: api.constrains('groupby', 'user_groupby').
    async fn _validate_groupby(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.line._validate_groupby".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:445`). Decoradores: api.constrains('parent_id').
    async fn _check_parent_line(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.line._check_parent_line".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:449`).
    async fn _copy_hierarchy(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.line._copy_hierarchy".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:480`).
    async fn _get_copied_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.line._get_copied_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:493`).
    async fn _inverse_domain_formula(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.line._inverse_domain_formula".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:496`).
    async fn _inverse_aggregation_formula(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.line._inverse_aggregation_formula".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:499`).
    async fn _inverse_aggregation_tax_formula(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.line._inverse_aggregation_tax_formula".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:502`).
    async fn _inverse_account_codes_formula(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.line._inverse_account_codes_formula".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:505`).
    async fn _inverse_external_formula(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.line._inverse_external_formula".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:508`).
    async fn _create_report_expression(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.line._create_report_expression".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py:569`). Decoradores: api.ondelete().
    async fn _unlink_child_expressions(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.report.line._unlink_child_expressions".into(),
        ))
    }

}
