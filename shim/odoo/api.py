# Parte del shim RUSTOO: réplica de odoo.api.
#
# Los decoradores NO cambian el comportamiento de la función: solo le cuelgan
# metadatos (_depends, _constrains, ...) que MetaModel recolecta y envía al
# kernel Rust en register_fragment. Así @api.depends alimenta el motor de
# recálculo de Rust sin tocar el código del addon.

import _nexus

SUPERUSER_ID = 1


# --------------------------------------------------------------------------
# Decoradores
# --------------------------------------------------------------------------

def depends(*paths):
    def decorator(fn):
        fn._depends = tuple(paths)
        return fn
    return decorator


def depends_context(*keys):
    def decorator(fn):
        fn._depends_context = tuple(keys)
        return fn
    return decorator


def constrains(*field_names):
    def decorator(fn):
        fn._constrains = tuple(field_names)
        return fn
    return decorator


def onchange(*field_names):
    def decorator(fn):
        fn._onchange = tuple(field_names)
        return fn
    return decorator


def model(fn):
    """Método de modelo (no requiere registros): self es el recordset vacío."""
    fn._api = "model"
    return fn


def model_create_multi(fn):
    """create() que recibe lista de dicts. Como en Odoo, el decorador ENVUELVE
    la función para normalizar un dict suelto a [dict]: el addon siempre ve
    una lista. El super() interno no se ve afectado (la celda __class__ es de
    la función original, que es la que se ejecuta)."""
    import functools

    @functools.wraps(fn)
    def wrapper(self, vals_list, **kwargs):
        if isinstance(vals_list, dict):
            vals_list = [vals_list]
        return fn(self, vals_list, **kwargs)

    wrapper._api = "model_create_multi"
    return wrapper


def returns(model_name, downgrade=None, upgrade=None):
    def decorator(fn):
        fn._returns = (model_name, downgrade, upgrade)
        return fn
    return decorator


def ondelete(*, at_uninstall):
    def decorator(fn):
        fn._ondelete = at_uninstall
        return fn
    return decorator


def autovacuum(fn):
    fn._autovacuum = True
    return fn


def readonly(fn):
    fn._readonly = True
    return fn


def private(fn):
    fn._api_private = True
    return fn


# --------------------------------------------------------------------------
# Environment: el `self.env` que ven los addons
# --------------------------------------------------------------------------

class Environment:
    """Envoltorio del entorno del kernel Rust.

    - env['res.partner']  -> recordset vacío del modelo (clase dinámica)
    - env.cr               -> CursorProxy sobre la transacción SQLx activa
    - env.uid / env.user / env.context / env.company
    - caché de campos compartida (model, id) -> {field: raw_value}
    """

    def __init__(self, handle, uid, context=None):
        self._handle = handle
        self.uid = uid
        self.context = dict(context or {})
        self.cache = {}            # (model_name, id) -> {field_name: raw}
        self.cr = CursorProxy(handle)
        self._in_compute = 0       # >0: las asignaciones escriben solo caché

    # -- fábrica de recordsets ------------------------------------------------

    def __getitem__(self, model_name):
        from . import models
        cls = models.registry_class(model_name)
        return cls._browse(self, ())

    def __contains__(self, model_name):
        from . import models
        return model_name in models.REGISTRY

    # -- identidad ------------------------------------------------------------

    @property
    def user(self):
        return self["res.users"].browse(self.uid)

    @property
    def company(self):
        return self["res.company"].browse(
            self.context.get("allowed_company_ids", [1])[0]
        )

    @property
    def companies(self):
        ids = self.context.get("allowed_company_ids", [1])
        return self["res.company"].browse(ids)

    @property
    def su(self):
        return self.uid == 1

    def ref(self, xml_id, raise_if_not_found=True):
        res = _nexus.xmlid_lookup(self._handle, xml_id)
        if not res:
            if raise_if_not_found:
                raise ValueError("External ID not found: %s" % xml_id)
            return None
        model_name, rec_id = res
        return self[model_name].browse(rec_id)

    # -- variantes (devuelven entornos derivados que comparten caché) ---------

    def _derive(self, uid=None, context=None):
        env = Environment.__new__(Environment)
        env._handle = self._handle
        env.uid = self.uid if uid is None else uid
        env.context = dict(self.context if context is None else context)
        env.cache = self.cache          # caché compartida
        env.cr = self.cr
        env._in_compute = self._in_compute
        return env

    # -- caché / invalidación ---------------------------------------------------

    def invalidate_all(self):
        self.cache.clear()
        _nexus.invalidate(self._handle, None, None)

    def invalidate(self, model_name, ids=None, fnames=None):
        if ids is None:
            stale = [k for k in self.cache if k[0] == model_name]
        else:
            stale = [(model_name, i) for i in ids]
        for key in stale:
            if fnames is None:
                self.cache.pop(key, None)
            else:
                slot = self.cache.get(key)
                if slot:
                    for f in fnames:
                        slot.pop(f, None)

    def flush_all(self):     # los addons lo llaman; el kernel es write-through
        _nexus.flush(self._handle)

    # Compat con código que usa env.registry / env.cr.commit
    @property
    def registry(self):
        from . import models
        return models.REGISTRY


