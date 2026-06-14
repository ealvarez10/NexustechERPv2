//! nexus-py: el módulo de extensión `_nexus` (PyO3 + CPython).
//!
//! Implementa el contrato ABI que el shim `odoo` espera para comunicarse
//! con el kernel de Rust y la base de datos PostgreSQL real.

use pyo3::exceptions::{PyKeyError, PyNotImplementedError, PyValueError, PyTypeError, PyRuntimeError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyBool, PyFloat, PyInt, PyString};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock, Arc};
use sqlx::PgPool;
use nexus_orm::registry::Registry;
use nexus_orm::value::OVal;

// ---------------------------------------------------------------------------
// Estado del kernel durante la fase de registro y pool de BD
// ---------------------------------------------------------------------------

#[derive(Default)]
struct RegistryState {
    /// orden de declaración de módulos
    module_order: Vec<String>,
    module_deps: HashMap<String, Vec<String>>,
    /// (fragment_id, model, module)
    fragments: Vec<(u64, String, String)>,
    /// specs serializadas para la fase F2
    specs: HashMap<u64, String>,
    next_fragment_id: u64,
}

static STATE: Mutex<Option<RegistryState>> = Mutex::new(None);
static DB_POOL: OnceLock<PgPool> = OnceLock::new();
static REGISTRY: OnceLock<Arc<Registry>> = OnceLock::new();
static HANDLES: OnceLock<Mutex<HashMap<u64, i64>>> = OnceLock::new();

pub fn init_db_pool(pool: PgPool) {
    let _ = DB_POOL.set(pool);
}

pub fn init_registry(registry: Arc<Registry>) {
    let _ = REGISTRY.set(registry);
}

fn with_state<R>(f: impl FnOnce(&mut RegistryState) -> R) -> R {
    let mut guard = STATE.lock().expect("nexus state poisoned");
    f(guard.get_or_insert_with(RegistryState::default))
}

fn get_env(handle: u64) -> PyResult<nexus_orm::env::Env> {
    let pool = DB_POOL.get().ok_or_else(|| PyNotImplementedError::new_err("Database pool not initialized"))?;
    let registry = REGISTRY.get().ok_or_else(|| PyNotImplementedError::new_err("Registry not initialized"))?;
    
    let uid = {
        let map = HANDLES.get_or_init(|| Mutex::new(HashMap::new())).lock().unwrap();
        *map.get(&handle).unwrap_or(&2) // Por defecto administrador (uid = 2)
    };
    
    let ctx = nexus_orm::env::EnvCtx {
        uid,
        company_id: Some(1),
        lang: "es_MX".to_string(),
        su: uid == 1 || uid == 2,
    };
    
    Ok(nexus_orm::env::Env::new(registry.clone(), pool.clone(), ctx))
}

fn py_to_oval(val: &Bound<'_, PyAny>) -> PyResult<OVal> {
    if val.is_none() {
        Ok(OVal::Null)
    } else if let Ok(b) = val.downcast::<PyBool>() {
        Ok(OVal::Bool(b.is_true()))
    } else if let Ok(i) = val.downcast::<PyInt>() {
        let n: i64 = i.extract()?;
        Ok(OVal::Int(n))
    } else if let Ok(f) = val.downcast::<PyFloat>() {
        let n: f64 = f.extract()?;
        Ok(OVal::Float(n))
    } else if let Ok(s) = val.downcast::<PyString>() {
        let st: String = s.extract()?;
        Ok(OVal::Str(st.into()))
    } else {
        let st: String = val.str()?.extract()?;
        Ok(OVal::Str(st.into()))
    }
}

