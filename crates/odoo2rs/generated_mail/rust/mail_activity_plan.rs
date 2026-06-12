//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_plan.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.activity.plan`

use nexus_orm::prelude::*;

pub struct MailActivityPlanFragment;

#[async_trait]
impl ModelFragment for MailActivityPlanFragment {
    fn model_name(&self) -> &str {
        "mail.activity.plan"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Activity Plan".into();
        def.order = "id DESC".into();
        def.add_field(FieldDef::char("name").string("Name").required());
        def.add_field(FieldDef::many2one("company_id", "res.company"));
        def.add_field(FieldDef::one2many("template_ids", "mail.activity.plan.template", "plan_id").string("Activities"));
        def.add_field(FieldDef::boolean("active").default_val(true));
        def.add_field(FieldDef::many2one("res_model_id", "ir.model").string("Applies to").required().computed("_compute_res_model_id", &["res_model"]).stored());
        def.add_field(FieldDef::selection("res_model", &[]).string("Model").required());
        def.add_field(FieldDef::integer("steps_count").computed("_compute_steps_count", &["template_ids"]).stored());
        def.add_field(FieldDef::boolean("has_user_on_demand").string("Has on demand responsible").computed("_compute_has_user_on_demand", &["template_ids.responsible_type"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_get_model_selection", "_compute_res_model_id", "_check_res_model_compatibility_with_templates", "_compute_steps_count", "_compute_has_user_on_demand", "copy_data"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_get_model_selection" => self._get_model_selection(env, ctx, rs, args).await,
            "_compute_res_model_id" => self._compute_res_model_id(env, ctx, rs, args).await,
            "_check_res_model_compatibility_with_templates" => self._check_res_model_compatibility_with_templates(env, ctx, rs, args).await,
            "_compute_steps_count" => self._compute_steps_count(env, ctx, rs, args).await,
            "_compute_has_user_on_demand" => self._compute_has_user_on_demand(env, ctx, rs, args).await,
            "copy_data" => self.copy_data(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailActivityPlanFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_plan.py:12`).
    async fn _get_model_selection(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.plan._get_model_selection".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_plan.py:38`). Decoradores: api.depends('res_model').
    async fn _compute_res_model_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.plan._compute_res_model_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_plan.py:48`). Decoradores: api.constrains('res_model').
    async fn _check_res_model_compatibility_with_templates(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.plan._check_res_model_compatibility_with_templates".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_plan.py:52`). Decoradores: api.depends('template_ids').
    async fn _compute_steps_count(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.plan._compute_steps_count".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_plan.py:57`). Decoradores: api.depends('template_ids.responsible_type').
    async fn _compute_has_user_on_demand(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.plan._compute_has_user_on_demand".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_plan.py:62`).
    async fn copy_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.plan.copy_data".into(),
        ))
    }

}
