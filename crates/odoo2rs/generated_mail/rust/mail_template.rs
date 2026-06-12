//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.template`

use nexus_orm::prelude::*;

pub struct MailTemplateFragment;

#[async_trait]
impl ModelFragment for MailTemplateFragment {
    fn model_name(&self) -> &str {
        "mail.template"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Email Templates".into();
        def.order = "user_id, name, id".into();
        def.add_field(FieldDef::char("name").string("Name"));
        def.add_field(FieldDef::text("description").string("Template Description"));
        def.add_field(FieldDef::boolean("active").default_val(true));
        def.add_field(FieldDef::selection("template_category", &[("base_template", "Base Template"), ("hidden_template", "Hidden Template"), ("custom_template", "Custom Template")]).computed("_compute_template_category", &["active", "description"]).stored());
        def.add_field(FieldDef::many2one("model_id", "ir.model").string("Applies to"));
        def.add_field({ let mut f = FieldDef::char("model").string("Related Document Model").readonly(); f.related = Some("model_id.model".into()); f });
        def.add_field(FieldDef::char("subject").string("Subject"));
        def.add_field(FieldDef::char("email_from").string("Send From"));
        def.add_field(FieldDef::many2one("user_id", "res.users").string("Owner"));
        def.add_field(FieldDef::boolean("use_default_to").string("Default Recipients").default_val(true));
        def.add_field(FieldDef::char("email_to").string("To (Emails)"));
        def.add_field(FieldDef::char("partner_to").string("To (Partners)"));
        def.add_field(FieldDef::char("email_cc").string("Cc"));
        def.add_field(FieldDef::char("reply_to").string("Reply To"));
        def.add_field(FieldDef::html("body_html").string("Body"));
        def.add_field(FieldDef::many2many("attachment_ids", "ir.attachment").string("Attachments"));
        def.add_field(FieldDef::many2many("report_template_ids", "ir.actions.report").string("Dynamic Reports"));
        def.add_field(FieldDef::char("email_layout_xmlid").string("Email Notification Layout"));
        def.add_field(FieldDef::many2one("mail_server_id", "ir.mail_server").string("Outgoing Mail Server"));
        def.add_field(FieldDef::char("scheduled_date").string("Scheduled Date"));
        def.add_field(FieldDef::boolean("auto_delete").string("Auto Delete").default_val(true));
        def.add_field(FieldDef::many2one("ref_ir_act_window", "ir.actions.act_window").string("Sidebar action").readonly());
        def.add_field(FieldDef::boolean("can_write").computed("_compute_can_write", &[]).stored());
        def.add_field(FieldDef::boolean("is_template_editor").computed("_compute_is_template_editor", &[]).stored());
        def.add_field(FieldDef::boolean("has_dynamic_reports").computed("_compute_has_dynamic_reports", &["model"]).stored());
        def.add_field(FieldDef::boolean("has_mail_server").computed("_compute_has_mail_server", &[]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["default_get", "_get_non_abstract_models_domain", "_compute_has_dynamic_reports", "_compute_has_mail_server", "_compute_render_model", "_compute_can_write", "_compute_is_template_editor", "_compute_template_category", "_search_template_category", "_onchange_model", "_fix_attachment_ownership", "_check_abstract_models", "_check_can_be_rendered", "_get_dynamic_field_names", "create", "write", "unlink", "copy_data", "copy", "unlink_action", "create_action", "action_open_mail_preview", "_generate_template_attachments", "_generate_template_recipients", "_generate_template_scheduled_date", "_generate_template_static_values", "_generate_template", "_parse_partner_to", "_send_check_access", "send_mail", "send_mail_batch", "_has_unsafe_expression_template_qweb", "_has_unsafe_expression_template_inline_template", "_expression_is_default"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "default_get" => self.default_get(env, ctx, rs, args).await,
            "_get_non_abstract_models_domain" => self._get_non_abstract_models_domain(env, ctx, rs, args).await,
            "_compute_has_dynamic_reports" => self._compute_has_dynamic_reports(env, ctx, rs, args).await,
            "_compute_has_mail_server" => self._compute_has_mail_server(env, ctx, rs, args).await,
            "_compute_render_model" => self._compute_render_model(env, ctx, rs, args).await,
            "_compute_can_write" => self._compute_can_write(env, ctx, rs, args).await,
            "_compute_is_template_editor" => self._compute_is_template_editor(env, ctx, rs, args).await,
            "_compute_template_category" => self._compute_template_category(env, ctx, rs, args).await,
            "_search_template_category" => self._search_template_category(env, ctx, rs, args).await,
            "_onchange_model" => self._onchange_model(env, ctx, rs, args).await,
            "_fix_attachment_ownership" => self._fix_attachment_ownership(env, ctx, rs, args).await,
            "_check_abstract_models" => self._check_abstract_models(env, ctx, rs, args).await,
            "_check_can_be_rendered" => self._check_can_be_rendered(env, ctx, rs, args).await,
            "_get_dynamic_field_names" => self._get_dynamic_field_names(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "copy_data" => self.copy_data(env, ctx, rs, args).await,
            "copy" => self.copy(env, ctx, rs, args).await,
            "unlink_action" => self.unlink_action(env, ctx, rs, args).await,
            "create_action" => self.create_action(env, ctx, rs, args).await,
            "action_open_mail_preview" => self.action_open_mail_preview(env, ctx, rs, args).await,
            "_generate_template_attachments" => self._generate_template_attachments(env, ctx, rs, args).await,
            "_generate_template_recipients" => self._generate_template_recipients(env, ctx, rs, args).await,
            "_generate_template_scheduled_date" => self._generate_template_scheduled_date(env, ctx, rs, args).await,
            "_generate_template_static_values" => self._generate_template_static_values(env, ctx, rs, args).await,
            "_generate_template" => self._generate_template(env, ctx, rs, args).await,
            "_parse_partner_to" => self._parse_partner_to(env, ctx, rs, args).await,
            "_send_check_access" => self._send_check_access(env, ctx, rs, args).await,
            "send_mail" => self.send_mail(env, ctx, rs, args).await,
            "send_mail_batch" => self.send_mail_batch(env, ctx, rs, args).await,
            "_has_unsafe_expression_template_qweb" => self._has_unsafe_expression_template_qweb(env, ctx, rs, args).await,
            "_has_unsafe_expression_template_inline_template" => self._has_unsafe_expression_template_inline_template(env, ctx, rs, args).await,
            "_expression_is_default" => self._expression_is_default(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailTemplateFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:27`). Decoradores: api.model.
    async fn default_get(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template.default_get".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:33`).
    async fn _get_non_abstract_models_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._get_non_abstract_models_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:109`). Decoradores: api.depends('model').
    async fn _compute_has_dynamic_reports(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._compute_has_dynamic_reports".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:119`).
    async fn _compute_has_mail_server(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._compute_has_mail_server".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:126`). Decoradores: api.depends('model').
    async fn _compute_render_model(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._compute_render_model".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:131`). Decoradores: api.depends_context('uid').
    async fn _compute_can_write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._compute_can_write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:137`). Decoradores: api.depends_context('uid').
    async fn _compute_is_template_editor(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._compute_is_template_editor".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:141`). Decoradores: api.depends('active', 'description').
    async fn _compute_template_category(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._compute_template_category".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:161`). Decoradores: api.model.
    async fn _search_template_category(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._search_template_category".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:184`). Decoradores: api.onchange('model').
    async fn _onchange_model(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._onchange_model".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:195`).
    async fn _fix_attachment_ownership(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._fix_attachment_ownership".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:200`).
    async fn _check_abstract_models(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._check_abstract_models".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:208`).
    async fn _check_can_be_rendered(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._check_can_be_rendered".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:230`).
    async fn _get_dynamic_field_names(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._get_dynamic_field_names".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:244`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:251`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:258`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:262`).
    async fn copy_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template.copy_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:269`).
    async fn copy(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template.copy".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:290`).
    async fn unlink_action(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template.unlink_action".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:296`).
    async fn create_action(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template.create_action".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:320`).
    async fn action_open_mail_preview(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template.action_open_mail_preview".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:329`).
    async fn _generate_template_attachments(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._generate_template_attachments".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:404`).
    async fn _generate_template_recipients(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._generate_template_recipients".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:509`).
    async fn _generate_template_scheduled_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._generate_template_scheduled_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:531`).
    async fn _generate_template_static_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._generate_template_static_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:564`).
    async fn _generate_template(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._generate_template".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:652`). Decoradores: classmethod.
    async fn _parse_partner_to(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._parse_partner_to".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:668`).
    async fn _send_check_access(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._send_check_access".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:672`).
    async fn send_mail(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template.send_mail".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:696`).
    async fn send_mail_batch(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template.send_mail_batch".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:810`).
    async fn _has_unsafe_expression_template_qweb(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._has_unsafe_expression_template_qweb".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:815`).
    async fn _has_unsafe_expression_template_inline_template(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._has_unsafe_expression_template_inline_template".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_template.py:820`).
    async fn _expression_is_default(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.template._expression_is_default".into(),
        ))
    }

}
