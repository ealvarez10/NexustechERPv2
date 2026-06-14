# STUB de _nexus (Python puro, en memoria).
#
# Implementa el MISMO contrato ABI que el módulo de extensión PyO3 real
# (nexus-py/src/lib.rs), de modo que el shim `odoo` puede desarrollarse y
# probarse hoy con CPython, sin compilar Rust. En producción, el `_nexus.so`
# compilado se coloca antes en sys.path y eclipsa este archivo.
#
# Cobertura del stub: registro de fragmentos, orden topológico por módulo,
# linealización MRO, almacenamiento en memoria, dominios básicos, comandos
# x2many, one2many por campo inverso y defaults literales. El cursor SQL y
# los datos XML/CSV requieren el kernel real (lanzan NotImplementedError o
# son no-op documentados).

_modules = {}          # nombre -> depends
_module_order = []     # orden de declaración (el loader ya viene topo-ordenado)
_fragments = []        # [(fragment_id, spec)]
_models = {}           # model_name -> {"fields": {...}, "fragments": [ids]}
_tables = {}           # model_name -> {"next_id": int, "rows": {id: {f: v}}}
_natives = {}          # (model, method) -> callable  (overrides nativos/transpilados)


# ---------------------------------------------------------------------------
# Fase de registro
# ---------------------------------------------------------------------------

def declare_module(name, depends):
    _modules[name] = list(depends)
    if name not in _module_order:
        _module_order.append(name)


def register_fragment(spec):
    fragment_id = len(_fragments) + 1
    _fragments.append((fragment_id, spec))
    return fragment_id


def finalize():
    """Construye el plan de linealización: {model: [fragment_id, ...]} con el
    fragmento MÁS DERIVADO primero (orden inverso de carga), que es justo el
    orden de bases que Python necesita para que super() recorra los módulos
    como el MRO de Odoo."""
    _models.clear()
    module_rank = {name: i for i, name in enumerate(_module_order)}

    ordered = sorted(
        _fragments,
        key=lambda item: (module_rank.get(item[1]["module"], 10**9), item[0]),
    )
    for fragment_id, spec in ordered:           # orden de carga
        model = _models.setdefault(
            spec["model"],
            {"fields": {}, "fragments": [], "order": "id"},
        )
        model["fields"].update(spec["fields"])  # el módulo posterior pisa
        model["fragments"].append(fragment_id)
        if spec.get("order"):
            model["order"] = spec["order"]

    return {
        name: list(reversed(info["fragments"]))   # más derivado primero
        for name, info in _models.items()
    }


def load_module_data(name, path, data_files):
    # Fase 4+5: en el kernel real, aquí se sincroniza esquema y se cargan
    # XML/CSV. El stub en memoria no necesita esquema y omite los datos.
    return None


def register_native(model, method, fn):
    """Registra una implementación nativa (en el kernel real: Rust/transpilada)."""
    _natives[(model, method)] = fn


def has_native(model, method):
    return (model, method) in _natives


def call_native(handle, model, method, ids, args, kwargs):
    return _natives[(model, method)](handle, model, ids, *args, **kwargs)


# ---------------------------------------------------------------------------
# Entorno y almacenamiento
# ---------------------------------------------------------------------------

def env_new(uid, context):
    return 1   # el stub mantiene un único entorno global


def _table(model):
    return _tables.setdefault(model, {"next_id": 0, "rows": {}})


def _field_def(model, fname):
    return _models.get(model, {}).get("fields", {}).get(fname)


def _o2m_ids(model, fname, rid):
    fdef = _field_def(model, fname)
    comodel, inverse = fdef["comodel"], fdef["inverse_name"]
    rows = _table(comodel)["rows"]
    return sorted(i for i, row in rows.items() if row.get(inverse) == rid)


# ---------------------------------------------------------------------------
# CRUD
# ---------------------------------------------------------------------------

