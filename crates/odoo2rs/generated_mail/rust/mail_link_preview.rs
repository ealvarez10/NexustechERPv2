//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_link_preview.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.link.preview`

use nexus_orm::prelude::*;

pub struct MailLinkPreviewFragment;

#[async_trait]
impl ModelFragment for MailLinkPreviewFragment {
    fn model_name(&self) -> &str {
        "mail.link.preview"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Store link preview data".into();
        def.add_field(FieldDef::char("source_url").string("URL").required());
        def.add_field(FieldDef::char("og_type").string("Type"));
        def.add_field(FieldDef::char("og_title").string("Title"));
        def.add_field(FieldDef::char("og_site_name").string("Site name"));
        def.add_field(FieldDef::char("og_image").string("Image"));
        def.add_field(FieldDef::text("og_description").string("Description"));
        def.add_field(FieldDef::char("og_mimetype").string("MIME type"));
        def.add_field(FieldDef::char("image_mimetype").string("Image MIME type"));
        def.add_field(FieldDef::datetime("create_date"));
        def.add_field(FieldDef::one2many("message_link_preview_ids", "mail.message.link.preview", "link_preview_id"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_create_from_message_and_notify", "_is_link_preview_enabled", "_is_domain_thottled", "_search_or_create_from_url", "_to_store_defaults"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_create_from_message_and_notify" => self._create_from_message_and_notify(env, ctx, rs, args).await,
            "_is_link_preview_enabled" => self._is_link_preview_enabled(env, ctx, rs, args).await,
            "_is_domain_thottled" => self._is_domain_thottled(env, ctx, rs, args).await,
            "_search_or_create_from_url" => self._search_or_create_from_url(env, ctx, rs, args).await,
            "_to_store_defaults" => self._to_store_defaults(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailLinkPreviewFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_link_preview.py:38`). Decoradores: api.model.
    async fn _create_from_message_and_notify(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.link.preview._create_from_message_and_notify".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_link_preview.py:105`). Decoradores: api.model.
    async fn _is_link_preview_enabled(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.link.preview._is_link_preview_enabled".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_link_preview.py:109`).
    async fn _is_domain_thottled(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.link.preview._is_domain_thottled".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_link_preview.py:121`). Decoradores: api.model.
    async fn _search_or_create_from_url(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.link.preview._search_or_create_from_url".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_link_preview.py:133`).
    async fn _to_store_defaults(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.link.preview._to_store_defaults".into(),
        ))
    }

}
