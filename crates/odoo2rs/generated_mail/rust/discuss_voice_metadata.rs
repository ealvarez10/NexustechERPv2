//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_voice_metadata.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `discuss.voice.metadata`

use nexus_orm::prelude::*;

pub struct DiscussVoiceMetadataFragment;

#[async_trait]
impl ModelFragment for DiscussVoiceMetadataFragment {
    fn model_name(&self) -> &str {
        "discuss.voice.metadata"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Metadata for voice attachments".into();
        def.add_field(FieldDef::many2one("attachment_id", "ir.attachment"));
    }
}
