//! # nexus-py — el «seguro de vida»: Python embebido sobre nexus-orm
//!
//! FASE 3 del plan odoo2rs (`docs/transpilador-odoo-a-rust.md`). Los métodos
//! de negocio de Odoo que el transpilador no puede traducir mecánicamente
//! se ejecutan TAL CUAL, en un mini-intérprete RustPython compilado dentro
//! del binario, con acceso al `Env` y la transacción de nexus-orm: el
//! `self.env` de Python y el `Env` de Rust son EL MISMO objeto (misma
//! caché de registros, misma conexión).
//!
//! ## Piezas
//!
//! | Pieza            | Rol                                                       |
//! |------------------|-----------------------------------------------------------|
//! | [`PyRuntime`]    | Hilo dedicado que posee el intérprete; canal = GIL        |
//! | [`PyFragment`]   | `ModelFragment` con métodos Python — entra a la vtable    |
//! |                  | `_inherit` del Registry igual que un fragmento Rust       |
//! | `_nexus`         | Módulo nativo: syscalls Python → ORM (get/set/call/…)     |
//! | `bootstrap.py`   | `Env`/`Recordset`/`UserError` con la ergonomía de Odoo    |
//!
//! ## Ejemplo
//!
//! ```rust,no_run
//! use std::sync::Arc;
//! use nexus_orm::prelude::*;
//! use nexus_py::{PyMethod, PyModelSpec, PyRuntime};
//!
//! # async fn demo() -> OResult<()> {
//! let py = PyRuntime::new()?;
//!
//! // Un método Odoo intraducible, tal cual, en Python:
//! let frag = py.register_fragment(PyModelSpec {
//!     model: "demo.task".into(),
//!     module: "demo".into(),
//!     extension: true,
//!     methods: vec![PyMethod::new("action_done", r#"
//! def action_done(self):
//!     for task in self:
//!         if not task.name:
//!             raise UserError("Tarea sin nombre")
//!         task.state = "done"
//!     return True
//! "#)],
//! }).await?;
//!
//! let registry = Arc::new(
//!     RegistryBuilder::new()
//!         .module("demo", &[])
//!         .register_ir_json(r#"{
//!             "model": "demo.task", "module": "demo",
//!             "fields": [{"name": "name", "type": "char"},
//!                        {"name": "state", "type": "char"}]
//!         }"#)?
//!         .register(frag)
//!         .build()?,
//! );
//!
//! let env = Env::mock(registry);
//! env.seed("demo.task", 1, vec![("name", "Migrar".into()), ("state", "open".into())])?;
//! let task = env.browse("demo.task", vec![1])?;
//! task.call("action_done", &[]).await?;          // despacho normal del ORM
//! assert_eq!(task.get_str("state")?, "done");    // lo escribió Python
//! # Ok(())
//! # }
//! ```
//!
//! ## Fronteras v0 (deliberadas)
//!
//! - Un solo hilo intérprete por `PyRuntime`: la ejecución Python se
//!   serializa (como el GIL de CPython). Crear un `PyRuntime` por proceso.
//! - `Decimal` cruza a Python como `float`; la persistencia Monetary del
//!   lado Rust sigue en `Decimal`.
//! - `Date`/`DateTime` cruzan como cadenas ISO.
//! - Sin stdlib de Python (sin `import json`, etc.): los métodos de negocio
//!   de Odoo usan el ORM, no la stdlib; se puede habilitar después.
//! - `super().metodo(...)` se escribe `self.super_(...)` (reescritura del
//!   transpilador).

mod module;
mod state;

pub mod fragment;
pub mod runtime;

pub use fragment::{PyFragment, PyMethod, PyModelSpec};
pub use runtime::PyRuntime;
