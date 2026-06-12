//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/product.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `product.category` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct ProductCategoryExtFragment;

#[async_trait]
impl ModelFragment for ProductCategoryExtFragment {
    fn model_name(&self) -> &str {
        "product.category"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        def.add_field(FieldDef::many2one("property_account_income_categ_id", "account.account").string("Income Account"));
        def.add_field(FieldDef::many2one("property_account_expense_categ_id", "account.account").string("Expense Account"));
    }
}
