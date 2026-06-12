//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `res.users` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct ResUsersExtFragment;

#[async_trait]
impl ModelFragment for ResUsersExtFragment {
    fn model_name(&self) -> &str {
        "res.users"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::many2many("role_ids", "res.role").string("User Roles"));
        def.add_field(FieldDef::boolean("can_edit_role").computed("_compute_can_edit_role", &[]).stored());
        def.add_field(FieldDef::selection("notification_type", &[("email", "By Emails"), ("inbox", "In Odoo")]).string("Notification").required().computed("_compute_notification_type", &["share", "all_group_ids"]).stored().default_val("email"));
        def.add_field(FieldDef::one2many("presence_ids", "mail.presence", "user_id"));
        def.add_field(FieldDef::datetime("out_of_office_from"));
        def.add_field(FieldDef::datetime("out_of_office_to"));
        def.add_field(FieldDef::html("out_of_office_message").string("Vacation Responder"));
        def.add_field(FieldDef::boolean("is_out_of_office").string("Out of Office").computed("_compute_is_out_of_office", &["out_of_office_from", "out_of_office_to"]).stored());
        def.add_field(FieldDef::char("im_status").string("IM Status").computed("_compute_im_status", &["manual_im_status", "presence_ids.status"]).stored());
        def.add_field(FieldDef::selection("manual_im_status", &[("away", "Away"), ("busy", "Do Not Disturb"), ("offline", "Offline")]).string("IM status manually set by the user"));
        def.add_field(FieldDef::many2one("outgoing_mail_server_id", "ir.mail_server").string("Outgoing Mail Server").computed("_compute_outgoing_mail_server_id", &["email"]).stored());
        def.add_field(FieldDef::selection("outgoing_mail_server_type", &[("default", "Default")]).string("Outgoing Mail Server Type").required().computed("_compute_outgoing_mail_server_id", &["email"]).stored().default_val("default"));
        def.add_field(FieldDef::boolean("has_external_mail_server").computed("_compute_has_external_mail_server", &[]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_has_external_mail_server", "_compute_notification_type", "_compute_is_out_of_office", "_compute_im_status", "_inverse_notification_type", "_compute_can_edit_role", "_compute_outgoing_mail_server_id", "SELF_READABLE_FIELDS", "SELF_WRITEABLE_FIELDS", "create", "write", "action_archive", "_notify_security_setting_update", "_notify_security_setting_update_prepare_values", "_get_portal_access_update_body", "_deactivate_portal_user", "_init_store_data", "_init_messaging", "_get_activity_groups", "_get_store_avatar_card_fields", "_gc_personal_mail_servers", "_get_mail_server_values", "action_setup_outgoing_mail_server", "action_test_outgoing_mail_server", "_get_mail_server_setup_end_action"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_has_external_mail_server" => self._compute_has_external_mail_server(env, ctx, rs, args).await,
            "_compute_notification_type" => self._compute_notification_type(env, ctx, rs, args).await,
            "_compute_is_out_of_office" => self._compute_is_out_of_office(env, ctx, rs, args).await,
            "_compute_im_status" => self._compute_im_status(env, ctx, rs, args).await,
            "_inverse_notification_type" => self._inverse_notification_type(env, ctx, rs, args).await,
            "_compute_can_edit_role" => self._compute_can_edit_role(env, ctx, rs, args).await,
            "_compute_outgoing_mail_server_id" => self._compute_outgoing_mail_server_id(env, ctx, rs, args).await,
            "SELF_READABLE_FIELDS" => self.SELF_READABLE_FIELDS(env, ctx, rs, args).await,
            "SELF_WRITEABLE_FIELDS" => self.SELF_WRITEABLE_FIELDS(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "action_archive" => self.action_archive(env, ctx, rs, args).await,
            "_notify_security_setting_update" => self._notify_security_setting_update(env, ctx, rs, args).await,
            "_notify_security_setting_update_prepare_values" => self._notify_security_setting_update_prepare_values(env, ctx, rs, args).await,
            "_get_portal_access_update_body" => self._get_portal_access_update_body(env, ctx, rs, args).await,
            "_deactivate_portal_user" => self._deactivate_portal_user(env, ctx, rs, args).await,
            "_init_store_data" => self._init_store_data(env, ctx, rs, args).await,
            "_init_messaging" => self._init_messaging(env, ctx, rs, args).await,
            "_get_activity_groups" => self._get_activity_groups(env, ctx, rs, args).await,
            "_get_store_avatar_card_fields" => self._get_store_avatar_card_fields(env, ctx, rs, args).await,
            "_gc_personal_mail_servers" => self._gc_personal_mail_servers(env, ctx, rs, args).await,
            "_get_mail_server_values" => self._get_mail_server_values(env, ctx, rs, args).await,
            "action_setup_outgoing_mail_server" => self.action_setup_outgoing_mail_server(env, ctx, rs, args).await,
            "action_test_outgoing_mail_server" => self.action_test_outgoing_mail_server(env, ctx, rs, args).await,
            "_get_mail_server_setup_end_action" => self._get_mail_server_setup_end_action(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl ResUsersExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:66`).
    async fn _compute_has_external_mail_server(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users._compute_has_external_mail_server".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:76`). Decoradores: api.depends('share', 'all_group_ids').
    async fn _compute_notification_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users._compute_notification_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:98`). Decoradores: api.depends('out_of_office_from', 'out_of_office_to').
    async fn _compute_is_out_of_office(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users._compute_is_out_of_office".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:112`). Decoradores: api.depends('manual_im_status', 'presence_ids.status').
    async fn _compute_im_status(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users._compute_im_status".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:120`).
    async fn _inverse_notification_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users._inverse_notification_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:127`). Decoradores: api.depends_context('uid').
    async fn _compute_can_edit_role(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users._compute_can_edit_role".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:131`). Decoradores: api.depends('email').
    async fn _compute_outgoing_mail_server_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users._compute_outgoing_mail_server_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:156`). Decoradores: property.
    async fn SELF_READABLE_FIELDS(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users.SELF_READABLE_FIELDS".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:171`). Decoradores: property.
    async fn SELF_WRITEABLE_FIELDS(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users.SELF_WRITEABLE_FIELDS".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:180`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:197`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:258`).
    async fn action_archive(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users.action_archive".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:263`).
    async fn _notify_security_setting_update(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users._notify_security_setting_update".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:317`).
    async fn _notify_security_setting_update_prepare_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users._notify_security_setting_update_prepare_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:352`).
    async fn _get_portal_access_update_body(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users._get_portal_access_update_body".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:358`).
    async fn _deactivate_portal_user(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users._deactivate_portal_user".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:391`). Decoradores: api.model.
    async fn _init_store_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users._init_store_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:433`).
    async fn _init_messaging(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users._init_messaging".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:456`). Decoradores: api.model.
    async fn _get_activity_groups(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users._get_activity_groups".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:537`).
    async fn _get_store_avatar_card_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users._get_store_avatar_card_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:545`). Decoradores: api.autovacuum.
    async fn _gc_personal_mail_servers(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users._gc_personal_mail_servers".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:553`). Decoradores: api.model.
    async fn _get_mail_server_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users._get_mail_server_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:557`). Decoradores: api.model.
    async fn action_setup_outgoing_mail_server(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users.action_setup_outgoing_mail_server".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:636`). Decoradores: api.model.
    async fn action_test_outgoing_mail_server(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users.action_test_outgoing_mail_server".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users.py:658`). Decoradores: api.model.
    async fn _get_mail_server_setup_end_action(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users._get_mail_server_setup_end_action".into(),
        ))
    }

}
