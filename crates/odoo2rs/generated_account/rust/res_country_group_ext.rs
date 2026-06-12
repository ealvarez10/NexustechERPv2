//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/res_country_group.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `res.country.group` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct ResCountryGroupExtFragment;

#[async_trait]
impl ModelFragment for ResCountryGroupExtFragment {
    fn model_name(&self) -> &str {
        "res.country.group"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::many2many("exclude_state_ids", "res.country.state").string("Fiscal Exceptions"));
    }
}
