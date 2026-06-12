//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/uom_uom.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `uom.uom` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct UomUomExtFragment;

#[async_trait]
impl ModelFragment for UomUomExtFragment {
    fn model_name(&self) -> &str {
        "uom.uom"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::char("fiscal_country_codes").computed("_compute_fiscal_country_codes", &[]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_fiscal_country_codes", "_get_unece_code", "_get_uom_from_unece_code"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_fiscal_country_codes" => self._compute_fiscal_country_codes(env, ctx, rs, args).await,
            "_get_unece_code" => self._get_unece_code(env, ctx, rs, args).await,
            "_get_uom_from_unece_code" => self._get_uom_from_unece_code(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl UomUomExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/uom_uom.py:44`). Decoradores: api.depends_context('allowed_company_ids').
    async fn _compute_fiscal_country_codes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): uom.uom._compute_fiscal_country_codes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/uom_uom.py:48`).
    async fn _get_unece_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): uom.uom._get_unece_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/uom_uom.py:56`). Decoradores: api.model.
    async fn _get_uom_from_unece_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): uom.uom._get_uom_from_unece_code".into(),
        ))
    }

}
