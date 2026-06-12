//! Generado por odoo2rs desde `tests/fixtures/sale_demo.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `demo.order`

use nexus_orm::prelude::*;

pub struct DemoOrderFragment;

#[async_trait]
impl ModelFragment for DemoOrderFragment {
    fn model_name(&self) -> &str {
        "demo.order"
    }

    fn module(&self) -> &str {
        "demo"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Orden demo".into();
        def.order = "id desc".into();
        def.add_field(FieldDef::char("name").string("Referencia").required().default_val("Nuevo"));
        def.add_field(FieldDef::many2one("partner_id", "res.partner").string("Cliente").required());
        def.add_field(FieldDef::selection("state", &[("draft", "Borrador"), ("done", "Hecha")]).string("Estado").default_val("draft"));
        def.add_field(FieldDef::one2many("line_ids", "demo.order.line", "order_id").string("Líneas"));
        def.add_field(FieldDef::monetary("amount_total").string("Total").computed("_compute_total", &["line_ids.subtotal"]).stored());
        def.add_field({ let mut f = FieldDef::char("partner_vat").string("RFC"); f.related = Some("partner_id.vat".into()); f });
        def.add_field({ let mut f = FieldDef::text("draft_note"); f.store = false; f });
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_total", "action_done"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_total" => self._compute_total(env, ctx, rs, args).await,
            "action_done" => self.action_done(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl DemoOrderFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`tests/fixtures/sale_demo.py:21`). Decoradores: api.depends('line_ids.subtotal').
    async fn _compute_total(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): demo.order._compute_total".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`tests/fixtures/sale_demo.py:25`).
    async fn action_done(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): demo.order.action_done".into(),
        ))
    }

}
