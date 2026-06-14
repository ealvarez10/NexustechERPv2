//! `PyRuntime` — el intérprete embebido y su canal de peticiones.
//!
//! Un único hilo (`nexus-py`) posee el `Interpreter` de RustPython; toda
//! ejecución Python se serializa por un canal mpsc — un GIL implícito y
//! explícito a la vez. Los `PyObjectRef` jamás salen de ese hilo.
//!
//! Caso especial: si un método Python invoca al ORM y el despacho cae en
//! OTRO método Python (cadena `_inherit`), `dispatch` detecta que ya está
//! en el hilo del intérprete y ejecuta directo (vía la VM publicada en
//! thread-local) en lugar de auto-bloquearse esperando su propio canal.

use std::sync::{mpsc, Arc};
use std::thread::{self, ThreadId};

use nexus_orm::prelude::*;
use rustpython_vm::{Interpreter, Settings};
use tokio::sync::oneshot;

use crate::fragment::{PyFragment, PyModelSpec};
use crate::state;

pub(crate) enum Request {
    Exec {
        env: Env,
        source: String,
        reply: oneshot::Sender<OResult<()>>,
    },
    Eval {
        env: Env,
        source: String,
        reply: oneshot::Sender<OResult<OVal>>,
    },
    Register {
        key: String,
        source: String,
        func: String,
        reply: oneshot::Sender<OResult<()>>,
    },
    Call {
        key: String,
        rs: Recordset,
        ctx: CallCtx,
        args: Vec<OVal>,
        reply: oneshot::Sender<OResult<OVal>>,
    },
    Shutdown,
}

struct Inner {
    tx: mpsc::Sender<Request>,
    thread_id: ThreadId,
}

impl Drop for Inner {
    fn drop(&mut self) {
        let _ = self.tx.send(Request::Shutdown);
    }
}

/// Handle clonable al intérprete embebido. Crear UNO por proceso y
/// compartirlo (cada `new()` arranca un intérprete completo).
#[derive(Clone)]
pub struct PyRuntime {
    inner: Arc<Inner>,
}

impl PyRuntime {
    /// Arranca el hilo del intérprete y carga el bootstrap. Captura el
    /// runtime tokio actual (si existe) para resolver los futures del ORM;
    /// sin runtime solo funcionan Envs `mock` (sin Postgres).
    pub fn new() -> OResult<PyRuntime> {
        let rt = tokio::runtime::Handle::try_current().ok();
        let (tx, rx) = mpsc::channel::<Request>();
        let (init_tx, init_rx) = mpsc::channel();

        thread::Builder::new()
            .name("nexus-py".to_owned())
            .spawn(move || worker(rx, init_tx, rt))
            .map_err(|e| {
                OError::Internal(format!("nexus-py: no se pudo crear el hilo del intérprete: {e}"))
            })?;

        let (thread_id, init) = init_rx
            .recv()
            .map_err(|_| OError::Internal("nexus-py: el intérprete murió durante el arranque".into()))?;
        init?;

        tracing::info!("nexus-py: intérprete RustPython embebido listo");
        Ok(PyRuntime {
            inner: Arc::new(Inner { tx, thread_id }),
        })
    }

    async fn request<T>(
        &self,
        build: impl FnOnce(oneshot::Sender<OResult<T>>) -> Request,
    ) -> OResult<T> {
        let (tx, rx) = oneshot::channel();
        self.inner
            .tx
            .send(build(tx))
            .map_err(|_| OError::Internal("nexus-py: el intérprete ya no está disponible".into()))?;
        rx.await
            .map_err(|_| OError::Internal("nexus-py: el intérprete no respondió".into()))?
    }

    /// Ejecuta código Python arbitrario en el scope global, con el `Env`
    /// dado disponible como global `env` (estilo shell de Odoo).
    pub async fn exec(&self, env: &Env, source: &str) -> OResult<()> {
        let env = env.clone();
        let source = source.to_owned();
        self.request(move |reply| Request::Exec { env, source, reply }).await
    }

    /// Evalúa una expresión Python y devuelve el resultado como `OVal`.
    pub async fn eval(&self, env: &Env, source: &str) -> OResult<OVal> {
        let env = env.clone();
        let source = source.to_owned();
        self.request(move |reply| Request::Eval { env, source, reply }).await
    }

    /// Registra las funciones Python de `spec` en el intérprete y devuelve
    /// el fragmento listo para `RegistryBuilder::register(...)`.
    pub async fn register_fragment(&self, spec: PyModelSpec) -> OResult<Arc<PyFragment>> {
        for m in &spec.methods {
            let key = state::method_key(&spec.model, &spec.module, &m.name);
            let source = m.source.clone();
            let func = m.func.clone();
            self.request(move |reply| Request::Register { key, source, func, reply })
                .await?;
        }
        Ok(Arc::new(PyFragment::new(self.clone(), spec)))
    }

    /// Despacho de un método de negocio (lo invoca `PyFragment::call`).
    pub(crate) async fn dispatch(
        &self,
        key: String,
        rs: Recordset,
        ctx: CallCtx,
        args: Vec<OVal>,
    ) -> OResult<OVal> {
        if thread::current().id() == self.inner.thread_id {
            // Reentrada Python → ORM → Python: ejecutar con la VM en curso.
            return state::with_current_vm(|vm| state::call_method_in_vm(vm, &key, rs, ctx, args))
                .unwrap_or_else(|| {
                    Err(OError::Internal(
                        "nexus-py: reentrada en el hilo del intérprete sin VM activa".into(),
                    ))
                });
        }
        self.request(move |reply| Request::Call { key, rs, ctx, args, reply })
            .await
    }
}

fn worker(
    rx: mpsc::Receiver<Request>,
    init_tx: mpsc::Sender<(ThreadId, OResult<()>)>,
    rt: Option<tokio::runtime::Handle>,
) {
    let interp = Interpreter::with_init(Settings::default(), |vm| {
        vm.add_native_module("_nexus".to_owned(), Box::new(crate::module::make_module));
    });

    state::init(rt);
    let init = interp.enter(|vm| {
        let _g = state::vm_guard(vm);
        state::init_bootstrap(vm)
    });
    let ok = init.is_ok();
    let _ = init_tx.send((thread::current().id(), init));
    if !ok {
        return;
    }

    while let Ok(req) = rx.recv() {
        match req {
            Request::Shutdown => break,
            Request::Exec { env, source, reply } => {
                let r = interp.enter(|vm| {
                    let _g = state::vm_guard(vm);
                    state::push_env(env);
                    let r = state::exec_source(vm, &source, "<nexus-exec>");
                    state::pop_env();
                    r
                });
                let _ = reply.send(r);
            }
            Request::Eval { env, source, reply } => {
                let r = interp.enter(|vm| {
                    let _g = state::vm_guard(vm);
                    state::push_env(env);
                    let r = state::eval_source(vm, &source);
                    state::pop_env();
                    r
                });
                let _ = reply.send(r);
            }
            Request::Register { key, source, func, reply } => {
                let r = interp.enter(|vm| {
                    let _g = state::vm_guard(vm);
                    state::register_method(vm, &key, &source, &func)
                });
                let _ = reply.send(r);
            }
            Request::Call { key, rs, ctx, args, reply } => {
                let r = interp.enter(|vm| {
                    let _g = state::vm_guard(vm);
                    state::call_method_in_vm(vm, &key, rs, ctx, args)
                });
                let _ = reply.send(r);
            }
        }
    }
}
