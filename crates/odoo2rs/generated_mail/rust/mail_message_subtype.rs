//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message_subtype.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.message.subtype`

use nexus_orm::prelude::*;

pub struct MailMessageSubtypeFragment;

#[async_trait]
impl ModelFragment for MailMessageSubtypeFragment {
    fn model_name(&self) -> &str {
        "mail.message.subtype"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Message subtypes".into();
        def.order = "sequence, id".into();
        def.add_field(FieldDef::char("name").string("Message Type").required());
        def.add_field(FieldDef::text("description").string("Description"));
        def.add_field(FieldDef::boolean("internal").string("Internal Only"));
        def.add_field(FieldDef::many2one("parent_id", "mail.message.subtype").string("Parent"));
        def.add_field(FieldDef::char("relation_field").string("Relation field"));
        def.add_field(FieldDef::char("res_model").string("Model"));
        def.add_field(FieldDef::boolean("default").string("Default").default_val(true));
        def.add_field(FieldDef::integer("sequence").string("Sequence").default_val(1i64));
        def.add_field(FieldDef::boolean("hidden").string("Hidden"));
        def.add_field(FieldDef::boolean("track_recipients").string("Track Recipients"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["create", "write", "unlink", "_get_auto_subscription_subtypes", "default_subtypes", "_default_subtypes"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "_get_auto_subscription_subtypes" => self._get_auto_subscription_subtypes(env, ctx, rs, args).await,
            "default_subtypes" => self.default_subtypes(env, ctx, rs, args).await,
            "_default_subtypes" => self._default_subtypes(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailMessageSubtypeFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message_subtype.py:47`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.subtype.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message_subtype.py:51`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.subtype.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message_subtype.py:55`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.subtype.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message_subtype.py:60`). Decoradores: tools.ormcache('model_name').
    async fn _get_auto_subscription_subtypes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.subtype._get_auto_subscription_subtypes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message_subtype.py:101`). Decoradores: api.model.
    async fn default_subtypes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.subtype.default_subtypes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message_subtype.py:107`). Decoradores: tools.ormcache('self.env.uid', 'self.env.su', 'model_name').
    async fn _default_subtypes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.subtype._default_subtypes".into(),
        ))
    }

}
