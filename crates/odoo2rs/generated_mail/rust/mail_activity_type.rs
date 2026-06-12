//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_type.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.activity.type`

use nexus_orm::prelude::*;

pub struct MailActivityTypeFragment;

#[async_trait]
impl ModelFragment for MailActivityTypeFragment {
    fn model_name(&self) -> &str {
        "mail.activity.type"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Activity Type".into();
        def.order = "sequence, id".into();
        def.rec_name = "name".into();
        def.add_field(FieldDef::char("name").string("Name").required());
        def.add_field(FieldDef::char("summary").string("Default Summary"));
        def.add_field(FieldDef::integer("sequence").string("Sequence").default_val(10i64));
        def.add_field(FieldDef::boolean("active").default_val(true));
        def.add_field(FieldDef::many2one("create_uid", "res.users"));
        def.add_field(FieldDef::integer("delay_count").string("Schedule").default_val(0i64));
        def.add_field(FieldDef::selection("delay_unit", &[("days", "days"), ("weeks", "weeks"), ("months", "months")]).string("Delay units").required().default_val("days"));
        def.add_field(FieldDef::char("delay_label").computed("_compute_delay_label", &["delay_unit", "delay_count"]).stored());
        def.add_field(FieldDef::selection("delay_from", &[("current_date", "after previous activity completion date"), ("previous_activity", "after previous activity deadline")]).string("Delay Type").required().default_val("previous_activity"));
        def.add_field(FieldDef::char("icon").string("Icon"));
        def.add_field(FieldDef::selection("decoration_type", &[("warning", "Alert"), ("danger", "Error")]).string("Decoration Type"));
        def.add_field(FieldDef::selection("res_model", &[]).string("Model"));
        def.add_field(FieldDef::many2one("triggered_next_type_id", "mail.activity.type").string("Trigger").computed("_compute_triggered_next_type_id", &["chaining_type"]).stored());
        def.add_field(FieldDef::selection("chaining_type", &[("suggest", "Suggest Next Activity"), ("trigger", "Trigger Next Activity")]).string("Chaining Type").required().default_val("suggest"));
        def.add_field(FieldDef::many2many("suggested_next_type_ids", "mail.activity.type").string("Suggest").computed("_compute_suggested_next_type_ids", &["chaining_type"]).stored());
        def.add_field(FieldDef::many2many("previous_type_ids", "mail.activity.type").string("Preceding Activities"));
        def.add_field(FieldDef::selection("category", &[("default", "None"), ("upload_file", "Upload Document"), ("phonecall", "Phonecall")]).string("Action").default_val("default"));
        def.add_field(FieldDef::many2many("mail_template_ids", "mail.template").string("Email templates"));
        def.add_field(FieldDef::many2one("default_user_id", "res.users").string("Default User"));
        def.add_field(FieldDef::html("default_note").string("Default Note"));
        def.add_field(FieldDef::selection("initial_res_model", &[]).string("Initial model").computed("_compute_initial_res_model", &[]));
        def.add_field({ let mut f = FieldDef::boolean("res_model_change").string("Model has change"); f.store = false; f });
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_get_model_selection", "_check_activity_type_res_model", "_onchange_res_model", "_compute_initial_res_model", "_compute_delay_label", "_compute_suggested_next_type_ids", "_inverse_suggested_next_type_ids", "_compute_triggered_next_type_id", "_inverse_triggered_next_type_id", "write", "_unlink_except_todo", "action_archive", "unlink", "_get_date_deadline", "_get_model_info_by_xmlid"]
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
            "_check_activity_type_res_model" => self._check_activity_type_res_model(env, ctx, rs, args).await,
            "_onchange_res_model" => self._onchange_res_model(env, ctx, rs, args).await,
            "_compute_initial_res_model" => self._compute_initial_res_model(env, ctx, rs, args).await,
            "_compute_delay_label" => self._compute_delay_label(env, ctx, rs, args).await,
            "_compute_suggested_next_type_ids" => self._compute_suggested_next_type_ids(env, ctx, rs, args).await,
            "_inverse_suggested_next_type_ids" => self._inverse_suggested_next_type_ids(env, ctx, rs, args).await,
            "_compute_triggered_next_type_id" => self._compute_triggered_next_type_id(env, ctx, rs, args).await,
            "_inverse_triggered_next_type_id" => self._inverse_triggered_next_type_id(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "_unlink_except_todo" => self._unlink_except_todo(env, ctx, rs, args).await,
            "action_archive" => self.action_archive(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "_get_date_deadline" => self._get_date_deadline(env, ctx, rs, args).await,
            "_get_model_info_by_xmlid" => self._get_model_info_by_xmlid(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailActivityTypeFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_type.py:20`).
    async fn _get_model_selection(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.type._get_model_selection".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_type.py:84`). Decoradores: api.constrains('res_model').
    async fn _check_activity_type_res_model(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.type._check_activity_type_res_model".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_type.py:89`). Decoradores: api.onchange('res_model').
    async fn _onchange_res_model(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.type._onchange_res_model".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_type.py:93`).
    async fn _compute_initial_res_model(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.type._compute_initial_res_model".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_type.py:98`). Decoradores: api.depends('delay_unit', 'delay_count').
    async fn _compute_delay_label(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.type._compute_delay_label".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_type.py:106`). Decoradores: api.depends('chaining_type').
    async fn _compute_suggested_next_type_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.type._compute_suggested_next_type_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_type.py:112`).
    async fn _inverse_suggested_next_type_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.type._inverse_suggested_next_type_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_type.py:118`). Decoradores: api.depends('chaining_type').
    async fn _compute_triggered_next_type_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.type._compute_triggered_next_type_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_type.py:124`).
    async fn _inverse_triggered_next_type_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.type._inverse_triggered_next_type_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_type.py:131`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.type.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_type.py:153`). Decoradores: api.ondelete().
    async fn _unlink_except_todo(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.type._unlink_except_todo".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_type.py:165`).
    async fn action_archive(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.type.action_archive".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_type.py:170`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.type.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_type.py:178`).
    async fn _get_date_deadline(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.type._get_date_deadline".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_activity_type.py:188`). Decoradores: api.model.
    async fn _get_model_info_by_xmlid(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.activity.type._get_model_info_by_xmlid".into(),
        ))
    }

}
