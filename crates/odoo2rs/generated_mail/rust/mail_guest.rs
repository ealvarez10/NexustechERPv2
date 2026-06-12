//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/mail_guest.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.guest`

use nexus_orm::prelude::*;

pub struct MailGuestFragment;

#[async_trait]
impl ModelFragment for MailGuestFragment {
    fn model_name(&self) -> &str {
        "mail.guest"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Guest".into();
        def.add_field(FieldDef::char("name").string("Name").required());
        def.add_field(FieldDef::char("access_token").string("Access Token").required().readonly());
        def.add_field(FieldDef::many2one("country_id", "res.country").string("Country"));
        def.add_field(FieldDef::char("email"));
        def.add_field(FieldDef::selection("lang", &[]).string("Language"));
        def.add_field(FieldDef::selection("timezone", &[]).string("Timezone"));
        def.add_field(FieldDef::many2many("channel_ids", "discuss.channel").string("Channels"));
        def.add_field(FieldDef::one2many("presence_ids", "mail.presence", "guest_id"));
        def.add_field(FieldDef::char("im_status").string("IM Status").computed("_compute_im_status", &["presence_ids.status"]).stored());
        def.add_field(FieldDef::datetime("offline_since").string("Offline since").computed("_compute_im_status", &["presence_ids.status"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_lang_get", "_compute_im_status", "_get_guest_from_token", "_get_guest_from_context", "_get_or_create_guest", "_get_timezone_from_request", "_update_name", "_update_timezone", "_get_im_status_access_token", "_field_store_repr", "_to_store_defaults", "_set_auth_cookie", "_format_auth_cookie"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_lang_get" => self._lang_get(env, ctx, rs, args).await,
            "_compute_im_status" => self._compute_im_status(env, ctx, rs, args).await,
            "_get_guest_from_token" => self._get_guest_from_token(env, ctx, rs, args).await,
            "_get_guest_from_context" => self._get_guest_from_context(env, ctx, rs, args).await,
            "_get_or_create_guest" => self._get_or_create_guest(env, ctx, rs, args).await,
            "_get_timezone_from_request" => self._get_timezone_from_request(env, ctx, rs, args).await,
            "_update_name" => self._update_name(env, ctx, rs, args).await,
            "_update_timezone" => self._update_timezone(env, ctx, rs, args).await,
            "_get_im_status_access_token" => self._get_im_status_access_token(env, ctx, rs, args).await,
            "_field_store_repr" => self._field_store_repr(env, ctx, rs, args).await,
            "_to_store_defaults" => self._to_store_defaults(env, ctx, rs, args).await,
            "_set_auth_cookie" => self._set_auth_cookie(env, ctx, rs, args).await,
            "_format_auth_cookie" => self._format_auth_cookie(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailGuestFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/mail_guest.py:25`). Decoradores: api.model.
    async fn _lang_get(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.guest._lang_get".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/mail_guest.py:41`). Decoradores: api.depends('presence_ids.status').
    async fn _compute_im_status(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.guest._compute_im_status".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/mail_guest.py:50`).
    async fn _get_guest_from_token(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.guest._get_guest_from_token".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/mail_guest.py:62`).
    async fn _get_guest_from_context(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.guest._get_guest_from_context".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/mail_guest.py:70`).
    async fn _get_or_create_guest(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.guest._get_or_create_guest".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/mail_guest.py:83`).
    async fn _get_timezone_from_request(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.guest._get_timezone_from_request".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/mail_guest.py:87`).
    async fn _update_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.guest._update_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/mail_guest.py:99`).
    async fn _update_timezone(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.guest._update_timezone".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/mail_guest.py:110`).
    async fn _get_im_status_access_token(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.guest._get_im_status_access_token".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/mail_guest.py:119`).
    async fn _field_store_repr(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.guest._field_store_repr".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/mail_guest.py:132`).
    async fn _to_store_defaults(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.guest._to_store_defaults".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/mail_guest.py:135`).
    async fn _set_auth_cookie(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.guest._set_auth_cookie".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/mail_guest.py:150`).
    async fn _format_auth_cookie(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.guest._format_auth_cookie".into(),
        ))
    }

}
