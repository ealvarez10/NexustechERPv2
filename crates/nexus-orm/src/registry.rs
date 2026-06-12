//! El Registry — registro dinámico de modelos en arranque (§3.2).
//!
//! Igual que Odoo construye sus clases combinando definiciones por orden
//! de carga de módulos, `RegistryBuilder` recibe fragmentos (generados por
//! el transpilador, escritos a mano, o parseados de IR JSON), ordena los
//! módulos topológicamente por `depends`, pliega los fragmentos en
//! `ModelDef`s y construye:
//!  - la **vtable encadenada** por método (más derivado primero) que
//!    reproduce el MRO, y
//!  - los **disparadores de recálculo** a partir de `@api.depends`.

use std::collections::HashMap;
use std::sync::Arc;

use smol_str::SmolStr;

use crate::error::{OError, OResult};
use crate::model::{ModelDef, ModelFragment};
use crate::recordset::Recordset;
use crate::value::{ModelId, OVal};

type MethodChain = Arc<Vec<Arc<dyn ModelFragment>>>;

/// Contexto de una llamada de método en curso: posición dentro de la
/// cadena `_inherit`. `call_super` es el `super()` de Python.
#[derive(Clone)]
pub struct CallCtx {
    method: SmolStr,
    chain: MethodChain,
    pos: usize,
}

impl CallCtx {
    pub(crate) fn new(method: &str, chain: MethodChain) -> Self {
        CallCtx {
            method: method.into(),
            chain,
            pos: 0,
        }
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    /// Invoca el siguiente fragmento de la cadena MRO (el `super()` de
    /// Python transpilado). Error si ya no hay más eslabones.
    pub async fn call_super(&self, rs: &Recordset, args: &[OVal]) -> OResult<OVal> {
        let next = self.pos + 1;
        match self.chain.get(next) {
            Some(frag) => {
                let ctx = CallCtx {
                    method: self.method.clone(),
                    chain: Arc::clone(&self.chain),
                    pos: next,
                };
                frag.call(rs.env(), &ctx, rs, args).await
            }
            None => Err(OError::key(format!(
                "super(): no hay más fragmentos en la cadena MRO para '{}'",
                self.method
            ))),
        }
    }
}

/// Entrada de un modelo ya resuelto.
pub struct ModelEntry {
    pub def: ModelDef,
    /// Fragmentos en orden de carga (base primero) — informativo.
    fragments: Vec<Arc<dyn ModelFragment>>,
    /// método → cadena de fragmentos que lo implementan, más derivado primero.
    methods: HashMap<SmolStr, MethodChain>,
    /// campo escrito → métodos compute a disparar (dependencias intra-modelo).
    pub recompute_triggers: HashMap<String, Vec<String>>,
    /// Dependencias con ruta (`order_line.price_subtotal`) — cross-model,
    /// registradas pero aún sin cablear en v0.
    pub deferred_triggers: Vec<(String, String)>,
}

impl ModelEntry {
    pub fn method_chain(&self, method: &str) -> Option<MethodChain> {
        self.methods.get(method).cloned()
    }

    pub fn fragment_count(&self) -> usize {
        self.fragments.len()
    }
}

/// El registro global de modelos, inmutable tras `build()` (se comparte
/// vía `Arc` entre el Env, los handlers y —en el futuro— nexus-pyvm).
pub struct Registry {
    entries: Vec<ModelEntry>,
    by_name: HashMap<String, ModelId>,
}

impl std::fmt::Debug for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut nombres: Vec<&str> = self.by_name.keys().map(|s| s.as_str()).collect();
        nombres.sort_unstable();
        f.debug_struct("Registry").field("models", &nombres).finish()
    }
}

impl Registry {
    pub fn model_id(&self, name: &str) -> OResult<ModelId> {
        self.by_name
            .get(name)
            .copied()
            .ok_or_else(|| OError::UnknownModel(name.to_string()))
    }

    pub fn entry(&self, id: ModelId) -> &ModelEntry {
        &self.entries[id.index()]
    }

    pub fn def(&self, id: ModelId) -> &ModelDef {
        &self.entries[id.index()].def
    }

