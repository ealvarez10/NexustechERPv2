//! Prueba de integración del kernel: reproduce en miniatura el escenario
//! `sale.order` del reporte — un fragmento base (módulo `sale`) y una
//! extensión `_inherit` (módulo `sale_ext`) que sobrecarga
//! `action_confirm` y llama a `super()`, con un campo Monetary computado
//! vía `@api.depends`. Todo en modo prototipo (sin Postgres): es el mismo
//! kernel que luego ejecutará contra las tablas reales.

use std::sync::Arc;

use rust_decimal::Decimal;
use nexus_orm::prelude::*;

// ─── Fragmento base: módulo `sale` ──────────────────────────────────────

struct SaleBase;

#[async_trait]
impl ModelFragment for SaleBase {
    fn model_name(&self) -> &str {
        "x.order"
    }

    fn module(&self) -> &str {
        "sale"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Orden de venta (mini)".into();
        def.add_field(FieldDef::char("name").required());
        def.add_field(FieldDef::selection(
            "state",
            &[("draft", "Borrador"), ("sent", "Enviada"), ("sale", "Confirmada")],
        ).default_val("draft"));
        def.add_field(FieldDef::many2one("partner_id", "x.partner"));
        def.add_field(FieldDef::monetary("amount_untaxed"));
        def.add_field(
            FieldDef::monetary("amount_tax")
                .computed("_compute_amounts", &["amount_untaxed"]),
        );
        def.add_field(
            FieldDef::monetary("amount_total")
                .computed("_compute_amounts", &["amount_untaxed"]),
        );
        def.add_field(FieldDef::char("note"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["action_confirm", "_compute_amounts"]
    }

    async fn call(
        &self,
        _env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            // Transpilación 1:1 del action_confirm del reporte (§4.2)
            "action_confirm" => {
                for order in rs.iter() {
                    if !matches!(order.get_str("state")?.as_str(), "draft" | "sent") {
                        return Err(OError::user("Solo borradores"));
                    }
                    order.set("state", "sale").await?;
                }
                Ok(OVal::Null)
            }
            // IVA 16 % — siempre Decimal, nunca float
            "_compute_amounts" => {
                let tasa = Decimal::new(16, 2); // 0.16
                for order in rs.iter() {
                    let base = order.get_decimal("amount_untaxed")?;
                    let tax = base * tasa;
                    order.set("amount_tax", tax).await?;
                    order.set("amount_total", base + tax).await?;
                }
                Ok(OVal::Null)
            }
            otro => Err(OError::key(format!("método desconocido: {otro}"))),
        }
    }
}

// ─── Extensión `_inherit`: módulo `sale_ext` (depende de `sale`) ────────

struct SaleExt;

#[async_trait]
impl ModelFragment for SaleExt {
    fn model_name(&self) -> &str {
        "x.order"
    }

    fn module(&self) -> &str {
        "sale_ext"
    }

    fn is_extension(&self) -> bool {
        true
    }

    fn build(&self, def: &mut ModelDef) {
        // La extensión agrega un campo, como un addon real.
        def.add_field(FieldDef::char("ext_ref").string("Referencia externa"));
    }

    fn methods(&self) -> Vec<&str> {
        vec!["action_confirm"]
    }

    async fn call(
        &self,
        _env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            // def action_confirm(self): res = super().action_confirm(); ...
            "action_confirm" => {
                let res = ctx.call_super(rs, args).await?;
                rs.set("note", "confirmada-por-extension").await?;
                Ok(res)
            }
            otro => Err(OError::key(format!("método desconocido: {otro}"))),
        }
    }
}

fn registry() -> Arc<Registry> {
    Arc::new(
        RegistryBuilder::new()
            // Declarados en desorden a propósito: el topo-sort por depends
            // debe cargar sale antes que sale_ext.
            .module("sale_ext", &["sale"])
            .module("sale", &[])
            .register(Arc::new(SaleExt))
            .register(Arc::new(SaleBase))
            .build()
            .expect("registry"),
    )
}

fn env_con_datos() -> Env {
    let env = Env::mock(registry());
    for (id, name, state, untaxed) in [
        (1, "SO001", "draft", 100),
        (2, "SO002", "sent", 250),
        (3, "SO003", "sale", 999),
    ] {
        env.seed(
            "x.order",
            id,
            vec![
                ("name", name.into()),
                ("state", state.into()),
                ("amount_untaxed", Decimal::from(untaxed).into()),
            ],
        )
        .unwrap();
    }
    env
}

#[tokio::test]
async fn herencia_con_super_y_vtable() {
    let env = env_con_datos();
    let orders = env.browse("x.order", vec![1, 2]).unwrap();

    // El despacho entra por SaleExt (más derivado), que llama a super()
    // → SaleBase confirma, y la extensión deja su marca.
    orders.call("action_confirm", &[]).await.unwrap();

    for o in orders.iter() {
        assert_eq!(o.get_str("state").unwrap(), "sale");
        assert_eq!(o.get_str("note").unwrap(), "confirmada-por-extension");
    }
}

#[tokio::test]
async fn user_error_en_estado_invalido() {
    let env = env_con_datos();
    let confirmada = env.browse("x.order", vec![3]).unwrap();
    let err = confirmada.call("action_confirm", &[]).await.unwrap_err();
    assert!(matches!(err, OError::User(_)), "se esperaba UserError: {err}");
}

