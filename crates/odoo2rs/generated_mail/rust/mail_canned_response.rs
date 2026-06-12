//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_canned_response.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.canned.response`

use nexus_orm::prelude::*;

pub struct MailCannedResponseFragment;

#[async_trait]
impl ModelFragment for MailCannedResponseFragment {
    fn model_name(&self) -> &str {
        "mail.canned.response"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Canned Response".into();
        def.order = "id desc".into();
        def.rec_name = "source".into();
        def.add_field(FieldDef::char("source").string("Shortcut").required());
        def.add_field(FieldDef::text("substitution").string("Substitution").required());
        def.add_field(FieldDef::datetime("last_used").string("Last Used"));
        def.add_field(FieldDef::many2many("group_ids", "res.groups").string("Authorized Groups"));
        def.add_field(FieldDef::boolean("is_shared").string("Determines if the canned_response is currently shared with other users").computed("_compute_is_shared", &["group_ids"]).stored());
        def.add_field(FieldDef::boolean("is_editable").string("Determines if the canned response can be edited by the current user").computed("_compute_is_editable", &["create_uid"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_is_shared", "_compute_is_editable", "create", "write", "unlink", "_broadcast", "_to_store_defaults"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_is_shared" => self._compute_is_shared(env, ctx, rs, args).await,
            "_compute_is_editable" => self._compute_is_editable(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "_broadcast" => self._broadcast(env, ctx, rs, args).await,
            "_to_store_defaults" => self._to_store_defaults(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailCannedResponseFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_canned_response.py:42`). Decoradores: api.depends('group_ids').
    async fn _compute_is_shared(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.canned.response._compute_is_shared".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_canned_response.py:48`). Decoradores: api.depends_context('uid'), api.depends('create_uid').
    async fn _compute_is_editable(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.canned.response._compute_is_editable".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_canned_response.py:56`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.canned.response.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_canned_response.py:61`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.canned.response.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_canned_response.py:66`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.canned.response.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_canned_response.py:70`).
    async fn _broadcast(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.canned.response._broadcast".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_canned_response.py:84`).
    async fn _to_store_defaults(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.canned.response._to_store_defaults".into(),
        ))
    }

}
