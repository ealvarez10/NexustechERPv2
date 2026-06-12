//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.group`

use nexus_orm::prelude::*;

pub struct AccountGroupFragment;

#[async_trait]
impl ModelFragment for AccountGroupFragment {
    fn model_name(&self) -> &str {
        "account.group"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Account Group".into();
        def.order = "code_prefix_start".into();
        def.add_field(FieldDef::many2one("parent_id", "account.group").readonly());
        def.add_field(FieldDef::char("name").required());
        def.add_field(FieldDef::char("code_prefix_start").computed("_compute_code_prefix_start", &["code_prefix_end"]).stored());
        def.add_field(FieldDef::char("code_prefix_end").computed("_compute_code_prefix_end", &["code_prefix_start"]).stored());
        def.add_field(FieldDef::many2one("company_id", "res.company").required().readonly());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_code_prefix_end", "_compute_code_prefix_start", "_compute_display_name", "_search_display_name", "_constraint_prefix_overlap", "_sanitize_vals", "_check_parent_not_circular", "create", "write", "unlink", "_adapt_parent_account_group"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_code_prefix_end" => self._compute_code_prefix_end(env, ctx, rs, args).await,
            "_compute_code_prefix_start" => self._compute_code_prefix_start(env, ctx, rs, args).await,
            "_compute_display_name" => self._compute_display_name(env, ctx, rs, args).await,
            "_search_display_name" => self._search_display_name(env, ctx, rs, args).await,
            "_constraint_prefix_overlap" => self._constraint_prefix_overlap(env, ctx, rs, args).await,
            "_sanitize_vals" => self._sanitize_vals(env, ctx, rs, args).await,
            "_check_parent_not_circular" => self._check_parent_not_circular(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "_adapt_parent_account_group" => self._adapt_parent_account_group(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountGroupFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1503`). Decoradores: api.depends('code_prefix_start').
    async fn _compute_code_prefix_end(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.group._compute_code_prefix_end".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1509`). Decoradores: api.depends('code_prefix_end').
    async fn _compute_code_prefix_start(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.group._compute_code_prefix_start".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1515`). Decoradores: api.depends('code_prefix_start', 'code_prefix_end').
    async fn _compute_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.group._compute_display_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1523`). Decoradores: api.model.
    async fn _search_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.group._search_display_name".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1537`). Decoradores: api.constrains('code_prefix_start', 'code_prefix_end').
    async fn _constraint_prefix_overlap(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.group._constraint_prefix_overlap".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1557`).
    async fn _sanitize_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.group._sanitize_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1565`). Decoradores: api.constrains('parent_id').
    async fn _check_parent_not_circular(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.group._check_parent_not_circular".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1570`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.group.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1575`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.group.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1581`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.group.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_account.py:1587`).
    async fn _adapt_parent_account_group(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.group._adapt_parent_account_group".into(),
        ))
    }

}
