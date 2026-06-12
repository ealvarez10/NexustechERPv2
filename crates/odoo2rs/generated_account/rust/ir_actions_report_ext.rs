//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/ir_actions_report.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `ir.actions.report` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct IrActionsReportExtFragment;

#[async_trait]
impl ModelFragment for IrActionsReportExtFragment {
    fn model_name(&self) -> &str {
        "ir.actions.report"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::boolean("is_invoice_report").string("Invoice report"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_render_qweb_pdf_prepare_streams", "_is_invoice_report", "_get_splitted_report", "_pre_render_qweb_pdf", "_unlink_except_master_tags", "_get_rendering_context"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_render_qweb_pdf_prepare_streams" => self._render_qweb_pdf_prepare_streams(env, ctx, rs, args).await,
            "_is_invoice_report" => self._is_invoice_report(env, ctx, rs, args).await,
            "_get_splitted_report" => self._get_splitted_report(env, ctx, rs, args).await,
            "_pre_render_qweb_pdf" => self._pre_render_qweb_pdf(env, ctx, rs, args).await,
            "_unlink_except_master_tags" => self._unlink_except_master_tags(env, ctx, rs, args).await,
            "_get_rendering_context" => self._get_rendering_context(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl IrActionsReportExtFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/ir_actions_report.py:18`).
    async fn _render_qweb_pdf_prepare_streams(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.report._render_qweb_pdf_prepare_streams".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/ir_actions_report.py:48`).
    async fn _is_invoice_report(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.report._is_invoice_report".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/ir_actions_report.py:52`).
    async fn _get_splitted_report(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.report._get_splitted_report".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/ir_actions_report.py:63`).
    async fn _pre_render_qweb_pdf(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.report._pre_render_qweb_pdf".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/ir_actions_report.py:77`). Decoradores: api.ondelete().
    async fn _unlink_except_master_tags(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.report._unlink_except_master_tags".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/ir_actions_report.py:92`).
    async fn _get_rendering_context(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): ir.actions.report._get_rendering_context".into(),
        ))
    }

}
