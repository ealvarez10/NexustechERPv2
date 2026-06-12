//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `res.partner` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct ResPartnerExtFragment;

#[async_trait]
impl ModelFragment for ResPartnerExtFragment {
    fn model_name(&self) -> &str {
        "res.partner"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::char("fiscal_country_codes").computed("_compute_fiscal_country_codes", &["company_id", "country_code"]).stored());
        def.add_field(FieldDef::json("fiscal_country_group_codes").computed("_compute_fiscal_country_group_codes", &["company_id"]).stored());
        def.add_field(FieldDef::char("partner_vat_placeholder").computed("_compute_partner_vat_placeholder", &["country_id"]).stored());
        def.add_field(FieldDef::char("partner_company_registry_placeholder").computed("_compute_partner_company_registry_placeholder", &["country_id"]).stored());
        // TODO(odoo2rs): campo 'duplicate_bank_partner_ids' (many2many) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::char("name"));
        def.add_field(FieldDef::monetary("credit").string("Total Receivable").computed("_credit_debit_get", &[]).stored());
        def.add_field(FieldDef::monetary("credit_to_invoice").computed("_compute_credit_to_invoice", &[]).stored());
        def.add_field(FieldDef::float("credit_limit").string("Credit Limit"));
        def.add_field(FieldDef::boolean("use_partner_credit_limit").string("Partner Limit").computed("_compute_use_partner_credit_limit", &[]).stored());
        def.add_field(FieldDef::boolean("show_credit_limit").computed("_compute_show_credit_limit", &[]).stored());
        def.add_field(FieldDef::float("days_sales_outstanding").string("Days Sales Outstanding (DSO)").computed("_compute_days_sales_outstanding", &["credit"]).stored());
        def.add_field(FieldDef::monetary("debit").string("Total Payable").computed("_credit_debit_get", &[]).stored());
        def.add_field(FieldDef::monetary("total_invoiced").string("Total Invoiced").computed("_invoice_total", &[]).stored());
        def.add_field(FieldDef::many2one("currency_id", "res.currency").string("Currency").readonly().computed("_get_company_currency", &[]).stored());
        def.add_field(FieldDef::many2one("property_account_payable_id", "account.account").string("Account Payable"));
        def.add_field(FieldDef::many2one("property_account_receivable_id", "account.account").string("Account Receivable"));
        def.add_field(FieldDef::many2one("property_account_position_id", "account.fiscal.position").string("Fiscal Position"));
        def.add_field(FieldDef::many2one("property_payment_term_id", "account.payment.term").string("Customer Payment Terms"));
        def.add_field(FieldDef::many2one("property_supplier_payment_term_id", "account.payment.term").string("Vendor Payment Terms"));
        def.add_field(FieldDef::one2many("ref_company_ids", "res.company", "partner_id").string("Companies that refers to partner"));
        def.add_field(FieldDef::integer("supplier_invoice_count").string("# Vendor Bills").computed("_compute_supplier_invoice_count", &[]).stored());
        def.add_field(FieldDef::integer("account_move_count").computed("_compute_account_move_count", &[]).stored());
        def.add_field(FieldDef::one2many("invoice_ids", "account.move", "partner_id").string("Invoices").readonly());
        def.add_field(FieldDef::one2many("contract_ids", "account.analytic.account", "partner_id").string("Partner Contracts").readonly());
        def.add_field(FieldDef::integer("bank_account_count").string("Bank").computed("_compute_bank_count", &[]).stored());
        def.add_field(FieldDef::selection("trust", &[("good", "Good Debtor"), ("normal", "Normal Debtor"), ("bad", "Bad Debtor")]).string("Degree of trust you have in this debtor"));
        def.add_field(FieldDef::boolean("ignore_abnormal_invoice_date"));
        def.add_field(FieldDef::boolean("ignore_abnormal_invoice_amount"));
        def.add_field(FieldDef::selection("invoice_sending_method", &[("manual", "Manual"), ("email", "by Email")]).string("Invoice sending"));
        def.add_field(FieldDef::selection("invoice_edi_format", &[]).string("eInvoice format").computed("_compute_invoice_edi_format", &["country_code"]).stored());
        def.add_field(FieldDef::char("invoice_edi_format_store"));
        def.add_field({ let mut f = FieldDef::boolean("display_invoice_edi_format"); f.store = false; f });
        def.add_field(FieldDef::many2one("invoice_template_pdf_report_id", "ir.actions.report").string("Invoice report"));
        // TODO(odoo2rs): campo 'available_invoice_template_pdf_report_ids' (one2many) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field({ let mut f = FieldDef::boolean("display_invoice_template_pdf_report_id"); f.store = false; f });
        def.add_field(FieldDef::integer("supplier_rank").default_val(0i64));
        def.add_field(FieldDef::integer("customer_rank").default_val(0i64));
        def.add_field(FieldDef::selection("autopost_bills", &[("always", "Always"), ("ask", "Ask after 3 validations without edits"), ("never", "Never")]).string("Auto-post bills").required().default_val("ask"));
        def.add_field(FieldDef::many2one("property_outbound_payment_method_line_id", "account.payment.method.line"));
        def.add_field(FieldDef::many2one("property_inbound_payment_method_line_id", "account.payment.method.line"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_fiscal_country_codes", "_compute_fiscal_country_group_codes", "_order", "_credit_debit_get", "_compute_credit_to_invoice", "_asset_difference_search", "_credit_search", "_debit_search", "_invoice_total", "_compute_days_sales_outstanding", "_compute_available_invoice_template_pdf_report_ids", "_get_company_currency", "_default_display_invoice_template_pdf_report_id", "_compute_bank_count", "_compute_supplier_invoice_count", "_compute_invoice_edi_format", "_inverse_invoice_edi_format", "_compute_use_partner_credit_limit", "_inverse_use_partner_credit_limit", "_compute_show_credit_limit", "_compute_application_statistics_hook", "_get_account_statistics_count", "_get_suggested_invoice_edi_format", "_find_accounting_partner", "_commercial_fields", "action_view_partner_invoices", "_has_invoice", "_can_edit_country", "can_edit_vat", "write", "create", "_unlink_if_partner_in_account_move", "_increase_rank", "_get_frontend_writable_fields", "_check_vat", "_run_vat_checks", "_get_vat_required_valid", "get_partner_localisation_fields_required_to_invoice", "_retrieve_partner_with_vat", "_retrieve_partner_with_phone_email", "_retrieve_partner_with_name", "_retrieve_partner", "_merge_method", "_deduce_country_code", "_compute_partner_vat_placeholder", "_compute_partner_company_registry_placeholder", "_compute_account_move_count", "action_open_business_doc", "_clear_removed_edi_formats"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_fiscal_country_codes" => self._compute_fiscal_country_codes(env, ctx, rs, args).await,
            "_compute_fiscal_country_group_codes" => self._compute_fiscal_country_group_codes(env, ctx, rs, args).await,
            "_order" => self._order(env, ctx, rs, args).await,
            "_credit_debit_get" => self._credit_debit_get(env, ctx, rs, args).await,
            "_compute_credit_to_invoice" => self._compute_credit_to_invoice(env, ctx, rs, args).await,
            "_asset_difference_search" => self._asset_difference_search(env, ctx, rs, args).await,
            "_credit_search" => self._credit_search(env, ctx, rs, args).await,
            "_debit_search" => self._debit_search(env, ctx, rs, args).await,
            "_invoice_total" => self._invoice_total(env, ctx, rs, args).await,
            "_compute_days_sales_outstanding" => self._compute_days_sales_outstanding(env, ctx, rs, args).await,
            "_compute_available_invoice_template_pdf_report_ids" => self._compute_available_invoice_template_pdf_report_ids(env, ctx, rs, args).await,
            "_get_company_currency" => self._get_company_currency(env, ctx, rs, args).await,
            "_default_display_invoice_template_pdf_report_id" => self._default_display_invoice_template_pdf_report_id(env, ctx, rs, args).await,
            "_compute_bank_count" => self._compute_bank_count(env, ctx, rs, args).await,
            "_compute_supplier_invoice_count" => self._compute_supplier_invoice_count(env, ctx, rs, args).await,
            "_compute_invoice_edi_format" => self._compute_invoice_edi_format(env, ctx, rs, args).await,
            "_inverse_invoice_edi_format" => self._inverse_invoice_edi_format(env, ctx, rs, args).await,
            "_compute_use_partner_credit_limit" => self._compute_use_partner_credit_limit(env, ctx, rs, args).await,
            "_inverse_use_partner_credit_limit" => self._inverse_use_partner_credit_limit(env, ctx, rs, args).await,
            "_compute_show_credit_limit" => self._compute_show_credit_limit(env, ctx, rs, args).await,
            "_compute_application_statistics_hook" => self._compute_application_statistics_hook(env, ctx, rs, args).await,
            "_get_account_statistics_count" => self._get_account_statistics_count(env, ctx, rs, args).await,
            "_get_suggested_invoice_edi_format" => self._get_suggested_invoice_edi_format(env, ctx, rs, args).await,
            "_find_accounting_partner" => self._find_accounting_partner(env, ctx, rs, args).await,
            "_commercial_fields" => self._commercial_fields(env, ctx, rs, args).await,
            "action_view_partner_invoices" => self.action_view_partner_invoices(env, ctx, rs, args).await,
            "_has_invoice" => self._has_invoice(env, ctx, rs, args).await,
            "_can_edit_country" => self._can_edit_country(env, ctx, rs, args).await,
            "can_edit_vat" => self.can_edit_vat(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "_unlink_if_partner_in_account_move" => self._unlink_if_partner_in_account_move(env, ctx, rs, args).await,
            "_increase_rank" => self._increase_rank(env, ctx, rs, args).await,
            "_get_frontend_writable_fields" => self._get_frontend_writable_fields(env, ctx, rs, args).await,
            "_check_vat" => self._check_vat(env, ctx, rs, args).await,
            "_run_vat_checks" => self._run_vat_checks(env, ctx, rs, args).await,
            "_get_vat_required_valid" => self._get_vat_required_valid(env, ctx, rs, args).await,
            "get_partner_localisation_fields_required_to_invoice" => self.get_partner_localisation_fields_required_to_invoice(env, ctx, rs, args).await,
            "_retrieve_partner_with_vat" => self._retrieve_partner_with_vat(env, ctx, rs, args).await,
            "_retrieve_partner_with_phone_email" => self._retrieve_partner_with_phone_email(env, ctx, rs, args).await,
            "_retrieve_partner_with_name" => self._retrieve_partner_with_name(env, ctx, rs, args).await,
            "_retrieve_partner" => self._retrieve_partner(env, ctx, rs, args).await,
            "_merge_method" => self._merge_method(env, ctx, rs, args).await,
            "_deduce_country_code" => self._deduce_country_code(env, ctx, rs, args).await,
            "_compute_partner_vat_placeholder" => self._compute_partner_vat_placeholder(env, ctx, rs, args).await,
            "_compute_partner_company_registry_placeholder" => self._compute_partner_company_registry_placeholder(env, ctx, rs, args).await,
            "_compute_account_move_count" => self._compute_account_move_count(env, ctx, rs, args).await,
            "action_open_business_doc" => self.action_open_business_doc(env, ctx, rs, args).await,
            "_clear_removed_edi_formats" => self._clear_removed_edi_formats(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl ResPartnerExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:337`). Decoradores: api.depends('company_id', 'country_code'), api.depends_context('allowed_company_ids').
    async fn _compute_fiscal_country_codes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._compute_fiscal_country_codes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:347`). Decoradores: api.depends('company_id'), api.depends_context('allowed_company_ids').
    async fn _compute_fiscal_country_group_codes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._compute_fiscal_country_group_codes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:357`). Decoradores: property.
    async fn _order(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._order".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:366`). Decoradores: api.depends_context('company').
    async fn _credit_debit_get(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._credit_debit_get".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:411`). Decoradores: api.depends_context('company').
    async fn _compute_credit_to_invoice(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._compute_credit_to_invoice".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:415`).
    async fn _asset_difference_search(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._asset_difference_search".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:444`). Decoradores: api.model.
    async fn _credit_search(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._credit_search".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:448`). Decoradores: api.model.
    async fn _debit_search(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._debit_search".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:451`).
    async fn _invoice_total(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._invoice_total".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:473`). Decoradores: api.depends('credit').
    async fn _compute_days_sales_outstanding(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._compute_days_sales_outstanding".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:492`).
    async fn _compute_available_invoice_template_pdf_report_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._compute_available_invoice_template_pdf_report_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:496`).
    async fn _get_company_currency(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._get_company_currency".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:503`).
    async fn _default_display_invoice_template_pdf_report_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._default_display_invoice_template_pdf_report_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:624`).
    async fn _compute_bank_count(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._compute_bank_count".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:630`).
    async fn _compute_supplier_invoice_count(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._compute_supplier_invoice_count".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:653`). Decoradores: api.depends_context('company'), api.depends('country_code').
    async fn _compute_invoice_edi_format(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._compute_invoice_edi_format".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:660`).
    async fn _inverse_invoice_edi_format(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._inverse_invoice_edi_format".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:670`). Decoradores: api.depends_context('company').
    async fn _compute_use_partner_credit_limit(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._compute_use_partner_credit_limit".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:675`).
    async fn _inverse_use_partner_credit_limit(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._inverse_use_partner_credit_limit".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:682`). Decoradores: api.depends_context('company').
    async fn _compute_show_credit_limit(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._compute_show_credit_limit".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:685`).
    async fn _compute_application_statistics_hook(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._compute_application_statistics_hook".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:694`).
    async fn _get_account_statistics_count(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._get_account_statistics_count".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:697`).
    async fn _get_suggested_invoice_edi_format(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._get_suggested_invoice_edi_format".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:702`).
    async fn _find_accounting_partner(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._find_accounting_partner".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:707`). Decoradores: api.model.
    async fn _commercial_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._commercial_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:712`).
    async fn action_view_partner_invoices(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.action_view_partner_invoices".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:723`).
    async fn _has_invoice(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._has_invoice".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:737`).
    async fn _can_edit_country(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._can_edit_country".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:743`).
    async fn can_edit_vat(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.can_edit_vat".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:749`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:770`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:783`). Decoradores: api.ondelete().
    async fn _unlink_if_partner_in_account_move(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._unlink_if_partner_in_account_move".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:796`).
    async fn _increase_rank(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._increase_rank".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:831`).
    async fn _get_frontend_writable_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._get_frontend_writable_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:837`).
    async fn _check_vat(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._check_vat".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:845`). Decoradores: api.model.
    async fn _run_vat_checks(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._run_vat_checks".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:863`).
    async fn _get_vat_required_valid(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._get_vat_required_valid".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:871`). Decoradores: api.model.
    async fn get_partner_localisation_fields_required_to_invoice(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.get_partner_localisation_fields_required_to_invoice".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:887`). Decoradores: api.model.
    async fn _retrieve_partner_with_vat(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._retrieve_partner_with_vat".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:938`). Decoradores: api.model.
    async fn _retrieve_partner_with_phone_email(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._retrieve_partner_with_phone_email".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:954`). Decoradores: api.model.
    async fn _retrieve_partner_with_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._retrieve_partner_with_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:959`).
    async fn _retrieve_partner(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._retrieve_partner".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:998`).
    async fn _merge_method(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._merge_method".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:1006`).
    async fn _deduce_country_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._deduce_country_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:1018`). Decoradores: api.depends('country_id').
    async fn _compute_partner_vat_placeholder(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._compute_partner_vat_placeholder".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:1029`). Decoradores: api.depends('country_id').
    async fn _compute_partner_company_registry_placeholder(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._compute_partner_company_registry_placeholder".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:1037`).
    async fn _compute_account_move_count(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._compute_account_move_count".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:1059`).
    async fn action_open_business_doc(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.action_open_business_doc".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:1063`). Decoradores: api.model.
    async fn _clear_removed_edi_formats(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._clear_removed_edi_formats".into(),
        ))
    }

}
