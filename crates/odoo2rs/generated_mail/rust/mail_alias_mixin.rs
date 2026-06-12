//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_mixin.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.alias.mixin`

use nexus_orm::prelude::*;

pub struct MailAliasMixinFragment;

#[async_trait]
impl ModelFragment for MailAliasMixinFragment {
    fn model_name(&self) -> &str {
        "mail.alias.mixin"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Email Aliases Mixin".into();
        // TODO(odoo2rs): campo 'alias_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::char("alias_name"));
        def.add_field(FieldDef::text("alias_defaults"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_require_new_alias", "_init_column", "_init_column_alias_id"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_require_new_alias" => self._require_new_alias(env, ctx, rs, args).await,
            "_init_column" => self._init_column(env, ctx, rs, args).await,
            "_init_column_alias_id" => self._init_column_alias_id(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailAliasMixinFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_mixin.py:27`).
    async fn _require_new_alias(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.mixin._require_new_alias".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_mixin.py:31`).
    async fn _init_column(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.mixin._init_column".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_alias_mixin.py:39`).
    async fn _init_column_alias_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.alias.mixin._init_column_alias_id".into(),
        ))
    }

}
