//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_line.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.analytic.line` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct AccountAnalyticLineExtFragment;

#[async_trait]
impl ModelFragment for AccountAnalyticLineExtFragment {
    fn model_name(&self) -> &str {
        "account.analytic.line"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Analytic Line".into();
        def.add_field(FieldDef::many2one("product_id", "product.product").string("Product"));
        def.add_field(FieldDef::many2one("general_account_id", "account.account").string("Financial Account").computed("_compute_general_account_id", &["move_line_id"]).stored());
        def.add_field({ let mut f = FieldDef::many2one("journal_id", "account.journal").string("Financial Journal").readonly(); f.related = Some("move_line_id.journal_id".into()); f });
        // TODO(odoo2rs): campo 'partner_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::many2one("move_line_id", "account.move.line").string("Journal Item"));
        def.add_field(FieldDef::char("code"));
        def.add_field(FieldDef::char("ref").string("Ref."));
        def.add_field(FieldDef::selection("category", &[]));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_general_account_id", "_check_general_account_id", "_compute_partner_id", "on_change_unit_amount", "view_header_get", "create", "write", "unlink"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_general_account_id" => self._compute_general_account_id(env, ctx, rs, args).await,
            "_check_general_account_id" => self._check_general_account_id(env, ctx, rs, args).await,
            "_compute_partner_id" => self._compute_partner_id(env, ctx, rs, args).await,
            "on_change_unit_amount" => self.on_change_unit_amount(env, ctx, rs, args).await,
            "view_header_get" => self.view_header_get(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountAnalyticLineExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_line.py:48`). Decoradores: api.depends('move_line_id').
    async fn _compute_general_account_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.analytic.line._compute_general_account_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_line.py:53`). Decoradores: api.constrains('move_line_id', 'general_account_id').
    async fn _check_general_account_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.analytic.line._check_general_account_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_line.py:59`). Decoradores: api.depends('move_line_id.partner_id').
    async fn _compute_partner_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.analytic.line._compute_partner_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_line.py:64`). Decoradores: api.onchange('product_id', 'product_uom_id', 'unit_amount', 'currency_id').
    async fn on_change_unit_amount(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.analytic.line.on_change_unit_amount".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_line.py:83`). Decoradores: api.model.
    async fn view_header_get(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.analytic.line.view_header_get".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_line.py:92`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.analytic.line.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_line.py:97`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.analytic.line.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_analytic_line.py:106`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.analytic.line.unlink".into(),
        ))
    }

}
