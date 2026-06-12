//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.activity.mixin`

use nexus_orm::prelude::*;

pub struct MailActivityMixinFragment;

#[async_trait]
impl ModelFragment for MailActivityMixinFragment {
    fn model_name(&self) -> &str {
        "mail.activity.mixin"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Activity Mixin".into();
        def.add_field(FieldDef::one2many("activity_ids", "mail.activity", "res_id").string("Activities"));
        def.add_field(FieldDef::selection("activity_state", &[("overdue", "Overdue"), ("today", "Today"), ("planned", "Planned")]).string("Activity State").computed("_compute_activity_state", &["activity_ids.state"]).stored());
        def.add_field(FieldDef::many2one("activity_user_id", "res.users").string("Responsible User").readonly().computed("_compute_activity_user_id", &["activity_ids.user_id"]).stored());
        def.add_field({ let mut f = FieldDef::many2one("activity_type_id", "mail.activity.type").string("Next Activity Type"); f.related = Some("activity_ids.activity_type_id".into()); f });
        def.add_field({ let mut f = FieldDef::char("activity_type_icon").string("Activity Type Icon"); f.related = Some("activity_ids.icon".into()); f });
        def.add_field(FieldDef::date("activity_date_deadline").string("Next Activity Deadline").readonly().computed("_compute_activity_date_deadline", &["activity_ids.date_deadline"]));
        def.add_field(FieldDef::date("my_activity_date_deadline").string("My Activity Deadline").readonly().computed("_compute_my_activity_date_deadline", &["activity_ids.date_deadline", "activity_ids.user_id"]).stored());
        def.add_field({ let mut f = FieldDef::char("activity_summary").string("Next Activity Summary"); f.related = Some("activity_ids.summary".into()); f });
        def.add_field(FieldDef::selection("activity_exception_decoration", &[("warning", "Alert"), ("danger", "Error")]).computed("_compute_activity_exception_type", &["activity_ids.activity_type_id.decoration_type", "activity_ids.activity_type_id.icon"]).stored());
        def.add_field(FieldDef::char("activity_exception_icon").string("Icon").computed("_compute_activity_exception_type", &["activity_ids.activity_type_id.decoration_type", "activity_ids.activity_type_id.icon"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_default_activity_type", "_compute_activity_exception_type", "_compute_activity_user_id", "_search_activity_exception_decoration", "_compute_activity_state", "_search_activity_state", "_compute_activity_date_deadline", "_search_activity_date_deadline", "_search_activity_user_id", "_search_activity_type_id", "_search_activity_summary", "_compute_my_activity_date_deadline", "_search_my_activity_date_deadline", "_read_group_groupby", "action_reschedule_my_next_today", "action_reschedule_my_next_tomorrow", "action_reschedule_my_next_nextweek", "activity_send_mail", "activity_search", "activity_schedule", "_activity_schedule_with_view", "activity_reschedule", "activity_feedback", "activity_unlink"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_default_activity_type" => self._default_activity_type(env, ctx, rs, args).await,
            "_compute_activity_exception_type" => self._compute_activity_exception_type(env, ctx, rs, args).await,
            "_compute_activity_user_id" => self._compute_activity_user_id(env, ctx, rs, args).await,
            "_search_activity_exception_decoration" => self._search_activity_exception_decoration(env, ctx, rs, args).await,
            "_compute_activity_state" => self._compute_activity_state(env, ctx, rs, args).await,
            "_search_activity_state" => self._search_activity_state(env, ctx, rs, args).await,
            "_compute_activity_date_deadline" => self._compute_activity_date_deadline(env, ctx, rs, args).await,
            "_search_activity_date_deadline" => self._search_activity_date_deadline(env, ctx, rs, args).await,
            "_search_activity_user_id" => self._search_activity_user_id(env, ctx, rs, args).await,
            "_search_activity_type_id" => self._search_activity_type_id(env, ctx, rs, args).await,
            "_search_activity_summary" => self._search_activity_summary(env, ctx, rs, args).await,
            "_compute_my_activity_date_deadline" => self._compute_my_activity_date_deadline(env, ctx, rs, args).await,
            "_search_my_activity_date_deadline" => self._search_my_activity_date_deadline(env, ctx, rs, args).await,
            "_read_group_groupby" => self._read_group_groupby(env, ctx, rs, args).await,
            "action_reschedule_my_next_today" => self.action_reschedule_my_next_today(env, ctx, rs, args).await,
            "action_reschedule_my_next_tomorrow" => self.action_reschedule_my_next_tomorrow(env, ctx, rs, args).await,
            "action_reschedule_my_next_nextweek" => self.action_reschedule_my_next_nextweek(env, ctx, rs, args).await,
            "activity_send_mail" => self.activity_send_mail(env, ctx, rs, args).await,
            "activity_search" => self.activity_search(env, ctx, rs, args).await,
            "activity_schedule" => self.activity_schedule(env, ctx, rs, args).await,
            "_activity_schedule_with_view" => self._activity_schedule_with_view(env, ctx, rs, args).await,
            "activity_reschedule" => self.activity_reschedule(env, ctx, rs, args).await,
            "activity_feedback" => self.activity_feedback(env, ctx, rs, args).await,
            "activity_unlink" => self.activity_unlink(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailActivityMixinFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:41`).
    async fn _default_activity_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin._default_activity_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:97`). Decoradores: api.depends('activity_ids.activity_type_id.decoration_type', 'activity_ids.activity_type_id.icon').
    async fn _compute_activity_exception_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin._compute_activity_exception_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:114`). Decoradores: api.depends('activity_ids.user_id').
    async fn _compute_activity_user_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin._compute_activity_user_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:118`).
    async fn _search_activity_exception_decoration(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin._search_activity_exception_decoration".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:122`). Decoradores: api.depends('activity_ids.state').
    async fn _compute_activity_state(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin._compute_activity_state".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:134`).
    async fn _search_activity_state(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin._search_activity_state".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:192`). Decoradores: api.depends('activity_ids.date_deadline').
    async fn _compute_activity_date_deadline(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin._compute_activity_date_deadline".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:197`).
    async fn _search_activity_date_deadline(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin._search_activity_date_deadline".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:205`). Decoradores: api.model.
    async fn _search_activity_user_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin._search_activity_user_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:224`). Decoradores: api.model.
    async fn _search_activity_type_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin._search_activity_type_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:230`). Decoradores: api.model.
    async fn _search_activity_summary(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin._search_activity_summary".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:237`). Decoradores: api.depends('activity_ids.date_deadline', 'activity_ids.user_id'), api.depends_context('uid').
    async fn _compute_my_activity_date_deadline(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin._compute_my_activity_date_deadline".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:245`).
    async fn _search_my_activity_date_deadline(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin._search_my_activity_date_deadline".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:255`).
    async fn _read_group_groupby(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin._read_group_groupby".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:295`).
    async fn action_reschedule_my_next_today(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin.action_reschedule_my_next_today".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:301`).
    async fn action_reschedule_my_next_tomorrow(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin.action_reschedule_my_next_tomorrow".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:307`).
    async fn action_reschedule_my_next_nextweek(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin.action_reschedule_my_next_nextweek".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:312`).
    async fn activity_send_mail(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin.activity_send_mail".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:324`).
    async fn activity_search(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin.activity_search".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:357`).
    async fn activity_schedule(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin.activity_schedule".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:411`).
    async fn _activity_schedule_with_view(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin._activity_schedule_with_view".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:433`).
    async fn activity_reschedule(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin.activity_reschedule".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:459`).
    async fn activity_feedback(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin.activity_feedback".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_mixin.py:475`).
    async fn activity_unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.mixin.activity_unlink".into(),
        ))
    }

}
