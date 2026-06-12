//! `_nexus` — el módulo nativo dentro del intérprete: cada función es una
//! «syscall» de Python hacia nexus-orm. Las operaciones async del ORM se
//! resuelven con [`crate::state::block_on`] (pollster + contexto tokio).

pub(crate) use _nexus::make_module;

#[rustpython_vm::pymodule]
mod _nexus {
    use nexus_orm::prelude::*;
    use rustpython_vm::builtins::PyStrRef;
    use rustpython_vm::{PyObjectRef, PyResult, VirtualMachine};

    use crate::state;

    // ─── Recordset ──────────────────────────────────────────────────────

    #[pyfunction]
    fn rs_ids(h: usize, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        let ids = state::handle_rs(vm, h)?.ids().to_vec();
        Ok(vm
            .ctx
            .new_list(ids.into_iter().map(|i| vm.ctx.new_int(i).into()).collect())
            .into())
    }

    #[pyfunction]
    fn rs_model(h: usize, vm: &VirtualMachine) -> PyResult<String> {
        Ok(state::handle_rs(vm, h)?.model_name().to_owned())
    }

    #[pyfunction]
    fn rs_browse(h: usize, ids: Vec<i64>, vm: &VirtualMachine) -> PyResult<usize> {
        let rs = state::handle_rs(vm, h)?;
        Ok(state::insert_handle(rs.browse(ids)))
    }

    #[pyfunction]
    fn rs_has_field(h: usize, name: PyStrRef, vm: &VirtualMachine) -> PyResult<bool> {
        Ok(state::handle_rs(vm, h)?.def().has_field(name.as_str()))
    }

    #[pyfunction]
    fn rs_has_method(h: usize, name: PyStrRef, vm: &VirtualMachine) -> PyResult<bool> {
        let rs = state::handle_rs(vm, h)?;
        Ok(rs
            .env()
            .registry()
            .entry(rs.model_id())
            .method_chain(name.as_str())
            .is_some())
    }

    #[pyfunction]
    fn rs_get(h: usize, field: PyStrRef, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        let rs = state::handle_rs(vm, h)?;
        let v = state::block_on(rs.get(field.as_str())).map_err(|e| state::oerr_to_py(vm, e))?;
        state::oval_to_py(vm, rs.env(), v)
    }

    #[pyfunction]
    fn rs_set(h: usize, field: PyStrRef, value: PyObjectRef, vm: &VirtualMachine) -> PyResult<()> {
        let rs = state::handle_rs(vm, h)?;
        let v = state::py_to_oval(vm, &value)?;
        state::block_on(rs.set(field.as_str(), v)).map_err(|e| state::oerr_to_py(vm, e))
    }

    #[pyfunction]
    fn rs_write(
        h: usize,
        fields: Vec<PyStrRef>,
        values: Vec<PyObjectRef>,
        vm: &VirtualMachine,
    ) -> PyResult<()> {
        let rs = state::handle_rs(vm, h)?;
        let mut vals = Vec::with_capacity(fields.len());
        for (f, v) in fields.iter().zip(values.iter()) {
            vals.push((f.as_str().to_owned(), state::py_to_oval(vm, v)?));
        }
        state::block_on(rs.write(vals)).map_err(|e| state::oerr_to_py(vm, e))
    }

    #[pyfunction]
    fn rs_read(h: usize, fields: Vec<PyStrRef>, vm: &VirtualMachine) -> PyResult<()> {
        let rs = state::handle_rs(vm, h)?;
        let refs: Vec<&str> = fields.iter().map(|f| f.as_str()).collect();
        state::block_on(rs.read(&refs)).map_err(|e| state::oerr_to_py(vm, e))
    }

    /// Despacho dinámico: entra a `Recordset::call`, que recorre la cadena
    /// `_inherit` — el método destino puede ser Rust o Python (reentrada).
    #[pyfunction]
    fn rs_call(
        h: usize,
        method: PyStrRef,
        args: Vec<PyObjectRef>,
        vm: &VirtualMachine,
    ) -> PyResult<PyObjectRef> {
        let rs = state::handle_rs(vm, h)?;
        let ovals: Vec<OVal> = args
            .iter()
            .map(|a| state::py_to_oval(vm, a))
            .collect::<PyResult<_>>()?;
        let v = state::block_on(rs.call(method.as_str(), &ovals))
            .map_err(|e| state::oerr_to_py(vm, e))?;
        state::oval_to_py(vm, rs.env(), v)
    }

    #[pyfunction]
    fn rs_union(a: usize, b: usize, vm: &VirtualMachine) -> PyResult<usize> {
        let (ra, rb) = (state::handle_rs(vm, a)?, state::handle_rs(vm, b)?);
        let r = ra.union(&rb).map_err(|e| state::oerr_to_py(vm, e))?;
        Ok(state::insert_handle(r))
    }

