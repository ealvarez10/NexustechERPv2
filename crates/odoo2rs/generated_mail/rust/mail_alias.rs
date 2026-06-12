//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.alias`

use nexus_orm::prelude::*;

pub struct MailAliasFragment;

#[async_trait]
impl ModelFragment for MailAliasFragment {
    fn model_name(&self) -> &str {
        "mail.alias"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Email Aliases".into();
        def.order = "alias_model_id, alias_name".into();
        def.rec_name = "alias_name".into();
        def.add_field(FieldDef::char("alias_name").string("Alias Name"));
        def.add_field(FieldDef::char("alias_full_name").string("Alias Email").computed("_compute_alias_full_name", &["alias_domain_id.name", "alias_name"]).stored());
        def.add_field(FieldDef::many2one("alias_domain_id", "mail.alias.domain").string("Alias Domain"));
        def.add_field({ let mut f = FieldDef::char("alias_domain").string("Alias domain name"); f.related = Some("alias_domain_id.name".into()); f });
        def.add_field(FieldDef::many2one("alias_model_id", "ir.model").string("Aliased Model").required());
        def.add_field(FieldDef::text("alias_defaults").string("Default Values").required().default_val("{}"));
        def.add_field(FieldDef::integer("alias_force_thread_id").string("Record Thread ID"));
        def.add_field(FieldDef::many2one("alias_parent_model_id", "ir.model").string("Parent Model"));
        def.add_field(FieldDef::integer("alias_parent_thread_id").string("Parent Record Thread ID"));
        def.add_field(FieldDef::selection("alias_contact", &[("everyone", "Everyone"), ("partners", "Authenticated Partners"), ("followers", "Followers only")]).string("Alias Contact Security").required().default_val("everyone"));
        def.add_field(FieldDef::boolean("alias_incoming_local").string("Local-part based incoming detection").default_val(false));
        def.add_field(FieldDef::html("alias_bounced_content").string("Custom Bounced Message"));
        def.add_field(FieldDef::selection("alias_status", &[("not_tested", "Not Tested"), ("valid", "Valid"), ("invalid", "Invalid")]).computed("_compute_alias_status", &["alias_contact", "alias_defaults", "alias_model_id"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_check_alias_domain_id_mc", "_check_alias_is_ascii", "_check_alias_defaults", "_check_alias_domain_clash", "_compute_alias_full_name", "_compute_display_name", "_compute_alias_status", "create", "write", "_check_unique", "_sanitize_allowed_domains", "_sanitize_alias_name", "_is_encodable", "open_document", "open_parent_document", "_get_alias_bounced_body", "_get_alias_bounced_body_fallback", "_get_alias_contact_description", "_get_alias_invalid_body", "_alias_bounce_incoming_email"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_check_alias_domain_id_mc" => self._check_alias_domain_id_mc(env, ctx, rs, args).await,
            "_check_alias_is_ascii" => self._check_alias_is_ascii(env, ctx, rs, args).await,
            "_check_alias_defaults" => self._check_alias_defaults(env, ctx, rs, args).await,
            "_check_alias_domain_clash" => self._check_alias_domain_clash(env, ctx, rs, args).await,
            "_compute_alias_full_name" => self._compute_alias_full_name(env, ctx, rs, args).await,
            "_compute_display_name" => self._compute_display_name(env, ctx, rs, args).await,
            "_compute_alias_status" => self._compute_alias_status(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "_check_unique" => self._check_unique(env, ctx, rs, args).await,
            "_sanitize_allowed_domains" => self._sanitize_allowed_domains(env, ctx, rs, args).await,
            "_sanitize_alias_name" => self._sanitize_alias_name(env, ctx, rs, args).await,
            "_is_encodable" => self._is_encodable(env, ctx, rs, args).await,
            "open_document" => self.open_document(env, ctx, rs, args).await,
            "open_parent_document" => self.open_parent_document(env, ctx, rs, args).await,
            "_get_alias_bounced_body" => self._get_alias_bounced_body(env, ctx, rs, args).await,
            "_get_alias_bounced_body_fallback" => self._get_alias_bounced_body_fallback(env, ctx, rs, args).await,
            "_get_alias_contact_description" => self._get_alias_contact_description(env, ctx, rs, args).await,
            "_get_alias_invalid_body" => self._get_alias_invalid_body(env, ctx, rs, args).await,
            "_alias_bounce_incoming_email" => self._alias_bounce_incoming_email(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailAliasFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py:100`). Decoradores: api.constrains('alias_domain_id', 'alias_force_thread_id', 'alias_parent_model_id', 'alias_parent_thread_id', 'alias_model_id').
    async fn _check_alias_domain_id_mc(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias._check_alias_domain_id_mc".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py:179`). Decoradores: api.constrains('alias_name').
    async fn _check_alias_is_ascii(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias._check_alias_is_ascii".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py:194`). Decoradores: api.constrains('alias_defaults').
    async fn _check_alias_defaults(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias._check_alias_defaults".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py:204`). Decoradores: api.constrains('alias_name', 'alias_domain_id').
    async fn _check_alias_domain_clash(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias._check_alias_domain_clash".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py:217`). Decoradores: api.depends('alias_domain_id.name', 'alias_name').
    async fn _compute_alias_full_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias._compute_alias_full_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py:229`). Decoradores: api.depends('alias_domain', 'alias_name').
    async fn _compute_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias._compute_display_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py:242`). Decoradores: api.depends('alias_contact', 'alias_defaults', 'alias_model_id').
    async fn _compute_alias_status(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias._compute_alias_status".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py:247`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py:265`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py:292`).
    async fn _check_unique(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias._check_unique".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py:352`). Decoradores: api.model.
    async fn _sanitize_allowed_domains(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias._sanitize_allowed_domains".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py:366`). Decoradores: api.model.
    async fn _sanitize_alias_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias._sanitize_alias_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py:396`). Decoradores: api.model.
    async fn _is_encodable(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias._is_encodable".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py:411`).
    async fn open_document(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.open_document".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py:421`).
    async fn open_parent_document(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.open_parent_document".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py:435`).
    async fn _get_alias_bounced_body(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias._get_alias_bounced_body".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py:462`).
    async fn _get_alias_bounced_body_fallback(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias._get_alias_bounced_body_fallback".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py:482`).
    async fn _get_alias_contact_description(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias._get_alias_contact_description".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py:487`).
    async fn _get_alias_invalid_body(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias._get_alias_invalid_body".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias.py:512`).
    async fn _alias_bounce_incoming_email(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias._alias_bounce_incoming_email".into(),
        ))
    }

}
