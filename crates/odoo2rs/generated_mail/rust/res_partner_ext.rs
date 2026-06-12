//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/res_partner.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `res.partner` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct ResPartnerExtFragment;

#[async_trait]
impl ModelFragment for ResPartnerExtFragment {
    fn model_name(&self) -> &str {
        "res.partner"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::many2many("channel_ids", "discuss.channel").string("Channels"));
        def.add_field(FieldDef::one2many("channel_member_ids", "discuss.channel.member", "partner_id"));
        def.add_field(FieldDef::boolean("is_in_call").computed("_compute_is_in_call", &["rtc_session_ids"]).stored());
        def.add_field(FieldDef::one2many("rtc_session_ids", "discuss.channel.rtc.session", "partner_id"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_is_in_call", "search_for_channel_invite", "_search_for_channel_invite", "_search_for_channel_invite_to_store", "get_mention_suggestions_from_channel"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_is_in_call" => self._compute_is_in_call(env, ctx, rs, args).await,
            "search_for_channel_invite" => self.search_for_channel_invite(env, ctx, rs, args).await,
            "_search_for_channel_invite" => self._search_for_channel_invite(env, ctx, rs, args).await,
            "_search_for_channel_invite_to_store" => self._search_for_channel_invite_to_store(env, ctx, rs, args).await,
            "get_mention_suggestions_from_channel" => self.get_mention_suggestions_from_channel(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl ResPartnerExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/res_partner.py:26`). Decoradores: api.depends('rtc_session_ids').
    async fn _compute_is_in_call(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._compute_is_in_call".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/res_partner.py:32`). Decoradores: api.readonly, api.model.
    async fn search_for_channel_invite(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.search_for_channel_invite".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/res_partner.py:81`). Decoradores: api.readonly, api.model.
    async fn _search_for_channel_invite(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._search_for_channel_invite".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/res_partner.py:108`).
    async fn _search_for_channel_invite_to_store(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._search_for_channel_invite_to_store".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/res_partner.py:113`). Decoradores: api.readonly, api.model.
    async fn get_mention_suggestions_from_channel(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.get_mention_suggestions_from_channel".into(),
        ))
    }

}
