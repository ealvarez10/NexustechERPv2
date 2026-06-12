//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_company.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `res.company` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct ResCompanyExtFragment;

#[async_trait]
impl ModelFragment for ResCompanyExtFragment {
    fn model_name(&self) -> &str {
        "res.company"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::many2one("alias_domain_id", "mail.alias.domain").string("Email Domain"));
        def.add_field(FieldDef::char("bounce_email").string("Bounce Email").computed("_compute_bounce", &["alias_domain_id", "name"]).stored());
        def.add_field(FieldDef::char("bounce_formatted").string("Bounce").computed("_compute_bounce", &["alias_domain_id", "name"]).stored());
        def.add_field(FieldDef::char("catchall_email").string("Catchall Email").computed("_compute_catchall", &["alias_domain_id", "name"]).stored());
        def.add_field(FieldDef::char("catchall_formatted").string("Catchall").computed("_compute_catchall", &["alias_domain_id", "name"]).stored());
        def.add_field({ let mut f = FieldDef::char("default_from_email").string("Default From").readonly(); f.related = Some("alias_domain_id.default_from_email".into()); f });
        def.add_field(FieldDef::char("email_formatted").string("Formatted Email").computed("_compute_email_formatted", &["partner_id", "catchall_formatted"]).stored());
        def.add_field(FieldDef::char("email_primary_color").string("Email Button Text").default_val("#FFFFFF"));
        def.add_field(FieldDef::char("email_secondary_color").string("Email Button Color").default_val("#875A7B"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_default_alias_domain_id", "_compute_bounce", "_compute_catchall", "_compute_email_formatted"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_default_alias_domain_id" => self._default_alias_domain_id(env, ctx, rs, args).await,
            "_compute_bounce" => self._compute_bounce(env, ctx, rs, args).await,
            "_compute_catchall" => self._compute_catchall(env, ctx, rs, args).await,
            "_compute_email_formatted" => self._compute_email_formatted(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl ResCompanyExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_company.py:10`).
    async fn _default_alias_domain_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._default_alias_domain_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_company.py:36`). Decoradores: api.depends('alias_domain_id', 'name').
    async fn _compute_bounce(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._compute_bounce".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_company.py:46`). Decoradores: api.depends('alias_domain_id', 'name').
    async fn _compute_catchall(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._compute_catchall".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_company.py:56`). Decoradores: api.depends('partner_id', 'catchall_formatted').
    async fn _compute_email_formatted(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.company._compute_email_formatted".into(),
        ))
    }

}
