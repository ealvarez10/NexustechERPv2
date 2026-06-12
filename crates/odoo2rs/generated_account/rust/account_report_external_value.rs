//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/community/addons/account/models/account_report.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `account.report.external.value`

use nexus_orm::prelude::*;

pub struct AccountReportExternalValueFragment;

#[async_trait]
impl ModelFragment for AccountReportExternalValueFragment {
    fn model_name(&self) -> &str {
        "account.report.external.value"
    }

    fn module(&self) -> &str {
        "account"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Accounting Report External Value".into();
        def.order = "date, id".into();
        def.add_field(FieldDef::char("name").required());
        def.add_field(FieldDef::float("value").string("Numeric Value"));
        def.add_field(FieldDef::char("text_value").string("Text Value"));
        def.add_field(FieldDef::date("date").required());
        def.add_field(FieldDef::many2one("target_report_expression_id", "account.report.expression").string("Target Expression").required());
        // TODO(odoo2rs): campo 'target_report_line_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field({ let mut f = FieldDef::char("target_report_expression_label").string("Target Expression Label"); f.related = Some("target_report_expression_id.label".into()); f });
        // TODO(odoo2rs): campo 'report_country_id' (many2one) no generable — falta comodel/inverse o tipo sin equivalente.
        def.add_field(FieldDef::many2one("company_id", "res.company").string("Company").required());
        def.add_field(FieldDef::char("carryover_origin_expression_label").string("Origin Expression Label"));
        def.add_field(FieldDef::many2one("carryover_origin_report_line_id", "account.report.line").string("Origin Line"));
    }
}
