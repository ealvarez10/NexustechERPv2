//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_model_fields.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `ir.model.fields` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct IrModelFieldsExtFragment;

#[async_trait]
impl ModelFragment for IrModelFieldsExtFragment {
    fn model_name(&self) -> &str {
        "ir.model.fields"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::integer("tracking").string("Enable Ordered Tracking"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_reflect_field_params", "_instanciate_attrs", "unlink"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_reflect_field_params" => self._reflect_field_params(env, ctx, rs, args).await,
            "_instanciate_attrs" => self._instanciate_attrs(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl IrModelFieldsExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_model_fields.py:16`).
    async fn _reflect_field_params(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.model.fields._reflect_field_params".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_model_fields.py:29`).
    async fn _instanciate_attrs(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.model.fields._instanciate_attrs".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_model_fields.py:35`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.model.fields.unlink".into(),
        ))
    }

}
