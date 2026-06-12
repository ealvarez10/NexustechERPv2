//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/res_role.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `res.role`

use nexus_orm::prelude::*;

pub struct ResRoleFragment;

#[async_trait]
impl ModelFragment for ResRoleFragment {
    fn model_name(&self) -> &str {
        "res.role"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Represents a role in the system used to categorize users. Each role has a unique name and can be associated with multiple users. Roles can be mentioned in messages to notify all associated users.".into();
        def.add_field(FieldDef::char("name").required());
        def.add_field(FieldDef::many2many("user_ids", "res.users").string("Users"));
    }
}
