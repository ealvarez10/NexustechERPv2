//! Estado del puente Rust↔Python. Vive como `thread_local` del hilo del
//! intérprete (ver [`crate::runtime`]): RustPython no es `Send`, así que
//! todo lo que toca `PyObjectRef` queda confinado a ese hilo.
//!
//! Regla de oro de este módulo: ningún borrow de `STATE` sobrevive a un
//! cruce hacia Python ni a un `block_on` — la reentrada (Python → ORM →
//! Python) volvería a pedir el estado y un borrow vivo sería un pánico.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ptr::NonNull;

use rust_decimal::prelude::ToPrimitive;
use rustpython_vm::builtins::{PyBaseExceptionRef, PyFloat, PyInt, PyList, PyStr, PyTuple, PyType, PyTypeRef};
use rustpython_vm::compiler::Mode;
use rustpython_vm::convert::TryFromObject;
use rustpython_vm::scope::Scope;
use rustpython_vm::{AsObject, PyObjectRef, PyResult, VirtualMachine};

use nexus_orm::prelude::*;
use nexus_orm::value::RecordId;

/// Las clases `Env`/`Recordset` y las excepciones estilo Odoo, en Python.
const BOOTSTRAP: &str = include_str!("bootstrap.py");

/// Etiqueta con la que un recordset cruza la frontera como `("__rs__", handle)`.
pub(crate) const TAG_RS: &str = "__rs__";

/// Marco de una llamada de método Odoo en curso (para `super_()`).
pub(crate) struct Frame {
    pub ctx: CallCtx,
    #[allow(dead_code)]
    pub self_handle: usize,
}

pub(crate) struct BridgeState {
    /// Handle del runtime tokio del proceso, para `block_on` de futures ORM.
    pub rt: Option<tokio::runtime::Handle>,
    /// Env activo durante una ejecución (pila: las llamadas se anidan).
    pub env_stack: Vec<Env>,
    /// Recordsets vivos referenciados desde Python por handle entero.
    pub handles: HashMap<usize, Recordset>,
    pub next_handle: usize,
    /// Pila de llamadas de método (CallCtx para `super_()`).
    pub ctx_stack: Vec<Frame>,
    /// Scope global compartido: bootstrap + fuentes de métodos registrados.
    pub scope: Option<Scope>,
    /// `model::module::method` → función Python compilada.
    pub methods: HashMap<String, PyObjectRef>,
    // Objetos del bootstrap que el lado Rust necesita reconocer/invocar.
    pub recordset_cls: Option<PyTypeRef>,
    pub user_error_cls: Option<PyTypeRef>,
    pub validation_error_cls: Option<PyTypeRef>,
    pub wrap_handle_fn: Option<PyObjectRef>,
}

thread_local! {
    static STATE: RefCell<Option<BridgeState>> = const { RefCell::new(None) };
    /// VM actualmente en ejecución en este hilo — permite que un
    /// `PyFragment` invocado de forma reentrante ejecute Python sin
    /// pasar (y bloquearse) por el canal del intérprete.
    static CURRENT_VM: Cell<Option<NonNull<VirtualMachine>>> = const { Cell::new(None) };
}

pub(crate) fn init(rt: Option<tokio::runtime::Handle>) {
    STATE.with(|s| {
        *s.borrow_mut() = Some(BridgeState {
            rt,
            env_stack: Vec::new(),
            handles: HashMap::new(),
            next_handle: 1,
            ctx_stack: Vec::new(),
            scope: None,
            methods: HashMap::new(),
            recordset_cls: None,
            user_error_cls: None,
            validation_error_cls: None,
            wrap_handle_fn: None,
        });
    });
}

/// `None` si este hilo no es el del intérprete (o no se inicializó).
pub(crate) fn with_state<R>(f: impl FnOnce(&mut BridgeState) -> R) -> Option<R> {
    STATE.with(|s| s.borrow_mut().as_mut().map(f))
}

fn state_or_err<R>(vm: &VirtualMachine, f: impl FnOnce(&mut BridgeState) -> R) -> PyResult<R> {
    with_state(f)
        .ok_or_else(|| vm.new_runtime_error("nexus-py: estado del puente no inicializado".to_owned()))
}

// ─── VM actual (reentrada) ──────────────────────────────────────────────────

pub(crate) struct VmGuard {
    prev: Option<NonNull<VirtualMachine>>,
}

