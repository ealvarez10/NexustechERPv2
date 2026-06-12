//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.move.send`

use nexus_orm::prelude::*;

pub struct AccountMoveSendFragment;

#[async_trait]
impl ModelFragment for AccountMoveSendFragment {
    fn model_name(&self) -> &str {
        "account.move.send"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Account Move Send".into();
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_get_default_sending_methods", "_get_all_extra_edis", "_get_default_extra_edis", "_get_default_invoice_edi_format", "_get_default_pdf_report_id", "_get_default_mail_template_id", "_get_default_sending_settings", "_get_alerts", "_get_mail_default_field_value_from_template", "_get_default_mail_lang", "_get_default_mail_body", "_get_default_mail_subject", "_get_default_mail_partner_ids", "_get_default_mail_attachments_widget", "_get_placeholder_mail_attachments_data", "_get_placeholder_mail_template_dynamic_attachments_data", "_get_invoice_extra_attachments", "_get_invoice_extra_attachments_data", "_get_mail_template_attachments_data", "_raise_danger_alerts", "_check_move_constraints", "_get_move_constraints", "_check_invoice_report", "_format_error_text", "_format_error_html", "_display_attachments_widget", "_is_applicable_to_company", "_is_applicable_to_move", "_hook_invoice_document_before_pdf_report_render", "_prepare_invoice_pdf_report", "_prepare_invoice_proforma_pdf_report", "_hook_invoice_document_after_pdf_report_render", "_link_invoice_documents", "_hook_if_errors", "_hook_if_success", "_send_notifications_to_partners", "_send_mail", "_get_mail_layout", "_get_mail_params", "_generate_dynamic_reports", "_send_mails", "_can_commit", "_call_web_service_before_invoice_pdf_render", "_call_web_service_after_invoice_pdf_render", "_generate_invoice_documents", "_generate_invoice_fallback_documents", "_check_sending_data", "_generate_and_send_invoices"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_get_default_sending_methods" => self._get_default_sending_methods(env, ctx, rs, args).await,
            "_get_all_extra_edis" => self._get_all_extra_edis(env, ctx, rs, args).await,
            "_get_default_extra_edis" => self._get_default_extra_edis(env, ctx, rs, args).await,
            "_get_default_invoice_edi_format" => self._get_default_invoice_edi_format(env, ctx, rs, args).await,
            "_get_default_pdf_report_id" => self._get_default_pdf_report_id(env, ctx, rs, args).await,
            "_get_default_mail_template_id" => self._get_default_mail_template_id(env, ctx, rs, args).await,
            "_get_default_sending_settings" => self._get_default_sending_settings(env, ctx, rs, args).await,
            "_get_alerts" => self._get_alerts(env, ctx, rs, args).await,
            "_get_mail_default_field_value_from_template" => self._get_mail_default_field_value_from_template(env, ctx, rs, args).await,
            "_get_default_mail_lang" => self._get_default_mail_lang(env, ctx, rs, args).await,
            "_get_default_mail_body" => self._get_default_mail_body(env, ctx, rs, args).await,
            "_get_default_mail_subject" => self._get_default_mail_subject(env, ctx, rs, args).await,
            "_get_default_mail_partner_ids" => self._get_default_mail_partner_ids(env, ctx, rs, args).await,
            "_get_default_mail_attachments_widget" => self._get_default_mail_attachments_widget(env, ctx, rs, args).await,
            "_get_placeholder_mail_attachments_data" => self._get_placeholder_mail_attachments_data(env, ctx, rs, args).await,
            "_get_placeholder_mail_template_dynamic_attachments_data" => self._get_placeholder_mail_template_dynamic_attachments_data(env, ctx, rs, args).await,
            "_get_invoice_extra_attachments" => self._get_invoice_extra_attachments(env, ctx, rs, args).await,
            "_get_invoice_extra_attachments_data" => self._get_invoice_extra_attachments_data(env, ctx, rs, args).await,
            "_get_mail_template_attachments_data" => self._get_mail_template_attachments_data(env, ctx, rs, args).await,
            "_raise_danger_alerts" => self._raise_danger_alerts(env, ctx, rs, args).await,
            "_check_move_constraints" => self._check_move_constraints(env, ctx, rs, args).await,
            "_get_move_constraints" => self._get_move_constraints(env, ctx, rs, args).await,
            "_check_invoice_report" => self._check_invoice_report(env, ctx, rs, args).await,
            "_format_error_text" => self._format_error_text(env, ctx, rs, args).await,
            "_format_error_html" => self._format_error_html(env, ctx, rs, args).await,
            "_display_attachments_widget" => self._display_attachments_widget(env, ctx, rs, args).await,
            "_is_applicable_to_company" => self._is_applicable_to_company(env, ctx, rs, args).await,
            "_is_applicable_to_move" => self._is_applicable_to_move(env, ctx, rs, args).await,
            "_hook_invoice_document_before_pdf_report_render" => self._hook_invoice_document_before_pdf_report_render(env, ctx, rs, args).await,
            "_prepare_invoice_pdf_report" => self._prepare_invoice_pdf_report(env, ctx, rs, args).await,
            "_prepare_invoice_proforma_pdf_report" => self._prepare_invoice_proforma_pdf_report(env, ctx, rs, args).await,
            "_hook_invoice_document_after_pdf_report_render" => self._hook_invoice_document_after_pdf_report_render(env, ctx, rs, args).await,
            "_link_invoice_documents" => self._link_invoice_documents(env, ctx, rs, args).await,
            "_hook_if_errors" => self._hook_if_errors(env, ctx, rs, args).await,
            "_hook_if_success" => self._hook_if_success(env, ctx, rs, args).await,
            "_send_notifications_to_partners" => self._send_notifications_to_partners(env, ctx, rs, args).await,
            "_send_mail" => self._send_mail(env, ctx, rs, args).await,
            "_get_mail_layout" => self._get_mail_layout(env, ctx, rs, args).await,
            "_get_mail_params" => self._get_mail_params(env, ctx, rs, args).await,
            "_generate_dynamic_reports" => self._generate_dynamic_reports(env, ctx, rs, args).await,
            "_send_mails" => self._send_mails(env, ctx, rs, args).await,
            "_can_commit" => self._can_commit(env, ctx, rs, args).await,
            "_call_web_service_before_invoice_pdf_render" => self._call_web_service_before_invoice_pdf_render(env, ctx, rs, args).await,
            "_call_web_service_after_invoice_pdf_render" => self._call_web_service_after_invoice_pdf_render(env, ctx, rs, args).await,
            "_generate_invoice_documents" => self._generate_invoice_documents(env, ctx, rs, args).await,
            "_generate_invoice_fallback_documents" => self._generate_invoice_fallback_documents(env, ctx, rs, args).await,
            "_check_sending_data" => self._check_sending_data(env, ctx, rs, args).await,
            "_generate_and_send_invoices" => self._generate_and_send_invoices(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountMoveSendFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:26`). Decoradores: api.model.
    async fn _get_default_sending_methods(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_default_sending_methods".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:31`). Decoradores: api.model.
    async fn _get_all_extra_edis(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_all_extra_edis".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:38`). Decoradores: api.model.
    async fn _get_default_extra_edis(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_default_extra_edis".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:44`). Decoradores: api.model.
    async fn _get_default_invoice_edi_format(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_default_invoice_edi_format".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:49`). Decoradores: api.model.
    async fn _get_default_pdf_report_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_default_pdf_report_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:64`). Decoradores: api.model.
    async fn _get_default_mail_template_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_default_mail_template_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:68`). Decoradores: api.model.
    async fn _get_default_sending_settings(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_default_sending_settings".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:110`). Decoradores: api.model.
    async fn _get_alerts(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_alerts".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:161`). Decoradores: api.model.
    async fn _get_mail_default_field_value_from_template(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_mail_default_field_value_from_template".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:169`). Decoradores: api.model.
    async fn _get_default_mail_lang(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_default_mail_lang".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:173`). Decoradores: api.model.
    async fn _get_default_mail_body(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_default_mail_body".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:183`). Decoradores: api.model.
    async fn _get_default_mail_subject(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_default_mail_subject".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:192`). Decoradores: api.model.
    async fn _get_default_mail_partner_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_default_mail_partner_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:227`). Decoradores: api.model.
    async fn _get_default_mail_attachments_widget(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_default_mail_attachments_widget".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:234`). Decoradores: api.model.
    async fn _get_placeholder_mail_attachments_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_placeholder_mail_attachments_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:255`). Decoradores: api.model.
    async fn _get_placeholder_mail_template_dynamic_attachments_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_placeholder_mail_template_dynamic_attachments_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:284`). Decoradores: api.model.
    async fn _get_invoice_extra_attachments(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_invoice_extra_attachments".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:288`). Decoradores: api.model.
    async fn _get_invoice_extra_attachments_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_invoice_extra_attachments_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:301`). Decoradores: api.model.
    async fn _get_mail_template_attachments_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_mail_template_attachments_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:320`). Decoradores: api.model.
    async fn _raise_danger_alerts(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._raise_danger_alerts".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:326`). Decoradores: api.model.
    async fn _check_move_constraints(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._check_move_constraints".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:332`). Decoradores: api.model.
    async fn _get_move_constraints(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_move_constraints".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:341`). Decoradores: api.model.
    async fn _check_invoice_report(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._check_invoice_report".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:351`). Decoradores: api.model.
    async fn _format_error_text(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._format_error_text".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:361`). Decoradores: api.model.
    async fn _format_error_html(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._format_error_html".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:373`). Decoradores: api.model.
    async fn _display_attachments_widget(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._display_attachments_widget".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:381`). Decoradores: api.model.
    async fn _is_applicable_to_company(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._is_applicable_to_company".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:386`). Decoradores: api.model.
    async fn _is_applicable_to_move(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._is_applicable_to_move".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:393`). Decoradores: api.model.
    async fn _hook_invoice_document_before_pdf_report_render(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._hook_invoice_document_before_pdf_report_render".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:402`). Decoradores: api.model.
    async fn _prepare_invoice_pdf_report(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._prepare_invoice_pdf_report".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:432`). Decoradores: api.model.
    async fn _prepare_invoice_proforma_pdf_report(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._prepare_invoice_proforma_pdf_report".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:450`). Decoradores: api.model.
    async fn _hook_invoice_document_after_pdf_report_render(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._hook_invoice_document_after_pdf_report_render".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:459`). Decoradores: api.model.
    async fn _link_invoice_documents(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._link_invoice_documents".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:480`). Decoradores: api.model.
    async fn _hook_if_errors(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._hook_if_errors".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:492`). Decoradores: api.model.
    async fn _hook_if_success(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._hook_if_success".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:523`). Decoradores: api.model.
    async fn _send_notifications_to_partners(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._send_notifications_to_partners".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:550`). Decoradores: api.model.
    async fn _send_mail(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._send_mail".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:578`). Decoradores: api.model.
    async fn _get_mail_layout(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_mail_layout".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:582`). Decoradores: api.model.
    async fn _get_mail_params(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._get_mail_params".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:613`). Decoradores: api.model.
    async fn _generate_dynamic_reports(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._generate_dynamic_reports".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:649`). Decoradores: api.model.
    async fn _send_mails(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._send_mails".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:689`). Decoradores: api.model.
    async fn _can_commit(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._can_commit".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:696`). Decoradores: api.model.
    async fn _call_web_service_before_invoice_pdf_render(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._call_web_service_before_invoice_pdf_render".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:702`). Decoradores: api.model.
    async fn _call_web_service_after_invoice_pdf_render(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._call_web_service_after_invoice_pdf_render".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:708`). Decoradores: api.model.
    async fn _generate_invoice_documents(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._generate_invoice_documents".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:784`). Decoradores: api.model.
    async fn _generate_invoice_fallback_documents(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._generate_invoice_fallback_documents".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:796`).
    async fn _check_sending_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._check_sending_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_move_send.py:808`). Decoradores: api.model.
    async fn _generate_and_send_invoices(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.move.send._generate_and_send_invoices".into(),
        ))
    }

}
