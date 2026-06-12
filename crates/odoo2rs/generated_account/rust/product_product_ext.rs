//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `product.product` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct ProductProductExtFragment;

#[async_trait]
impl ModelFragment for ProductProductExtFragment {
    fn model_name(&self) -> &str {
        "product.product"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::char("tax_string").computed("_compute_tax_string", &["lst_price", "product_tmpl_id", "taxes_id"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_get_product_accounts", "_get_tax_included_unit_price", "_get_tax_included_unit_price_from_price", "_compute_tax_string", "_retrieve_product", "_get_product_domain_search_order"]
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
            "_get_tax_included_unit_price" => self._get_tax_included_unit_price(env, ctx, rs, args).await,
            "_get_tax_included_unit_price_from_price" => self._get_tax_included_unit_price_from_price(env, ctx, rs, args).await,
            "_compute_tax_string" => self._compute_tax_string(env, ctx, rs, args).await,
            "_retrieve_product" => self._retrieve_product(env, ctx, rs, args).await,
            "_get_product_domain_search_order" => self._get_product_domain_search_order(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl ProductProductExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py:213`).
    async fn _get_product_accounts(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.product._get_product_accounts".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py:216`).
    async fn _get_tax_included_unit_price(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.product._get_tax_included_unit_price".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py:267`).
    async fn _get_tax_included_unit_price_from_price(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.product._get_tax_included_unit_price_from_price".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py:290`). Decoradores: api.depends('lst_price', 'product_tmpl_id', 'taxes_id'), api.depends_context('company').
    async fn _compute_tax_string(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.product._compute_tax_string".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py:298`).
    async fn _retrieve_product(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.product._retrieve_product".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py:319`).
    async fn _get_product_domain_search_order(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.product._get_product_domain_search_order".into(),
        ))
    }

}
