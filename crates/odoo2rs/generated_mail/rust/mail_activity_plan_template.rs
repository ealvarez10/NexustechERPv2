//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_plan_template.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.activity.plan.template`

use nexus_orm::prelude::*;

pub struct MailActivityPlanTemplateFragment;

#[async_trait]
impl ModelFragment for MailActivityPlanTemplateFragment {
    fn model_name(&self) -> &str {
        "mail.activity.plan.template"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Activity plan template".into();
        def.order = "sequence, id".into();
        def.rec_name = "summary".into();
        def.add_field(FieldDef::many2one("plan_id", "mail.activity.plan").string("Plan").required());
        def.add_field({ let mut f = FieldDef::selection("res_model", &[]); f.related = Some("plan_id.res_model".into()); f });
        // TODO(odoo2rs): campo 'company_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::integer("sequence").default_val(10i64));
        def.add_field(FieldDef::many2one("activity_type_id", "mail.activity.type").string("Activity Type").required());
        def.add_field(FieldDef::integer("delay_count").string("Interval").default_val(0i64));
        def.add_field(FieldDef::selection("delay_unit", &[("days", "days"), ("weeks", "weeks"), ("months", "months")]).string("Delay units").required().default_val("days"));
        def.add_field(FieldDef::selection("delay_from", &[("before_plan_date", "Before Plan Date"), ("after_plan_date", "After Plan Date")]).string("Trigger").required().default_val("before_plan_date"));
        def.add_field({ let mut f = FieldDef::char("icon").string("Icon").readonly(); f.related = Some("activity_type_id.icon".into()); f });
        def.add_field(FieldDef::char("summary").string("Summary").computed("_compute_summary", &["activity_type_id"]).stored());
        def.add_field(FieldDef::selection("responsible_type", &[("on_demand", "Ask at launch"), ("other", "Default user")]).string("Assignment").required().computed("_compute_responsible_type", &["activity_type_id"]).stored().default_val("on_demand"));
        def.add_field(FieldDef::many2one("responsible_id", "res.users").string("Assigned to").computed("_compute_responsible_id", &["activity_type_id", "responsible_type"]).stored());
        def.add_field(FieldDef::html("note").string("Note").computed("_compute_note", &["activity_type_id"]).stored());
        def.add_field(FieldDef::many2many("next_activity_ids", "mail.activity.type").string("Next Activities").computed("_compute_next_activity_ids", &["activity_type_id"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_check_activity_type_res_model", "_check_responsible", "_compute_next_activity_ids", "_compute_note", "_compute_responsible_id", "_compute_responsible_type", "_compute_summary", "_get_date_deadline", "_determine_responsible"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_check_activity_type_res_model" => self._check_activity_type_res_model(env, ctx, rs, args).await,
            "_check_responsible" => self._check_responsible(env, ctx, rs, args).await,
            "_compute_next_activity_ids" => self._compute_next_activity_ids(env, ctx, rs, args).await,
            "_compute_note" => self._compute_note(env, ctx, rs, args).await,
            "_compute_responsible_id" => self._compute_responsible_id(env, ctx, rs, args).await,
            "_compute_responsible_type" => self._compute_responsible_type(env, ctx, rs, args).await,
            "_compute_summary" => self._compute_summary(env, ctx, rs, args).await,
            "_get_date_deadline" => self._get_date_deadline(env, ctx, rs, args).await,
            "_determine_responsible" => self._determine_responsible(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailActivityPlanTemplateFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_plan_template.py:59`). Decoradores: api.constrains('activity_type_id', 'plan_id').
    async fn _check_activity_type_res_model(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.plan.template._check_activity_type_res_model".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_plan_template.py:77`). Decoradores: api.constrains('responsible_id', 'responsible_type').
    async fn _check_responsible(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.plan.template._check_responsible".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_plan_template.py:84`). Decoradores: api.depends('activity_type_id').
    async fn _compute_next_activity_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.plan.template._compute_next_activity_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_plan_template.py:97`). Decoradores: api.depends('activity_type_id').
    async fn _compute_note(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.plan.template._compute_note".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_plan_template.py:102`). Decoradores: api.depends('activity_type_id', 'responsible_type').
    async fn _compute_responsible_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.plan.template._compute_responsible_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_plan_template.py:109`). Decoradores: api.depends('activity_type_id').
    async fn _compute_responsible_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.plan.template._compute_responsible_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_plan_template.py:117`). Decoradores: api.depends('activity_type_id').
    async fn _compute_summary(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.plan.template._compute_summary".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_plan_template.py:121`).
    async fn _get_date_deadline(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.plan.template._get_date_deadline".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_plan_template.py:130`).
    async fn _determine_responsible(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.plan.template._determine_responsible".into(),
        ))
    }

}
