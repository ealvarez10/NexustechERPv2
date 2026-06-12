//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_method.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.payment.method.line`

use nexus_orm::prelude::*;

pub struct AccountPaymentMethodLineFragment;

#[async_trait]
impl ModelFragment for AccountPaymentMethodLineFragment {
    fn model_name(&self) -> &str {
        "account.payment.method.line"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Payment Methods".into();
        def.order = "sequence, id".into();
        def.add_field(FieldDef::char("name").computed("_compute_name", &["payment_method_id.name"]).stored());
        def.add_field(FieldDef::integer("sequence").default_val(10i64));
        def.add_field(FieldDef::many2one("payment_method_id", "account.payment.method").string("Payment Method").required());
        def.add_field(FieldDef::many2one("payment_account_id", "account.account"));
        def.add_field(FieldDef::many2one("journal_id", "account.journal"));
        // TODO(odoo2rs): campo 'default_account_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field({ let mut f = FieldDef::char("code"); f.related = Some("payment_method_id.code".into()); f });
        def.add_field({ let mut f = FieldDef::selection("payment_type", &[]); f.related = Some("payment_method_id.payment_type".into()); f });
        // TODO(odoo2rs): campo 'company_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        // TODO(odoo2rs): campo 'available_payment_method_ids' (many2many) no generable — falta comodel/inverse o tipo sin equivalente.
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_display_name", "_compute_name", "_ensure_unique_name_for_journal", "unlink", "_auto_toggle_account_to_reconcile"]
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
            "_compute_name" => self._compute_name(env, ctx, rs, args).await,
            "_ensure_unique_name_for_journal" => self._ensure_unique_name_for_journal(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "_auto_toggle_account_to_reconcile" => self._auto_toggle_account_to_reconcile(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountPaymentMethodLineFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_method.py:133`). Decoradores: api.depends('journal_id'), api.depends_context('hide_payment_journal_id').
    async fn _compute_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.method.line._compute_display_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_method.py:140`). Decoradores: api.depends('payment_method_id.name').
    async fn _compute_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.method.line._compute_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_method.py:146`). Decoradores: api.constrains('name').
    async fn _ensure_unique_name_for_journal(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.method.line._ensure_unique_name_for_journal".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_method.py:149`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.method.line.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_method.py:165`). Decoradores: api.model.
    async fn _auto_toggle_account_to_reconcile(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.method.line._auto_toggle_account_to_reconcile".into(),
        ))
    }

}
