//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_code_mapping.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.code.mapping`

use nexus_orm::prelude::*;

pub struct AccountCodeMappingFragment;

#[async_trait]
impl ModelFragment for AccountCodeMappingFragment {
    fn model_name(&self) -> &str {
        "account.code.mapping"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Mapping of account codes per company".into();
        def.add_field(FieldDef::many2one("account_id", "account.account").string("Account").computed("_compute_account_id", &[]).stored());
        def.add_field(FieldDef::many2one("company_id", "res.company").string("Company").computed("_compute_company_id", &[]).stored());
        def.add_field(FieldDef::char("code").string("Code").computed("_compute_code", &["account_id.code"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["create", "_search", "_compute_account_id", "_compute_company_id", "_compute_code", "_inverse_code"]
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
            "_search" => self._search(env, ctx, rs, args).await,
            "_compute_account_id" => self._compute_account_id(env, ctx, rs, args).await,
            "_compute_company_id" => self._compute_company_id(env, ctx, rs, args).await,
            "_compute_code" => self._compute_code(env, ctx, rs, args).await,
            "_inverse_code" => self._inverse_code(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountCodeMappingFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_code_mapping.py:40`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.code.mapping.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_code_mapping.py:49`).
    async fn _search(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.code.mapping._search".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_code_mapping.py:70`).
    async fn _compute_account_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.code.mapping._compute_account_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_code_mapping.py:74`).
    async fn _compute_company_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.code.mapping._compute_company_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_code_mapping.py:79`). Decoradores: api.depends('account_id.code').
    async fn _compute_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.code.mapping._compute_code".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_code_mapping.py:84`).
    async fn _inverse_code(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.code.mapping._inverse_code".into(),
        ))
    }

}
