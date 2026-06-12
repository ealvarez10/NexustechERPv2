//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_partner.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `res.partner`

use nexus_orm::prelude::*;

pub struct ResPartnerFragment;

#[async_trait]
impl ModelFragment for ResPartnerFragment {
    fn model_name(&self) -> &str {
        "res.partner"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::char("name"));
        def.add_field(FieldDef::char("email"));
        def.add_field(FieldDef::char("phone"));
        // TODO(odoo2rs): campo 'parent_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        // TODO(odoo2rs): campo 'user_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::char("vat"));
        def.add_field(FieldDef::char("contact_address_inline").string("Inlined Complete Address").computed("_compute_contact_address_inline", &["contact_address"]).stored());
        def.add_field(FieldDef::char("im_status").string("IM Status").computed("_compute_im_status", &["user_ids.manual_im_status", "user_ids.presence_ids.status"]).stored());
        def.add_field(FieldDef::datetime("offline_since").string("Offline since").computed("_compute_im_status", &["user_ids.manual_im_status", "user_ids.presence_ids.status"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_contact_address_inline", "_compute_im_status", "_get_needaction_count", "_mail_get_partners", "_get_view_cache_key", "find_or_create", "_find_or_create_from_emails", "_get_im_status_access_token", "_get_mention_token", "_get_store_mention_fields", "_get_store_avatar_card_fields", "_field_store_repr", "_to_store_defaults", "get_mention_suggestions", "_get_mention_suggestions_domain", "_search_mention_suggestions", "_get_current_persona"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_contact_address_inline" => self._compute_contact_address_inline(env, ctx, rs, args).await,
            "_compute_im_status" => self._compute_im_status(env, ctx, rs, args).await,
            "_get_needaction_count" => self._get_needaction_count(env, ctx, rs, args).await,
            "_mail_get_partners" => self._mail_get_partners(env, ctx, rs, args).await,
            "_get_view_cache_key" => self._get_view_cache_key(env, ctx, rs, args).await,
            "find_or_create" => self.find_or_create(env, ctx, rs, args).await,
            "_find_or_create_from_emails" => self._find_or_create_from_emails(env, ctx, rs, args).await,
            "_get_im_status_access_token" => self._get_im_status_access_token(env, ctx, rs, args).await,
            "_get_mention_token" => self._get_mention_token(env, ctx, rs, args).await,
            "_get_store_mention_fields" => self._get_store_mention_fields(env, ctx, rs, args).await,
            "_get_store_avatar_card_fields" => self._get_store_avatar_card_fields(env, ctx, rs, args).await,
            "_field_store_repr" => self._field_store_repr(env, ctx, rs, args).await,
            "_to_store_defaults" => self._to_store_defaults(env, ctx, rs, args).await,
            "get_mention_suggestions" => self.get_mention_suggestions(env, ctx, rs, args).await,
            "_get_mention_suggestions_domain" => self._get_mention_suggestions_domain(env, ctx, rs, args).await,
            "_search_mention_suggestions" => self._search_mention_suggestions(env, ctx, rs, args).await,
            "_get_current_persona" => self._get_current_persona(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl ResPartnerFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_partner.py:34`). Decoradores: api.depends('contact_address').
    async fn _compute_contact_address_inline(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._compute_contact_address_inline".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_partner.py:41`). Decoradores: api.depends('user_ids.manual_im_status', 'user_ids.presence_ids.status').
    async fn _compute_im_status(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._compute_im_status".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_partner.py:69`).
    async fn _get_needaction_count(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._get_needaction_count".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_partner.py:83`).
    async fn _mail_get_partners(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._mail_get_partners".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_partner.py:90`). Decoradores: api.model.
    async fn _get_view_cache_key(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._get_view_cache_key".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_partner.py:96`). Decoradores: api.model.
    async fn find_or_create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.find_or_create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_partner.py:119`). Decoradores: api.model.
    async fn _find_or_create_from_emails(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._find_or_create_from_emails".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_partner.py:238`).
    async fn _get_im_status_access_token(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._get_im_status_access_token".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_partner.py:247`).
    async fn _get_mention_token(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._get_mention_token".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_partner.py:256`).
    async fn _get_store_mention_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._get_store_mention_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_partner.py:259`).
    async fn _get_store_avatar_card_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._get_store_avatar_card_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_partner.py:269`).
    async fn _field_store_repr(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._field_store_repr".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_partner.py:282`).
    async fn _to_store_defaults(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._to_store_defaults".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_partner.py:297`). Decoradores: api.readonly, api.model.
    async fn get_mention_suggestions(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner.get_mention_suggestions".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_partner.py:313`). Decoradores: api.model.
    async fn _get_mention_suggestions_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._get_mention_suggestions_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_partner.py:317`). Decoradores: api.model.
    async fn _search_mention_suggestions(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._search_mention_suggestions".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_partner.py:340`). Decoradores: api.model.
    async fn _get_current_persona(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.partner._get_current_persona".into(),
        ))
    }

}
