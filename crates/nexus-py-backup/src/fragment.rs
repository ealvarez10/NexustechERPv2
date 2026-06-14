//! `PyFragment` — un `ModelFragment` cuyos métodos son funciones Python.
//!
//! Es la pieza que enchufa el intérprete a la vtable `_inherit` de
//! nexus-orm: se registra en el `RegistryBuilder` como cualquier fragmento
//! Rust, así que un método Python puede sobreescribir uno Rust (y
//! viceversa), y `self.super_()` recorre la misma cadena MRO.

use async_trait::async_trait;

use nexus_orm::prelude::*;

use crate::runtime::PyRuntime;
use crate::state;

/// Un método de negocio en Python. `source` debe definir la función `func`
/// (por convención, del mismo nombre que el método) con firma
/// `def metodo(self, *args)`.
#[derive(Debug, Clone)]
pub struct PyMethod {
    pub name: String,
    pub func: String,
    pub source: String,
}

impl PyMethod {
    pub fn new(name: impl Into<String>, source: impl Into<String>) -> Self {
        let name = name.into();
        PyMethod {
            func: name.clone(),
            name,
            source: source.into(),
        }
    }
}

/// Declaración de un fragmento Python: el equivalente a una clase Odoo
/// cuyos métodos no fueron transpilables.
#[derive(Debug, Clone)]
pub struct PyModelSpec {
    /// `_name`/`_inherit` del modelo ("sale.order").
    pub model: String,
    /// Módulo Odoo de origen (orden de carga).
    pub module: String,
    /// `true` si es `_inherit` puro (extiende un modelo ya definido).
    pub extension: bool,
    pub methods: Vec<PyMethod>,
}

/// Fragmento registrable en el `RegistryBuilder`. Se obtiene vía
/// [`PyRuntime::register_fragment`] (que compila los fuentes primero).
pub struct PyFragment {
    runtime: PyRuntime,
    model: String,
    module: String,
    extension: bool,
    methods: Vec<String>,
}

impl PyFragment {
    pub(crate) fn new(runtime: PyRuntime, spec: PyModelSpec) -> Self {
        PyFragment {
            runtime,
            methods: spec.methods.iter().map(|m| m.name.clone()).collect(),
            model: spec.model,
            module: spec.module,
            extension: spec.extension,
        }
    }
}

#[async_trait]
impl ModelFragment for PyFragment {
    fn model_name(&self) -> &str {
        &self.model
    }

    fn module(&self) -> &str {
        &self.module
    }

    fn is_extension(&self) -> bool {
        self.extension
    }

    fn methods(&self) -> Vec<&str> {
        self.methods.iter().map(|s| s.as_str()).collect()
    }

    async fn call(
        &self,
        _env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        let key = state::method_key(&self.model, &self.module, ctx.method());
        self.runtime
            .dispatch(key, rs.clone(), ctx.clone(), args.to_vec())
            .await
    }
}
