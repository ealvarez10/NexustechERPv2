//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users_settings_volumes.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `res.users.settings.volumes`

use nexus_orm::prelude::*;

pub struct ResUsersSettingsVolumesFragment;

#[async_trait]
impl ModelFragment for ResUsersSettingsVolumesFragment {
    fn model_name(&self) -> &str {
        "res.users.settings.volumes"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "User Settings Volumes".into();
        def.add_field(FieldDef::many2one("user_setting_id", "res.users.settings").required());
        def.add_field(FieldDef::many2one("partner_id", "res.partner"));
        def.add_field(FieldDef::many2one("guest_id", "res.partner"));
        def.add_field(FieldDef::float("volume").default_val(0.5f64));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_display_name", "_discuss_users_settings_volume_format"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_display_name" => self._compute_display_name(env, ctx, rs, args).await,
            "_discuss_users_settings_volume_format" => self._discuss_users_settings_volume_format(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl ResUsersSettingsVolumesFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users_settings_volumes.py:25`). Decoradores: api.depends('user_setting_id', 'partner_id', 'guest_id').
    async fn _compute_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users.settings.volumes._compute_display_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_users_settings_volumes.py:29`).
    async fn _discuss_users_settings_volume_format(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.users.settings.volumes._discuss_users_settings_volume_format".into(),
        ))
    }

}
