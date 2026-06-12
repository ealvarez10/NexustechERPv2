//! Extracción de vistas XML → ViewIr, y smoke de los generadores.

use odoo2rs::codegen::{js_gen, rust_gen};
use odoo2rs::py::extract_models;
use odoo2rs::xml::extract_views;

const SALE_XML: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<odoo>
    <data>
        <record id="view_order_form" model="ir.ui.view">
            <field name="name">sale.order.form</field>
            <field name="model">sale.order</field>
            <field name="arch" type="xml">
                <form string="Orden de venta">
                    <header>
                        <button name="action_confirm" type="object" string="Confirmar"
                                class="btn-primary" states="draft,sent"/>
                        <button name="action_cancel" type="object" string="Cancelar"/>
                        <field name="state" widget="statusbar" statusbar_visible="draft,sale"/>
                    </header>
                    <sheet>
                        <group>
                            <group>
                                <field name="partner_id" widget="res_partner_many2one"/>
                                <field name="date_order"/>
                            </group>
                            <group>
                                <field name="amount_untaxed" readonly="1"/>
                            </group>
                        </group>
                        <notebook>
                            <page string="Líneas">
                                <field name="order_line">
                                    <tree editable="bottom">
                                        <field name="product_id"/>
                                        <field name="price_unit"/>
                                    </tree>
                                </field>
                            </page>
                        </notebook>
                    </sheet>
                </form>
            </field>
        </record>

        <record id="view_order_tree" model="ir.ui.view">
            <field name="name">sale.order.tree</field>
            <field name="model">sale.order</field>
            <field name="arch" type="xml">
                <list>
                    <field name="name"/>
                    <field name="partner_id"/>
                    <field name="state"/>
                </list>
            </field>
        </record>

        <record id="action_orders" model="ir.actions.act_window">
            <field name="name">Órdenes de venta</field>
            <field name="res_model">sale.order</field>
            <field name="view_mode">tree,form</field>
        </record>

        <menuitem id="menu_sale_order" name="Órdenes" action="action_orders"
                  parent="menu_sale_root" sequence="2"/>
    </data>
</odoo>
"#;

#[test]
fn extrae_vistas_acciones_menus() {
    let ex = extract_views(SALE_XML, "sale_views.xml").unwrap();
    let b = &ex.bundle;
    assert_eq!(b.views.len(), 2);
    assert_eq!(b.actions.len(), 1);
    assert_eq!(b.menus.len(), 1);

    let form = &b.views[0];
    assert_eq!(form.view_type, "form");
    assert_eq!(form.model.as_deref(), Some("sale.order"));
    assert_eq!(form.xml_id.as_deref(), Some("view_order_form"));

    // Campos aplanados, incluida la sub-vista del one2many.
    let names: Vec<&str> = form.fields.iter().map(|f| f.name.as_str()).collect();
    assert!(names.contains(&"partner_id"));
    assert!(names.contains(&"product_id"), "sub-vista o2m aplanada");
    let state = form.fields.iter().find(|f| f.name == "state").unwrap();
    assert_eq!(state.widget.as_deref(), Some("statusbar"));
    assert_eq!(
        state.attrs.get("statusbar_visible").map(String::as_str),
        Some("draft,sale")
    );

    // Botones con metadatos.
    assert_eq!(form.buttons.len(), 2);
    assert_eq!(form.buttons[0].name.as_deref(), Some("action_confirm"));
    assert_eq!(form.buttons[0].states.as_deref(), Some("draft,sent"));

    // <list> normalizado a tree.
    assert_eq!(b.views[1].view_type, "tree");

    // Acción y menú.
    assert_eq!(b.actions[0].res_model.as_deref(), Some("sale.order"));
    assert_eq!(b.menus[0].action.as_deref(), Some("action_orders"));

    // El arch conserva la estructura (header → sheet → notebook).
    assert_eq!(form.arch.tag, "form");
    assert!(form.arch.children.iter().any(|c| c.tag == "header"));
}

#[test]
fn js_gen_form_y_tree() {
    let ex = extract_views(SALE_XML, "sale_views.xml").unwrap();
    let form_js = js_gen::view_js(&ex.bundle.views[0]);
    assert!(form_js.contains("renderFormPage"), "usa el componente existente");
    assert!(form_js.contains("export const DESCRIPTOR"));
    assert!(form_js.contains("renderSaleOrderForm"));
    assert!(form_js.contains("action_confirm"), "descriptor incluye botones");

    let tree_js = js_gen::view_js(&ex.bundle.views[1]);
    assert!(tree_js.contains("renderSaleOrderTree"));
    assert!(tree_js.contains("o-list-table"));

    assert_eq!(js_gen::js_file_name(&ex.bundle.views[0]), "sale_order_form.js");
}

#[test]
fn rust_gen_compila_la_forma_esperada() {
    let py = r#"
from odoo import api, fields, models

class StockRule(models.Model):
    _name = 'stock.rule'
    _order = 'sequence, id'

    name = fields.Char(required=True)
    move_type = fields.Selection([('direct', 'Directo')], default='direct')
    partner_id = fields.Many2one('res.partner')
    total = fields.Float(compute='_compute_total', store=True)

    @api.depends('name')
    def _compute_total(self):
        pass

    def move(self):
        pass
"#;
    let ex = extract_models(py, "stock_rule.py", Some("stock")).unwrap();
    let code = rust_gen::fragment_rs(&ex.models[0], "stock_rule.py");

    assert!(code.contains("pub struct StockRuleFragment;"));
    assert!(code.contains("impl ModelFragment for StockRuleFragment"));
    assert!(code.contains(r#"def.order = "sequence, id".into();"#));
    assert!(code.contains(r#"FieldDef::char("name").required()"#));
    assert!(code.contains(r#"FieldDef::selection("move_type", &[("direct", "Directo")])"#));
    assert!(code.contains(r#".computed("_compute_total", &["name"]).stored()"#));
    // `move` es keyword de Rust: el stub se renombra, el dispatch no.
    assert!(code.contains("async fn move_("));
    assert!(code.contains(r#""move" => self.move_(env, ctx, rs, args).await,"#));
    assert!(code.contains("pendiente de transpilar (FASE 3): stock.rule._compute_total"));
}