/// Publica `vm` como la VM del hilo mientras el guard viva. El puntero solo
/// se deref-erencia desde este mismo hilo y dentro de la vida del préstamo
/// (`interp.enter` engloba todo el uso), por eso el `unsafe` de abajo es sano.
pub(crate) fn vm_guard(vm: &VirtualMachine) -> VmGuard {
    let prev = CURRENT_VM.with(|c| c.replace(Some(NonNull::from(vm))));
    VmGuard { prev }
}

impl Drop for VmGuard {
    fn drop(&mut self) {
        let prev = self.prev;
        CURRENT_VM.with(|c| c.set(prev));
    }
}

pub(crate) fn with_current_vm<R>(f: impl FnOnce(&VirtualMachine) -> R) -> Option<R> {
    CURRENT_VM
        .with(|c| c.get())
        .map(|p| f(unsafe { p.as_ref() }))
}

// ─── Puente síncrono → async ────────────────────────────────────────────────

/// Bloquea sobre un future del ORM desde dentro del intérprete. Usa
/// `pollster` (re-entrante, a diferencia de `Handle::block_on`) y entra al
/// contexto tokio si hay runtime — necesario para los futures de sqlx.
pub(crate) fn block_on<F: std::future::Future>(fut: F) -> F::Output {
    let rt = with_state(|st| st.rt.clone()).flatten();
    match rt {
        Some(h) => {
            let _g = h.enter();
            pollster::block_on(fut)
        }
        None => pollster::block_on(fut),
    }
}

// ─── Handles de recordsets ──────────────────────────────────────────────────

pub(crate) fn insert_handle(rs: Recordset) -> usize {
    with_state(|st| {
        let h = st.next_handle;
        st.next_handle += 1;
        st.handles.insert(h, rs);
        h
    })
    .unwrap_or(0)
}

pub(crate) fn handle_rs(vm: &VirtualMachine, h: usize) -> PyResult<Recordset> {
    state_or_err(vm, |st| st.handles.get(&h).cloned())?
        .ok_or_else(|| vm.new_runtime_error(format!("nexus-py: handle {h} inválido o ya liberado")))
}

pub(crate) fn free_handle(h: usize) {
    with_state(|st| {
        st.handles.remove(&h);
    });
}

// ─── Env activo ─────────────────────────────────────────────────────────────

pub(crate) fn push_env(env: Env) {
    with_state(|st| st.env_stack.push(env));
}

pub(crate) fn pop_env() {
    with_state(|st| {
        st.env_stack.pop();
    });
}

pub(crate) fn current_env(vm: &VirtualMachine) -> PyResult<Env> {
    state_or_err(vm, |st| st.env_stack.last().cloned())?.ok_or_else(|| {
        vm.new_runtime_error("nexus-py: no hay Env activo (¿código fuera de exec/eval/método?)".to_owned())
    })
}

pub(crate) fn current_frame(vm: &VirtualMachine) -> PyResult<CallCtx> {
    state_or_err(vm, |st| st.ctx_stack.last().map(|f| f.ctx.clone()))?
        .ok_or_else(|| vm.new_runtime_error("super_(): no hay un método Odoo en ejecución".to_owned()))
}

// ─── Conversión de valores ──────────────────────────────────────────────────

/// Registra `rs` y devuelve la tupla etiquetada que el bootstrap envuelve
/// en una instancia de `Recordset` Python.
pub(crate) fn tagged_rs(vm: &VirtualMachine, rs: Recordset) -> PyResult<PyObjectRef> {
    let h = insert_handle(rs);
    Ok(vm
        .ctx
        .new_tuple(vec![
            vm.ctx.new_str(TAG_RS).into(),
            vm.ctx.new_int(h).into(),
        ])
        .into())
}

fn browse_model(vm: &VirtualMachine, env: &Env, m: ModelId, ids: Vec<RecordId>) -> PyResult<Recordset> {
    let name = env.registry().def(m).name.clone();
    env.browse(&name, ids).map_err(|e| oerr_to_py(vm, e))
}