    #[pyfunction]
    fn rs_minus(a: usize, b: usize, vm: &VirtualMachine) -> PyResult<usize> {
        let (ra, rb) = (state::handle_rs(vm, a)?, state::handle_rs(vm, b)?);
        let r = ra.minus(&rb).map_err(|e| state::oerr_to_py(vm, e))?;
        Ok(state::insert_handle(r))
    }

    #[pyfunction]
    fn rs_intersect(a: usize, b: usize, vm: &VirtualMachine) -> PyResult<usize> {
        let (ra, rb) = (state::handle_rs(vm, a)?, state::handle_rs(vm, b)?);
        let r = ra.intersect(&rb).map_err(|e| state::oerr_to_py(vm, e))?;
        Ok(state::insert_handle(r))
    }

    #[pyfunction]
    fn rs_mapped(h: usize, field: PyStrRef, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        let rs = state::handle_rs(vm, h)?;
        match rs.mapped(field.as_str()).map_err(|e| state::oerr_to_py(vm, e))? {
            Mapped::Values(vs) => {
                let items = vs
                    .into_iter()
                    .map(|v| state::oval_to_py(vm, rs.env(), v))
                    .collect::<PyResult<Vec<_>>>()?;
                Ok(vm.ctx.new_list(items).into())
            }
            Mapped::Records(r) => state::tagged_rs(vm, r),
        }
    }

    #[pyfunction]
    fn rs_free(h: usize) {
        state::free_handle(h);
    }

    /// `super()` de Odoo: siguiente fragmento de la cadena MRO en curso.
    #[pyfunction]
    fn call_super(h: usize, args: Vec<PyObjectRef>, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        let ctx = state::current_frame(vm)?;
        let rs = state::handle_rs(vm, h)?;
        let ovals: Vec<OVal> = args
            .iter()
            .map(|a| state::py_to_oval(vm, a))
            .collect::<PyResult<_>>()?;
        let v = state::block_on(ctx.call_super(&rs, &ovals))
            .map_err(|e| state::oerr_to_py(vm, e))?;
        state::oval_to_py(vm, rs.env(), v)
    }

    // ─── Env ────────────────────────────────────────────────────────────

    #[pyfunction]
    fn env_model(name: PyStrRef, vm: &VirtualMachine) -> PyResult<usize> {
        let env = state::current_env(vm)?;
        let rs = env.model(name.as_str()).map_err(|e| state::oerr_to_py(vm, e))?;
        Ok(state::insert_handle(rs))
    }

    #[pyfunction]
    fn env_create(
        model: PyStrRef,
        fields: Vec<PyStrRef>,
        values: Vec<PyObjectRef>,
        vm: &VirtualMachine,
    ) -> PyResult<usize> {
        let env = state::current_env(vm)?;
        let mut vals = Vec::with_capacity(fields.len());
        for (f, v) in fields.iter().zip(values.iter()) {
            vals.push((f.as_str().to_owned(), state::py_to_oval(vm, v)?));
        }
        let rs = state::block_on(env.create(model.as_str(), vals))
            .map_err(|e| state::oerr_to_py(vm, e))?;
        Ok(state::insert_handle(rs))
    }

    /// `limit`/`offset` negativos = sin límite (evita Optional en la frontera).
    #[pyfunction]
    fn env_search(
        model: PyStrRef,
        domain: PyObjectRef,
        limit: i64,
        offset: i64,
        vm: &VirtualMachine,
    ) -> PyResult<usize> {
        let env = state::current_env(vm)?;
        let dj = state::py_to_json(vm, &domain)?;
        let dom = Domain::from_json(&dj).map_err(|e| state::oerr_to_py(vm, e))?;
        let rs = state::block_on(env.search(
            model.as_str(),
            &dom,
            (limit >= 0).then_some(limit),
            (offset >= 0).then_some(offset),
        ))
        .map_err(|e| state::oerr_to_py(vm, e))?;
        Ok(state::insert_handle(rs))
    }

    #[pyfunction]
    fn env_ctx(vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        let env = state::current_env(vm)?;
        let c = env.ctx().clone();
        let d = vm.ctx.new_dict();
        d.set_item("uid", vm.ctx.new_int(c.uid).into(), vm)?;
        d.set_item(
            "company_id",
            match c.company_id {
                Some(x) => vm.ctx.new_int(x).into(),
                None => vm.ctx.none(),
            },
            vm,
        )?;
        d.set_item("lang", vm.ctx.new_str(c.lang.as_str()).into(), vm)?;
        d.set_item("su", vm.ctx.new_bool(c.su).into(), vm)?;
        Ok(d.into())
    }
}