fn serde_to_py_obj<'py>(py: Python<'py>, val: &serde_json::Value) -> PyResult<Bound<'py, PyAny>> {
    match val {
        serde_json::Value::Null => Ok(py.None().into_bound(py)),
        serde_json::Value::Bool(b) => Ok(b.into_py(py).into_bound(py)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_py(py).into_bound(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_py(py).into_bound(py))
            } else {
                Ok(py.None().into_bound(py))
            }
        }
        serde_json::Value::String(s) => Ok(PyString::new_bound(py, s).into_any()),
        serde_json::Value::Array(arr) => {
            let list = PyList::empty_bound(py);
            for item in arr {
                list.append(serde_to_py_obj(py, item)?)?;
            }
            Ok(list.into_any())
        }
        serde_json::Value::Object(obj) => {
            let dict = PyDict::new_bound(py);
            for (k, v) in obj {
                dict.set_item(k, serde_to_py_obj(py, v)?)?;
            }
            Ok(dict.into_any())
        }
    }
}

fn oval_to_py<'py>(py: Python<'py>, val: &OVal) -> PyResult<Bound<'py, PyAny>> {
    match val {
        OVal::Null => Ok(py.None().into_bound(py)),
        OVal::Bool(b) => Ok(b.into_py(py).into_bound(py)),
        OVal::Int(i) => Ok(i.into_py(py).into_bound(py)),
        OVal::Float(f) => Ok(f.into_py(py).into_bound(py)),
        OVal::Decimal(d) => {
            let s = d.to_string();
            Ok(PyString::new_bound(py, &s).into_any())
        }
        OVal::Str(s) => {
            Ok(PyString::new_bound(py, s.as_str()).into_any())
        }
        OVal::Date(d) => {
            let s = d.format("%Y-%m-%d").to_string();
            Ok(PyString::new_bound(py, &s).into_any())
        }
        OVal::DateTime(dt) => {
            let s = dt.format("%Y-%m-%d %H:%M:%S").to_string();
            Ok(PyString::new_bound(py, &s).into_any())
        }
        OVal::Ref(_, id) => Ok(id.into_py(py).into_bound(py)),
        OVal::RefSet(_, ids) => {
            let list = PyList::new_bound(py, ids);
            Ok(list.into_any())
        }
        OVal::Json(v) => serde_to_py_obj(py, v),
    }
}

// ---------------------------------------------------------------------------
// Fase de registro
// ---------------------------------------------------------------------------

#[pyfunction]
fn declare_module(name: String, depends: Vec<String>) -> PyResult<()> {
    with_state(|s| {
        if !s.module_order.contains(&name) {
            s.module_order.push(name.clone());
        }
        s.module_deps.insert(name, depends);
    });
    Ok(())
}

#[pyfunction]
fn register_fragment(py: Python<'_>, spec: Bound<'_, PyDict>) -> PyResult<u64> {
    let model: String = spec
        .get_item("model")?
        .ok_or_else(|| PyKeyError::new_err("spec sin 'model'"))?
        .extract()?;
    let module: String = spec
        .get_item("module")?
        .map(|m| m.extract())
        .transpose()?
        .unwrap_or_else(|| "base".to_string());

    let json = py.import_bound("json")?;
    let spec_json: String = json
        .call_method1("dumps", (spec,))?
        .extract()?;

    let fid = with_state(|s| {
        s.next_fragment_id += 1;
        let fid = s.next_fragment_id;
        s.fragments.push((fid, model, module));
        s.specs.insert(fid, spec_json);
        fid
    });
    Ok(fid)
}

#[pyfunction]
fn finalize(py: Python<'_>) -> PyResult<Py<PyDict>> {
    let plan = with_state(|s| {
        let rank: HashMap<&str, usize> = s
            .module_order
            .iter()
            .enumerate()
            .map(|(i, m)| (m.as_str(), i))
            .collect();

        let mut ordered = s.fragments.clone();
        ordered.sort_by_key(|(fid, _, module)| {
            (*rank.get(module.as_str()).unwrap_or(&usize::MAX), *fid)
        });

        let mut per_model: HashMap<String, Vec<u64>> = HashMap::new();
        for (fid, model, _) in ordered {
            per_model.entry(model).or_default().push(fid);
        }
        for ids in per_model.values_mut() {
            ids.reverse();
        }
        per_model
    });

    let dict = PyDict::new_bound(py);
    for (model, ids) in plan {
        dict.set_item(model, PyList::new_bound(py, ids))?;
    }
    Ok(dict.into())
}

