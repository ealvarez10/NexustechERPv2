//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_lock_exception.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.lock_exception`

use nexus_orm::prelude::*;

pub struct AccountLockExceptionFragment;

#[async_trait]
impl ModelFragment for AccountLockExceptionFragment {
    fn model_name(&self) -> &str {
        "account.lock_exception"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Account Lock Exception".into();
        def.add_field(FieldDef::boolean("active").string("Active").default_val(true));
        def.add_field(FieldDef::selection("state", &[("active", "Active"), ("revoked", "Revoked"), ("expired", "Expired")]).string("State").computed("_compute_state", &["active", "end_datetime"]).stored());
        def.add_field(FieldDef::many2one("company_id", "res.company").string("Company").required().readonly());
        def.add_field(FieldDef::many2one("user_id", "res.users").string("User"));
        def.add_field(FieldDef::char("reason").string("Reason"));
        def.add_field(FieldDef::datetime("end_datetime").string("End Date"));
        def.add_field(FieldDef::selection("lock_date_field", &[("fiscalyear_lock_date", "Global Lock Date"), ("tax_lock_date", "Tax Return Lock Date"), ("sale_lock_date", "Sales Lock Date"), ("purchase_lock_date", "Purchase Lock Date")]).string("Lock Date Field").required());
        def.add_field(FieldDef::date("lock_date").string("Changed Lock Date"));
        def.add_field(FieldDef::date("company_lock_date").string("Original Lock Date"));
        def.add_field(FieldDef::date("fiscalyear_lock_date").string("Global Lock Date").computed("_compute_lock_dates", &["lock_date_field", "lock_date"]).stored());
        def.add_field(FieldDef::date("tax_lock_date").string("Tax Return Lock Date").computed("_compute_lock_dates", &["lock_date_field", "lock_date"]).stored());
        def.add_field(FieldDef::date("sale_lock_date").string("Sales Lock Date").computed("_compute_lock_dates", &["lock_date_field", "lock_date"]).stored());
        def.add_field(FieldDef::date("purchase_lock_date").string("Purchase Lock Date").computed("_compute_lock_dates", &["lock_date_field", "lock_date"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_display_name", "_compute_state", "_compute_lock_dates", "_search_state", "_search_lock_date", "_search_fiscalyear_lock_date", "_search_tax_lock_date", "_search_sale_lock_date", "_search_purchase_lock_date", "_invalidate_affected_user_lock_dates", "create", "copy", "_recreate", "action_revoke", "_get_active_exceptions_domain", "_get_audit_trail_during_exception_domain", "action_show_audit_trail_during_exception"]
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
            "_compute_state" => self._compute_state(env, ctx, rs, args).await,
            "_compute_lock_dates" => self._compute_lock_dates(env, ctx, rs, args).await,
            "_search_state" => self._search_state(env, ctx, rs, args).await,
            "_search_lock_date" => self._search_lock_date(env, ctx, rs, args).await,
            "_search_fiscalyear_lock_date" => self._search_fiscalyear_lock_date(env, ctx, rs, args).await,
            "_search_tax_lock_date" => self._search_tax_lock_date(env, ctx, rs, args).await,
            "_search_sale_lock_date" => self._search_sale_lock_date(env, ctx, rs, args).await,
            "_search_purchase_lock_date" => self._search_purchase_lock_date(env, ctx, rs, args).await,
            "_invalidate_affected_user_lock_dates" => self._invalidate_affected_user_lock_dates(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "copy" => self.copy(env, ctx, rs, args).await,
            "_recreate" => self._recreate(env, ctx, rs, args).await,
            "action_revoke" => self.action_revoke(env, ctx, rs, args).await,
            "_get_active_exceptions_domain" => self._get_active_exceptions_domain(env, ctx, rs, args).await,
            "_get_audit_trail_during_exception_domain" => self._get_audit_trail_during_exception_domain(env, ctx, rs, args).await,
            "action_show_audit_trail_during_exception" => self.action_show_audit_trail_during_exception(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountLockExceptionFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_lock_exception.py:100`).
    async fn _compute_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.lock_exception._compute_display_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_lock_exception.py:105`). Decoradores: api.depends('active', 'end_datetime').
    async fn _compute_state(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.lock_exception._compute_state".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_lock_exception.py:115`). Decoradores: api.depends('lock_date_field', 'lock_date').
    async fn _compute_lock_dates(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.lock_exception._compute_lock_dates".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_lock_exception.py:123`).
    async fn _search_state(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.lock_exception._search_state".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_lock_exception.py:136`).
    async fn _search_lock_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.lock_exception._search_lock_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_lock_exception.py:146`).
    async fn _search_fiscalyear_lock_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.lock_exception._search_fiscalyear_lock_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_lock_exception.py:149`).
    async fn _search_tax_lock_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.lock_exception._search_tax_lock_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_lock_exception.py:152`).
    async fn _search_sale_lock_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.lock_exception._search_sale_lock_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_lock_exception.py:155`).
    async fn _search_purchase_lock_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.lock_exception._search_purchase_lock_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_lock_exception.py:158`).
    async fn _invalidate_affected_user_lock_dates(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.lock_exception._invalidate_affected_user_lock_dates".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_lock_exception.py:165`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.lock_exception.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_lock_exception.py:218`).
    async fn copy(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.lock_exception.copy".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_lock_exception.py:221`).
    async fn _recreate(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.lock_exception._recreate".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_lock_exception.py:234`).
    async fn action_revoke(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.lock_exception.action_revoke".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_lock_exception.py:246`). Decoradores: api.model.
    async fn _get_active_exceptions_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.lock_exception._get_active_exceptions_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_lock_exception.py:257`).
    async fn _get_audit_trail_during_exception_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.lock_exception._get_audit_trail_during_exception_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_lock_exception.py:298`).
    async fn action_show_audit_trail_during_exception(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.lock_exception.action_show_audit_trail_during_exception".into(),
        ))
    }

}