class CursorProxy:
    """Proxy del cursor: `self.env.cr.execute(...)` corre sobre la MISMA
    transacción SQLx que el ORM de Rust. Traduce los placeholders de psycopg2
    (%s y %(name)s) a $1..$n de Postgres."""

    def __init__(self, handle):
        self._handle = handle
        self._rows = []
        self.rowcount = -1
        self.description = None

    def _translate(self, query, params):
        if params is None:
            return query, []
        if isinstance(params, dict):
            ordered = []
            out = []
            i = 0
            n = len(query)
            while i < n:
                if query[i] == "%" and i + 1 < n and query[i + 1] == "(":
                    j = query.index(")s", i)
                    ordered.append(params[query[i + 2:j]])
                    out.append("$%d" % len(ordered))
                    i = j + 2
                elif query[i] == "%" and i + 1 < n and query[i + 1] == "%":
                    out.append("%")
                    i += 2
                else:
                    out.append(query[i])
                    i += 1
            return "".join(out), ordered
        # secuencia posicional con %s
        ordered = list(params)
        parts = query.split("%%")  # respeta los %% literales
        counter = [0]

        def repl(text):
            out = []
            i = 0
            while True:
                j = text.find("%s", i)
                if j < 0:
                    out.append(text[i:])
                    break
                counter[0] += 1
                out.append(text[i:j])
                out.append("$%d" % counter[0])
                i = j + 2
            return "".join(out)

        return "%".join(repl(p) for p in parts), ordered

    def execute(self, query, params=None):
        sql, args = self._translate(query, params)
        result = _nexus.cr_execute(self._handle, sql, args)
        self._rows = result.get("rows", [])
        self.description = result.get("description")
        self.rowcount = result.get("rowcount", len(self._rows))
        return self.rowcount

    def fetchall(self):
        rows, self._rows = self._rows, []
        return [tuple(r) for r in rows]

    def fetchone(self):
        if not self._rows:
            return None
        return tuple(self._rows.pop(0))

    def dictfetchall(self):
        # Extensión propia de Odoo, muy usada en addons reales.
        desc = self.description
        rows, self._rows = self._rows, []
        if desc:
            keys = [col[0] for col in desc]
            return [dict(zip(keys, row)) for row in rows]
        return [dict(r) if isinstance(r, dict) else r for r in rows]

    def commit(self):
        _nexus.cr_commit(self._handle)

    def rollback(self):
        _nexus.cr_rollback(self._handle)

    def savepoint(self):
        from contextlib import contextmanager

        @contextmanager
        def _sp():
            name = _nexus.cr_savepoint(self._handle)
            try:
                yield
            except Exception:
                _nexus.cr_rollback_to(self._handle, name)
                raise
            else:
                _nexus.cr_release(self._handle, name)

        return _sp()
