//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.fiscal.position`

use nexus_orm::prelude::*;

pub struct AccountFiscalPositionFragment;

#[async_trait]
impl ModelFragment for AccountFiscalPositionFragment {
    fn model_name(&self) -> &str {
        "account.fiscal.position"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Fiscal Position".into();
        def.order = "sequence".into();
        def.add_field(FieldDef::integer("sequence"));
        def.add_field(FieldDef::char("name").string("Fiscal Position").required());
        def.add_field(FieldDef::boolean("active").default_val(true));
        def.add_field(FieldDef::many2one("company_id", "res.company").string("Company").required().readonly());
        def.add_field(FieldDef::one2many("account_ids", "account.fiscal.position.account", "position_id").string("Account Mapping"));
        def.add_field(FieldDef::new("account_map", FieldType::Binary).computed("_compute_account_map", &["account_ids.account_src_id", "account_ids.account_dest_id"]).stored());
        def.add_field(FieldDef::many2many("tax_ids", "account.tax").string("Taxes"));
        def.add_field(FieldDef::new("tax_map", FieldType::Binary).computed("_compute_tax_map", &["tax_ids"]).stored());
        def.add_field(FieldDef::html("note").string("Notes"));
        def.add_field(FieldDef::boolean("auto_apply").string("Detect Automatically"));
        def.add_field(FieldDef::boolean("vat_required").string("VAT required"));
        // TODO(odoo2rs): campo 'company_country_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field({ let mut f = FieldDef::char("fiscal_country_codes").string("Company Fiscal Country Code"); f.related = Some("company_country_id.code".into()); f });
        def.add_field(FieldDef::many2one("country_id", "res.country").string("Country"));
        def.add_field(FieldDef::boolean("is_domestic").computed("_compute_is_domestic", &["company_id.domestic_fiscal_position_id"]).stored());
        def.add_field(FieldDef::many2one("country_group_id", "res.country.group").string("Country Group"));
        def.add_field(FieldDef::many2many("state_ids", "res.country.state").string("Federal States"));
        def.add_field(FieldDef::char("zip_from").string("Zip Range From"));
        def.add_field(FieldDef::char("zip_to").string("Zip Range To"));
        def.add_field(FieldDef::integer("states_count").computed("_compute_states_count", &[]).stored());
        def.add_field(FieldDef::char("foreign_vat").string("Foreign Tax ID"));
        def.add_field(FieldDef::selection("foreign_vat_header_mode", &[("templates_found", "Templates Found"), ("no_template", "No Template")]).computed("_compute_foreign_vat_header_mode", &["foreign_vat", "country_id"]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_is_domestic", "_compute_states_count", "_compute_foreign_vat_header_mode", "_compute_tax_map", "_compute_account_map", "_check_zip", "_validate_foreign_vat_country", "_onchange_foreign_vat", "_inverse_foreign_vat", "map_tax", "map_account", "_onchange_country_id", "_onchange_country_group_id", "_convert_zip_values", "create", "write", "_get_first_matching_fpos", "_get_fpos_validation_functions", "_get_fiscal_position", "action_open_related_taxes", "action_create_foreign_taxes"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_is_domestic" => self._compute_is_domestic(env, ctx, rs, args).await,
            "_compute_states_count" => self._compute_states_count(env, ctx, rs, args).await,
            "_compute_foreign_vat_header_mode" => self._compute_foreign_vat_header_mode(env, ctx, rs, args).await,
            "_compute_tax_map" => self._compute_tax_map(env, ctx, rs, args).await,
            "_compute_account_map" => self._compute_account_map(env, ctx, rs, args).await,
            "_check_zip" => self._check_zip(env, ctx, rs, args).await,
            "_validate_foreign_vat_country" => self._validate_foreign_vat_country(env, ctx, rs, args).await,
            "_onchange_foreign_vat" => self._onchange_foreign_vat(env, ctx, rs, args).await,
            "_inverse_foreign_vat" => self._inverse_foreign_vat(env, ctx, rs, args).await,
            "map_tax" => self.map_tax(env, ctx, rs, args).await,
            "map_account" => self.map_account(env, ctx, rs, args).await,
            "_onchange_country_id" => self._onchange_country_id(env, ctx, rs, args).await,
            "_onchange_country_group_id" => self._onchange_country_group_id(env, ctx, rs, args).await,
            "_convert_zip_values" => self._convert_zip_values(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "_get_first_matching_fpos" => self._get_first_matching_fpos(env, ctx, rs, args).await,
            "_get_fpos_validation_functions" => self._get_fpos_validation_functions(env, ctx, rs, args).await,
            "_get_fiscal_position" => self._get_fiscal_position(env, ctx, rs, args).await,
            "action_open_related_taxes" => self.action_open_related_taxes(env, ctx, rs, args).await,
            "action_create_foreign_taxes" => self.action_create_foreign_taxes(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountFiscalPositionFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:75`). Decoradores: api.depends('company_id.domestic_fiscal_position_id').
    async fn _compute_is_domestic(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position._compute_is_domestic".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:79`).
    async fn _compute_states_count(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position._compute_states_count".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:84`). Decoradores: api.depends('foreign_vat', 'country_id').
    async fn _compute_foreign_vat_header_mode(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position._compute_foreign_vat_header_mode".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:99`). Decoradores: api.depends('tax_ids').
    async fn _compute_tax_map(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position._compute_tax_map".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:108`). Decoradores: api.depends('account_ids.account_src_id', 'account_ids.account_dest_id').
    async fn _compute_account_map(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position._compute_account_map".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:113`). Decoradores: api.constrains('zip_from', 'zip_to').
    async fn _check_zip(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position._check_zip".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:119`). Decoradores: api.constrains('country_id', 'country_group_id', 'state_ids', 'foreign_vat').
    async fn _validate_foreign_vat_country(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position._validate_foreign_vat_country".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:142`). Decoradores: api.onchange('country_id', 'foreign_vat').
    async fn _onchange_foreign_vat(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position._onchange_foreign_vat".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:145`).
    async fn _inverse_foreign_vat(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position._inverse_foreign_vat".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:154`).
    async fn map_tax(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position.map_tax".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:165`).
    async fn map_account(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position.map_account".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:169`). Decoradores: api.onchange('country_id').
    async fn _onchange_country_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position._onchange_country_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:176`). Decoradores: api.onchange('country_group_id').
    async fn _onchange_country_group_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position._onchange_country_group_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:182`). Decoradores: api.model.
    async fn _convert_zip_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position._convert_zip_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:192`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:200`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:208`).
    async fn _get_first_matching_fpos(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position._get_first_matching_fpos".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:215`).
    async fn _get_fpos_validation_functions(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position._get_fpos_validation_functions".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:247`). Decoradores: api.model.
    async fn _get_fiscal_position(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position._get_fiscal_position".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:281`).
    async fn action_open_related_taxes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position.action_open_related_taxes".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py:292`).
    async fn action_create_foreign_taxes(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.fiscal.position.action_create_foreign_taxes".into(),
        ))
    }

}
