//! La prueba definitiva del codegen FASE 3a: el Rust que emite `rust_gen`
//! se compila aquí contra el kernel real (nexus-orm) y se registra en un
//! `RegistryBuilder`. El fixture `fixtures/demo_order.rs` está generado
//! por el propio CLI; el test de drift garantiza que siga sincronizado.

use std::sync::Arc;

use nexus_orm::prelude::*;

// El fragmento generado, compilado tal cual como módulo de este test.
#[path = "fixtures/demo_order.rs"]
mod demo_order;

#[test]
fn el_fragmento_generado_se_registra_y_opera() {
    let registry = Arc::new(
        RegistryBuilder::new()
            .module("demo", &[])
            .register(Arc::new(demo_order::DemoOrderFragment))
            .build()
            .expect("registry con el fragmento generado"),
    );

    let def = registry.def(registry.model_id("demo.order").unwrap());
    assert_eq!(def.order, "id desc");
    assert_eq!(def.description, "Orden demo");
    assert!(def.has_field("amount_total"));
    assert!(def.has_field("line_ids"));

    let total = def.field("amount_total").unwrap();
    let compute = total.compute.as_ref().expect("campo computado");
    assert_eq!(compute.method, "_compute_total");
    assert_eq!(compute.depends, vec!["line_ids.subtotal"]);
    assert!(total.store, "compute store=True es columna");

    // related= y store=False sobrevivieron el codegen.
    assert_eq!(
        def.field("partner_vat").unwrap().related.as_deref(),
        Some("partner_id.vat")
    );
    assert!(!def.field("draft_note").unwrap().store);

    // El modelo es operable en un Env de prototipo.
    let env = Env::mock(registry);
    env.seed(
        "demo.order",
        1,
        vec![("name", "D0001".into()), ("state", "draft".into())],
    )
    .unwrap();
    let order = env.browse("demo.order", vec![1]).unwrap();
    assert_eq!(order.get_str("name").unwrap(), "D0001");
}

/// Guardia de drift: el fixture debe ser byte-a-byte lo que el generador
/// produce hoy. Si falla, regenerar:
/// `cargo run -p odoo2rs -- gen-rust tests/fixtures/sale_demo.py --module demo -o tests/fixtures`
#[test]
fn fixture_sincronizado_con_el_generador() {
    let src = include_str!("fixtures/sale_demo.py");
    let ex = odoo2rs::py::extract_models(src, "tests/fixtures/sale_demo.py", Some("demo"))
        .unwrap();
    assert_eq!(ex.models.len(), 1);
    let code = odoo2rs::codegen::rust_gen::fragment_rs(
        &ex.models[0],
        "tests/fixtures/sale_demo.py",
    );
    assert_eq!(
        code,
        include_str!("fixtures/demo_order.rs"),
        "el fixture quedó desfasado del generador — regenerarlo"
    );
}