pub(crate) fn oval_to_py(vm: &VirtualMachine, env: &Env, v: OVal) -> PyResult<PyObjectRef> {
    Ok(match v {
        OVal::Null => vm.ctx.none(),
        OVal::Bool(b) => vm.ctx.new_bool(b).into(),
        OVal::Int(i) => vm.ctx.new_int(i).into(),
        OVal::Float(f) => vm.ctx.new_float(f).into(),
        // v0: Decimal cruza como float para la lógica Python; el lado Rust
        // (persistencia Monetary) sigue operando en Decimal.
        OVal::Decimal(d) => vm.ctx.new_float(d.to_f64().unwrap_or(0.0)).into(),
        OVal::Str(s) => vm.ctx.new_str(s.as_str()).into(),
        OVal::Date(d) => vm.ctx.new_str(d.format("%Y-%m-%d").to_string()).into(),
        OVal::DateTime(dt) => vm
            .ctx
            .new_str(dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .into(),
        OVal::Ref(m, id) => {
            let rs = browse_model(vm, env, m, vec![id])?;
            tagged_rs(vm, rs)?
        }
        OVal::RefSet(m, ids) => {
            let rs = browse_model(vm, env, m, ids)?;
            tagged_rs(vm, rs)?
        }
        OVal::Json(j) => json_to_py(vm, &j),
    })
}

pub(crate) fn json_to_py(vm: &VirtualMachine, v: &serde_json::Value) -> PyObjectRef {
    use serde_json::Value;
    match v {
        Value::Null => vm.ctx.none(),
        Value::Bool(b) => vm.ctx.new_bool(*b).into(),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                vm.ctx.new_int(i).into()
            } else {
                vm.ctx.new_float(n.as_f64().unwrap_or(0.0)).into()
            }
        }
        Value::String(s) => vm.ctx.new_str(s.as_str()).into(),
        Value::Array(items) => vm
            .ctx
            .new_list(items.iter().map(|x| json_to_py(vm, x)).collect())
            .into(),
        Value::Object(map) => {
            let d = vm.ctx.new_dict();
            for (k, x) in map {
                let _ = d.set_item(k.as_str(), json_to_py(vm, x), vm);
            }
            d.into()
        }
    }
}

pub(crate) fn py_to_oval(vm: &VirtualMachine, obj: &PyObjectRef) -> PyResult<OVal> {
    if vm.is_none(obj) {
        return Ok(OVal::Null);
    }
    // bool primero: en Python `bool` es subclase de `int`.
    if obj.class().is(vm.ctx.types.bool_type) {
        return Ok(OVal::Bool(obj.clone().try_to_bool(vm)?));
    }
    if let Some(i) = obj.downcast_ref::<PyInt>() {
        return Ok(OVal::Int(i.try_to_primitive::<i64>(vm)?));
    }
    if let Some(f) = obj.downcast_ref::<PyFloat>() {
        return Ok(OVal::Float(f.to_f64()));
    }
    if let Some(s) = obj.downcast_ref::<PyStr>() {
        return Ok(OVal::Str(s.as_str().into()));
    }
    if let Some(t) = obj.downcast_ref::<PyTuple>() {
        let items = t.as_slice();
        if items.len() == 2 {
            if let Some(tag) = items[0].downcast_ref::<PyStr>() {
                if tag.as_str() == TAG_RS {
                    let h = usize::try_from_object(vm, items[1].clone())?;
                    let rs = handle_rs(vm, h)?;
                    let mid = rs.model_id();
                    let ids = rs.ids().to_vec();
                    // Singleton → Ref (asignación many2one); set → RefSet.
                    return Ok(if ids.len() == 1 {
                        OVal::Ref(mid, ids[0])
                    } else {
                        OVal::RefSet(mid, ids)
                    });
                }
            }
        }
    }
    let cname = obj.class().name().to_string();
    Err(vm.new_type_error(format!(
        "nexus-py: tipo Python no convertible a OVal: {cname}"
    )))
}

/// Como `py_to_oval`, pero reconoce instancias `Recordset` del bootstrap
/// (resultado típico de un método de negocio).
pub(crate) fn py_result_to_oval(vm: &VirtualMachine, obj: PyObjectRef) -> OResult<OVal> {
    let cls = with_state(|st| st.recordset_cls.clone()).flatten();
    if let Some(cls) = cls {
        if obj.fast_isinstance(&cls) {
            let h = obj
                .get_attr("_h", vm)
                .and_then(|a| usize::try_from_object(vm, a))
                .map_err(|exc| pyexc_to_oerr(vm, exc))?;
            let rs = with_state(|st| st.handles.get(&h).cloned())
                .flatten()
                .ok_or_else(|| OError::Internal(format!("nexus-py: handle {h} inválido")))?;
            return Ok(OVal::RefSet(rs.model_id(), rs.ids().to_vec()));
        }
    }
    py_to_oval(vm, &obj).map_err(|exc| pyexc_to_oerr(vm, exc))
}

