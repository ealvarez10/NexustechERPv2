//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_domain.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.alias.domain`

use nexus_orm::prelude::*;

pub struct MailAliasDomainFragment;

#[async_trait]
impl ModelFragment for MailAliasDomainFragment {
    fn model_name(&self) -> &str {
        "mail.alias.domain"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Email Domain".into();
        def.order = "sequence ASC, id ASC".into();
        def.add_field(FieldDef::char("name").string("Name").required());
        def.add_field(FieldDef::one2many("company_ids", "res.company", "alias_domain_id").string("Companies"));
        def.add_field(FieldDef::integer("sequence").default_val(10i64));
        def.add_field(FieldDef::char("bounce_alias").string("Bounce Alias").required().default_val("bounce"));
        def.add_field(FieldDef::char("bounce_email").string("Bounce Email").computed("_compute_bounce_email", &["bounce_alias", "name"]).stored());
        def.add_field(FieldDef::char("catchall_alias").string("Catchall Alias").required().default_val("catchall"));
        def.add_field(FieldDef::char("catchall_email").string("Catchall Email").computed("_compute_catchall_email", &["catchall_alias", "name"]).stored());
        def.add_field(FieldDef::char("default_from").string("Default From Alias").default_val("notifications"));
        def.add_field(FieldDef::char("default_from_email").string("Default From").computed("_compute_default_from_email", &["default_from", "name"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_bounce_email", "_compute_catchall_email", "_compute_default_from_email", "_check_bounce_catchall_uniqueness", "_check_name", "create", "write", "_check_default_from_not_used_by_users", "_sanitize_configuration", "_find_aliases", "_migrate_icp_to_domain"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_bounce_email" => self._compute_bounce_email(env, ctx, rs, args).await,
            "_compute_catchall_email" => self._compute_catchall_email(env, ctx, rs, args).await,
            "_compute_default_from_email" => self._compute_default_from_email(env, ctx, rs, args).await,
            "_check_bounce_catchall_uniqueness" => self._check_bounce_catchall_uniqueness(env, ctx, rs, args).await,
            "_check_name" => self._check_name(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "_check_default_from_not_used_by_users" => self._check_default_from_not_used_by_users(env, ctx, rs, args).await,
            "_sanitize_configuration" => self._sanitize_configuration(env, ctx, rs, args).await,
            "_find_aliases" => self._find_aliases(env, ctx, rs, args).await,
            "_migrate_icp_to_domain" => self._migrate_icp_to_domain(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailAliasDomainFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_domain.py:54`). Decoradores: api.depends('bounce_alias', 'name').
    async fn _compute_bounce_email(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.domain._compute_bounce_email".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_domain.py:60`). Decoradores: api.depends('catchall_alias', 'name').
    async fn _compute_catchall_email(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.domain._compute_catchall_email".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_domain.py:66`). Decoradores: api.depends('default_from', 'name').
    async fn _compute_default_from_email(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.domain._compute_default_from_email".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_domain.py:78`). Decoradores: api.constrains('bounce_alias', 'catchall_alias').
    async fn _check_bounce_catchall_uniqueness(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.domain._check_bounce_catchall_uniqueness".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_domain.py:129`). Decoradores: api.constrains('name').
    async fn _check_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.domain._check_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_domain.py:140`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.domain.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_domain.py:164`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.domain.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_domain.py:171`).
    async fn _check_default_from_not_used_by_users(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.domain._check_default_from_not_used_by_users".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_domain.py:183`). Decoradores: api.model.
    async fn _sanitize_configuration(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.domain._sanitize_configuration".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_domain.py:196`). Decoradores: api.model.
    async fn _find_aliases(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.domain._find_aliases".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_domain.py:238`). Decoradores: api.model.
    async fn _migrate_icp_to_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.domain._migrate_icp_to_domain".into(),
        ))
    }

}
