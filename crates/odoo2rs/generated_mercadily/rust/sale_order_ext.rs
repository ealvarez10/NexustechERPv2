//! Generado por odoo2rs desde `/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/sale_order.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `sale.order` (fragmento _inherit)

use nexus_orm::prelude::*;

pub struct SaleOrderExtFragment;

#[async_trait]
impl ModelFragment for SaleOrderExtFragment {
    fn model_name(&self) -> &str {
        "sale.order"
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
        def.add_field(FieldDef::char("mercadily_status").string("Estatus Mercadily"));
        def.add_field(FieldDef::char("mercadily_payment_method").string("Método de Pago Mercadily"));
        def.add_field(FieldDef::text("mercadily_shipping_address").string("Dirección de Envío Mercadily"));
    }
}
