//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message_link_preview.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.message.link.preview`

use nexus_orm::prelude::*;

pub struct MailMessageLinkPreviewFragment;

#[async_trait]
impl ModelFragment for MailMessageLinkPreviewFragment {
    fn model_name(&self) -> &str {
        "mail.message.link.preview"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Link between link previews and messages".into();
        def.order = "sequence, id".into();
        def.add_field(FieldDef::many2one("message_id", "mail.message").required());
        def.add_field(FieldDef::many2one("link_preview_id", "mail.link.preview").required());
        def.add_field(FieldDef::integer("sequence").string("Sequence"));
        def.add_field(FieldDef::boolean("is_hidden"));
        // TODO(odoo2rs): campo 'author_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_bus_channel", "_hide_and_notify", "_unlink_and_notify", "_to_store_defaults"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_bus_channel" => self._bus_channel(env, ctx, rs, args).await,
            "_hide_and_notify" => self._hide_and_notify(env, ctx, rs, args).await,
            "_unlink_and_notify" => self._unlink_and_notify(env, ctx, rs, args).await,
            "_to_store_defaults" => self._to_store_defaults(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailMessageLinkPreviewFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message_link_preview.py:23`).
    async fn _bus_channel(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.link.preview._bus_channel".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message_link_preview.py:26`).
    async fn _hide_and_notify(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.link.preview._hide_and_notify".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message_link_preview.py:33`).
    async fn _unlink_and_notify(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.link.preview._unlink_and_notify".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_message_link_preview.py:40`).
    async fn _to_store_defaults(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.link.preview._to_store_defaults".into(),
        ))
    }

}
