//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_ice_server.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.ice.server`

use nexus_orm::prelude::*;

pub struct MailIceServerFragment;

#[async_trait]
impl ModelFragment for MailIceServerFragment {
    fn model_name(&self) -> &str {
        "mail.ice.server"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "ICE Server".into();
        def.rec_name = "uri".into();
        def.add_field(FieldDef::selection("server_type", &[("stun", "stun:"), ("turn", "turn:")]).string("Type").required().default_val("stun"));
        def.add_field(FieldDef::char("uri").string("URI").required());
        def.add_field(FieldDef::char("username"));
        def.add_field(FieldDef::char("credential"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_get_local_ice_servers", "_get_ice_servers"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_get_local_ice_servers" => self._get_local_ice_servers(env, ctx, rs, args).await,
            "_get_ice_servers" => self._get_ice_servers(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailIceServerFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_ice_server.py:21`).
    async fn _get_local_ice_servers(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.ice.server._get_local_ice_servers".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_ice_server.py:39`).
    async fn _get_ice_servers(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.ice.server._get_ice_servers".into(),
        ))
    }

}
