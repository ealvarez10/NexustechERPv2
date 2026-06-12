//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_currency.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `res.currency` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct ResCurrencyExtFragment;

#[async_trait]
impl ModelFragment for ResCurrencyExtFragment {
    fn model_name(&self) -> &str {
        "res.currency"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::boolean("display_rounding_warning").string("Display Rounding Warning").computed("_compute_display_rounding_warning", &["rounding"]).stored());
        def.add_field({ let mut f = FieldDef::char("fiscal_country_codes"); f.store = false; f });
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_get_fiscal_country_codes", "_compute_display_rounding_warning", "write", "_has_accounting_entries", "_get_simple_currency_table", "_check_currency_table_monocurrency", "_get_monocurrency_currency_table_sql", "_create_currency_table", "_get_table_builder_domestic_currency", "_get_table_builder_current", "_get_table_builder_historical", "_get_table_builder_average"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_get_fiscal_country_codes" => self._get_fiscal_country_codes(env, ctx, rs, args).await,
            "_compute_display_rounding_warning" => self._compute_display_rounding_warning(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "_has_accounting_entries" => self._has_accounting_entries(env, ctx, rs, args).await,
            "_get_simple_currency_table" => self._get_simple_currency_table(env, ctx, rs, args).await,
            "_check_currency_table_monocurrency" => self._check_currency_table_monocurrency(env, ctx, rs, args).await,
            "_get_monocurrency_currency_table_sql" => self._get_monocurrency_currency_table_sql(env, ctx, rs, args).await,
            "_create_currency_table" => self._create_currency_table(env, ctx, rs, args).await,
            "_get_table_builder_domestic_currency" => self._get_table_builder_domestic_currency(env, ctx, rs, args).await,
            "_get_table_builder_current" => self._get_table_builder_current(env, ctx, rs, args).await,
            "_get_table_builder_historical" => self._get_table_builder_historical(env, ctx, rs, args).await,
            "_get_table_builder_average" => self._get_table_builder_average(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl ResCurrencyExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_currency.py:12`).
    async fn _get_fiscal_country_codes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.currency._get_fiscal_country_codes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_currency.py:20`). Decoradores: api.depends('rounding').
    async fn _compute_display_rounding_warning(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.currency._compute_display_rounding_warning".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_currency.py:26`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.currency.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_currency.py:35`).
    async fn _has_accounting_entries(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.currency._has_accounting_entries".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_currency.py:42`).
    async fn _get_simple_currency_table(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.currency._get_simple_currency_table".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_currency.py:52`).
    async fn _check_currency_table_monocurrency(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.currency._check_currency_table_monocurrency".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_currency.py:60`).
    async fn _get_monocurrency_currency_table_sql(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.currency._get_monocurrency_currency_table_sql".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_currency.py:74`).
    async fn _create_currency_table(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.currency._create_currency_table".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_currency.py:142`).
    async fn _get_table_builder_domestic_currency(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.currency._get_table_builder_domestic_currency".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_currency.py:166`).
    async fn _get_table_builder_current(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.currency._get_table_builder_current".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_currency.py:192`).
    async fn _get_table_builder_historical(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.currency._get_table_builder_historical".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_currency.py:218`).
    async fn _get_table_builder_average(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): res.currency._get_table_builder_average".into(),
        ))
    }

}
