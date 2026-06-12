//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/partner.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.fiscal.position.account`

use nexus_orm::prelude::*;

pub struct AccountFiscalPositionAccountFragment;

#[async_trait]
impl ModelFragment for AccountFiscalPositionAccountFragment {
    fn model_name(&self) -> &str {
        "account.fiscal.position.account"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Accounts Mapping of Fiscal Position".into();
        def.rec_name = "position_id".into();
        def.add_field(FieldDef::many2one("position_id", "account.fiscal.position").string("Fiscal Position").required());
        def.add_field({ let mut f = FieldDef::many2one("company_id", "res.company").string("Company"); f.related = Some("position_id.company_id".into()); f });
        def.add_field(FieldDef::many2one("account_src_id", "account.account").string("Account on Product").required());
        def.add_field(FieldDef::many2one("account_dest_id", "account.account").string("Account to Use Instead").required());
    }
}
