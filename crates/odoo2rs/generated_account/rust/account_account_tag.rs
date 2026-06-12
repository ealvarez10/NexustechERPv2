//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account_tag.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.account.tag`

use nexus_orm::prelude::*;

pub struct AccountAccountTagFragment;

#[async_trait]
impl ModelFragment for AccountAccountTagFragment {
    fn model_name(&self) -> &str {
        "account.account.tag"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Account Tag".into();
        def.add_field(FieldDef::char("name").string("Tag Name").required());
        def.add_field(FieldDef::selection("applicability", &[("accounts", "Accounts"), ("taxes", "Taxes"), ("products", "Products")]).required().default_val("accounts"));
        def.add_field(FieldDef::integer("color").string("Color Index"));
        def.add_field(FieldDef::boolean("active").default_val(true));
        def.add_field(FieldDef::many2one("country_id", "res.country").string("Country"));
        def.add_field(FieldDef::many2one("report_expression_id", "account.report.expression").computed("_compute_report_expression_id", &["name"]).stored());
        def.add_field(FieldDef::boolean("balance_negate").computed("_compute_report_expression_id", &["name"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_display_name", "_compute_report_expression_id", "_field_to_sql", "create", "_get_tax_tags", "_get_tax_tags_domain", "_get_related_tax_report_expressions", "_unlink_except_master_tags", "_translate_tax_tags"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_display_name" => self._compute_display_name(env, ctx, rs, args).await,
            "_compute_report_expression_id" => self._compute_report_expression_id(env, ctx, rs, args).await,
            "_field_to_sql" => self._field_to_sql(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "_get_tax_tags" => self._get_tax_tags(env, ctx, rs, args).await,
            "_get_tax_tags_domain" => self._get_tax_tags_domain(env, ctx, rs, args).await,
            "_get_related_tax_report_expressions" => self._get_related_tax_report_expressions(env, ctx, rs, args).await,
            "_unlink_except_master_tags" => self._unlink_except_master_tags(env, ctx, rs, args).await,
            "_translate_tax_tags" => self._translate_tax_tags(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountAccountTagFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account_tag.py:29`). Decoradores: api.depends('applicability', 'country_id'), api.depends_context('company').
    async fn _compute_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account.tag._compute_display_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account_tag.py:40`). Decoradores: api.depends('name').
    async fn _compute_report_expression_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account.tag._compute_report_expression_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account_tag.py:50`).
    async fn _field_to_sql(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account.tag._field_to_sql".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account_tag.py:71`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account.tag.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account_tag.py:78`). Decoradores: api.model.
    async fn _get_tax_tags(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account.tag._get_tax_tags".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account_tag.py:88`). Decoradores: api.model.
    async fn _get_tax_tags_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account.tag._get_tax_tags_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account_tag.py:98`).
    async fn _get_related_tax_report_expressions(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account.tag._get_related_tax_report_expressions".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account_tag.py:111`). Decoradores: api.ondelete().
    async fn _unlink_except_master_tags(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account.tag._unlink_except_master_tags".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account_tag.py:122`).
    async fn _translate_tax_tags(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.account.tag._translate_tax_tags".into(),
        ))
    }

}