#[pyfunction]
fn load_module_data(_name: String, _path: String, _data_files: Vec<String>) -> PyResult<()> {
    Ok(())
}

// ---------------------------------------------------------------------------
// Entorno y CRUD
// ---------------------------------------------------------------------------

#[pyfunction]
fn env_new(uid: u32, _context: Bound<'_, PyDict>) -> PyResult<u64> {
    static NEXT_HANDLE: Mutex<u64> = Mutex::new(1);
    let mut next = NEXT_HANDLE.lock().unwrap();
    let handle = *next;
    *next += 1;
    
    let mut map = HANDLES.get_or_init(|| Mutex::new(HashMap::new())).lock().unwrap();
    map.insert(handle, uid as i64);
    
    Ok(handle)
}

#[pyfunction]
fn create(
    py: Python<'_>,
    handle: u64,
    model: String,
    vals_list: Bound<'_, PyList>,
) -> PyResult<Vec<i64>> {
    let env = get_env(handle)?;
    let mut new_ids = Vec::new();
    
    for item in vals_list.iter() {
        let vals_dict: Bound<'_, PyDict> = item.downcast().map_err(|_| {
            PyTypeError::new_err("vals_list debe contener diccionarios")
        })?.clone();
        
        let mut rust_vals = Vec::new();
        let mut o2m_vals = Vec::new();
        
        for (k, v) in vals_dict.iter() {
            let key: String = k.extract()?;
            if let Ok(lst) = v.downcast::<PyList>() {
                if lst.len() > 0 {
                    o2m_vals.push((key, lst.clone()));
                    continue;
                }
            }
            
            let oval = py_to_oval(&v)?;
            rust_vals.push((key, oval));
        }
        
        let recordset = tokio::runtime::Handle::current().block_on(async {
            env.create(&model, rust_vals).await
        }).map_err(|e| PyRuntimeError::new_err(format!("Error en create: {:?}", e)))?;
        
        let parent_id = recordset.ids()[0];
        new_ids.push(parent_id);
        
        for (field_name, lst) in o2m_vals {
            apply_o2m_rust(py, handle, &model, &field_name, parent_id, &lst)?;
        }
    }
    
    Ok(new_ids)
}

