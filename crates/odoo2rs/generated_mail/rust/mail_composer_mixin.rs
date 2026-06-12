//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_composer_mixin.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.composer.mixin`

use nexus_orm::prelude::*;

pub struct MailComposerMixinFragment;

#[async_trait]
impl ModelFragment for MailComposerMixinFragment {
    fn model_name(&self) -> &str {
        "mail.composer.mixin"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Mail Composer Mixin".into();
        def.add_field(FieldDef::char("subject").string("Subject").computed("_compute_subject", &["template_id"]).stored());
        def.add_field(FieldDef::html("body").string("Contents").computed("_compute_body", &["template_id"]).stored());
        def.add_field(FieldDef::boolean("body_has_template_value").string("Body content is the same as the template").computed("_compute_body_has_template_value", &["body", "template_id"]).stored());
        def.add_field(FieldDef::many2one("template_id", "mail.template").string("Mail Template"));
        def.add_field(FieldDef::char("lang").computed("_compute_lang", &["template_id"]).stored());
        def.add_field(FieldDef::boolean("is_mail_template_editor").string("Is Editor").computed("_compute_is_mail_template_editor", &[]).stored());
        def.add_field(FieldDef::boolean("can_edit_body").string("Can Edit Body").computed("_compute_can_edit_body", &["template_id", "is_mail_template_editor"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_subject", "_compute_body", "_compute_body_has_template_value", "_compute_lang", "_compute_is_mail_template_editor", "_compute_can_edit_body", "_render_lang", "_render_field"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_subject" => self._compute_subject(env, ctx, rs, args).await,
            "_compute_body" => self._compute_body(env, ctx, rs, args).await,
            "_compute_body_has_template_value" => self._compute_body_has_template_value(env, ctx, rs, args).await,
            "_compute_lang" => self._compute_lang(env, ctx, rs, args).await,
            "_compute_is_mail_template_editor" => self._compute_is_mail_template_editor(env, ctx, rs, args).await,
            "_compute_can_edit_body" => self._compute_can_edit_body(env, ctx, rs, args).await,
            "_render_lang" => self._render_lang(env, ctx, rs, args).await,
            "_render_field" => self._render_field(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailComposerMixinFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_composer_mixin.py:40`). Decoradores: api.depends('template_id').
    async fn _compute_subject(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.composer.mixin._compute_subject".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_composer_mixin.py:51`). Decoradores: api.depends('template_id').
    async fn _compute_body(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.composer.mixin._compute_body".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_composer_mixin.py:62`). Decoradores: api.depends('body', 'template_id').
    async fn _compute_body_has_template_value(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.composer.mixin._compute_body_has_template_value".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_composer_mixin.py:88`). Decoradores: api.depends('template_id').
    async fn _compute_lang(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.composer.mixin._compute_lang".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_composer_mixin.py:99`). Decoradores: api.depends_context('uid').
    async fn _compute_is_mail_template_editor(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.composer.mixin._compute_is_mail_template_editor".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_composer_mixin.py:105`). Decoradores: api.depends('template_id', 'is_mail_template_editor').
    async fn _compute_can_edit_body(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.composer.mixin._compute_can_edit_body".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_composer_mixin.py:112`).
    async fn _render_lang(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.composer.mixin._render_lang".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_composer_mixin.py:139`).
    async fn _render_field(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.composer.mixin._render_field".into(),
        ))
    }

}
