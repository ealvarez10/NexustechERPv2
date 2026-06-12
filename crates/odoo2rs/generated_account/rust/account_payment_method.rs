//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_method.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.payment.method`

use nexus_orm::prelude::*;

pub struct AccountPaymentMethodFragment;

#[async_trait]
impl ModelFragment for AccountPaymentMethodFragment {
    fn model_name(&self) -> &str {
        "account.payment.method"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Payment Methods".into();
        def.add_field(FieldDef::char("name").required());
        def.add_field(FieldDef::char("code").required());
        def.add_field(FieldDef::selection("payment_type", &[("inbound", "Inbound"), ("outbound", "Outbound")]).required());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["create", "_auto_link_payment_methods", "_get_payment_method_domain", "_get_payment_method_information", "_get_sdd_payment_method_code", "unlink"]
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
            "_auto_link_payment_methods" => self._auto_link_payment_methods(env, ctx, rs, args).await,
            "_get_payment_method_domain" => self._get_payment_method_domain(env, ctx, rs, args).await,
            "_get_payment_method_information" => self._get_payment_method_information(env, ctx, rs, args).await,
            "_get_sdd_payment_method_code" => self._get_sdd_payment_method_code(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountPaymentMethodFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_method.py:21`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.method.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_method.py:26`).
    async fn _auto_link_payment_methods(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.method._auto_link_payment_methods".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_method.py:42`). Decoradores: api.model.
    async fn _get_payment_method_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.method._get_payment_method_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_method.py:65`). Decoradores: api.model.
    async fn _get_payment_method_information(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.method._get_payment_method_information".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_method.py:83`). Decoradores: api.model.
    async fn _get_sdd_payment_method_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.method._get_sdd_payment_method_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment_method.py:90`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.payment.method.unlink".into(),
        ))
    }

}
