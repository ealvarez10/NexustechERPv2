//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.activity`

use nexus_orm::prelude::*;

pub struct MailActivityFragment;

#[async_trait]
impl ModelFragment for MailActivityFragment {
    fn model_name(&self) -> &str {
        "mail.activity"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Activity".into();
        def.order = "date_deadline ASC, id ASC".into();
        def.rec_name = "summary".into();
        def.add_field(FieldDef::many2one("res_model_id", "ir.model").string("Document Model"));
        def.add_field({ let mut f = FieldDef::char("res_model").string("Related Document Model").readonly(); f.related = Some("res_model_id.model".into()); f });
        def.add_field(FieldDef::char("res_name").string("Document Name").readonly().computed("_compute_res_name", &["res_model", "res_id"]).stored());
        def.add_field(FieldDef::many2one("activity_type_id", "mail.activity.type").string("Activity Type"));
        def.add_field({ let mut f = FieldDef::selection("activity_category", &[]).readonly(); f.related = Some("activity_type_id.category".into()); f });
        def.add_field({ let mut f = FieldDef::selection("activity_decoration", &[]).readonly(); f.related = Some("activity_type_id.decoration_type".into()); f });
        def.add_field({ let mut f = FieldDef::char("icon").string("Icon").readonly(); f.related = Some("activity_type_id.icon".into()); f });
        def.add_field(FieldDef::char("summary").string("Summary"));
        def.add_field(FieldDef::html("note").string("Note"));
        def.add_field(FieldDef::date("date_deadline").string("Due Date").required());
        def.add_field(FieldDef::date("date_done").string("Done Date").computed("_compute_date_done", &["active"]).stored());
        def.add_field(FieldDef::text("feedback").string("Feedback"));
        def.add_field(FieldDef::boolean("automated").string("Automated activity").readonly());
        def.add_field(FieldDef::many2many("attachment_ids", "ir.attachment").string("Attachments"));
        def.add_field(FieldDef::many2one("user_id", "res.users").string("Assigned to"));
        def.add_field({ let mut f = FieldDef::selection("user_tz", &[]).string("Timezone"); f.related = Some("user_id.tz".into()); f });
        def.add_field(FieldDef::selection("state", &[("overdue", "Overdue"), ("today", "Today"), ("planned", "Planned"), ("done", "Done")]).string("State").computed("_compute_state", &["active", "date_deadline"]).stored());
        def.add_field(FieldDef::many2one("recommended_activity_type_id", "mail.activity.type").string("Recommended Activity Type"));
        def.add_field(FieldDef::many2one("previous_activity_type_id", "mail.activity.type").string("Previous Activity Type").readonly());
        def.add_field(FieldDef::boolean("has_recommended_activities").string("Next activities available").computed("_compute_has_recommended_activities", &[]).stored());
        // TODO(odoo2rs): campo 'mail_template_ids' (many2many) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field({ let mut f = FieldDef::selection("chaining_type", &[]).readonly(); f.related = Some("activity_type_id.chaining_type".into()); f });
        def.add_field(FieldDef::boolean("can_write").computed("_compute_can_write", &["res_model", "res_id", "user_id"]).stored());
        def.add_field(FieldDef::boolean("active").default_val(true));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["default_get", "_default_activity_type", "_default_activity_type_for_model", "_compute_has_recommended_activities", "_onchange_previous_activity_type_id", "_compute_date_done", "_compute_res_name", "_compute_state", "_compute_state_from_date", "_compute_can_write", "_onchange_activity_type_id", "_onchange_recommended_activity_type_id", "_check_access", "_make_access_error", "create", "write", "unlink", "_search", "_compute_display_name", "action_notify", "action_done", "action_done_redirect_to_other", "action_feedback", "action_done_schedule_next", "action_feedback_schedule_next", "_action_done", "action_close_dialog", "action_open_document", "action_reschedule_today", "action_reschedule_tomorrow", "action_reschedule_nextweek", "action_cancel", "activity_format", "_to_store_defaults", "get_activity_data", "_classify_by_model", "_prepare_next_activity_values", "_gc_delete_old_overdue_activities"]
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
            "_default_activity_type" => self._default_activity_type(env, ctx, rs, args).await,
            "_default_activity_type_for_model" => self._default_activity_type_for_model(env, ctx, rs, args).await,
            "_compute_has_recommended_activities" => self._compute_has_recommended_activities(env, ctx, rs, args).await,
            "_onchange_previous_activity_type_id" => self._onchange_previous_activity_type_id(env, ctx, rs, args).await,
            "_compute_date_done" => self._compute_date_done(env, ctx, rs, args).await,
            "_compute_res_name" => self._compute_res_name(env, ctx, rs, args).await,
            "_compute_state" => self._compute_state(env, ctx, rs, args).await,
            "_compute_state_from_date" => self._compute_state_from_date(env, ctx, rs, args).await,
            "_compute_can_write" => self._compute_can_write(env, ctx, rs, args).await,
            "_onchange_activity_type_id" => self._onchange_activity_type_id(env, ctx, rs, args).await,
            "_onchange_recommended_activity_type_id" => self._onchange_recommended_activity_type_id(env, ctx, rs, args).await,
            "_check_access" => self._check_access(env, ctx, rs, args).await,
            "_make_access_error" => self._make_access_error(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "_search" => self._search(env, ctx, rs, args).await,
            "_compute_display_name" => self._compute_display_name(env, ctx, rs, args).await,
            "action_notify" => self.action_notify(env, ctx, rs, args).await,
            "action_done" => self.action_done(env, ctx, rs, args).await,
            "action_done_redirect_to_other" => self.action_done_redirect_to_other(env, ctx, rs, args).await,
            "action_feedback" => self.action_feedback(env, ctx, rs, args).await,
            "action_done_schedule_next" => self.action_done_schedule_next(env, ctx, rs, args).await,
            "action_feedback_schedule_next" => self.action_feedback_schedule_next(env, ctx, rs, args).await,
            "_action_done" => self._action_done(env, ctx, rs, args).await,
            "action_close_dialog" => self.action_close_dialog(env, ctx, rs, args).await,
            "action_open_document" => self.action_open_document(env, ctx, rs, args).await,
            "action_reschedule_today" => self.action_reschedule_today(env, ctx, rs, args).await,
            "action_reschedule_tomorrow" => self.action_reschedule_tomorrow(env, ctx, rs, args).await,
            "action_reschedule_nextweek" => self.action_reschedule_nextweek(env, ctx, rs, args).await,
            "action_cancel" => self.action_cancel(env, ctx, rs, args).await,
            "activity_format" => self.activity_format(env, ctx, rs, args).await,
            "_to_store_defaults" => self._to_store_defaults(env, ctx, rs, args).await,
            "get_activity_data" => self.get_activity_data(env, ctx, rs, args).await,
            "_classify_by_model" => self._classify_by_model(env, ctx, rs, args).await,
            "_prepare_next_activity_values" => self._prepare_next_activity_values(env, ctx, rs, args).await,
            "_gc_delete_old_overdue_activities" => self._gc_delete_old_overdue_activities(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailActivityFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:32`). Decoradores: api.model.
    async fn default_get(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.default_get".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:39`). Decoradores: api.model.
    async fn _default_activity_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity._default_activity_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:47`). Decoradores: api.model.
    async fn _default_activity_type_for_model(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity._default_activity_type_for_model".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:126`). Decoradores: api.onchange('previous_activity_type_id').
    async fn _compute_has_recommended_activities(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity._compute_has_recommended_activities".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:131`). Decoradores: api.onchange('previous_activity_type_id').
    async fn _onchange_previous_activity_type_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity._onchange_previous_activity_type_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:137`). Decoradores: api.depends('active').
    async fn _compute_date_done(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity._compute_date_done".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:145`). Decoradores: api.depends('res_model', 'res_id').
    async fn _compute_res_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity._compute_res_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:153`). Decoradores: api.depends('active', 'date_deadline').
    async fn _compute_state(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity._compute_state".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:160`). Decoradores: api.model.
    async fn _compute_state_from_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity._compute_state_from_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:177`). Decoradores: api.depends('res_model', 'res_id', 'user_id').
    async fn _compute_can_write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity._compute_can_write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:183`). Decoradores: api.onchange('activity_type_id').
    async fn _onchange_activity_type_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity._onchange_activity_type_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:193`). Decoradores: api.onchange('recommended_activity_type_id').
    async fn _onchange_recommended_activity_type_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity._onchange_recommended_activity_type_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:197`).
    async fn _check_access(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity._check_access".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:257`).
    async fn _make_access_error(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity._make_access_error".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:274`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:311`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:355`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:363`). Decoradores: api.model.
    async fn _search(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity._search".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:404`). Decoradores: api.depends('summary', 'activity_type_id').
    async fn _compute_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity._compute_display_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:413`).
    async fn action_notify(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.action_notify".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:451`).
    async fn action_done(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.action_done".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:456`).
    async fn action_done_redirect_to_other(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.action_done_redirect_to_other".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:482`).
    async fn action_feedback(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.action_feedback".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:488`).
    async fn action_done_schedule_next(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.action_done_schedule_next".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:493`).
    async fn action_feedback_schedule_next(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.action_feedback_schedule_next".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:514`).
    async fn _action_done(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity._action_done".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:601`). Decoradores: api.readonly.
    async fn action_close_dialog(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.action_close_dialog".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:605`). Decoradores: api.readonly.
    async fn action_open_document(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.action_open_document".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:638`).
    async fn action_reschedule_today(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.action_reschedule_today".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:641`).
    async fn action_reschedule_tomorrow(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.action_reschedule_tomorrow".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:644`).
    async fn action_reschedule_nextweek(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.action_reschedule_nextweek".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:647`).
    async fn action_cancel(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.action_cancel".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:653`). Decoradores: api.readonly.
    async fn activity_format(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.activity_format".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:656`).
    async fn _to_store_defaults(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity._to_store_defaults".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:679`). Decoradores: api.readonly, api.model.
    async fn get_activity_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.get_activity_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:818`).
    async fn _classify_by_model(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity._classify_by_model".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:841`).
    async fn _prepare_next_activity_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity._prepare_next_activity_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity.py:860`). Decoradores: api.autovacuum.
    async fn _gc_delete_old_overdue_activities(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity._gc_delete_old_overdue_activities".into(),
        ))
    }

}