#[tokio::test]
async fn api_depends_dispara_computes() {
    let env = env_con_datos();
    let order = env.browse("x.order", vec![1]).unwrap();

    // Escribir la dependencia recalcula amount_tax y amount_total solos.
    order.set("amount_untaxed", Decimal::from(200)).await.unwrap();

    assert_eq!(order.get_decimal("amount_tax").unwrap(), Decimal::from(32));
    assert_eq!(order.get_decimal("amount_total").unwrap(), Decimal::from(232));
}

#[tokio::test]
async fn create_dispara_computes_y_defaults() {
    let env = env_con_datos();
    let nueva = env
        .create(
            "x.order",
            vec![
                ("name".into(), "SO100".into()),
                ("amount_untaxed".into(), Decimal::from(1000).into()),
            ],
        )
        .await
        .unwrap();

    assert_eq!(nueva.len(), 1);
    // default declarado en el campo state
    assert_eq!(nueva.get_str("state").unwrap(), "draft");
    // compute disparado en el create
    assert_eq!(nueva.get_decimal("amount_total").unwrap(), Decimal::from(1160));
}

#[tokio::test]
async fn operaciones_de_recordset() {
    let env = env_con_datos();
    let todas = env.browse("x.order", vec![1, 2, 3]).unwrap();

    // filtered (lambda sobre singletons)
    let borradores = todas.filtered(|o| o.get_str("state").map(|s| s == "draft").unwrap_or(false));
    assert_eq!(borradores.ids(), &[1]);

    // sorted por monto, descendente
    let ordenadas = todas.sorted("amount_untaxed", true).unwrap();
    assert_eq!(ordenadas.ids(), &[3, 2, 1]);

    // mapped escalar
    let nombres = todas.mapped("name").unwrap().values().unwrap();
    assert_eq!(nombres.len(), 3);

    // álgebra de conjuntos estilo Odoo
    let a = env.browse("x.order", vec![1, 2]).unwrap();
    let b = env.browse("x.order", vec![2, 3]).unwrap();
    assert_eq!((&a | &b).ids(), &[1, 2, 3]);
    assert_eq!((&a & &b).ids(), &[2]);
    assert_eq!((&a - &b).ids(), &[1]);

    // ensure_one
    assert!(todas.ensure_one().is_err());
    assert_eq!(todas.first().id().unwrap(), 1);
}

#[tokio::test]
async fn registro_dinamico_desde_ir_json() {
    // Un "módulo" entero llega como IR declarativo (FASE 2) y extiende
    // x.order sin recompilar nada.
    let registry = Arc::new(
        RegistryBuilder::new()
            .module("sale", &[])
            .module("sale_ext", &["sale"])
            .module("sale_ir", &["sale"])
            .register(Arc::new(SaleBase))
            .register(Arc::new(SaleExt))
            .register_ir_json(
                r#"[{
                    "model": "x.order",
                    "module": "sale_ir",
                    "inherit": true,
                    "fields": [
                        {"name": "priority", "type": "selection",
                         "selection": [["0","Normal"],["1","Alta"]]}
                    ]
                },
                {
                    "model": "x.partner",
                    "module": "sale_ir",
                    "fields": [
                        {"name": "name", "type": "char", "required": true},
                        {"name": "vat", "type": "char"}
                    ]
                }]"#,
            )
            .unwrap()
            .build()
            .unwrap(),
    );

    // El campo del IR quedó en el modelo compilado…
    let env = Env::mock(registry);
    env.seed("x.order", 1, vec![("priority", "1".into())]).unwrap();
    let o = env.browse("x.order", vec![1]).unwrap();
    assert_eq!(o.get_str("priority").unwrap(), "1");

    // …y el modelo nuevo existe y es navegable.
    env.seed("x.partner", 7, vec![("name", "ACME".into())]).unwrap();
    let p = env.browse("x.partner", vec![7]).unwrap();
    assert_eq!(p.get_str("name").unwrap(), "ACME");
}

#[tokio::test]
async fn extension_sin_base_es_error() {
    let err = RegistryBuilder::new()
        .module("solo_ext", &[])
        .register(Arc::new(SaleExt))
        .build()
        .unwrap_err();
    assert!(matches!(err, OError::Registry(_)), "{err}");
}

#[tokio::test]
async fn mapped_relacional_resuelve_comodelo() {
    let registry = Arc::new(
        RegistryBuilder::new()
            .module("sale", &[])
            .module("sale_ext", &["sale"])
            .register(Arc::new(SaleBase))
            .register(Arc::new(SaleExt))
            .register_ir_json(
                r#"{"model": "x.partner", "module": "sale",
                    "fields": [{"name": "name", "type": "char"}]}"#,
            )
            .unwrap()
            .build()
            .unwrap(),
    );
    let env = Env::mock(registry);
    let partner_mid = env.registry().model_id("x.partner").unwrap();

    env.seed("x.order", 1, vec![("partner_id", OVal::Ref(partner_mid, 7))]).unwrap();
    env.seed("x.order", 2, vec![("partner_id", OVal::Ref(partner_mid, 7))]).unwrap();
    env.seed("x.order", 3, vec![("partner_id", OVal::Null)]).unwrap();

    let orders = env.browse("x.order", vec![1, 2, 3]).unwrap();
    let partners = orders.mapped("partner_id").unwrap().records().unwrap();

    // dedup + se salta Null, como Odoo
    assert_eq!(partners.ids(), &[7]);
    assert_eq!(partners.model_name(), "x.partner");
}
