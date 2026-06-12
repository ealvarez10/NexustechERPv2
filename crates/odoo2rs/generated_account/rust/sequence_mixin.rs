//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `sequence.mixin`

use nexus_orm::prelude::*;

pub struct SequenceMixinFragment;

#[async_trait]
impl ModelFragment for SequenceMixinFragment {
    fn model_name(&self) -> &str {
        "sequence.mixin"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Automatic sequence".into();
        def.add_field(FieldDef::char("sequence_prefix").computed("_compute_split_sequence", &[]).stored());
        def.add_field(FieldDef::integer("sequence_number").computed("_compute_split_sequence", &[]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["init", "_get_sequence_cache", "write", "_get_sequence_date_range", "_must_check_constrains_date_sequence", "_year_match", "_truncate_year_to_length", "_sequence_matches_date", "_constrains_date_sequence", "_compute_split_sequence", "_deduce_sequence_number_reset", "_make_regex_non_capturing", "_get_last_sequence_domain", "_get_starting_sequence", "_get_last_sequence", "_get_sequence_format_param", "_locked_increment", "_set_next_sequence", "_get_next_sequence_format", "_is_last_from_seq_chain", "_is_end_of_seq_chain"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "init" => self.init(env, ctx, rs, args).await,
            "_get_sequence_cache" => self._get_sequence_cache(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "_get_sequence_date_range" => self._get_sequence_date_range(env, ctx, rs, args).await,
            "_must_check_constrains_date_sequence" => self._must_check_constrains_date_sequence(env, ctx, rs, args).await,
            "_year_match" => self._year_match(env, ctx, rs, args).await,
            "_truncate_year_to_length" => self._truncate_year_to_length(env, ctx, rs, args).await,
            "_sequence_matches_date" => self._sequence_matches_date(env, ctx, rs, args).await,
            "_constrains_date_sequence" => self._constrains_date_sequence(env, ctx, rs, args).await,
            "_compute_split_sequence" => self._compute_split_sequence(env, ctx, rs, args).await,
            "_deduce_sequence_number_reset" => self._deduce_sequence_number_reset(env, ctx, rs, args).await,
            "_make_regex_non_capturing" => self._make_regex_non_capturing(env, ctx, rs, args).await,
            "_get_last_sequence_domain" => self._get_last_sequence_domain(env, ctx, rs, args).await,
            "_get_starting_sequence" => self._get_starting_sequence(env, ctx, rs, args).await,
            "_get_last_sequence" => self._get_last_sequence(env, ctx, rs, args).await,
            "_get_sequence_format_param" => self._get_sequence_format_param(env, ctx, rs, args).await,
            "_locked_increment" => self._locked_increment(env, ctx, rs, args).await,
            "_set_next_sequence" => self._set_next_sequence(env, ctx, rs, args).await,
            "_get_next_sequence_format" => self._get_next_sequence_format(env, ctx, rs, args).await,
            "_is_last_from_seq_chain" => self._is_last_from_seq_chain(env, ctx, rs, args).await,
            "_is_end_of_seq_chain" => self._is_end_of_seq_chain(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl SequenceMixinFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:50`).
    async fn init(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin.init".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:88`).
    async fn _get_sequence_cache(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin._get_sequence_cache".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:112`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:117`).
    async fn _get_sequence_date_range(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin._get_sequence_date_range".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:127`).
    async fn _must_check_constrains_date_sequence(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin._must_check_constrains_date_sequence".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:130`).
    async fn _year_match(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin._year_match".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:133`).
    async fn _truncate_year_to_length(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin._truncate_year_to_length".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:136`).
    async fn _sequence_matches_date(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin._sequence_matches_date".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:155`). Decoradores: api.constrains().
    async fn _constrains_date_sequence(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin._constrains_date_sequence".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:182`). Decoradores: api.depends().
    async fn _compute_split_sequence(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin._compute_split_sequence".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:192`). Decoradores: api.model.
    async fn _deduce_sequence_number_reset(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin._deduce_sequence_number_reset".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:225`).
    async fn _make_regex_non_capturing(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin._make_regex_non_capturing".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:241`).
    async fn _get_last_sequence_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin._get_last_sequence_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:256`).
    async fn _get_starting_sequence(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin._get_starting_sequence".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:267`).
    async fn _get_last_sequence(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin._get_last_sequence".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:310`).
    async fn _get_sequence_format_param(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin._get_sequence_format_param".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:353`).
    async fn _locked_increment(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin._locked_increment".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:423`).
    async fn _set_next_sequence(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin._set_next_sequence".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:447`).
    async fn _get_next_sequence_format(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin._get_next_sequence_format".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:473`).
    async fn _is_last_from_seq_chain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin._is_last_from_seq_chain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/sequence_mixin.py:485`).
    async fn _is_end_of_seq_chain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): sequence.mixin._is_end_of_seq_chain".into(),
        ))
    }

}
