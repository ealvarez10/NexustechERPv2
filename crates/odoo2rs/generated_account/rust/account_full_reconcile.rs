//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_full_reconcile.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.full.reconcile`

use nexus_orm::prelude::*;

pub struct AccountFullReconcileFragment;

#[async_trait]
impl ModelFragment for AccountFullReconcileFragment {
    fn model_name(&self) -> &str {
        "account.full.reconcile"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Full Reconcile".into();
        def.add_field(FieldDef::one2many("partial_reconcile_ids", "account.partial.reconcile", "full_reconcile_id").string("Reconciliation Parts"));
        def.add_field(FieldDef::one2many("reconciled_line_ids", "account.move.line", "full_reconcile_id").string("Matched Journal Items"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["create"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "create" => self.create(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountFullReconcileFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_full_reconcile.py:13`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.full.reconcile.create".into(),
        ))
    }

}
