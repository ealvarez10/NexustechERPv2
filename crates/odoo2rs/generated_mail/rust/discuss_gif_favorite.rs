//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/discuss/discuss_gif_favorite.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `discuss.gif.favorite`

use nexus_orm::prelude::*;

pub struct DiscussGifFavoriteFragment;

#[async_trait]
impl ModelFragment for DiscussGifFavoriteFragment {
    fn model_name(&self) -> &str {
        "discuss.gif.favorite"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Save favorite GIF from Tenor API".into();
        def.add_field(FieldDef::char("tenor_gif_id").string("GIF id from Tenor").required());
    }
}
