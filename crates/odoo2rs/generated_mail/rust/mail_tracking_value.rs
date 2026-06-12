//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_tracking_value.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.tracking.value`

use nexus_orm::prelude::*;

pub struct MailTrackingValueFragment;

#[async_trait]
impl ModelFragment for MailTrackingValueFragment {
    fn model_name(&self) -> &str {
        "mail.tracking.value"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Mail Tracking Value".into();
        def.order = "id DESC".into();
        def.rec_name = "field_id".into();
        def.add_field(FieldDef::many2one("field_id", "ir.model.fields").readonly());
        def.add_field(FieldDef::json("field_info").string("Removed field information"));
        def.add_field(FieldDef::integer("old_value_integer").string("Old Value Integer").readonly());
        def.add_field(FieldDef::float("old_value_float").string("Old Value Float").readonly());
        def.add_field(FieldDef::char("old_value_char").string("Old Value Char").readonly());
        def.add_field(FieldDef::text("old_value_text").string("Old Value Text").readonly());
        def.add_field(FieldDef::datetime("old_value_datetime").string("Old Value DateTime").readonly());
        def.add_field(FieldDef::integer("new_value_integer").string("New Value Integer").readonly());
        def.add_field(FieldDef::float("new_value_float").string("New Value Float").readonly());
        def.add_field(FieldDef::char("new_value_char").string("New Value Char").readonly());
        def.add_field(FieldDef::text("new_value_text").string("New Value Text").readonly());
        def.add_field(FieldDef::datetime("new_value_datetime").string("New Value Datetime").readonly());
        def.add_field(FieldDef::many2one("currency_id", "res.currency").string("Currency").readonly());
        def.add_field(FieldDef::many2one("mail_message_id", "mail.message").string("Message ID").required());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_filter_has_field_access", "_filter_free_field_access", "_create_tracking_values", "_create_tracking_values_property", "_tracking_value_format", "_tracking_value_format_model", "_format_display_value"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_filter_has_field_access" => self._filter_has_field_access(env, ctx, rs, args).await,
            "_filter_free_field_access" => self._filter_free_field_access(env, ctx, rs, args).await,
            "_create_tracking_values" => self._create_tracking_values(env, ctx, rs, args).await,
            "_create_tracking_values_property" => self._create_tracking_values_property(env, ctx, rs, args).await,
            "_tracking_value_format" => self._tracking_value_format(env, ctx, rs, args).await,
            "_tracking_value_format_model" => self._tracking_value_format_model(env, ctx, rs, args).await,
            "_format_display_value" => self._format_display_value(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailTrackingValueFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_tracking_value.py:37`).
    async fn _filter_has_field_access(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.tracking.value._filter_has_field_access".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_tracking_value.py:54`).
    async fn _filter_free_field_access(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.tracking.value._filter_free_field_access".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_tracking_value.py:68`). Decoradores: api.model.
    async fn _create_tracking_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.tracking.value._create_tracking_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_tracking_value.py:177`). Decoradores: api.model.
    async fn _create_tracking_values_property(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.tracking.value._create_tracking_values_property".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_tracking_value.py:194`).
    async fn _tracking_value_format(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.tracking.value._tracking_value_format".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_tracking_value.py:213`).
    async fn _tracking_value_format_model(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.tracking.value._tracking_value_format_model".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_tracking_value.py:277`).
    async fn _format_display_value(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.tracking.value._format_display_value".into(),
        ))
    }

}
