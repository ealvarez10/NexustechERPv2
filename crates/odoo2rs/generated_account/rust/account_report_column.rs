//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.report.column`

use nexus_orm::prelude::*;

pub struct AccountReportColumnFragment;

#[async_trait]
impl ModelFragment for AccountReportColumnFragment {
    fn model_name(&self) -> &str {
        "account.report.column"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Accounting Report Column".into();
        def.order = "sequence, id".into();
        def.add_field(FieldDef::char("name").string("Name").required());
        def.add_field(FieldDef::char("expression_label").string("Expression Label").required());
        def.add_field(FieldDef::integer("sequence").string("Sequence"));
        def.add_field(FieldDef::many2one("report_id", "account.report").string("Report"));
        def.add_field(FieldDef::boolean("sortable").string("Sortable"));
        def.add_field(FieldDef::selection("figure_type", &[]).string("Figure Type").required().default_val("monetary"));
        def.add_field(FieldDef::boolean("blank_if_zero").string("Blank if Zero"));
        def.add_field(FieldDef::many2one("custom_audit_action_id", "ir.actions.act_window").string("Custom Audit Action"));
    }
}
