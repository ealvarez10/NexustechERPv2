//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `base` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct BaseExtFragment;

#[async_trait]
impl ModelFragment for BaseExtFragment {
    fn model_name(&self) -> &str {
        "base"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_valid_field_parameter", "with_user", "unlink", "_mail_get_operation_for_mail_message_operation", "_mail_group_by_operation_for_mail_message_operation", "_mail_get_alias_domains", "_mail_get_company_field", "_mail_get_companies", "_mail_get_customer", "_mail_get_partner_fields", "mail_get_partner_fields", "_mail_get_partners", "_mail_get_primary_email_field", "_mail_get_primary_email", "mail_allowed_qweb_expressions", "_mail_track", "_mail_track_order_fields", "_mail_track_get_field_sequence", "_message_add_default_recipients", "_message_get_default_recipients", "_message_add_suggested_recipients", "_message_get_suggested_recipients_batch", "_sort_suggested_messages", "_message_get_suggested_recipients", "_notify_get_reply_to", "_notify_get_reply_to_batch", "_notify_get_reply_to_formatted_email", "_alias_get_error", "_get_default_activity_view", "_mail_get_message_subtypes", "_notify_by_email_get_headers", "_get_html_link", "_get_backend_root_menu_ids", "_find_value_from_field_path", "_mail_get_timezone"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_valid_field_parameter" => self._valid_field_parameter(env, ctx, rs, args).await,
            "with_user" => self.with_user(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "_mail_get_operation_for_mail_message_operation" => self._mail_get_operation_for_mail_message_operation(env, ctx, rs, args).await,
            "_mail_group_by_operation_for_mail_message_operation" => self._mail_group_by_operation_for_mail_message_operation(env, ctx, rs, args).await,
            "_mail_get_alias_domains" => self._mail_get_alias_domains(env, ctx, rs, args).await,
            "_mail_get_company_field" => self._mail_get_company_field(env, ctx, rs, args).await,
            "_mail_get_companies" => self._mail_get_companies(env, ctx, rs, args).await,
            "_mail_get_customer" => self._mail_get_customer(env, ctx, rs, args).await,
            "_mail_get_partner_fields" => self._mail_get_partner_fields(env, ctx, rs, args).await,
            "mail_get_partner_fields" => self.mail_get_partner_fields(env, ctx, rs, args).await,
            "_mail_get_partners" => self._mail_get_partners(env, ctx, rs, args).await,
            "_mail_get_primary_email_field" => self._mail_get_primary_email_field(env, ctx, rs, args).await,
            "_mail_get_primary_email" => self._mail_get_primary_email(env, ctx, rs, args).await,
            "mail_allowed_qweb_expressions" => self.mail_allowed_qweb_expressions(env, ctx, rs, args).await,
            "_mail_track" => self._mail_track(env, ctx, rs, args).await,
            "_mail_track_order_fields" => self._mail_track_order_fields(env, ctx, rs, args).await,
            "_mail_track_get_field_sequence" => self._mail_track_get_field_sequence(env, ctx, rs, args).await,
            "_message_add_default_recipients" => self._message_add_default_recipients(env, ctx, rs, args).await,
            "_message_get_default_recipients" => self._message_get_default_recipients(env, ctx, rs, args).await,
            "_message_add_suggested_recipients" => self._message_add_suggested_recipients(env, ctx, rs, args).await,
            "_message_get_suggested_recipients_batch" => self._message_get_suggested_recipients_batch(env, ctx, rs, args).await,
            "_sort_suggested_messages" => self._sort_suggested_messages(env, ctx, rs, args).await,
            "_message_get_suggested_recipients" => self._message_get_suggested_recipients(env, ctx, rs, args).await,
            "_notify_get_reply_to" => self._notify_get_reply_to(env, ctx, rs, args).await,
            "_notify_get_reply_to_batch" => self._notify_get_reply_to_batch(env, ctx, rs, args).await,
            "_notify_get_reply_to_formatted_email" => self._notify_get_reply_to_formatted_email(env, ctx, rs, args).await,
            "_alias_get_error" => self._alias_get_error(env, ctx, rs, args).await,
            "_get_default_activity_view" => self._get_default_activity_view(env, ctx, rs, args).await,
            "_mail_get_message_subtypes" => self._mail_get_message_subtypes(env, ctx, rs, args).await,
            "_notify_by_email_get_headers" => self._notify_by_email_get_headers(env, ctx, rs, args).await,
            "_get_html_link" => self._get_html_link(env, ctx, rs, args).await,
            "_get_backend_root_menu_ids" => self._get_backend_root_menu_ids(env, ctx, rs, args).await,
            "_find_value_from_field_path" => self._find_value_from_field_path(env, ctx, rs, args).await,
            "_mail_get_timezone" => self._mail_get_timezone(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl BaseExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:30`).
    async fn _valid_field_parameter(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._valid_field_parameter".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:37`).
    async fn with_user(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base.with_user".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:46`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:66`).
    async fn _mail_get_operation_for_mail_message_operation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._mail_get_operation_for_mail_message_operation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:85`).
    async fn _mail_group_by_operation_for_mail_message_operation(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._mail_group_by_operation_for_mail_message_operation".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:101`).
    async fn _mail_get_alias_domains(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._mail_get_alias_domains".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:128`). Decoradores: api.model.
    async fn _mail_get_company_field(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._mail_get_company_field".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:131`).
    async fn _mail_get_companies(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._mail_get_companies".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:146`).
    async fn _mail_get_customer(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._mail_get_customer".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:154`). Decoradores: api.model.
    async fn _mail_get_partner_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._mail_get_partner_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:175`).
    async fn mail_get_partner_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base.mail_get_partner_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:178`).
    async fn _mail_get_partners(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._mail_get_partners".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:195`). Decoradores: api.model.
    async fn _mail_get_primary_email_field(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._mail_get_primary_email_field".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:203`).
    async fn _mail_get_primary_email(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._mail_get_primary_email".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:213`). Decoradores: api.model.
    async fn mail_allowed_qweb_expressions(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base.mail_allowed_qweb_expressions".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:229`).
    async fn _mail_track(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._mail_track".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:292`).
    async fn _mail_track_order_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._mail_track_order_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:311`).
    async fn _mail_track_get_field_sequence(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._mail_track_get_field_sequence".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:331`).
    async fn _message_add_default_recipients(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._message_add_default_recipients".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:380`).
    async fn _message_get_default_recipients(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._message_get_default_recipients".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:457`).
    async fn _message_add_suggested_recipients(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._message_add_suggested_recipients".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:486`).
    async fn _message_get_suggested_recipients_batch(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._message_get_suggested_recipients_batch".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:619`).
    async fn _sort_suggested_messages(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._sort_suggested_messages".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:637`).
    async fn _message_get_suggested_recipients(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._message_get_suggested_recipients".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:645`).
    async fn _notify_get_reply_to(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._notify_get_reply_to".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:672`).
    async fn _notify_get_reply_to_batch(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._notify_get_reply_to_batch".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:740`).
    async fn _notify_get_reply_to_formatted_email(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._notify_get_reply_to_formatted_email".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:783`).
    async fn _alias_get_error(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._alias_get_error".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:809`). Decoradores: api.model.
    async fn _get_default_activity_view(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._get_default_activity_view".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:824`).
    async fn _mail_get_message_subtypes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._mail_get_message_subtypes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:833`).
    async fn _notify_by_email_get_headers(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._notify_by_email_get_headers".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:851`).
    async fn _get_html_link(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._get_html_link".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:865`). Decoradores: api.model.
    async fn _get_backend_root_menu_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._get_backend_root_menu_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:874`).
    async fn _find_value_from_field_path(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._find_value_from_field_path".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/models.py:918`).
    async fn _mail_get_timezone(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): base._mail_get_timezone".into(),
        ))
    }

}
