//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_mail_server.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `ir.mail_server` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct IrMailServerExtFragment;

#[async_trait]
impl ModelFragment for IrMailServerExtFragment {
    fn model_name(&self) -> &str {
        "ir.mail_server"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::one2many("mail_template_ids", "mail.template", "mail_server_id").string("Mail template using this mail server").readonly());
        def.add_field(FieldDef::many2one("owner_user_id", "res.users").string("Owner"));
        def.add_field(FieldDef::datetime("owner_limit_time").string("Owner Limit Time"));
        def.add_field(FieldDef::integer("owner_limit_count").string("Owner Limit Count"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_active_usages_compute", "_get_default_bounce_address", "_get_default_from_address", "_get_test_email_from", "_filter_mail_servers_fallback", "_find_mail_server_allowed_domain", "_check_forced_mail_server", "_get_personal_mail_servers_limit"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_active_usages_compute" => self._active_usages_compute(env, ctx, rs, args).await,
            "_get_default_bounce_address" => self._get_default_bounce_address(env, ctx, rs, args).await,
            "_get_default_from_address" => self._get_default_from_address(env, ctx, rs, args).await,
            "_get_test_email_from" => self._get_test_email_from(env, ctx, rs, args).await,
            "_filter_mail_servers_fallback" => self._filter_mail_servers_fallback(env, ctx, rs, args).await,
            "_find_mail_server_allowed_domain" => self._find_mail_server_allowed_domain(env, ctx, rs, args).await,
            "_check_forced_mail_server" => self._check_forced_mail_server(env, ctx, rs, args).await,
            "_get_personal_mail_servers_limit" => self._get_personal_mail_servers_limit(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl IrMailServerExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_mail_server.py:34`).
    async fn _active_usages_compute(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.mail_server._active_usages_compute".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_mail_server.py:44`). Decoradores: api.model.
    async fn _get_default_bounce_address(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.mail_server._get_default_bounce_address".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_mail_server.py:52`). Decoradores: api.model.
    async fn _get_default_from_address(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.mail_server._get_default_from_address".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_mail_server.py:59`).
    async fn _get_test_email_from(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.mail_server._get_test_email_from".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_mail_server.py:81`). Decoradores: api.model.
    async fn _filter_mail_servers_fallback(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.mail_server._filter_mail_servers_fallback".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_mail_server.py:84`).
    async fn _find_mail_server_allowed_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.mail_server._find_mail_server_allowed_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_mail_server.py:90`).
    async fn _check_forced_mail_server(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.mail_server._check_forced_mail_server".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_mail_server.py:101`).
    async fn _get_personal_mail_servers_limit(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.mail_server._get_personal_mail_servers_limit".into(),
        ))
    }

}
