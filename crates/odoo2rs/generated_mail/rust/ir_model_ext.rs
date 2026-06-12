//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_model.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `ir.model` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct IrModelExtFragment;

#[async_trait]
impl ModelFragment for IrModelExtFragment {
    fn model_name(&self) -> &str {
        "ir.model"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.order = "is_mail_thread DESC, name ASC".into();
        def.add_field(FieldDef::boolean("is_mail_thread").string("Has Mail Thread").default_val(false));
        def.add_field(FieldDef::boolean("is_mail_activity").string("Has Mail Activity").default_val(false));
        def.add_field(FieldDef::boolean("is_mail_blacklist").string("Has Mail Blacklist").default_val(false));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["unlink", "write", "_reflect_model_params", "_instanciate_attrs", "_get_definitions", "_get_model_definitions"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "_reflect_model_params" => self._reflect_model_params(env, ctx, rs, args).await,
            "_instanciate_attrs" => self._instanciate_attrs(env, ctx, rs, args).await,
            "_get_definitions" => self._get_definitions(env, ctx, rs, args).await,
            "_get_model_definitions" => self._get_model_definitions(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl IrModelExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_model.py:22`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.model.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_model.py:72`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.model.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_model.py:94`).
    async fn _reflect_model_params(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.model._reflect_model_params".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_model.py:102`). Decoradores: api.model.
    async fn _instanciate_attrs(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.model._instanciate_attrs".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_model.py:120`).
    async fn _get_definitions(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.model._get_definitions".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_model.py:132`).
    async fn _get_model_definitions(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.model._get_model_definitions".into(),
        ))
    }

}
