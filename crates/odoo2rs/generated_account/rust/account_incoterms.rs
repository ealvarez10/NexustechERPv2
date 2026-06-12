//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_incoterms.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.incoterms`

use nexus_orm::prelude::*;

pub struct AccountIncotermsFragment;

#[async_trait]
impl ModelFragment for AccountIncotermsFragment {
    fn model_name(&self) -> &str {
        "account.incoterms"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Incoterms".into();
        def.add_field(FieldDef::char("name").string("Name").required());
        def.add_field(FieldDef::char("code").string("Code").required());
        def.add_field(FieldDef::boolean("active").string("Active").default_val(true));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_display_name"]
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
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountIncotermsFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_incoterms.py:22`). Decoradores: api.depends('code').
    async fn _compute_display_name(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.incoterms._compute_display_name".into(),
        ))
    }

}
