//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_journal.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.journal.group`

use nexus_orm::prelude::*;

pub struct AccountJournalGroupFragment;

#[async_trait]
impl ModelFragment for AccountJournalGroupFragment {
    fn model_name(&self) -> &str {
        "account.journal.group"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Account Journal Group".into();
        def.add_field(FieldDef::char("name").string("Ledger group").required());
        def.add_field(FieldDef::many2one("company_id", "res.company"));
        def.add_field(FieldDef::many2many("excluded_journal_ids", "account.journal").string("Excluded Journals"));
        def.add_field(FieldDef::integer("sequence").default_val(10i64));
    }
}
