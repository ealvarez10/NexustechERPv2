//! Tests de integración del intérprete embebido: métodos de negocio Python
//! ejecutando sobre el Env (mock) de nexus-orm.

use std::sync::Arc;

use nexus_orm::prelude::*;
use nexus_py::{PyMethod, PyModelSpec, PyRuntime};

const IR_DEMO_TASK: &str = r#"{
    "model": "demo.task",
    "module": "demo",
    "fields": [
        {"name": "name", "type": "char", "required": true},
        {"name": "state", "type": "selection",
         "selection": [["open", "Abierta"], ["done", "Hecha"]]},
        {"name": "priority", "type": "integer"}
    ]
}"#;

/// Métodos "intraducibles" de ejemplo, tal cual los escribiría Odoo.
fn demo_methods() -> Vec<PyMethod> {
    vec![
        PyMethod::new(
            "action_done",
            r#"
def action_done(self):
    for task in self:
        if not task.name:
            raise UserError("Tarea sin nombre")
        task.state = "done"
    return True
"#,
        ),
        PyMethod::new(
            "set_priority",
            r#"
def set_priority(self, prio):
    for task in self:
        task.priority = prio
    return len(self)
"#,
        ),
        // Ejercita self.env[...] y la iteración/comparación de campos.
        PyMethod::new(
            "count_open",
            r#"
def count_open(self):
    tasks = self.env["demo.task"].browse([1, 2])
    return len([t for t in tasks if t.state != "done"])
"#,
        ),
        // Reentrada: Python → despacho ORM → otro método Python.
        PyMethod::new(
            "mark_all",
            r#"
def mark_all(self):
    return self.action_done()
"#,
        ),
    ]
}

async fn build_env(py: &PyRuntime) -> Env {
    let frag = py
        .register_fragment(PyModelSpec {
            model: "demo.task".into(),
            module: "demo".into(),
            extension: true,
            methods: demo_methods(),
        })
        .await
        .expect("registro del fragmento Python");

    let registry = Arc::new(
        RegistryBuilder::new()
            .module("demo", &[])
            .register_ir_json(IR_DEMO_TASK)
            .expect("IR válido")
            .register(frag)
            .build()
            .expect("registry"),
    );

    let env = Env::mock(registry);
    env.seed(
        "demo.task",
        1,
        vec![("name", "Migrar ventas".into()), ("state", "open".into()), ("priority", 1.into())],
    )
    .unwrap();
    env.seed(
        "demo.task",
        2,
        vec![("name", "Probar CFDI".into()), ("state", "open".into()), ("priority", 2.into())],
    )
    .unwrap();
    env
}

#[tokio::test]
async fn metodo_python_escribe_sobre_el_env_compartido() {
    let py = PyRuntime::new().unwrap();
    let env = build_env(&py).await;

    let tasks = env.browse("demo.task", vec![1, 2]).unwrap();
    let r = tasks.call("action_done", &[]).await.unwrap();
    assert_eq!(r, OVal::Bool(true));

    // Lo que Python escribió es visible desde Rust: mismo Env, misma caché.
    assert_eq!(tasks.at(0).unwrap().get_str("state").unwrap(), "done");
    assert_eq!(tasks.at(1).unwrap().get_str("state").unwrap(), "done");
}

#[tokio::test]
async fn argumentos_y_valor_de_retorno() {
    let py = PyRuntime::new().unwrap();
    let env = build_env(&py).await;

    let tasks = env.browse("demo.task", vec![1, 2]).unwrap();
    let r = tasks.call("set_priority", &[OVal::Int(7)]).await.unwrap();
    assert_eq!(r, OVal::Int(2)); // devuelve len(self)
    assert_eq!(tasks.at(0).unwrap().get_int("priority").unwrap(), 7);
    assert_eq!(tasks.at(1).unwrap().get_int("priority").unwrap(), 7);
}

#[tokio::test]
async fn python_accede_a_self_env() {
    let py = PyRuntime::new().unwrap();
    let env = build_env(&py).await;

    let one = env.browse("demo.task", vec![1]).unwrap();
    let r = one.call("count_open", &[]).await.unwrap();
    assert_eq!(r, OVal::Int(2));

    one.call("action_done", &[]).await.unwrap();
    let r = one.call("count_open", &[]).await.unwrap();
    assert_eq!(r, OVal::Int(1));
}

#[tokio::test]
async fn user_error_cruza_como_oerror_user() {
    let py = PyRuntime::new().unwrap();
    let env = build_env(&py).await;
    env.seed("demo.task", 3, vec![("name", "".into()), ("state", "open".into())])
        .unwrap();

    let bad = env.browse("demo.task", vec![3]).unwrap();
    let err = bad.call("action_done", &[]).await.unwrap_err();
    match err {
        OError::User(msg) => assert!(msg.contains("Tarea sin nombre"), "msg: {msg}"),
        other => panic!("esperaba OError::User, fue: {other:?}"),
    }
}

#[tokio::test]
async fn reentrada_python_orm_python() {
    let py = PyRuntime::new().unwrap();
    let env = build_env(&py).await;

    let tasks = env.browse("demo.task", vec![1, 2]).unwrap();
    // mark_all (Python) → rs_call → despacho ORM → action_done (Python)
    let r = tasks.call("mark_all", &[]).await.unwrap();
    assert_eq!(r, OVal::Bool(true));
    assert_eq!(tasks.at(0).unwrap().get_str("state").unwrap(), "done");
}

#[tokio::test]
async fn exec_y_eval_con_env_global() {
    let py = PyRuntime::new().unwrap();
    let env = build_env(&py).await;

    assert_eq!(py.eval(&env, "1 + 41").await.unwrap(), OVal::Int(42));

    // El global `env` del bootstrap apunta al Env activo.
    py.exec(&env, "env['demo.task'].browse([1]).priority = 99")
        .await
        .unwrap();
    let one = env.browse("demo.task", vec![1]).unwrap();
    assert_eq!(one.get_int("priority").unwrap(), 99);

    let n = py
        .eval(&env, "len(env['demo.task'].browse([1, 2]))")
        .await
        .unwrap();
    assert_eq!(n, OVal::Int(2));
}
