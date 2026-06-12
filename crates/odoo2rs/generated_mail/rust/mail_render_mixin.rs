//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.render.mixin`

use nexus_orm::prelude::*;

pub struct MailRenderMixinFragment;

#[async_trait]
impl ModelFragment for MailRenderMixinFragment {
    fn model_name(&self) -> &str {
        "mail.render.mixin"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Mail Render Mixin".into();
        def.add_field(FieldDef::char("lang").string("Language"));
        def.add_field(FieldDef::char("render_model").string("Rendering Model").computed("_compute_render_model", &[]));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_render_model", "_build_expression", "_valid_field_parameter", "create", "write", "_update_field_translations", "_replace_local_links", "_render_encapsulate", "_prepend_preview", "_has_unsafe_expression", "_has_unsafe_expression_template_qweb", "_has_unsafe_expression_template_inline_template", "_check_access_right_dynamic_template", "_render_eval_context", "_render_template_qweb", "_render_template_qweb_regex", "_render_template_qweb_view", "_render_template_inline_template", "_render_template_inline_template_regex", "_render_template_postprocess", "_process_scheduled_date", "_render_template_get_valid_options", "_render_template", "_render_lang", "_classify_per_lang", "_render_field"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_render_model" => self._compute_render_model(env, ctx, rs, args).await,
            "_build_expression" => self._build_expression(env, ctx, rs, args).await,
            "_valid_field_parameter" => self._valid_field_parameter(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "_update_field_translations" => self._update_field_translations(env, ctx, rs, args).await,
            "_replace_local_links" => self._replace_local_links(env, ctx, rs, args).await,
            "_render_encapsulate" => self._render_encapsulate(env, ctx, rs, args).await,
            "_prepend_preview" => self._prepend_preview(env, ctx, rs, args).await,
            "_has_unsafe_expression" => self._has_unsafe_expression(env, ctx, rs, args).await,
            "_has_unsafe_expression_template_qweb" => self._has_unsafe_expression_template_qweb(env, ctx, rs, args).await,
            "_has_unsafe_expression_template_inline_template" => self._has_unsafe_expression_template_inline_template(env, ctx, rs, args).await,
            "_check_access_right_dynamic_template" => self._check_access_right_dynamic_template(env, ctx, rs, args).await,
            "_render_eval_context" => self._render_eval_context(env, ctx, rs, args).await,
            "_render_template_qweb" => self._render_template_qweb(env, ctx, rs, args).await,
            "_render_template_qweb_regex" => self._render_template_qweb_regex(env, ctx, rs, args).await,
            "_render_template_qweb_view" => self._render_template_qweb_view(env, ctx, rs, args).await,
            "_render_template_inline_template" => self._render_template_inline_template(env, ctx, rs, args).await,
            "_render_template_inline_template_regex" => self._render_template_inline_template_regex(env, ctx, rs, args).await,
            "_render_template_postprocess" => self._render_template_postprocess(env, ctx, rs, args).await,
            "_process_scheduled_date" => self._process_scheduled_date(env, ctx, rs, args).await,
            "_render_template_get_valid_options" => self._render_template_get_valid_options(env, ctx, rs, args).await,
            "_render_template" => self._render_template(env, ctx, rs, args).await,
            "_render_lang" => self._render_lang(env, ctx, rs, args).await,
            "_classify_per_lang" => self._classify_per_lang(env, ctx, rs, args).await,
            "_render_field" => self._render_field(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailRenderMixinFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:60`).
    async fn _compute_render_model(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._compute_render_model".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:67`). Decoradores: api.model.
    async fn _build_expression(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._build_expression".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:89`).
    async fn _valid_field_parameter(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._valid_field_parameter".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:94`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:102`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:110`).
    async fn _update_field_translations(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._update_field_translations".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:123`).
    async fn _replace_local_links(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._replace_local_links".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:165`). Decoradores: api.model.
    async fn _render_encapsulate(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._render_encapsulate".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:230`). Decoradores: api.model.
    async fn _prepend_preview(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._prepend_preview".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:257`).
    async fn _has_unsafe_expression(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._has_unsafe_expression".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:270`). Decoradores: api.model.
    async fn _has_unsafe_expression_template_qweb(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._has_unsafe_expression_template_qweb".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:280`). Decoradores: api.model.
    async fn _has_unsafe_expression_template_inline_template(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._has_unsafe_expression_template_inline_template".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:288`).
    async fn _check_access_right_dynamic_template(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._check_access_right_dynamic_template".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:301`). Decoradores: api.model.
    async fn _render_eval_context(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._render_eval_context".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:325`). Decoradores: api.model.
    async fn _render_template_qweb(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._render_template_qweb".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:392`). Decoradores: api.model.
    async fn _render_template_qweb_regex(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._render_template_qweb_regex".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:430`). Decoradores: api.model.
    async fn _render_template_qweb_view(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._render_template_qweb_view".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:483`). Decoradores: api.model.
    async fn _render_template_inline_template(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._render_template_inline_template".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:545`). Decoradores: api.model.
    async fn _render_template_inline_template_regex(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._render_template_inline_template_regex".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:566`). Decoradores: api.model.
    async fn _render_template_postprocess(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._render_template_postprocess".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:585`). Decoradores: api.model.
    async fn _process_scheduled_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._process_scheduled_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:594`). Decoradores: api.model.
    async fn _render_template_get_valid_options(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._render_template_get_valid_options".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:598`). Decoradores: api.model.
    async fn _render_template(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._render_template".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:662`).
    async fn _render_lang(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._render_lang".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:691`).
    async fn _classify_per_lang(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._classify_per_lang".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_render_mixin.py:716`).
    async fn _render_field(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.render.mixin._render_field".into(),
        ))
    }

}
