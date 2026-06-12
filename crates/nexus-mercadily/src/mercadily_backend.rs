//! Generado por odoo2rs desde `/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mercadily.backend`

use nexus_orm::prelude::*;

pub struct MercadilyBackendFragment;

#[async_trait]
impl ModelFragment for MercadilyBackendFragment {
    fn model_name(&self) -> &str {
        "mercadily.backend"
    }

    fn module(&self) -> &str {
        "mercadily_connector"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Mercadily Backend Configuration".into();
        def.add_field(FieldDef::char("name").string("Nombre de la Tienda").required());
        def.add_field(FieldDef::char("api_url").string("URL de la API").required());
        def.add_field(FieldDef::char("api_key").string("API Key").required());
        def.add_field(FieldDef::boolean("active").default_val(true));
        def.add_field(FieldDef::many2one("company_id", "res.company").string("Compañía").required());
        def.add_field(FieldDef::datetime("last_lead_sync").string("Última Sincronización de Leads").readonly());
        def.add_field(FieldDef::datetime("last_order_sync").string("Última Sincronización de Pedidos").readonly());
        def.add_field(FieldDef::datetime("last_customer_sync").string("Última Sincronización de Clientes").readonly());
        def.add_field(FieldDef::integer("lead_count").string("Leads").computed("_compute_lead_count", &[]).stored());
        def.add_field(FieldDef::integer("order_count").string("Pedidos").computed("_compute_order_count", &[]).stored());
        def.add_field(FieldDef::integer("customer_count").string("Clientes").computed("_compute_customer_count", &[]).stored());
        def.add_field(FieldDef::one2many("sync_log_ids", "mercadily.sync.log", "backend_id").string("Logs de Sincronización"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_lead_count", "_compute_order_count", "_compute_customer_count", "_get_headers", "_api_get", "_api_get_all_pages", "action_test_connection", "action_sync_leads", "_sync_leads", "_prepare_lead_vals", "action_sync_customers", "_sync_customers", "_prepare_customer_vals", "_prepare_address_vals", "action_sync_orders", "_sync_orders", "_prepare_order_vals", "_prepare_order_line_vals", "_get_or_create_partner_from_order", "action_sync_all", "_cron_sync_all", "_create_sync_log", "action_view_leads", "action_view_orders", "action_view_customers"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_lead_count" => self._compute_lead_count(env, ctx, rs, args).await,
            "_compute_order_count" => self._compute_order_count(env, ctx, rs, args).await,
            "_compute_customer_count" => self._compute_customer_count(env, ctx, rs, args).await,
            "_get_headers" => self._get_headers(env, ctx, rs, args).await,
            "_api_get" => self._api_get(env, ctx, rs, args).await,
            "_api_get_all_pages" => self._api_get_all_pages(env, ctx, rs, args).await,
            "action_test_connection" => self.action_test_connection(env, ctx, rs, args).await,
            "action_sync_leads" => self.action_sync_leads(env, ctx, rs, args).await,
            "_sync_leads" => self._sync_leads(env, ctx, rs, args).await,
            "_prepare_lead_vals" => self._prepare_lead_vals(env, ctx, rs, args).await,
            "action_sync_customers" => self.action_sync_customers(env, ctx, rs, args).await,
            "_sync_customers" => self._sync_customers(env, ctx, rs, args).await,
            "_prepare_customer_vals" => self._prepare_customer_vals(env, ctx, rs, args).await,
            "_prepare_address_vals" => self._prepare_address_vals(env, ctx, rs, args).await,
            "action_sync_orders" => self.action_sync_orders(env, ctx, rs, args).await,
            "_sync_orders" => self._sync_orders(env, ctx, rs, args).await,
            "_prepare_order_vals" => self._prepare_order_vals(env, ctx, rs, args).await,
            "_prepare_order_line_vals" => self._prepare_order_line_vals(env, ctx, rs, args).await,
            "_get_or_create_partner_from_order" => self._get_or_create_partner_from_order(env, ctx, rs, args).await,
            "action_sync_all" => self.action_sync_all(env, ctx, rs, args).await,
            "_cron_sync_all" => self._cron_sync_all(env, ctx, rs, args).await,
            "_create_sync_log" => self._create_sync_log(env, ctx, rs, args).await,
            "action_view_leads" => self.action_view_leads(env, ctx, rs, args).await,
            "action_view_orders" => self.action_view_orders(env, ctx, rs, args).await,
            "action_view_customers" => self.action_view_customers(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MercadilyBackendFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:28`).
    async fn _compute_lead_count(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend._compute_lead_count".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:34`).
    async fn _compute_order_count(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend._compute_order_count".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:40`).
    async fn _compute_customer_count(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend._compute_customer_count".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:46`).
    async fn _get_headers(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend._get_headers".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:50`).
    async fn _api_get(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend._api_get".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:61`).
    async fn _api_get_all_pages(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend._api_get_all_pages".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:78`).
    async fn action_test_connection(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend.action_test_connection".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:89`).
    async fn action_sync_leads(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend.action_sync_leads".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:93`).
    async fn _sync_leads(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend._sync_leads".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:119`).
    async fn _prepare_lead_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend._prepare_lead_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:123`).
    async fn action_sync_customers(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend.action_sync_customers".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:127`).
    async fn _sync_customers(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend._sync_customers".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:152`).
    async fn _prepare_customer_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend._prepare_customer_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:163`).
    async fn _prepare_address_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend._prepare_address_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:177`).
    async fn action_sync_orders(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend.action_sync_orders".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:181`).
    async fn _sync_orders(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend._sync_orders".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:208`).
    async fn _prepare_order_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend._prepare_order_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:231`).
    async fn _prepare_order_line_vals(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend._prepare_order_line_vals".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:240`).
    async fn _get_or_create_partner_from_order(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend._get_or_create_partner_from_order".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:263`).
    async fn action_sync_all(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend.action_sync_all".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:271`). Decoradores: api.model.
    async fn _cron_sync_all(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend._cron_sync_all".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:281`).
    async fn _create_sync_log(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend._create_sync_log".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:285`).
    async fn action_view_leads(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend.action_view_leads".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:289`).
    async fn action_view_orders(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend.action_view_orders".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_backend.py:293`).
    async fn action_view_customers(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mercadily.backend.action_view_customers".into(),
        ))
    }

}
