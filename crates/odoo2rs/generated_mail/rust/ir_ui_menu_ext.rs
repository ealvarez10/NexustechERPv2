//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_ui_menu.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `ir.ui.menu` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct IrUiMenuExtFragment;

#[async_trait]
impl ModelFragment for IrUiMenuExtFragment {
    fn model_name(&self) -> &str {
        "ir.ui.menu"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_get_best_backend_root_menu_id_for_model"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_get_best_backend_root_menu_id_for_model" => self._get_best_backend_root_menu_id_for_model(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl IrUiMenuExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_ui_menu.py:13`). Decoradores: api.model.
    async fn _get_best_backend_root_menu_id_for_model(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.ui.menu._get_best_backend_root_menu_id_for_model".into(),
        ))
    }

}
