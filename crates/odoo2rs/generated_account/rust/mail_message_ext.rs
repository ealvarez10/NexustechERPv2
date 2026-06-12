//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_message.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.message` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct MailMessageExtFragment;

#[async_trait]
impl ModelFragment for MailMessageExtFragment {
    fn model_name(&self) -> &str {
        "mail.message"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::text("account_audit_log_preview").string("Description").computed("_compute_account_audit_log_preview", &["tracking_value_ids"]).stored());
        def.add_field(FieldDef::many2one("account_audit_log_move_id", "account.move").string("Journal Entry").computed("_compute_account_audit_log_move_id", &[]).stored());
        def.add_field(FieldDef::many2one("account_audit_log_partner_id", "res.partner").string("Partner").computed("_compute_account_audit_log_partner_id", &[]).stored());
        def.add_field(FieldDef::many2one("account_audit_log_account_id", "account.account").string("Account").computed("_compute_account_audit_log_account_id", &[]).stored());
        def.add_field(FieldDef::many2one("account_audit_log_tax_id", "account.tax").string("Tax").computed("_compute_account_audit_log_tax_id", &[]).stored());
        def.add_field(FieldDef::many2one("account_audit_log_company_id", "res.company").string("Company ").computed("_compute_account_audit_log_company_id", &[]).stored());
        def.add_field(FieldDef::boolean("account_audit_log_restricted").string("Protected by restricted Audit Logs").computed("_compute_account_audit_log_restricted", &[]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_account_audit_log_preview", "_search_account_audit_log_preview", "_compute_account_audit_log_move_id", "_search_account_audit_log_move_id", "_compute_account_audit_log_account_id", "_search_account_audit_log_account_id", "_compute_account_audit_log_tax_id", "_search_account_audit_log_tax_id", "_compute_account_audit_log_company_id", "_search_account_audit_log_company_id", "_compute_account_audit_log_partner_id", "_search_account_audit_log_partner_id", "_compute_account_audit_log_restricted", "_search_account_audit_log_restricted", "_compute_audit_log_related_record_id", "_search_audit_log_related_record_id", "_except_audit_log", "write"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_account_audit_log_preview" => self._compute_account_audit_log_preview(env, ctx, rs, args).await,
            "_search_account_audit_log_preview" => self._search_account_audit_log_preview(env, ctx, rs, args).await,
            "_compute_account_audit_log_move_id" => self._compute_account_audit_log_move_id(env, ctx, rs, args).await,
            "_search_account_audit_log_move_id" => self._search_account_audit_log_move_id(env, ctx, rs, args).await,
            "_compute_account_audit_log_account_id" => self._compute_account_audit_log_account_id(env, ctx, rs, args).await,
            "_search_account_audit_log_account_id" => self._search_account_audit_log_account_id(env, ctx, rs, args).await,
            "_compute_account_audit_log_tax_id" => self._compute_account_audit_log_tax_id(env, ctx, rs, args).await,
            "_search_account_audit_log_tax_id" => self._search_account_audit_log_tax_id(env, ctx, rs, args).await,
            "_compute_account_audit_log_company_id" => self._compute_account_audit_log_company_id(env, ctx, rs, args).await,
            "_search_account_audit_log_company_id" => self._search_account_audit_log_company_id(env, ctx, rs, args).await,
            "_compute_account_audit_log_partner_id" => self._compute_account_audit_log_partner_id(env, ctx, rs, args).await,
            "_search_account_audit_log_partner_id" => self._search_account_audit_log_partner_id(env, ctx, rs, args).await,
            "_compute_account_audit_log_restricted" => self._compute_account_audit_log_restricted(env, ctx, rs, args).await,
            "_search_account_audit_log_restricted" => self._search_account_audit_log_restricted(env, ctx, rs, args).await,
            "_compute_audit_log_related_record_id" => self._compute_audit_log_related_record_id(env, ctx, rs, args).await,
            "_search_audit_log_related_record_id" => self._search_audit_log_related_record_id(env, ctx, rs, args).await,
            "_except_audit_log" => self._except_audit_log(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailMessageExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_message.py:78`). Decoradores: api.depends('tracking_value_ids').
    async fn _compute_account_audit_log_preview(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._compute_account_audit_log_preview".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_message.py:99`).
    async fn _search_account_audit_log_preview(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._search_account_audit_log_preview".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_message.py:110`).
    async fn _compute_account_audit_log_move_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._compute_account_audit_log_move_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_message.py:113`).
    async fn _search_account_audit_log_move_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._search_account_audit_log_move_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_message.py:116`).
    async fn _compute_account_audit_log_account_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._compute_account_audit_log_account_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_message.py:119`).
    async fn _search_account_audit_log_account_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._search_account_audit_log_account_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_message.py:122`).
    async fn _compute_account_audit_log_tax_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._compute_account_audit_log_tax_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_message.py:125`).
    async fn _search_account_audit_log_tax_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._search_account_audit_log_tax_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_message.py:128`).
    async fn _compute_account_audit_log_company_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._compute_account_audit_log_company_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_message.py:131`).
    async fn _search_account_audit_log_company_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._search_account_audit_log_company_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_message.py:134`).
    async fn _compute_account_audit_log_partner_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._compute_account_audit_log_partner_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_message.py:137`).
    async fn _search_account_audit_log_partner_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._search_account_audit_log_partner_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_message.py:140`).
    async fn _compute_account_audit_log_restricted(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._compute_account_audit_log_restricted".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_message.py:146`).
    async fn _search_account_audit_log_restricted(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._search_account_audit_log_restricted".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_message.py:155`).
    async fn _compute_audit_log_related_record_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._compute_audit_log_related_record_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_message.py:161`).
    async fn _search_audit_log_related_record_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._search_audit_log_related_record_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_message.py:181`). Decoradores: api.ondelete().
    async fn _except_audit_log(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message._except_audit_log".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/mail_message.py:190`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.message.write".into(),
        ))
    }

}