/// Serializa un dominio Python (`[("state", "=", "sale")]`) a JSON para
/// `Domain::from_json`.
pub(crate) fn py_to_json(vm: &VirtualMachine, obj: &PyObjectRef) -> PyResult<serde_json::Value> {
    use serde_json::Value;
    if vm.is_none(obj) {
        return Ok(Value::Null);
    }
    if obj.class().is(vm.ctx.types.bool_type) {
        return Ok(Value::Bool(obj.clone().try_to_bool(vm)?));
    }
    if let Some(i) = obj.downcast_ref::<PyInt>() {
        return Ok(Value::from(i.try_to_primitive::<i64>(vm)?));
    }
    if let Some(f) = obj.downcast_ref::<PyFloat>() {
        return Ok(serde_json::Number::from_f64(f.to_f64())
            .map(Value::Number)
            .unwrap_or(Value::Null));
    }
    if let Some(s) = obj.downcast_ref::<PyStr>() {
        return Ok(Value::String(s.as_str().to_owned()));
    }
    if let Some(t) = obj.downcast_ref::<PyTuple>() {
        return t
            .as_slice()
            .iter()
            .map(|x| py_to_json(vm, x))
            .collect::<PyResult<Vec<_>>>()
            .map(Value::Array);
    }
    if let Some(l) = obj.downcast_ref::<PyList>() {
        let items = l.borrow_vec().to_vec();
        return items
            .iter()
            .map(|x| py_to_json(vm, x))
            .collect::<PyResult<Vec<_>>>()
            .map(Value::Array);
    }
    Err(vm.new_type_error("nexus-py: dominio no serializable a JSON".to_owned()))
}

// ─── Errores ────────────────────────────────────────────────────────────────

/// `OError` → excepción Python (UserError/ValidationError del bootstrap
/// cuando aplica, RuntimeError para el resto).
pub(crate) fn oerr_to_py(vm: &VirtualMachine, e: OError) -> PyBaseExceptionRef {
    let classes = with_state(|st| (st.user_error_cls.clone(), st.validation_error_cls.clone()));
    if let Some((ue, ve)) = classes {
        match &e {
            OError::User(m) => {
                if let Some(c) = ue {
                    return vm.new_exception_msg(c, m.clone());
                }
            }
            OError::Validation(m) => {
                if let Some(c) = ve {
                    return vm.new_exception_msg(c, m.clone());
                }
            }
            _ => {}
        }
    }
    vm.new_runtime_error(e.to_string())
}

/// Excepción Python → `OError` (mapeo inverso de la taxonomía Odoo).
pub(crate) fn pyexc_to_oerr(vm: &VirtualMachine, exc: PyBaseExceptionRef) -> OError {
    let msg = exc
        .as_object()
        .str(vm)
        .map(|s| s.as_str().to_owned())
        .unwrap_or_default();
    let classes = with_state(|st| (st.user_error_cls.clone(), st.validation_error_cls.clone()));
    if let Some((ue, ve)) = classes {
        // ValidationError primero: es subclase de UserError.
        if let Some(ve) = ve {
            if exc.fast_isinstance(&ve) {
                return OError::Validation(msg);
            }
        }
        if let Some(ue) = ue {
            if exc.fast_isinstance(&ue) {
                return OError::User(msg);
            }
        }
    }
    let mut tb = String::new();
    let _ = vm.write_exception(&mut tb, &exc);
    OError::Internal(format!(
        "nexus-py: excepción Python no manejada:\n{}",
        tb.trim_end()
    ))
}

// ─── Ejecución ──────────────────────────────────────────────────────────────

fn scope(vm: &VirtualMachine) -> OResult<Scope> {
    let _ = vm;
    with_state(|st| st.scope.clone())
        .flatten()
        .ok_or_else(|| OError::Internal("nexus-py: scope global no inicializado".into()))
}

pub(crate) fn exec_source(vm: &VirtualMachine, source: &str, path: &str) -> OResult<()> {
    let scope = scope(vm)?;
    let code = vm
        .compile(source, Mode::Exec, path.to_owned())
        .map_err(|e| OError::Internal(format!("nexus-py: error de sintaxis en {path}: {e}")))?;
    vm.run_code_obj(code, scope)
        .map(|_| ())
        .map_err(|exc| pyexc_to_oerr(vm, exc))
}

