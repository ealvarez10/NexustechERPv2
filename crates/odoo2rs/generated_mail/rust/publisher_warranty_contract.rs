//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/update.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `publisher_warranty.contract`

use nexus_orm::prelude::*;

pub struct PublisherWarrantyContractFragment;

#[async_trait]
impl ModelFragment for PublisherWarrantyContractFragment {
    fn model_name(&self) -> &str {
        "publisher_warranty.contract"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Publisher Warranty Contract".into();
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_get_message", "_get_sys_logs", "update_notification"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_get_message" => self._get_message(env, ctx, rs, args).await,
            "_get_sys_logs" => self._get_sys_logs(env, ctx, rs, args).await,
            "update_notification" => self.update_notification(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl PublisherWarrantyContractFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/update.py:23`). Decoradores: api.model.
    async fn _get_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): publisher_warranty.contract._get_message".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/update.py:64`). Decoradores: api.model.
    async fn _get_sys_logs(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): publisher_warranty.contract._get_sys_logs".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/update.py:77`).
    async fn update_notification(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): publisher_warranty.contract.update_notification".into(),
        ))
    }

}