def create(handle, model, vals_list):
    fields = _models.get(model, {}).get("fields", {})
    table = _table(model)
    new_ids = []
    for vals in vals_list:
        row = {}
        pending_o2m = []
        for key, value in vals.items():
            fdef = fields.get(key)
            if fdef and fdef["type"] == "one2many":
                pending_o2m.append((key, fdef, value))
            elif fdef and fdef["type"] == "many2many":
                row[key] = _apply_m2m(handle, fdef, [], value)
            else:
                row[key] = value
        # defaults literales (los callables ya los resolvió el shim)
        for fname, fdef in fields.items():
            if fname not in row and "default" in fdef and fdef.get("store", True):
                row[fname] = fdef["default"]
        # obligatorios
        for fname, fdef in fields.items():
            if fdef.get("required") and fdef.get("store", True) and not fdef.get("compute"):
                if row.get(fname) in (None, False):
                    raise ValueError(
                        "campo obligatorio sin valor: %s.%s" % (model, fname)
                    )
        table["next_id"] += 1
        rid = table["next_id"]
        table["rows"][rid] = row
        for key, fdef, value in pending_o2m:
            _apply_o2m(handle, fdef, rid, value)
        new_ids.append(rid)
    return new_ids


def write(handle, model, ids, vals):
    fields = _models.get(model, {}).get("fields", {})
    rows = _table(model)["rows"]
    for rid in ids:
        if rid not in rows:
            raise KeyError("%s(%s) no existe" % (model, rid))
    for key, value in vals.items():
        fdef = fields.get(key)
        for rid in ids:
            if fdef and fdef["type"] == "one2many":
                _apply_o2m(handle, fdef, rid, value)
            elif fdef and fdef["type"] == "many2many" and isinstance(value, list) \
                    and value and isinstance(value[0], (list, tuple)):
                current = rows[rid].get(key) or []
                rows[rid][key] = _apply_m2m(handle, fdef, list(current), value)
            else:
                rows[rid][key] = value
    return True


def unlink(handle, model, ids):
    rows = _table(model)["rows"]
    for rid in ids:
        rows.pop(rid, None)
    return True


def _apply_o2m(handle, fdef, parent_id, value):
    comodel, inverse = fdef["comodel"], fdef["inverse_name"]
    if not isinstance(value, list):
        return
    for command in value:
        op = command[0]
        if op == 0:
            vals = dict(command[2])
            vals[inverse] = parent_id
            create(handle, comodel, [vals])
        elif op == 1:
            write(handle, comodel, [command[1]], dict(command[2]))
        elif op == 2:
            unlink(handle, comodel, [command[1]])
        elif op == 3:
            write(handle, comodel, [command[1]], {inverse: False})
        elif op == 4:
            write(handle, comodel, [command[1]], {inverse: parent_id})
        elif op == 5:
            rows = _table(comodel)["rows"]
            linked = [i for i, r in rows.items() if r.get(inverse) == parent_id]
            for rid in linked:
                rows[rid][inverse] = False
        elif op == 6:
            rows = _table(comodel)["rows"]
            wanted = set(command[2])
            for rid, row in rows.items():
                if row.get(inverse) == parent_id and rid not in wanted:
                    row[inverse] = False
            for rid in wanted:
                rows[rid][inverse] = parent_id


def _apply_m2m(handle, fdef, current, value):
    if not isinstance(value, list):
        return current
    if value and not isinstance(value[0], (list, tuple)):
        return list(value)          # lista directa de ids
    for command in value:
        op = command[0]
        if op == 0:
            current.extend(create(handle, fdef["comodel"], [dict(command[2])]))
        elif op == 1:
            write(handle, fdef["comodel"], [command[1]], dict(command[2]))
        elif op == 2:
            current = [i for i in current if i != command[1]]
            unlink(handle, fdef["comodel"], [command[1]])
        elif op == 3:
            current = [i for i in current if i != command[1]]
        elif op == 4:
            if command[1] not in current:
                current.append(command[1])
        elif op == 5:
            current = []
        elif op == 6:
            current = list(command[2])
    return current


