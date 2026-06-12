//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_payment.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.move` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct AccountMoveExtFragment;

#[async_trait]
impl ModelFragment for AccountMoveExtFragment {
    fn model_name(&self) -> &str {
        "account.move"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::one2many("payment_ids", "account.payment", "move_id").string("Payments"));
    }
}