pub(crate) fn eval_source(vm: &VirtualMachine, source: &str) -> OResult<OVal> {
    let scope = scope(vm)?;
    let code = vm
        .compile(source, Mode::Eval, "<nexus-eval>".to_owned())
        .map_err(|e| OError::Internal(format!("nexus-py: error de sintaxis: {e}")))?;
    let obj = vm
        .run_code_obj(code, scope)
        .map_err(|exc| pyexc_to_oerr(vm, exc))?;
    py_result_to_oval(vm, obj)
}

/// Compila el fuente de un método y guarda su función bajo `key`
/// (`model::module::method`).
pub(crate) fn register_method(vm: &VirtualMachine, key: &str, source: &str, func: &str) -> OResult<()> {
    exec_source(vm, source, &format!("<nexus:{key}>"))?;
    let scope = scope(vm)?;
    let f = scope.globals.get_item(func, vm).map_err(|_| {
        OError::Internal(format!(
            "nexus-py: el fuente de '{key}' no define la función '{func}'"
        ))
    })?;
    with_state(|st| st.methods.insert(key.to_owned(), f));
    Ok(())
}

pub(crate) fn method_key(model: &str, module: &str, method: &str) -> String {
    format!("{model}::{module}::{method}")
}

/// Ejecuta un método de negocio Python sobre `rs` — el corazón del puente.
/// Se invoca desde el canal del intérprete o, en reentrada, directamente
/// vía [`with_current_vm`].
pub(crate) fn call_method_in_vm(
    vm: &VirtualMachine,
    key: &str,
    rs: Recordset,
    ctx: CallCtx,
    args: Vec<OVal>,
) -> OResult<OVal> {
    let pair = with_state(|st| (st.methods.get(key).cloned(), st.wrap_handle_fn.clone()))
        .ok_or_else(|| OError::Internal("nexus-py: estado no inicializado".into()))?;
    let func = pair
        .0
        .ok_or_else(|| OError::Internal(format!("nexus-py: método '{key}' no registrado")))?;
    let wrap = pair
        .1
        .ok_or_else(|| OError::Internal("nexus-py: bootstrap incompleto (sin _wrap_handle)".into()))?;

    let env = rs.env().clone();
    let self_h = insert_handle(rs);
    push_env(env.clone());
    with_state(|st| {
        st.ctx_stack.push(Frame {
            ctx,
            self_handle: self_h,
        })
    });

    let result = (|| -> OResult<OVal> {
        let h_obj: PyObjectRef = vm.ctx.new_int(self_h).into();
        let self_obj = wrap.call((h_obj,), vm).map_err(|e| pyexc_to_oerr(vm, e))?;
        let mut py_args: Vec<PyObjectRef> = Vec::with_capacity(args.len() + 1);
        py_args.push(self_obj);
        for a in args {
            py_args.push(oval_to_py(vm, &env, a).map_err(|e| pyexc_to_oerr(vm, e))?);
        }
        let out = func.call(py_args, vm).map_err(|e| pyexc_to_oerr(vm, e))?;
        py_result_to_oval(vm, out)
    })();

    with_state(|st| {
        st.ctx_stack.pop();
    });
    pop_env();
    free_handle(self_h);
    result
}

// ─── Bootstrap ──────────────────────────────────────────────────────────────

pub(crate) fn init_bootstrap(vm: &VirtualMachine) -> OResult<()> {
    let new_scope = vm.new_scope_with_builtins();
    with_state(|st| st.scope = Some(new_scope.clone()));
    exec_source(vm, BOOTSTRAP, "<nexus-bootstrap>")?;

    let fetch = |name: &str| -> OResult<PyObjectRef> {
        new_scope.globals.get_item(name, vm).map_err(|_| {
            OError::Internal(format!("nexus-py: el bootstrap no define '{name}'"))
        })
    };
    let as_type = |obj: PyObjectRef, name: &str| -> OResult<PyTypeRef> {
        obj.downcast::<PyType>()
            .map_err(|_| OError::Internal(format!("nexus-py: '{name}' no es una clase")))
    };

    let recordset_cls = as_type(fetch("Recordset")?, "Recordset")?;
    let user_error_cls = as_type(fetch("UserError")?, "UserError")?;
    let validation_error_cls = as_type(fetch("ValidationError")?, "ValidationError")?;
    let wrap_handle_fn = fetch("_wrap_handle")?;

    with_state(|st| {
        st.recordset_cls = Some(recordset_cls);
        st.user_error_cls = Some(user_error_cls);
        st.validation_error_cls = Some(validation_error_cls);
        st.wrap_handle_fn = Some(wrap_handle_fn);
    });
    Ok(())
}
