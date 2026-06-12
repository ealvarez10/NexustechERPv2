//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product_catalog_mixin.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `product.catalog.mixin` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct ProductCatalogMixinExtFragment;

#[async_trait]
impl ModelFragment for ProductCatalogMixinExtFragment {
    fn model_name(&self) -> &str {
        "product.catalog.mixin"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_create_section", "_get_new_line_sequence", "_get_sections", "_get_default_create_section_values", "_get_parent_field_on_child_model", "_is_line_valid_for_section_line_count", "_resequence_sections"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_create_section" => self._create_section(env, ctx, rs, args).await,
            "_get_new_line_sequence" => self._get_new_line_sequence(env, ctx, rs, args).await,
            "_get_sections" => self._get_sections(env, ctx, rs, args).await,
            "_get_default_create_section_values" => self._get_default_create_section_values(env, ctx, rs, args).await,
            "_get_parent_field_on_child_model" => self._get_parent_field_on_child_model(env, ctx, rs, args).await,
            "_is_line_valid_for_section_line_count" => self._is_line_valid_for_section_line_count(env, ctx, rs, args).await,
            "_resequence_sections" => self._resequence_sections(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl ProductCatalogMixinExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product_catalog_mixin.py:9`).
    async fn _create_section(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.catalog.mixin._create_section".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product_catalog_mixin.py:48`).
    async fn _get_new_line_sequence(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.catalog.mixin._get_new_line_sequence".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product_catalog_mixin.py:80`).
    async fn _get_sections(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.catalog.mixin._get_sections".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product_catalog_mixin.py:117`).
    async fn _get_default_create_section_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.catalog.mixin._get_default_create_section_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product_catalog_mixin.py:125`).
    async fn _get_parent_field_on_child_model(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.catalog.mixin._get_parent_field_on_child_model".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product_catalog_mixin.py:133`).
    async fn _is_line_valid_for_section_line_count(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.catalog.mixin._is_line_valid_for_section_line_count".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product_catalog_mixin.py:146`).
    async fn _resequence_sections(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): product.catalog.mixin._resequence_sections".into(),
        ))
    }

}
