//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_config_settings.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `res.config.settings` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct ResConfigSettingsExtFragment;

#[async_trait]
impl ModelFragment for ResConfigSettingsExtFragment {
    fn model_name(&self) -> &str {
        "res.config.settings"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::boolean("external_email_server_default").string("Use Custom Email Servers"));
        def.add_field(FieldDef::integer("fail_counter").string("Fail Mail").computed("_compute_fail_counter", &[]).stored());
        def.add_field({ let mut f = FieldDef::many2one("alias_domain_id", "mail.alias.domain").string("Alias Domain"); f.related = Some("company_id.alias_domain_id".into()); f });
        def.add_field(FieldDef::boolean("module_google_gmail").string("Support Gmail Authentication"));
        def.add_field(FieldDef::boolean("module_microsoft_outlook").string("Support Outlook Authentication"));
        def.add_field(FieldDef::boolean("restrict_template_rendering").string("Restrict Template Rendering"));
        def.add_field(FieldDef::boolean("use_twilio_rtc_servers").string("Use Twilio ICE servers"));
        def.add_field(FieldDef::char("twilio_account_sid").string("Account SID"));
        def.add_field(FieldDef::char("twilio_account_token").string("Account Auth Token"));
        def.add_field(FieldDef::boolean("use_sfu_server").string("Use SFU server"));
        def.add_field(FieldDef::char("sfu_server_url").string("SFU Server URL"));
        def.add_field(FieldDef::char("sfu_server_key").string("SFU Server key"));
        def.add_field({ let mut f = FieldDef::char("email_primary_color"); f.related = Some("company_id.email_primary_color".into()); f });
        def.add_field({ let mut f = FieldDef::char("email_secondary_color"); f.related = Some("company_id.email_secondary_color".into()); f });
        def.add_field(FieldDef::char("tenor_api_key").string("Tenor API key"));
        def.add_field(FieldDef::char("google_translate_api_key").string("Message Translation API Key"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_fail_counter", "open_email_layout", "open_mail_templates"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_fail_counter" => self._compute_fail_counter(env, ctx, rs, args).await,
            "open_email_layout" => self.open_email_layout(env, ctx, rs, args).await,
            "open_mail_templates" => self.open_mail_templates(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl ResConfigSettingsExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_config_settings.py:64`).
    async fn _compute_fail_counter(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.config.settings._compute_fail_counter".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_config_settings.py:72`).
    async fn open_email_layout(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.config.settings.open_email_layout".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_config_settings.py:84`).
    async fn open_mail_templates(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.config.settings.open_mail_templates".into(),
        ))
    }

}
