//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.tax.repartition.line`

use nexus_orm::prelude::*;

pub struct AccountTaxRepartitionLineFragment;

#[async_trait]
impl ModelFragment for AccountTaxRepartitionLineFragment {
    fn model_name(&self) -> &str {
        "account.tax.repartition.line"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Tax Repartition Line".into();
        def.order = "document_type, repartition_type, sequence, id".into();
        def.add_field(FieldDef::float("factor_percent").string("%").required().default_val(100i64));
        def.add_field(FieldDef::float("factor").string("Factor Ratio").computed("_compute_factor", &["factor_percent"]).stored());
        def.add_field(FieldDef::selection("repartition_type", &[("base", "Base"), ("tax", "of tax")]).string("Based On").required().default_val("tax"));
        def.add_field(FieldDef::selection("document_type", &[("invoice", "Invoice"), ("refund", "Refund")]).string("Related to").required());
        def.add_field(FieldDef::many2one("account_id", "account.account").string("Account"));
        def.add_field(FieldDef::many2many("tag_ids", "account.account.tag").string("Tax Grids"));
        def.add_field(FieldDef::many2one("tax_id", "account.tax"));
        def.add_field({ let mut f = FieldDef::many2one("company_id", "res.company").string("Company"); f.related = Some("tax_id.company_id".into()); f });
        def.add_field(FieldDef::integer("sequence").string("Sequence").default_val(1i64));
        def.add_field(FieldDef::boolean("use_in_tax_closing").string("Tax Closing Entry").computed("_compute_use_in_tax_closing", &["account_id", "repartition_type"]).stored());
        def.add_field(FieldDef::new("tag_ids_domain", FieldType::Binary).string("tag domain").computed("_compute_tag_ids_domain", &["company_id.multi_vat_foreign_country_ids", "company_id.account_fiscal_country_id"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_tag_ids_domain", "_compute_use_in_tax_closing", "_compute_factor", "_onchange_repartition_type", "_get_aml_target_tax_account"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_tag_ids_domain" => self._compute_tag_ids_domain(env, ctx, rs, args).await,
            "_compute_use_in_tax_closing" => self._compute_use_in_tax_closing(env, ctx, rs, args).await,
            "_compute_factor" => self._compute_factor(env, ctx, rs, args).await,
            "_onchange_repartition_type" => self._onchange_repartition_type(env, ctx, rs, args).await,
            "_get_aml_target_tax_account" => self._get_aml_target_tax_account(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountTaxRepartitionLineFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:5039`). Decoradores: api.depends('company_id.multi_vat_foreign_country_ids', 'company_id.account_fiscal_country_id').
    async fn _compute_tag_ids_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax.repartition.line._compute_tag_ids_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:5045`). Decoradores: api.depends('account_id', 'repartition_type').
    async fn _compute_use_in_tax_closing(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax.repartition.line._compute_use_in_tax_closing".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:5054`). Decoradores: api.depends('factor_percent').
    async fn _compute_factor(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax.repartition.line._compute_factor".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:5059`). Decoradores: api.onchange('repartition_type').
    async fn _onchange_repartition_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax.repartition.line._onchange_repartition_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:5063`).
    async fn _get_aml_target_tax_account(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax.repartition.line._get_aml_target_tax_account".into(),
        ))
    }

}
