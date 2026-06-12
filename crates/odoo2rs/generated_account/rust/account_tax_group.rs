//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.tax.group`

use nexus_orm::prelude::*;

pub struct AccountTaxGroupFragment;

#[async_trait]
impl ModelFragment for AccountTaxGroupFragment {
    fn model_name(&self) -> &str {
        "account.tax.group"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Tax Group".into();
        def.order = "sequence asc, id".into();
        def.add_field(FieldDef::char("name").required());
        def.add_field(FieldDef::integer("sequence").default_val(10i64));
        def.add_field(FieldDef::many2one("company_id", "res.company").required());
        def.add_field(FieldDef::many2one("tax_payable_account_id", "account.account").string("Tax Payable Account"));
        def.add_field(FieldDef::many2one("tax_receivable_account_id", "account.account").string("Tax Receivable Account"));
        def.add_field(FieldDef::many2one("advance_tax_payment_account_id", "account.account").string("Tax Advance Account"));
        def.add_field(FieldDef::many2one("country_id", "res.country").string("Country").computed("_compute_country_id", &["company_id"]).stored());
        def.add_field({ let mut f = FieldDef::char("country_code"); f.related = Some("country_id.code".into()); f });
        def.add_field(FieldDef::char("preceding_subtotal").string("Preceding Subtotal"));
        def.add_field(FieldDef::char("pos_receipt_label").string("PoS receipt label"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_country_id"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_country_id" => self._compute_country_id(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountTaxGroupFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_tax.py:66`). Decoradores: api.depends('company_id').
    async fn _compute_country_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.tax.group._compute_country_id".into(),
        ))
    }

}
