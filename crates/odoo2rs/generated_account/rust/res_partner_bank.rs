//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `res.partner.bank`

use nexus_orm::prelude::*;

pub struct ResPartnerBankFragment;

#[async_trait]
impl ModelFragment for ResPartnerBankFragment {
    fn model_name(&self) -> &str {
        "res.partner.bank"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::one2many("journal_id", "account.journal", "bank_account_id").string("Account Journal").readonly());
        def.add_field(FieldDef::boolean("has_iban_warning").computed("_compute_display_account_warning", &["partner_id.country_id", "sanitized_acc_number", "allow_out_payment", "acc_type"]).stored());
        def.add_field({ let mut f = FieldDef::char("partner_country_name"); f.related = Some("partner_id.country_id.name".into()); f });
        def.add_field(FieldDef::boolean("has_money_transfer_warning").computed("_compute_display_account_warning", &["partner_id.country_id", "sanitized_acc_number", "allow_out_payment", "acc_type"]).stored());
        def.add_field(FieldDef::char("money_transfer_service").computed("_compute_money_transfer_service_name", &["sanitized_acc_number", "allow_out_payment"]).stored());
        def.add_field({ let mut f = FieldDef::integer("partner_supplier_rank"); f.related = Some("partner_id.supplier_rank".into()); f });
        def.add_field({ let mut f = FieldDef::integer("partner_customer_rank"); f.related = Some("partner_id.customer_rank".into()); f });
        def.add_field(FieldDef::one2many("related_moves", "account.move", "partner_bank_id"));
        // TODO(odoo2rs): campo 'bank_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::boolean("active"));
        def.add_field(FieldDef::char("acc_number"));
        def.add_field(FieldDef::char("acc_holder_name"));
        def.add_field(FieldDef::char("clearing_number"));
        // TODO(odoo2rs): campo 'partner_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::boolean("user_has_group_validate_bank_account").computed("_compute_user_has_group_validate_bank_account", &["acc_number"]).stored());
        def.add_field(FieldDef::boolean("allow_out_payment"));
        // TODO(odoo2rs): campo 'currency_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::boolean("lock_trust_fields").computed("_compute_lock_trust_fields", &["allow_out_payment"]).stored());
        def.add_field(FieldDef::many2many("duplicate_bank_partner_ids", "res.partner").computed("_compute_duplicate_bank_partner_ids", &["acc_number"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_check_journal_id", "_check_allow_out_payment", "_compute_duplicate_bank_partner_ids", "_compute_display_account_warning", "_compute_money_transfer_service_name", "_get_money_transfer_services", "_compute_user_has_group_validate_bank_account", "_compute_lock_trust_fields", "_build_qr_code_vals", "build_qr_code_url", "build_qr_code_base64", "_get_qr_vals", "_get_qr_code_generation_params", "_get_qr_code_url", "_get_qr_code_base64", "_get_available_qr_methods", "get_available_qr_methods_in_sequence", "_get_error_messages_for_qr", "_check_for_qr_code_errors", "_user_can_trust", "create", "write", "unlink", "default_get", "_compute_display_name"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_check_journal_id" => self._check_journal_id(env, ctx, rs, args).await,
            "_check_allow_out_payment" => self._check_allow_out_payment(env, ctx, rs, args).await,
            "_compute_duplicate_bank_partner_ids" => self._compute_duplicate_bank_partner_ids(env, ctx, rs, args).await,
            "_compute_display_account_warning" => self._compute_display_account_warning(env, ctx, rs, args).await,
            "_compute_money_transfer_service_name" => self._compute_money_transfer_service_name(env, ctx, rs, args).await,
            "_get_money_transfer_services" => self._get_money_transfer_services(env, ctx, rs, args).await,
            "_compute_user_has_group_validate_bank_account" => self._compute_user_has_group_validate_bank_account(env, ctx, rs, args).await,
            "_compute_lock_trust_fields" => self._compute_lock_trust_fields(env, ctx, rs, args).await,
            "_build_qr_code_vals" => self._build_qr_code_vals(env, ctx, rs, args).await,
            "build_qr_code_url" => self.build_qr_code_url(env, ctx, rs, args).await,
            "build_qr_code_base64" => self.build_qr_code_base64(env, ctx, rs, args).await,
            "_get_qr_vals" => self._get_qr_vals(env, ctx, rs, args).await,
            "_get_qr_code_generation_params" => self._get_qr_code_generation_params(env, ctx, rs, args).await,
            "_get_qr_code_url" => self._get_qr_code_url(env, ctx, rs, args).await,
            "_get_qr_code_base64" => self._get_qr_code_base64(env, ctx, rs, args).await,
            "_get_available_qr_methods" => self._get_available_qr_methods(env, ctx, rs, args).await,
            "get_available_qr_methods_in_sequence" => self.get_available_qr_methods_in_sequence(env, ctx, rs, args).await,
            "_get_error_messages_for_qr" => self._get_error_messages_for_qr(env, ctx, rs, args).await,
            "_check_for_qr_code_errors" => self._check_for_qr_code_errors(env, ctx, rs, args).await,
            "_user_can_trust" => self._user_can_trust(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "default_get" => self.default_get(env, ctx, rs, args).await,
            "_compute_display_name" => self._compute_display_name(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl ResPartnerBankFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:56`). Decoradores: api.constrains('journal_id').
    async fn _check_journal_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank._check_journal_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:61`).
    async fn _check_allow_out_payment(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank._check_allow_out_payment".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:68`). Decoradores: api.depends('acc_number').
    async fn _compute_duplicate_bank_partner_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank._compute_duplicate_bank_partner_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:95`). Decoradores: api.depends('partner_id.country_id', 'sanitized_acc_number', 'allow_out_payment', 'acc_type').
    async fn _compute_display_account_warning(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank._compute_display_account_warning".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:108`). Decoradores: api.depends('sanitized_acc_number', 'allow_out_payment').
    async fn _compute_money_transfer_service_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank._compute_money_transfer_service_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:116`).
    async fn _get_money_transfer_services(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank._get_money_transfer_services".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:125`). Decoradores: api.depends('acc_number'), api.depends_context('uid').
    async fn _compute_user_has_group_validate_bank_account(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank._compute_user_has_group_validate_bank_account".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:130`). Decoradores: api.depends('allow_out_payment').
    async fn _compute_lock_trust_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank._compute_lock_trust_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:137`).
    async fn _build_qr_code_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank._build_qr_code_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:178`).
    async fn build_qr_code_url(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank.build_qr_code_url".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:184`).
    async fn build_qr_code_base64(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank.build_qr_code_base64".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:190`).
    async fn _get_qr_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank._get_qr_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:193`).
    async fn _get_qr_code_generation_params(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank._get_qr_code_generation_params".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:196`).
    async fn _get_qr_code_url(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank._get_qr_code_url".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:212`).
    async fn _get_qr_code_base64(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank._get_qr_code_base64".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:234`). Decoradores: api.model.
    async fn _get_available_qr_methods(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank._get_available_qr_methods".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:245`). Decoradores: api.model.
    async fn get_available_qr_methods_in_sequence(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank.get_available_qr_methods_in_sequence".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:253`).
    async fn _get_error_messages_for_qr(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank._get_error_messages_for_qr".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:264`).
    async fn _check_for_qr_code_errors(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank._check_for_qr_code_errors".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:273`).
    async fn _user_can_trust(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank._user_can_trust".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:286`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:306`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:365`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:373`). Decoradores: api.model.
    async fn default_get(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank.default_get".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_partner_bank.py:385`). Decoradores: api.depends('allow_out_payment', 'acc_number', 'bank_id'), api.depends_context('display_account_trust').
    async fn _compute_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.bank._compute_display_name".into(),
        ))
    }

}
