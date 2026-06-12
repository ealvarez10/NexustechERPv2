//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_call_history.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `discuss.call.history`

use nexus_orm::prelude::*;

pub struct DiscussCallHistoryFragment;

#[async_trait]
impl ModelFragment for DiscussCallHistoryFragment {
    fn model_name(&self) -> &str {
        "discuss.call.history"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Keep the call history".into();
        def.order = "start_dt DESC, id DESC".into();
        def.add_field(FieldDef::many2one("channel_id", "discuss.channel").required());
        def.add_field(FieldDef::float("duration_hour").computed("_compute_duration_hour", &["start_dt", "end_dt"]).stored());
        def.add_field(FieldDef::datetime("start_dt").required());
        def.add_field(FieldDef::datetime("end_dt"));
        def.add_field(FieldDef::many2one("start_call_message_id", "mail.message"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_duration_hour"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_duration_hour" => self._compute_duration_hour(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl DiscussCallHistoryFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_call_history.py:29`). Decoradores: api.depends('start_dt', 'end_dt').
    async fn _compute_duration_hour(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): discuss.call.history._compute_duration_hour".into(),
        ))
    }

}
