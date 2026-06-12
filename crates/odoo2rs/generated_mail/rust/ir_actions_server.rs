//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_actions_server.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `ir.actions.server`

use nexus_orm::prelude::*;

pub struct IrActionsServerFragment;

#[async_trait]
impl ModelFragment for IrActionsServerFragment {
    fn model_name(&self) -> &str {
        "ir.actions.server"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Server Action".into();
        def.add_field(FieldDef::char("name"));
        // TODO(odoo2rs): campo 'model_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        // TODO(odoo2rs): campo 'crud_model_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        // TODO(odoo2rs): campo 'link_field_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::char("update_path"));
        def.add_field(FieldDef::text("value"));
        def.add_field(FieldDef::selection("evaluation_type", &[]));
        def.add_field(FieldDef::char("webhook_url"));
        def.add_field(FieldDef::selection("state", &[]));
        def.add_field(FieldDef::selection("followers_type", &[("specific", "Specific Followers"), ("generic", "Dynamic Followers")]).string("Followers Type").computed("_compute_followers_type", &["model_id", "state"]).stored());
        def.add_field(FieldDef::char("followers_partner_field_name").string("Followers Field").computed("_compute_followers_info", &["followers_type"]).stored());
        def.add_field(FieldDef::many2many("partner_ids", "res.partner").computed("_compute_followers_info", &["followers_type"]).stored());
        def.add_field(FieldDef::many2one("template_id", "mail.template").string("Email Template").computed("_compute_template_id", &["model_id", "state"]).stored());
        def.add_field(FieldDef::boolean("mail_post_autofollow").string("Subscribe Recipients").computed("_compute_mail_post_autofollow", &["state", "mail_post_method"]).stored());
        def.add_field(FieldDef::selection("mail_post_method", &[("email", "Email"), ("comment", "Message"), ("note", "Note")]).string("Send Email As").computed("_compute_mail_post_method", &["state"]).stored());
        def.add_field(FieldDef::many2one("activity_type_id", "mail.activity.type").string("Activity Type").computed("_compute_activity_info", &["model_id", "state"]).stored());
        def.add_field(FieldDef::char("activity_summary").string("Title").computed("_compute_activity_info", &["model_id", "state"]).stored());
        def.add_field(FieldDef::html("activity_note").string("Note").computed("_compute_activity_info", &["model_id", "state"]).stored());
        def.add_field(FieldDef::integer("activity_date_deadline_range").string("Due Date In").computed("_compute_activity_info", &["model_id", "state"]).stored());
        def.add_field(FieldDef::selection("activity_date_deadline_range_type", &[("days", "Days"), ("weeks", "Weeks"), ("months", "Months")]).string("Due type").computed("_compute_activity_info", &["model_id", "state"]).stored());
        def.add_field(FieldDef::selection("activity_user_type", &[("specific", "Specific User"), ("generic", "Dynamic User (based on record)")]).string("User Type").computed("_compute_activity_info", &["model_id", "state"]).stored());
        def.add_field(FieldDef::many2one("activity_user_id", "res.users").string("Responsible").computed("_compute_activity_user_info", &["model_id", "activity_user_type"]).stored());
        def.add_field(FieldDef::char("activity_user_field_name").string("User Field").computed("_compute_activity_user_info", &["model_id", "activity_user_type"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_name_depends", "_generate_action_name", "_compute_available_model_ids", "_compute_template_id", "_compute_mail_post_autofollow", "_compute_mail_post_method", "_compute_followers_type", "_compute_followers_info", "_compute_activity_info", "_compute_activity_user_info", "_warning_depends", "_get_warning_messages", "_run_action_followers_multi", "_run_action_remove_followers_multi", "_is_recompute", "_run_action_mail_post_multi", "_run_action_next_activity", "_get_eval_context"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_name_depends" => self._name_depends(env, ctx, rs, args).await,
            "_generate_action_name" => self._generate_action_name(env, ctx, rs, args).await,
            "_compute_available_model_ids" => self._compute_available_model_ids(env, ctx, rs, args).await,
            "_compute_template_id" => self._compute_template_id(env, ctx, rs, args).await,
            "_compute_mail_post_autofollow" => self._compute_mail_post_autofollow(env, ctx, rs, args).await,
            "_compute_mail_post_method" => self._compute_mail_post_method(env, ctx, rs, args).await,
            "_compute_followers_type" => self._compute_followers_type(env, ctx, rs, args).await,
            "_compute_followers_info" => self._compute_followers_info(env, ctx, rs, args).await,
            "_compute_activity_info" => self._compute_activity_info(env, ctx, rs, args).await,
            "_compute_activity_user_info" => self._compute_activity_user_info(env, ctx, rs, args).await,
            "_warning_depends" => self._warning_depends(env, ctx, rs, args).await,
            "_get_warning_messages" => self._get_warning_messages(env, ctx, rs, args).await,
            "_run_action_followers_multi" => self._run_action_followers_multi(env, ctx, rs, args).await,
            "_run_action_remove_followers_multi" => self._run_action_remove_followers_multi(env, ctx, rs, args).await,
            "_is_recompute" => self._is_recompute(env, ctx, rs, args).await,
            "_run_action_mail_post_multi" => self._run_action_mail_post_multi(env, ctx, rs, args).await,
            "_run_action_next_activity" => self._run_action_next_activity(env, ctx, rs, args).await,
            "_get_eval_context" => self._get_eval_context(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl IrActionsServerFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_actions_server.py:111`).
    async fn _name_depends(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.server._name_depends".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_actions_server.py:114`).
    async fn _generate_action_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.server._generate_action_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_actions_server.py:123`). Decoradores: api.depends('state').
    async fn _compute_available_model_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.server._compute_available_model_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_actions_server.py:134`). Decoradores: api.depends('model_id', 'state').
    async fn _compute_template_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.server._compute_template_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_actions_server.py:143`). Decoradores: api.depends('state', 'mail_post_method').
    async fn _compute_mail_post_autofollow(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.server._compute_mail_post_autofollow".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_actions_server.py:152`). Decoradores: api.depends('state').
    async fn _compute_mail_post_method(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.server._compute_mail_post_method".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_actions_server.py:161`). Decoradores: api.depends('model_id', 'state').
    async fn _compute_followers_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.server._compute_followers_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_actions_server.py:168`). Decoradores: api.depends('followers_type').
    async fn _compute_followers_info(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.server._compute_followers_info".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_actions_server.py:185`). Decoradores: api.depends('model_id', 'state').
    async fn _compute_activity_info(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.server._compute_activity_info".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_actions_server.py:205`). Decoradores: api.depends('model_id', 'activity_user_type').
    async fn _compute_activity_user_info(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.server._compute_activity_user_info".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_actions_server.py:222`). Decoradores: api.model.
    async fn _warning_depends(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.server._warning_depends".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_actions_server.py:234`).
    async fn _get_warning_messages(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.server._get_warning_messages".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_actions_server.py:274`).
    async fn _run_action_followers_multi(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.server._run_action_followers_multi".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_actions_server.py:286`).
    async fn _run_action_remove_followers_multi(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.server._run_action_remove_followers_multi".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_actions_server.py:298`).
    async fn _is_recompute(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.server._is_recompute".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_actions_server.py:324`).
    async fn _run_action_mail_post_multi(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.server._run_action_mail_post_multi".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_actions_server.py:360`).
    async fn _run_action_next_activity(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.server._run_action_next_activity".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_actions_server.py:388`). Decoradores: api.model.
    async fn _get_eval_context(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.server._get_eval_context".into(),
        ))
    }

}
