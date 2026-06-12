//! Generado por odoo2rs desde `/home/ealvarez/workspace/nexustech erp/core/nexustech/addons/mercadily_connector/models/mercadily_sync_log.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mercadily.sync.log`

use nexus_orm::prelude::*;

pub struct MercadilySyncLogFragment;

#[async_trait]
impl ModelFragment for MercadilySyncLogFragment {
    fn model_name(&self) -> &str {
        "mercadily.sync.log"
    }

    fn module(&self) -> &str {
        "mercadily_connector"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Mercadily Sync Log".into();
        def.order = "create_date desc".into();
        def.add_field(FieldDef::many2one("backend_id", "mercadily.backend").string("Backend").required());
        def.add_field(FieldDef::selection("sync_type", &[("leads", "Leads"), ("customers", "Clientes"), ("orders", "Pedidos")]).string("Tipo").required());
        def.add_field(FieldDef::integer("created_count").string("Creados"));
        def.add_field(FieldDef::integer("updated_count").string("Actualizados"));
        def.add_field(FieldDef::integer("error_count").string("Errores"));
        def.add_field(FieldDef::text("error_details").string("Detalle de Errores"));
        def.add_field(FieldDef::datetime("create_date").string("Fecha").readonly());
    }
}
