//! Extracción de modelos Python → IR, y round-trip contra nexus-orm:
//! el JSON que emite odoo2rs debe registrarse tal cual en el kernel.

use odoo2rs::py::extract_models;

const SALE_PY: &str = r#"
from odoo import api, fields, models


class SaleOrder(models.Model):
    _name = 'sale.order'
    _description = 'Orden de venta'
    _order = 'date_order desc, id desc'

    name = fields.Char('Referencia', required=True, default='Nuevo', copy=False)
    partner_id = fields.Many2one('res.partner', string='Cliente', required=True)
    date_order = fields.Datetime(string='Fecha', default=fields.Datetime.now)
    state = fields.Selection([
        ('draft', 'Borrador'),
        ('sale', 'Confirmada'),
        ('cancel', 'Cancelada'),
    ], string='Estado', default='draft', tracking=True)
    order_line = fields.One2many('sale.order.line', 'order_id', string='Líneas')
    amount_untaxed = fields.Monetary(string='Base', compute='_compute_amounts', store=True)
    amount_tax = fields.Monetary(string='Impuestos', compute='_compute_amounts', store=True)
    partner_vat = fields.Char(related='partner_id.vat', string='RFC')
    note = fields.Text()
    active = fields.Boolean(default=True)
    tag_ids = fields.Many2many('crm.tag', 'sale_order_tag_rel', 'order_id', 'tag_id')

    @api.depends('order_line.price_subtotal')
    def _compute_amounts(self):
        for order in self:
            order.amount_untaxed = sum(order.order_line.mapped('price_subtotal'))
            order.amount_tax = order.amount_untaxed * 0.16

    def action_confirm(self):
        self.write({'state': 'sale'})
        return True


class SaleOrderMx(models.Model):
    _inherit = 'sale.order'

    l10n_mx_usage = fields.Char(string='Uso CFDI')


class NotAModel:
    helper = 42
"#;

#[test]
fn extrae_modelo_base() {
    let ex = extract_models(SALE_PY, "sale.py", Some("sale")).unwrap();
    assert_eq!(ex.models.len(), 2, "dos fragmentos: base + _inherit");

    let so = &ex.models[0];
    assert_eq!(so.model, "sale.order");
    assert!(!so.inherit);
    assert_eq!(so.module.as_deref(), Some("sale"));
    assert_eq!(so.description.as_deref(), Some("Orden de venta"));
    assert_eq!(so.order.as_deref(), Some("date_order desc, id desc"));
    assert_eq!(so.fields.len(), 11);

    let f = |n: &str| so.fields.iter().find(|f| f.name == n).unwrap();

    // Char con label posicional, required y default literal.
    let name = f("name");
    assert_eq!(name.ftype, "char");
    assert_eq!(name.string.as_deref(), Some("Referencia"));
    assert!(name.required);
    assert_eq!(name.default, Some(serde_json::json!("Nuevo")));

    // Many2one posicional + string kw.
    let partner = f("partner_id");
    assert_eq!(partner.ftype, "many2one");
    assert_eq!(partner.comodel.as_deref(), Some("res.partner"));
    assert_eq!(partner.string.as_deref(), Some("Cliente"));

    // Selection con pares literales.
    let state = f("state");
    assert_eq!(state.selection.len(), 3);
    assert_eq!(state.selection[1], ("sale".into(), "Confirmada".into()));

    // One2many con inverso.
    let lines = f("order_line");
    assert_eq!(lines.comodel.as_deref(), Some("sale.order.line"));
    assert_eq!(lines.inverse.as_deref(), Some("order_id"));

    // Compute: depends cableado desde @api.depends del método.
    let amount = f("amount_untaxed");
    assert_eq!(amount.compute.as_deref(), Some("_compute_amounts"));
    assert_eq!(amount.depends, vec!["order_line.price_subtotal"]);
    assert!(amount.store, "store=True explícito");

    // related= se conserva.
    assert_eq!(f("partner_vat").related.as_deref(), Some("partner_id.vat"));

    // Many2many con relation/columnas posicionales.
    let tags = f("tag_ids");
    assert_eq!(tags.relation.as_deref(), Some("sale_order_tag_rel"));
    assert_eq!(tags.column1.as_deref(), Some("order_id"));

    // Métodos con firma y decoradores.
    assert_eq!(so.methods.len(), 2);
    let compute = &so.methods[0];
    assert_eq!(compute.name, "_compute_amounts");
    assert_eq!(compute.depends, vec!["order_line.price_subtotal"]);
    assert!(compute.line > 0);

    // default=fields.Datetime.now (callable) genera aviso, no error.
    assert!(
        ex.warnings.iter().any(|w| w.contains("date_order")),
        "aviso por default dinámico: {:?}",
        ex.warnings
    );
}

