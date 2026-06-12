//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users_settings.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `res.users.settings` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct ResUsersSettingsExtFragment;

#[async_trait]
impl ModelFragment for ResUsersSettingsExtFragment {
    fn model_name(&self) -> &str {
        "res.users.settings"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::boolean("is_discuss_sidebar_category_channel_open").string("Is discuss sidebar category channel open?").default_val(true));
        def.add_field(FieldDef::boolean("is_discuss_sidebar_category_chat_open").string("Is discuss sidebar category chat open?").default_val(true));
        def.add_field(FieldDef::char("push_to_talk_key").string("Push-To-Talk shortcut"));
        def.add_field(FieldDef::boolean("use_push_to_talk").string("Use the push to talk feature").default_val(false));
        def.add_field(FieldDef::integer("voice_active_duration").string("Duration of voice activity in ms").default_val(200i64));
        def.add_field(FieldDef::one2many("volume_settings_ids", "res.users.settings.volumes", "user_setting_id").string("Volumes of other partners"));
        def.add_field(FieldDef::selection("channel_notifications", &[("all", "All Messages"), ("no_notif", "Nothing")]).string("Channel Notifications"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_format_settings", "set_res_users_settings", "set_volume_setting"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_format_settings" => self._format_settings(env, ctx, rs, args).await,
            "set_res_users_settings" => self.set_res_users_settings(env, ctx, rs, args).await,
            "set_volume_setting" => self.set_volume_setting(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl ResUsersSettingsExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users_settings.py:26`). Decoradores: api.model.
    async fn _format_settings(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users.settings._format_settings".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users_settings.py:34`).
    async fn set_res_users_settings(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users.settings.set_res_users_settings".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users_settings.py:39`).
    async fn set_volume_setting(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users.settings.set_volume_setting".into(),
        ))
    }

}
