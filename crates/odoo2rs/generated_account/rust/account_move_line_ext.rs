//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line_tax_details.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.move.line` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct AccountMoveLineExtFragment;

#[async_trait]
impl ModelFragment for AccountMoveLineExtFragment {
    fn model_name(&self) -> &str {
        "account.move.line"
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
        vec!["_get_query_tax_details_from_domain", "_get_extra_query_base_tax_line_mapping", "_get_query_tax_details"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_get_query_tax_details_from_domain" => self._get_query_tax_details_from_domain(env, ctx, rs, args).await,
            "_get_extra_query_base_tax_line_mapping" => self._get_extra_query_base_tax_line_mapping(env, ctx, rs, args).await,
            "_get_query_tax_details" => self._get_query_tax_details(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountMoveLineExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line_tax_details.py:11`). Decoradores: api.model.
    async fn _get_query_tax_details_from_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_query_tax_details_from_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line_tax_details.py:23`). Decoradores: api.model.
    async fn _get_extra_query_base_tax_line_mapping(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_extra_query_base_tax_line_mapping".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_line_tax_details.py:28`). Decoradores: api.model.
    async fn _get_query_tax_details(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.line._get_query_tax_details".into(),
        ))
    }

}
