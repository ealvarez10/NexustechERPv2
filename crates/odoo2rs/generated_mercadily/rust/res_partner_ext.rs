//! Generado por odoo2rs desde `/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/res_partner.py` — NO EDITAR A MANO;
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
        "mercadily_connector"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::many2one("mercadily_backend_id", "mercadily.backend").string("Backend Mercadily"));
        def.add_field(FieldDef::char("mercadily_id").string("ID Mercadily"));
    }
}
