//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_reconcile_model.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.reconcile.model`

use nexus_orm::prelude::*;

pub struct AccountReconcileModelFragment;

#[async_trait]
impl ModelFragment for AccountReconcileModelFragment {
    fn model_name(&self) -> &str {
        "account.reconcile.model"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Preset to create journal entries during a invoices and payments matching".into();
        def.order = "sequence, id".into();
        def.add_field(FieldDef::boolean("active").default_val(true));
        def.add_field(FieldDef::char("name").string("Name").required());
        def.add_field(FieldDef::integer("sequence").required().default_val(10i64));
        def.add_field(FieldDef::many2one("company_id", "res.company").string("Company").required().readonly());
        def.add_field(FieldDef::selection("trigger", &[("manual", "Manual"), ("auto_reconcile", "Automated")]).required().default_val("manual"));
        def.add_field(FieldDef::many2one("next_activity_type_id", "mail.activity.type").string("Next Activity"));
        def.add_field(FieldDef::boolean("can_be_proposed").computed("_compute_can_be_proposed", &["mapped_partner_id", "match_label", "match_amount", "match_partner_ids", "trigger"]).stored());
        def.add_field(FieldDef::many2one("mapped_partner_id", "res.partner").computed("_compute_partner_mapping", &["match_label", "line_ids.partner_id", "line_ids.account_id"]).stored());
        def.add_field(FieldDef::many2many("match_journal_ids", "account.journal").string("Journals"));
        def.add_field(FieldDef::selection("match_amount", &[("lower", "Is lower than or equal to"), ("greater", "Is greater than or equal to"), ("between", "Is between")]).string("Amount"));
        def.add_field(FieldDef::float("match_amount_min").string("Amount Min Parameter"));
        def.add_field(FieldDef::float("match_amount_max").string("Amount Max Parameter"));
        def.add_field(FieldDef::selection("match_label", &[("contains", "Contains"), ("not_contains", "Not Contains"), ("match_regex", "Match Regex")]).string("Label"));
        def.add_field(FieldDef::char("match_label_param").string("Label Parameter"));
        def.add_field(FieldDef::many2many("match_partner_ids", "res.partner").string("Partners"));
        def.add_field(FieldDef::one2many("line_ids", "account.reconcile.model.line", "model_id"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_check_match_label_param", "_compute_can_be_proposed", "_compute_partner_mapping", "action_set_manual", "action_set_auto_reconcile", "action_reconcile_stat", "copy_data"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_check_match_label_param" => self._check_match_label_param(env, ctx, rs, args).await,
            "_compute_can_be_proposed" => self._compute_can_be_proposed(env, ctx, rs, args).await,
            "_compute_partner_mapping" => self._compute_partner_mapping(env, ctx, rs, args).await,
            "action_set_manual" => self.action_set_manual(env, ctx, rs, args).await,
            "action_set_auto_reconcile" => self.action_set_auto_reconcile(env, ctx, rs, args).await,
            "action_reconcile_stat" => self.action_reconcile_stat(env, ctx, rs, args).await,
            "copy_data" => self.copy_data(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountReconcileModelFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_reconcile_model.py:150`). Decoradores: api.constrains('match_label', 'match_label_param').
    async fn _check_match_label_param(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.reconcile.model._check_match_label_param".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_reconcile_model.py:159`). Decoradores: api.depends('mapped_partner_id', 'match_label', 'match_amount', 'match_partner_ids', 'trigger').
    async fn _compute_can_be_proposed(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.reconcile.model._compute_can_be_proposed".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_reconcile_model.py:164`). Decoradores: api.depends('match_label', 'line_ids.partner_id', 'line_ids.account_id').
    async fn _compute_partner_mapping(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.reconcile.model._compute_partner_mapping".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_reconcile_model.py:169`).
    async fn action_set_manual(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.reconcile.model.action_set_manual".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_reconcile_model.py:172`).
    async fn action_set_auto_reconcile(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.reconcile.model.action_set_auto_reconcile".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_reconcile_model.py:175`).
    async fn action_reconcile_stat(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.reconcile.model.action_reconcile_stat".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_reconcile_model.py:190`).
    async fn copy_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.reconcile.model.copy_data".into(),
        ))
    }

}
