//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_mixin_optional.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.alias.mixin.optional`

use nexus_orm::prelude::*;

pub struct MailAliasMixinOptionalFragment;

#[async_trait]
impl ModelFragment for MailAliasMixinOptionalFragment {
    fn model_name(&self) -> &str {
        "mail.alias.mixin.optional"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Email Aliases Mixin (light)".into();
        def.add_field(FieldDef::many2one("alias_id", "mail.alias").string("Alias"));
        def.add_field({ let mut f = FieldDef::char("alias_name"); f.related = Some("alias_id.alias_name".into()); f });
        def.add_field({ let mut f = FieldDef::many2one("alias_domain_id", "mail.alias.domain").string("Alias Domain"); f.related = Some("alias_id.alias_domain_id".into()); f });
        def.add_field({ let mut f = FieldDef::char("alias_domain").string("Alias Domain Name"); f.related = Some("alias_id.alias_domain".into()); f });
        def.add_field({ let mut f = FieldDef::text("alias_defaults"); f.related = Some("alias_id.alias_defaults".into()); f });
        def.add_field(FieldDef::char("alias_email").string("Email Alias").computed("_compute_alias_email", &["alias_domain", "alias_name"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_alias_email", "_search_alias_email", "create", "write", "unlink", "copy_data", "_require_new_alias", "_alias_get_alias_domain_id", "_alias_get_creation_values", "_alias_filter_fields"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_alias_email" => self._compute_alias_email(env, ctx, rs, args).await,
            "_search_alias_email" => self._search_alias_email(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "copy_data" => self.copy_data(env, ctx, rs, args).await,
            "_require_new_alias" => self._require_new_alias(env, ctx, rs, args).await,
            "_alias_get_alias_domain_id" => self._alias_get_alias_domain_id(env, ctx, rs, args).await,
            "_alias_get_creation_values" => self._alias_get_creation_values(env, ctx, rs, args).await,
            "_alias_filter_fields" => self._alias_filter_fields(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailAliasMixinOptionalFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_mixin_optional.py:31`). Decoradores: api.depends('alias_domain', 'alias_name').
    async fn _compute_alias_email(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.mixin.optional._compute_alias_email".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_mixin_optional.py:39`).
    async fn _search_alias_email(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.mixin.optional._search_alias_email".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_mixin_optional.py:47`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.mixin.optional.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_mixin_optional.py:103`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.mixin.optional.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_mixin_optional.py:138`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.mixin.optional.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_mixin_optional.py:145`).
    async fn copy_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.mixin.optional.copy_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_mixin_optional.py:155`). Decoradores: api.model.
    async fn _require_new_alias(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.mixin.optional._require_new_alias".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_mixin_optional.py:164`).
    async fn _alias_get_alias_domain_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.mixin.optional._alias_get_alias_domain_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_mixin_optional.py:181`).
    async fn _alias_get_creation_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.mixin.optional._alias_get_creation_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_mixin_optional.py:193`).
    async fn _alias_filter_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.mixin.optional._alias_filter_fields".into(),
        ))
    }

}
