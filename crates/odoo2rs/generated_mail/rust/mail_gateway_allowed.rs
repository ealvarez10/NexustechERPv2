//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_gateway_allowed.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.gateway.allowed`

use nexus_orm::prelude::*;

pub struct MailGatewayAllowedFragment;

#[async_trait]
impl ModelFragment for MailGatewayAllowedFragment {
    fn model_name(&self) -> &str {
        "mail.gateway.allowed"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Mail Gateway Allowed".into();
        def.add_field(FieldDef::char("email").string("Email Address").required());
        def.add_field(FieldDef::char("email_normalized").string("Normalized Email").computed("_compute_email_normalized", &["email"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_email_normalized", "get_empty_list_help"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_email_normalized" => self._compute_email_normalized(env, ctx, rs, args).await,
            "get_empty_list_help" => self.get_empty_list_help(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailGatewayAllowedFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_gateway_allowed.py:28`). Decoradores: api.depends('email').
    async fn _compute_email_normalized(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.gateway.allowed._compute_email_normalized".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_gateway_allowed.py:33`). Decoradores: api.model.
    async fn get_empty_list_help(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.gateway.allowed.get_empty_list_help".into(),
        ))
    }

}