#[test]
fn extrae_fragmento_inherit() {
    let ex = extract_models(SALE_PY, "sale.py", Some("sale")).unwrap();
    let mx = &ex.models[1];
    assert_eq!(mx.model, "sale.order", "_inherit apunta al modelo extendido");
    assert!(mx.inherit);
    assert_eq!(mx.inherits, vec!["sale.order"]);
    assert_eq!(mx.fields.len(), 1);
}

/// El contrato FASE 2 completo: el JSON emitido se registra en el kernel y
/// el modelo queda operable en un Env de prototipo. El IR es declarativo;
/// los métodos compute los aporta un fragmento aparte — el mismo reparto
/// que produce el codegen FASE 3a (IR JSON + stubs Rust).
#[test]
fn round_trip_contra_nexus_orm() {
    use nexus_orm::prelude::*;
    use std::sync::Arc;

    /// Lo que `rust_gen` generaría para los métodos de sale.order.
    struct SaleMethodsFragment;

    #[async_trait]
    impl ModelFragment for SaleMethodsFragment {
        fn model_name(&self) -> &str {
            "sale.order"
        }
        fn module(&self) -> &str {
            "sale"
        }
        fn is_extension(&self) -> bool {
            true
        }
        fn methods(&self) -> Vec<&str> {
            vec!["_compute_amounts", "action_confirm"]
        }
        async fn call(
            &self,
            _env: &Env,
            _ctx: &CallCtx,
            _rs: &Recordset,
            _args: &[OVal],
        ) -> OResult<OVal> {
            Ok(OVal::Null) // stub: el cuerpo real llega en FASE 3a
        }
    }

    let ex = extract_models(SALE_PY, "sale.py", Some("sale")).unwrap();
    let json = serde_json::to_string(&ex.models).unwrap();

    // El lado Deserialize del contrato.
    let irs = nexus_orm::ir::parse_ir(&json).expect("el kernel parsea el IR de odoo2rs");
    assert_eq!(irs.len(), 2);

    let registry = Arc::new(
        RegistryBuilder::new()
            .module("sale", &[])
            .register_ir_json(&json)
            .expect("registro dinámico del IR")
            .register(Arc::new(SaleMethodsFragment))
            .build()
            .expect("build del registry"),
    );

    let env = Env::mock(registry);
    env.seed(
        "sale.order",
        1,
        vec![
            ("name", "S00001".into()),
            ("state", "draft".into()),
            ("l10n_mx_usage", "G03".into()), // campo del fragmento _inherit
        ],
    )
    .expect("seed con campos de ambos fragmentos");

    let so = env.browse("sale.order", vec![1]).unwrap();
    assert_eq!(so.get_str("state").unwrap(), "draft");
    assert_eq!(so.get_str("l10n_mx_usage").unwrap(), "G03");
}

#[test]
fn manifiesto() {
    let m = odoo2rs::py::parse_manifest(
        r#"
{
    'name': 'Ventas MX',
    'version': '17.0.1.0.0',
    'depends': ['sale', 'l10n_mx_edi'],
    'data': ['views/sale_views.xml'],
    'installable': True,
}
"#,
        "__manifest__.py",
    )
    .unwrap();
    assert_eq!(m.name.as_deref(), Some("Ventas MX"));
    assert_eq!(m.depends, vec!["sale", "l10n_mx_edi"]);
    assert_eq!(m.data, vec!["views/sale_views.xml"]);
}