    pub fn model_names(&self) -> impl Iterator<Item = &str> {
        self.by_name.keys().map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn method_chain(&self, model: ModelId, method: &str) -> OResult<MethodChain> {
        self.entry(model).method_chain(method).ok_or_else(|| {
            OError::key(format!(
                "el modelo '{}' no tiene un método '{}'",
                self.def(model).name,
                method
            ))
        })
    }
}

struct ModuleDecl {
    name: String,
    depends: Vec<String>,
}

/// Constructor del Registry: acumula declaraciones de módulos y fragmentos
/// y resuelve todo en `build()` (la fase de «loading registry» de Odoo).
#[derive(Default)]
pub struct RegistryBuilder {
    modules: Vec<ModuleDecl>,
    fragments: Vec<Arc<dyn ModelFragment>>,
}

impl RegistryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Declara un módulo y sus dependencias (el `depends` del
    /// `__manifest__.py`). El orden de carga final es topológico.
    pub fn module(mut self, name: &str, depends: &[&str]) -> Self {
        self.modules.push(ModuleDecl {
            name: name.to_string(),
            depends: depends.iter().map(|d| d.to_string()).collect(),
        });
        self
    }

    /// Registra un fragmento (clase transpilada o manual).
    pub fn register(mut self, frag: Arc<dyn ModelFragment>) -> Self {
        self.fragments.push(frag);
        self
    }

    /// Registra modelos desde IR JSON (la salida declarativa de la FASE 2
    /// del transpilador). Acepta un objeto modelo o una lista de ellos.
    pub fn register_ir_json(self, json: &str) -> OResult<Self> {
        let irs = crate::ir::parse_ir(json)?;
        let mut b = self;
        for ir in irs {
            b.fragments.push(Arc::new(crate::ir::IrFragment::new(ir)));
        }
        Ok(b)
    }

    /// Resuelve orden de módulos, pliega fragmentos y construye el Registry.
    pub fn build(self) -> OResult<Registry> {
        let ranks = Self::module_ranks(&self.modules)?;
        let unknown_rank = self.modules.len();

        // Orden de carga: rank del módulo; sort estable preserva el orden
        // de registro dentro del mismo módulo.
        let mut frags = self.fragments;
        frags.sort_by_key(|f| ranks.get(f.module()).copied().unwrap_or(unknown_rank));

        // Agrupar por modelo preservando el orden de primera aparición.
        let mut order: Vec<String> = Vec::new();
        let mut groups: HashMap<String, Vec<Arc<dyn ModelFragment>>> = HashMap::new();
        for f in frags {
            let name = f.model_name().to_string();
            groups.entry(name.clone()).or_insert_with(|| {
                order.push(name.clone());
                Vec::new()
            });
            groups.get_mut(&name).unwrap().push(f);
        }

        let mut entries = Vec::with_capacity(order.len());
        let mut by_name = HashMap::with_capacity(order.len());

        for (idx, name) in order.iter().enumerate() {
            let group = groups.remove(name).unwrap();

            if group.iter().all(|f| f.is_extension()) {
                return Err(OError::Registry(format!(
                    "el modelo '{name}' solo tiene fragmentos _inherit; \
                     ningún módulo lo define como base"
                )));
            }

            // Plegar definiciones en orden de carga (base primero).
            let mut def = ModelDef::new(name);
            for f in &group {
                f.build(&mut def);
            }

            // Vtable: más derivado primero (orden de carga invertido).
            let mut methods: HashMap<SmolStr, Vec<Arc<dyn ModelFragment>>> = HashMap::new();
            for f in group.iter().rev() {
                for m in f.methods() {
                    methods
                        .entry(m.into())
                        .or_default()
                        .push(Arc::clone(f));
                }
            }
            let methods: HashMap<SmolStr, MethodChain> = methods
                .into_iter()
                .map(|(k, v)| (k, Arc::new(v)))
                .collect();

            // Grafo de recálculo desde @api.depends.
            let mut recompute_triggers: HashMap<String, Vec<String>> = HashMap::new();
            let mut deferred_triggers = Vec::new();
            for fdef in def.fields.values() {
                if let Some(c) = &fdef.compute {
                    if !methods.contains_key(c.method.as_str()) {
                        return Err(OError::Registry(format!(
                            "'{name}.{}' declara compute='{}' pero ningún fragmento lo implementa",
                            fdef.name, c.method
                        )));
                    }
                    for dep in &c.depends {
                        if dep.contains('.') {
                            deferred_triggers.push((dep.clone(), c.method.clone()));
                        } else {
                            if !def.has_field(dep) {
                                return Err(OError::Registry(format!(
                                    "'{name}.{}' depende de un campo inexistente: '{dep}'",
                                    fdef.name
                                )));
                            }
                            let methods_for = recompute_triggers.entry(dep.clone()).or_default();
                            if !methods_for.contains(&c.method) {
                                methods_for.push(c.method.clone());
                            }
                        }
                    }
                }
            }

            by_name.insert(name.clone(), ModelId(idx as u32));
            entries.push(ModelEntry {
                def,
                fragments: group,
                methods,
                recompute_triggers,
                deferred_triggers,
            });
        }

        tracing::info!(
            modelos = entries.len(),
            "nexus-orm: registry construido"
        );
        Ok(Registry { entries, by_name })
    }

    /// Orden topológico de módulos por `depends` (Kahn determinista:
    /// respeta el orden de declaración entre módulos sin restricción mutua).
    fn module_ranks(modules: &[ModuleDecl]) -> OResult<HashMap<String, usize>> {
        let declared: HashMap<&str, usize> = modules
            .iter()
            .enumerate()
            .map(|(i, m)| (m.name.as_str(), i))
            .collect();

        let mut placed: HashMap<String, usize> = HashMap::new();
        let mut remaining: Vec<usize> = (0..modules.len()).collect();

        while !remaining.is_empty() {
            let before = remaining.len();
            remaining.retain(|&i| {
                let m = &modules[i];
                let ready = m.depends.iter().all(|d| {
                    // Dependencia no declarada (p. ej. 'base' implícito) se
                    // considera satisfecha.
                    !declared.contains_key(d.as_str()) || placed.contains_key(d)
                });
                if ready {
                    let rank = placed.len();
                    placed.insert(m.name.clone(), rank);
                    false
                } else {
                    true
                }
            });
            if remaining.len() == before {
                let ciclo: Vec<&str> = remaining
                    .iter()
                    .map(|&i| modules[i].name.as_str())
                    .collect();
                return Err(OError::Registry(format!(
                    "ciclo de dependencias entre módulos: {ciclo:?}"
                )));
            }
        }
        Ok(placed)
    }
}