fn apply_o2m_rust(
    py: Python<'_>,
    handle: u64,
    model: &str,
    field_name: &str,
    parent_id: i64,
    commands: &Bound<'_, PyList>,
) -> PyResult<()> {
    let registry = REGISTRY.get().ok_or_else(|| PyNotImplementedError::new_err("Registry not initialized"))?;
    let mid = registry.model_id(model)
        .map_err(|e| PyValueError::new_err(format!("Modelo desconocido {}: {:?}", model, e)))?;
    let def = registry.def(mid);
    let fdef = def.field(field_name)
        .map_err(|e| PyValueError::new_err(format!("Campo desconocido {}.{}: {:?}", model, field_name, e)))?;
    
    let (comodel, inverse) = match &fdef.ftype {
        nexus_orm::fields::FieldType::One2many { comodel, inverse } => (comodel, inverse),
        _ => return Err(PyValueError::new_err(format!("Campo {}.{} no es One2many", model, field_name))),
    };
    
    for command in commands.iter() {
        let cmd_list: Bound<'_, PyList> = command.downcast().map_err(|_| {
            PyTypeError::new_err("comandos de o2m deben ser listas/tuplas")
        })?.clone();
        
        let op: u32 = cmd_list.get_item(0)?.extract()?;
        match op {
            0 => {
                let vals: Bound<'_, PyDict> = cmd_list.get_item(2)?.downcast()?.clone();
                vals.set_item(inverse, parent_id)?;
                
                let vals_list = PyList::new_bound(py, vec![vals]);
                create(py, handle, comodel.to_string(), vals_list)?;
            }
            1 => {
                let sub_id: i64 = cmd_list.get_item(1)?.extract()?;
                let vals: Bound<'_, PyDict> = cmd_list.get_item(2)?.downcast()?.clone();
                
                write(py, handle, comodel.to_string(), vec![sub_id], vals)?;
            }
            2 => {
                let sub_id: i64 = cmd_list.get_item(1)?.extract()?;
                unlink(handle, comodel.to_string(), vec![sub_id])?;
            }
            3 => {
                let sub_id: i64 = cmd_list.get_item(1)?.extract()?;
                let vals = PyDict::new_bound(py);
                vals.set_item(inverse, py.None())?;
                write(py, handle, comodel.to_string(), vec![sub_id], vals)?;
            }
            4 => {
                let sub_id: i64 = cmd_list.get_item(1)?.extract()?;
                let vals = PyDict::new_bound(py);
                vals.set_item(inverse, parent_id)?;
                write(py, handle, comodel.to_string(), vec![sub_id], vals)?;
            }
            5 => {
                let env = get_env(handle)?;
                let query_d = nexus_orm::domain::Domain::leaf(inverse, "=", parent_id)
                    .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
                
                let recordset = tokio::runtime::Handle::current().block_on(async {
                    env.search(comodel, &query_d, None, None).await
                }).map_err(|e| PyRuntimeError::new_err(format!("{:?}", e)))?;
                
                let ids = recordset.ids().to_vec();
                if !ids.is_empty() {
                    let vals = PyDict::new_bound(py);
                    vals.set_item(inverse, py.None())?;
                    write(py, handle, comodel.to_string(), ids, vals)?;
                }
            }
            6 => {
                let ids_to_link: Vec<i64> = cmd_list.get_item(2)?.extract()?;
                let env = get_env(handle)?;
                let query_d = nexus_orm::domain::Domain::leaf(inverse, "=", parent_id)
                    .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
                
                let recordset = tokio::runtime::Handle::current().block_on(async {
                    env.search(comodel, &query_d, None, None).await
                }).map_err(|e| PyRuntimeError::new_err(format!("{:?}", e)))?;
                
                let current_ids = recordset.ids();
                let ids_to_clear: Vec<i64> = current_ids.iter()
                    .filter(|&id| !ids_to_link.contains(id))
                    .cloned()
                    .collect();
                
                if !ids_to_clear.is_empty() {
                    let vals = PyDict::new_bound(py);
                    vals.set_item(inverse, py.None())?;
                    write(py, handle, comodel.to_string(), ids_to_clear, vals)?;
                }
                
                if !ids_to_link.is_empty() {
                    let vals = PyDict::new_bound(py);
                    vals.set_item(inverse, parent_id)?;
                    write(py, handle, comodel.to_string(), ids_to_link, vals)?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

#[pyfunction]
fn write(
    py: Python<'_>,
    handle: u64,
    model: String,
    ids: Vec<i64>,
    vals: Bound<'_, PyDict>,
) -> PyResult<bool> {
    let env = get_env(handle)?;
    
    let mut rust_vals = Vec::new();
    let mut o2m_vals = Vec::new();
    
    for (k, v) in vals.iter() {
        let key: String = k.extract()?;
        if let Ok(lst) = v.downcast::<PyList>() {
            if lst.len() > 0 {
                o2m_vals.push((key, lst.clone()));
                continue;
            }
        }
        
        let oval = py_to_oval(&v)?;
        rust_vals.push((key, oval));
    }
    
    if !rust_vals.is_empty() {
        let recordset = env.browse(&model, ids.clone())
            .map_err(|e| PyValueError::new_err(format!("Error en browse: {:?}", e)))?;
            
        tokio::runtime::Handle::current().block_on(async {
            env.write_values(&recordset, &rust_vals).await
        }).map_err(|e| PyRuntimeError::new_err(format!("Error en write: {:?}", e)))?;
    }
    
    for (field_name, lst) in o2m_vals {
        for &parent_id in &ids {
            apply_o2m_rust(py, handle, &model, &field_name, parent_id, &lst)?;
        }
    }
    
    Ok(true)
}

#[pyfunction]
#[allow(unused_variables)]
fn unlink(
    handle: u64,
    model: String,
    ids: Vec<i64>,
) -> PyResult<bool> {
    let pool = DB_POOL.get().ok_or_else(|| PyNotImplementedError::new_err("Database pool not initialized"))?;
    let table = model.replace('.', "_");
    
    if ids.is_empty() {
        return Ok(true);
    }
    let ids_list = ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let sql = format!("DELETE FROM {} WHERE id IN ({})", table, ids_list);
    
    tokio::runtime::Handle::current().block_on(async {
        sqlx::query(&sql).execute(pool).await
    }).map_err(|e| PyRuntimeError::new_err(format!("Error en unlink: {:?}", e)))?;
    
    Ok(true)
}

#[pyfunction]
fn read_batch(
    py: Python<'_>,
    handle: u64,
    model: String,
    ids: Vec<i64>,
    fields: Vec<String>,
) -> PyResult<Py<PyDict>> {
    let env = get_env(handle)?;
    let registry = REGISTRY.get().ok_or_else(|| PyNotImplementedError::new_err("Registry not initialized"))?;
    let mid = registry.model_id(&model)
        .map_err(|e| PyValueError::new_err(format!("Modelo desconocido {}: {:?}", model, e)))?;
    
    if !ids.is_empty() && !fields.is_empty() {
        tokio::runtime::Handle::current().block_on(async {
            env.fetch_into_cache(mid, &ids, &fields).await
        }).map_err(|e| PyRuntimeError::new_err(format!("Error en fetch_into_cache: {:?}", e)))?;
    }
    
    let dict = PyDict::new_bound(py);
    for &id in &ids {
        let row_dict = PyDict::new_bound(py);
        for field in &fields {
            if let Some(oval) = env.cache_get(mid, id, field) {
                let py_val = oval_to_py(py, &oval)?;
                row_dict.set_item(field, py_val)?;
            } else {
                row_dict.set_item(field, py.None())?;
            }
        }
        dict.set_item(id, row_dict)?;
    }
    
    Ok(dict.into())
}

#[pyfunction]
#[allow(unused_variables)]
#[pyo3(signature = (handle, model, domain, offset=0, limit=None, order=None))]
fn search(
    py: Python<'_>,
    handle: u64,
    model: String,
    domain: Bound<'_, PyList>,
    offset: i64,
    limit: Option<i64>,
    order: Option<String>,
) -> PyResult<Vec<i64>> {
    let env = get_env(handle)?;
    
    let json_module = py.import_bound("json")?;
    let domain_json: String = json_module
        .call_method1("dumps", (domain,))?
        .extract()?;
    
    let d = nexus_orm::domain::Domain::parse_json(&domain_json)
        .map_err(|e| PyValueError::new_err(format!("Error parsing domain: {:?}", e)))?;
    
    let recordset = tokio::runtime::Handle::current().block_on(async {
        env.search(&model, &d, limit, Some(offset)).await
    }).map_err(|e| PyRuntimeError::new_err(format!("Error en search: {:?}", e)))?;
    
    Ok(recordset.ids().to_vec())
}

#[pyfunction]
fn search_count(
    py: Python<'_>,
    handle: u64,
    model: String,
    domain: Bound<'_, PyList>,
) -> PyResult<i64> {
    let env = get_env(handle)?;
    
    let json_module = py.import_bound("json")?;
    let domain_json: String = json_module
        .call_method1("dumps", (domain,))?
        .extract()?;
    
    let d = nexus_orm::domain::Domain::parse_json(&domain_json)
        .map_err(|e| PyValueError::new_err(format!("Error parsing domain: {:?}", e)))?;
    
    let recordset = tokio::runtime::Handle::current().block_on(async {
        env.search(&model, &d, None, None).await
    }).map_err(|e| PyRuntimeError::new_err(format!("Error en search_count: {:?}", e)))?;
    
    Ok(recordset.ids().len() as i64)
}

#[pyfunction]
#[allow(unused_variables)]
fn exists(
    handle: u64,
    model: String,
    ids: Vec<i64>,
) -> PyResult<Vec<i64>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let pool = DB_POOL.get().ok_or_else(|| PyNotImplementedError::new_err("Database pool not initialized"))?;
    let table = model.replace('.', "_");
    let ids_list = ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let sql = format!("SELECT id FROM {} WHERE id IN ({})", table, ids_list);
    
    let rows = tokio::runtime::Handle::current().block_on(async {
        sqlx::query(&sql)
            .fetch_all(pool)
            .await
    }).map_err(|e| PyRuntimeError::new_err(format!("Error en exists: {:?}", e)))?;
    
    let mut existing = Vec::new();
    for row in rows {
        use sqlx::Row;
        let id: i32 = row.try_get("id").unwrap_or(0);
        existing.push(id as i64);
    }
    
    Ok(existing)
}

#[pyfunction]
fn has_native(_model: String, _method: String) -> PyResult<bool> {
    Ok(false)
}

#[pyfunction]
fn call_native(
    _handle: u64,
    _model: String,
    _method: String,
    _ids: Vec<i64>,
    _args: Bound<'_, PyList>,
    _kwargs: Bound<'_, PyDict>,
) -> PyResult<PyObject> {
    Err(not_implemented("call_native"))
}

#[pyfunction]
#[allow(unused_variables)]
fn cr_execute(
    py: Python<'_>,
    handle: u64,
    sql: String,
    params: Bound<'_, PyList>,
) -> PyResult<Py<PyDict>> {
    let pool = DB_POOL.get().ok_or_else(|| PyNotImplementedError::new_err("Database pool not initialized"))?;
    
    let mut rust_params = Vec::new();
    for p in params.iter() {
        rust_params.push(py_to_oval(&p)?);
    }
    
    let result = tokio::runtime::Handle::current().block_on(async {
        let mut q = sqlx::query(&sql);
        for p in &rust_params {
            q = match p {
                OVal::Null => q.bind::<Option<String>>(None),
                OVal::Bool(b) => q.bind(b),
                OVal::Int(i) => q.bind(i),
                OVal::Float(f) => q.bind(f),
                OVal::Decimal(d) => q.bind(d),
                OVal::Str(s) => q.bind(s.as_str()),
                _ => q.bind(p.to_string()),
            };
        }
        q.fetch_all(pool).await
    }).map_err(|e| PyRuntimeError::new_err(format!("Error en cr_execute: {:?}", e)))?;
    
    let rows_list = PyList::empty_bound(py);
    for row in &result {
        use sqlx::Row;
        let mut row_vals = Vec::new();
        for i in 0..row.len() {
            let oval = if let Ok(v) = row.try_get::<Option<String>, _>(i) {
                v.map(|s| OVal::Str(s.into())).unwrap_or(OVal::Null)
            } else if let Ok(v) = row.try_get::<Option<i64>, _>(i) {
                v.map(OVal::Int).unwrap_or(OVal::Null)
            } else if let Ok(v) = row.try_get::<Option<i32>, _>(i) {
                v.map(|n| OVal::Int(n as i64)).unwrap_or(OVal::Null)
            } else if let Ok(v) = row.try_get::<Option<bool>, _>(i) {
                v.map(OVal::Bool).unwrap_or(OVal::Null)
            } else if let Ok(v) = row.try_get::<Option<f64>, _>(i) {
                v.map(OVal::Float).unwrap_or(OVal::Null)
            } else if let Ok(v) = row.try_get::<Option<serde_json::Value>, _>(i) {
                v.map(OVal::Json).unwrap_or(OVal::Null)
            } else {
                OVal::Null
            };
            row_vals.push(oval_to_py(py, &oval)?);
        }
        let row_tuple = pyo3::types::PyTuple::new_bound(py, row_vals);
        rows_list.append(row_tuple)?;
    }
    
    let desc_list = PyList::empty_bound(py);
    if let Some(first_row) = result.first() {
        use sqlx::Row;
        use sqlx::Column;
        for i in 0..first_row.len() {
            let col_name = first_row.column(i).name();
            let col_tuple = pyo3::types::PyTuple::new_bound(py, vec![
                col_name.into_py(py),
                py.None(),
                py.None(),
                py.None(),
                py.None(),
                py.None(),
                py.None(),
            ]);
            desc_list.append(col_tuple)?;
        }
    }
    
    let res_dict = PyDict::new_bound(py);
    res_dict.set_item("rows", rows_list)?;
    res_dict.set_item("description", desc_list)?;
    res_dict.set_item("rowcount", result.len())?;
    
    Ok(res_dict.into())
}

#[pyfunction]
fn cr_commit(_handle: u64) -> PyResult<()> {
    Ok(())
}

#[pyfunction]
fn cr_rollback(_handle: u64) -> PyResult<()> {
    Ok(())
}

#[pyfunction]
fn cr_savepoint(_handle: u64) -> PyResult<String> {
    Ok("sp_savepoint".to_string())
}

#[pyfunction]
fn cr_rollback_to(_handle: u64, _name: String) -> PyResult<()> {
    Ok(())
}

#[pyfunction]
fn cr_release(_handle: u64, _name: String) -> PyResult<()> {
    Ok(())
}

#[pyfunction]
fn xmlid_lookup(_handle: u64, _xml_id: String) -> PyResult<Option<(String, i64)>> {
    Ok(None)
}

#[pyfunction]
#[pyo3(signature = (_handle, _model=None, _ids=None))]
fn invalidate(_handle: u64, _model: Option<String>, _ids: Option<Vec<i64>>) -> PyResult<()> {
    Ok(())
}

#[pyfunction]
fn flush(_handle: u64) -> PyResult<()> {
    Ok(())
}

#[pyfunction]
fn reset() -> PyResult<()> {
    *STATE.lock().expect("nexus state poisoned") = None;
    Ok(())
}

fn not_implemented(what: &str) -> PyErr {
    PyNotImplementedError::new_err(format!(
        "{what}: pendiente de integración"
    ))
}

#[pymodule]
pub fn _nexus(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(declare_module, m)?)?;
    m.add_function(wrap_pyfunction!(register_fragment, m)?)?;
    m.add_function(wrap_pyfunction!(finalize, m)?)?;
    m.add_function(wrap_pyfunction!(load_module_data, m)?)?;
    m.add_function(wrap_pyfunction!(env_new, m)?)?;
    m.add_function(wrap_pyfunction!(create, m)?)?;
    m.add_function(wrap_pyfunction!(write, m)?)?;
    m.add_function(wrap_pyfunction!(unlink, m)?)?;
    m.add_function(wrap_pyfunction!(read_batch, m)?)?;
    m.add_function(wrap_pyfunction!(search, m)?)?;
    m.add_function(wrap_pyfunction!(search_count, m)?)?;
    m.add_function(wrap_pyfunction!(exists, m)?)?;
    m.add_function(wrap_pyfunction!(has_native, m)?)?;
    m.add_function(wrap_pyfunction!(call_native, m)?)?;
    m.add_function(wrap_pyfunction!(cr_execute, m)?)?;
    m.add_function(wrap_pyfunction!(cr_commit, m)?)?;
    m.add_function(wrap_pyfunction!(cr_rollback, m)?)?;
    m.add_function(wrap_pyfunction!(cr_savepoint, m)?)?;
    m.add_function(wrap_pyfunction!(cr_rollback_to, m)?)?;
    m.add_function(wrap_pyfunction!(cr_release, m)?)?;
    m.add_function(wrap_pyfunction!(xmlid_lookup, m)?)?;
    m.add_function(wrap_pyfunction!(invalidate, m)?)?;
    m.add_function(wrap_pyfunction!(flush, m)?)?;
    m.add_function(wrap_pyfunction!(reset, m)?)?;
    Ok(())
}

/// Función pública auxiliar para el binario Axum (Rust).
pub fn build_registry_from_state() -> Result<nexus_orm::registry::Registry, nexus_orm::error::OError> {
    with_state(|s| {
        let mut builder = nexus_orm::registry::RegistryBuilder::new();

        for m_name in &s.module_order {
            let deps = s.module_deps.get(m_name).cloned().unwrap_or_default();
            let deps_refs: Vec<&str> = deps.iter().map(|d| d.as_str()).collect();
            builder = builder.module(m_name, &deps_refs);
        }

        let rank: HashMap<&str, usize> = s
            .module_order
            .iter()
            .enumerate()
            .map(|(i, m)| (m.as_str(), i))
            .collect();

        let mut ordered_frags = s.fragments.clone();
        ordered_frags.sort_by_key(|(fid, _, module)| {
            (*rank.get(module.as_str()).unwrap_or(&usize::MAX), *fid)
        });

        for (fid, model_name, _) in ordered_frags {
            if let Some(spec_json) = s.specs.get(&fid) {
                let mut val: serde_json::Value = serde_json::from_str(spec_json)
                    .map_err(|e| nexus_orm::error::OError::Registry(format!("JSON inválido para {}: {}", model_name, e)))?;

                if let Some(fields_val) = val.get_mut("fields") {
                    if fields_val.is_object() {
                        let fields_obj = fields_val.as_object_mut().unwrap();
                        let mut fields_arr = Vec::new();
                        for (field_name, field_val) in fields_obj.iter_mut() {
                            if let Some(field_map) = field_val.as_object_mut() {
                                field_map.insert("name".to_string(), serde_json::Value::String(field_name.clone()));
                                if let Some(inv_name) = field_map.remove("inverse_name") {
                                    field_map.insert("inverse".to_string(), inv_name);
                                }
                                fields_arr.push(field_val.clone());
                            }
                        }
                        *fields_val = serde_json::Value::Array(fields_arr);
                    }
                }

                if let Some(inherit_val) = val.get_mut("inherit") {
                    let has_inherit = if inherit_val.is_array() {
                        !inherit_val.as_array().unwrap().is_empty()
                    } else if inherit_val.is_string() {
                        !inherit_val.as_str().unwrap().is_empty()
                    } else {
                        inherit_val.as_bool().unwrap_or(false)
                    };
                    *inherit_val = serde_json::Value::Bool(has_inherit);
                }

                let wrapped_val = serde_json::Value::Array(vec![val]);
                let transformed_json = serde_json::to_string(&wrapped_val)
                    .map_err(|e| nexus_orm::error::OError::Registry(format!("Fallo al reserializar JSON para {}: {}", model_name, e)))?;

                match builder.register_ir_json(&transformed_json) {
                    Ok(new_builder) => builder = new_builder,
                    Err(e) => {
                        tracing::error!("Fallo al registrar IR JSON para modelo '{}': {:?}. Transformed JSON: {}", model_name, e, transformed_json);
                        return Err(e);
                    }
                }
            }
        }

        builder.build()
    })
}
