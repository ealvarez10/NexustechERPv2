//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `product.template` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct ProductTemplateExtFragment;

#[async_trait]
impl ModelFragment for ProductTemplateExtFragment {
    fn model_name(&self) -> &str {
        "product.template"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::many2many("taxes_id", "account.tax").string("Sales Taxes"));
        def.add_field(FieldDef::char("tax_string").computed("_compute_tax_string", &["taxes_id", "list_price"]).stored());
        def.add_field(FieldDef::many2many("supplier_taxes_id", "account.tax").string("Purchase Taxes"));
        def.add_field(FieldDef::many2one("property_account_income_id", "account.account").string("Income Account"));
        def.add_field(FieldDef::many2one("property_account_expense_id", "account.account").string("Expense Account"));
        def.add_field(FieldDef::many2many("account_tag_ids", "account.account.tag").string("Account Tags"));
        def.add_field(FieldDef::char("fiscal_country_codes").computed("_compute_fiscal_country_codes", &["company_id"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_get_product_accounts", "_get_category_account", "get_product_accounts", "_compute_fiscal_country_codes", "_compute_tax_string", "_construct_tax_string", "_check_uom_not_in_invoice", "_onchange_type", "_force_default_sale_tax", "_force_default_purchase_tax", "_force_default_tax", "create", "_get_list_price"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_get_product_accounts" => self._get_product_accounts(env, ctx, rs, args).await,
            "_get_category_account" => self._get_category_account(env, ctx, rs, args).await,
            "get_product_accounts" => self.get_product_accounts(env, ctx, rs, args).await,
            "_compute_fiscal_country_codes" => self._compute_fiscal_country_codes(env, ctx, rs, args).await,
            "_compute_tax_string" => self._compute_tax_string(env, ctx, rs, args).await,
            "_construct_tax_string" => self._construct_tax_string(env, ctx, rs, args).await,
            "_check_uom_not_in_invoice" => self._check_uom_not_in_invoice(env, ctx, rs, args).await,
            "_onchange_type" => self._onchange_type(env, ctx, rs, args).await,
            "_force_default_sale_tax" => self._force_default_sale_tax(env, ctx, rs, args).await,
            "_force_default_purchase_tax" => self._force_default_purchase_tax(env, ctx, rs, args).await,
            "_force_default_tax" => self._force_default_tax(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "_get_list_price" => self._get_list_price(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl ProductTemplateExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py:65`).
    async fn _get_product_accounts(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.template._get_product_accounts".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py:78`).
    async fn _get_category_account(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.template._get_category_account".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py:91`).
    async fn get_product_accounts(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.template.get_product_accounts".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py:99`). Decoradores: api.depends('company_id'), api.depends_context('allowed_company_ids').
    async fn _compute_fiscal_country_codes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.template._compute_fiscal_country_codes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py:106`). Decoradores: api.depends('taxes_id', 'list_price'), api.depends_context('company').
    async fn _compute_tax_string(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.template._compute_tax_string".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py:110`).
    async fn _construct_tax_string(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.template._construct_tax_string".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py:129`). Decoradores: api.constrains('uom_id').
    async fn _check_uom_not_in_invoice(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.template._check_uom_not_in_invoice".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py:150`). Decoradores: api.onchange('type').
    async fn _onchange_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.template._onchange_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py:156`).
    async fn _force_default_sale_tax(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.template._force_default_sale_tax".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py:166`).
    async fn _force_default_purchase_tax(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.template._force_default_purchase_tax".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py:176`).
    async fn _force_default_tax(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.template._force_default_tax".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py:181`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.template.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py:192`).
    async fn _get_list_price(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.template._get_list_price".into(),
        ))
    }

}
