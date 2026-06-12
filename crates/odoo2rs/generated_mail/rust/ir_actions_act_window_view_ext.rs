//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/ir_action_act_window.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `ir.actions.act_window.view` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct IrActionsActWindowViewExtFragment;

#[async_trait]
impl ModelFragment for IrActionsActWindowViewExtFragment {
    fn model_name(&self) -> &str {
        "ir.actions.act_window.view"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::selection("view_mode", &[]));
    }
}
