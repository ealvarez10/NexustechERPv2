//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_document_import_mixin.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.document.import.mixin`

use nexus_orm::prelude::*;

pub struct AccountDocumentImportMixinFragment;

#[async_trait]
impl ModelFragment for AccountDocumentImportMixinFragment {
    fn model_name(&self) -> &str {
        "account.document.import.mixin"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Business document import mixin".into();
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_create_records_from_attachments", "_group_files_data_by_origin_attachment", "_group_files_data_into_groups_of_mixed_types", "_assign_attachment_to_group_of_different_type", "_assign_attachment_to_group_with_same_origin_attachment", "_get_similarity_score", "_extend_with_attachments", "_get_edi_decoder", "_attachment_fields_to_clear", "_fix_attachments_on_record", "_should_attach_to_record", "_to_files_data", "_from_files_data", "_get_import_file_type", "_get_xml_tree", "_unwrap_attachments", "_unwrap_attachment", "_split_xml_into_new_attachments"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_create_records_from_attachments" => self._create_records_from_attachments(env, ctx, rs, args).await,
            "_group_files_data_by_origin_attachment" => self._group_files_data_by_origin_attachment(env, ctx, rs, args).await,
            "_group_files_data_into_groups_of_mixed_types" => self._group_files_data_into_groups_of_mixed_types(env, ctx, rs, args).await,
            "_assign_attachment_to_group_of_different_type" => self._assign_attachment_to_group_of_different_type(env, ctx, rs, args).await,
            "_assign_attachment_to_group_with_same_origin_attachment" => self._assign_attachment_to_group_with_same_origin_attachment(env, ctx, rs, args).await,
            "_get_similarity_score" => self._get_similarity_score(env, ctx, rs, args).await,
            "_extend_with_attachments" => self._extend_with_attachments(env, ctx, rs, args).await,
            "_get_edi_decoder" => self._get_edi_decoder(env, ctx, rs, args).await,
            "_attachment_fields_to_clear" => self._attachment_fields_to_clear(env, ctx, rs, args).await,
            "_fix_attachments_on_record" => self._fix_attachments_on_record(env, ctx, rs, args).await,
            "_should_attach_to_record" => self._should_attach_to_record(env, ctx, rs, args).await,
            "_to_files_data" => self._to_files_data(env, ctx, rs, args).await,
            "_from_files_data" => self._from_files_data(env, ctx, rs, args).await,
            "_get_import_file_type" => self._get_import_file_type(env, ctx, rs, args).await,
            "_get_xml_tree" => self._get_xml_tree(env, ctx, rs, args).await,
            "_unwrap_attachments" => self._unwrap_attachments(env, ctx, rs, args).await,
            "_unwrap_attachment" => self._unwrap_attachment(env, ctx, rs, args).await,
            "_split_xml_into_new_attachments" => self._split_xml_into_new_attachments(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl AccountDocumentImportMixinFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_document_import_mixin.py:131`). Decoradores: api.model.
    async fn _create_records_from_attachments(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.document.import.mixin._create_records_from_attachments".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_document_import_mixin.py:178`).
    async fn _group_files_data_by_origin_attachment(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.document.import.mixin._group_files_data_by_origin_attachment".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_document_import_mixin.py:190`).
    async fn _group_files_data_into_groups_of_mixed_types(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.document.import.mixin._group_files_data_into_groups_of_mixed_types".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_document_import_mixin.py:225`).
    async fn _assign_attachment_to_group_of_different_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.document.import.mixin._assign_attachment_to_group_of_different_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_document_import_mixin.py:252`).
    async fn _assign_attachment_to_group_with_same_origin_attachment(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.document.import.mixin._assign_attachment_to_group_with_same_origin_attachment".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_document_import_mixin.py:263`).
    async fn _get_similarity_score(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.document.import.mixin._get_similarity_score".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_document_import_mixin.py:278`).
    async fn _extend_with_attachments(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.document.import.mixin._extend_with_attachments".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_document_import_mixin.py:366`).
    async fn _get_edi_decoder(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.document.import.mixin._get_edi_decoder".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_document_import_mixin.py:382`).
    async fn _attachment_fields_to_clear(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.document.import.mixin._attachment_fields_to_clear".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_document_import_mixin.py:386`).
    async fn _fix_attachments_on_record(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.document.import.mixin._fix_attachments_on_record".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_document_import_mixin.py:410`).
    async fn _should_attach_to_record(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.document.import.mixin._should_attach_to_record".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_document_import_mixin.py:430`). Decoradores: api.model.
    async fn _to_files_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.document.import.mixin._to_files_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_document_import_mixin.py:452`). Decoradores: api.model.
    async fn _from_files_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.document.import.mixin._from_files_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_document_import_mixin.py:464`). Decoradores: api.model.
    async fn _get_import_file_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.document.import.mixin._get_import_file_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_document_import_mixin.py:470`). Decoradores: api.model.
    async fn _get_xml_tree(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.document.import.mixin._get_xml_tree".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_document_import_mixin.py:485`). Decoradores: api.model.
    async fn _unwrap_attachments(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.document.import.mixin._unwrap_attachments".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_document_import_mixin.py:495`). Decoradores: api.model.
    async fn _unwrap_attachment(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.document.import.mixin._unwrap_attachment".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_document_import_mixin.py:526`). Decoradores: api.model.
    async fn _split_xml_into_new_attachments(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): account.document.import.mixin._split_xml_into_new_attachments".into(),
        ))
    }

}