# ---------------------------------------------------------------------------
# Lectura por lotes (la syscall del prefetch) y búsqueda
# ---------------------------------------------------------------------------

def read_batch(handle, model, ids, field_names):
    fields = _models.get(model, {}).get("fields", {})
    rows = _table(model)["rows"]
    result = {}
    for rid in ids:
        row = rows.get(rid)
        if row is None:
            continue
        values = {}
        for fname in field_names:
            fdef = fields.get(fname)
            if fdef and fdef["type"] == "one2many":
                values[fname] = _o2m_ids(model, fname, rid)
            else:
                values[fname] = row.get(fname)
        result[rid] = values
    return result


def exists(handle, model, ids):
    rows = _table(model)["rows"]
    return [rid for rid in ids if rid in rows]


_OPS = {
    "=": lambda a, b: _eq(a, b),
    "!=": lambda a, b: not _eq(a, b),
    ">": lambda a, b: a is not None and a > b,
    ">=": lambda a, b: a is not None and a >= b,
    "<": lambda a, b: a is not None and a < b,
    "<=": lambda a, b: a is not None and a <= b,
    "in": lambda a, b: a in b,
    "not in": lambda a, b: a not in b,
    "like": lambda a, b: isinstance(a, str) and b in a,
    "ilike": lambda a, b: isinstance(a, str) and b.lower() in a.lower(),
}


def _eq(a, b):
    if a in (None, False) and b in (None, False):
        return True
    return a == b


def search(handle, model, domain, offset=0, limit=None, order=None):
    rows = _table(model)["rows"]
    hits = []
    for rid, row in rows.items():
        if _matches(model, rid, row, domain):
            hits.append(rid)
    order = order or _models.get(model, {}).get("order", "id")
    fname = order.split()[0]
    reverse = "desc" in order.lower()
    if fname == "id":
        hits.sort(reverse=reverse)
    else:
        hits.sort(key=lambda i: (rows[i].get(fname) is None, rows[i].get(fname)), reverse=reverse)
    if offset:
        hits = hits[offset:]
    if limit:
        hits = hits[:limit]
    return hits


def search_count(handle, model, domain):
    return len(search(handle, model, domain))


def _matches(model, rid, row, domain):
    # AND implícito; soporta '|', '&', '!' en notación polaca como Odoo.
    stack = []
    for item in reversed(list(domain) or []):
        if item == "&":
            a, b = stack.pop(), stack.pop()
            stack.append(a and b)
        elif item == "|":
            a, b = stack.pop(), stack.pop()
            stack.append(a or b)
        elif item == "!":
            stack.append(not stack.pop())
        else:
            fname, op, expected = item
            actual = rid if fname == "id" else row.get(fname)
            stack.append(_OPS[op](actual, expected))
    return all(stack) if stack else True


# ---------------------------------------------------------------------------
# Cursor, xml_ids y mantenimiento (requieren el kernel real)
# ---------------------------------------------------------------------------

def cr_execute(handle, sql, params):
    raise NotImplementedError(
        "self.env.cr requiere el kernel Rust (transacción SQLx compartida)"
    )


def cr_commit(handle):
    return None


def cr_rollback(handle):
    return None


def cr_savepoint(handle):
    return "sp0"


def cr_rollback_to(handle, name):
    return None


def cr_release(handle, name):
    return None


def xmlid_lookup(handle, xml_id):
    return None


def invalidate(handle, model, ids):
    return None


def flush(handle):
    return None


def reset():
    """Solo para tests: limpia todo el estado global del stub."""
    _modules.clear()
    _module_order.clear()
    _fragments.clear()
    _models.clear()
    _tables.clear()
    _natives.clear()
