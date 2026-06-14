# Parte del shim RUSTOO: réplica de odoo.models.
#
# Piezas:
#   * MetaModel  -- intercepta CADA clase de addon al importarse y registra su
#                   "fragmento" (campos, herencia, métodos, decoradores) en el
#                   kernel Rust vía _nexus.register_fragment. CORRECCIONES
#                   sobre la especificación original:
#                     1. Los métodos privados (_compute_*, _prepare_*) SÍ se
#                        registran: ahí vive la lógica de Odoo. Solo se
#                        excluyen los dunders.
#                     2. _inherit puede ser str o lista; el modelo destino de
#                        una extensión pura es _inherit, no bases[0]._name.
#                        También se captura _inherits (delegación).
#                     3. Los Field NO se borran de attrs: se conservan como
#                        descriptores (__get__/__set__), que es el camino
#                        rápido, y permiten introspección via _fields.
#   * BaseModel  -- el fondo del MRO. create/write/search/browse/read/unlink
#                   mapeados a _nexus, caché compartida en el Env y prefetch
#                   por lotes (una syscall por campo y lote, no por registro).
#   * build_registry -- pide a Rust el orden lineal de fragmentos por modelo y
#                   construye UNA clase dinámica por modelo:
#                       type(name, tuple(fragmentos) + (Model,), {...})
#                   con los fragmentos en orden inverso de carga. Así el
#                   super() NATIVO de Python recorre los módulos exactamente
#                   como el MRO de Odoo, sin syscalls intermedias.

import _nexus

from . import api as api_module
from .fields import Field, NO_VALUE

# ---------------------------------------------------------------------------
# Tablas laterales del lado Python
# ---------------------------------------------------------------------------

FRAGMENTS = {}        # fragment_id (int, lo asigna Rust) -> clase fragmento
REGISTRY = {}         # model_name -> clase dinámica construida
_CURRENT_MODULE = ["base"]   # fallback cuando la clase no vive en odoo.addons.*


def set_current_module(name):
    _CURRENT_MODULE[0] = name


def registry_class(model_name):
    try:
        return REGISTRY[model_name]
    except KeyError:
        raise KeyError("El modelo '%s' no existe en el registry" % model_name)


def _detect_module(qualmodule):
    # 'odoo.addons.sale.models.sale_order' -> 'sale'
    parts = qualmodule.split(".")
    if len(parts) >= 3 and parts[0] == "odoo" and parts[1] == "addons":
        return parts[2]
    return _CURRENT_MODULE[0]


# ---------------------------------------------------------------------------
# MetaModel
# ---------------------------------------------------------------------------

class MetaModel(type):
    def __new__(mcs, name, bases, attrs):
        cls = super().__new__(mcs, name, bases, attrs)

        # Las clases internas del shim no se registran.
        module = attrs.get("__module__", "")
        if attrs.get("_register") is False:
            return cls
        if module.startswith("odoo.") and not module.startswith("odoo.addons."):
            return cls

        # ---- resolución de _name / _inherit / _inherits (corregida) ----
        model_name = attrs.get("_name")
        inherit = attrs.get("_inherit") or []
        if isinstance(inherit, str):
            inherit = [inherit]
        inherits = dict(attrs.get("_inherits") or {})

        if not model_name and len(inherit) == 1:
            target = inherit[0]            # extensión pura: _inherit sin _name
        elif model_name:
            target = model_name
        else:
            # Mixin local sin _name ni _inherit: clase Python normal, sin registro.
            return cls

        # ---- campos: se conservan como descriptores y se serializan ----
        fields_def = {}
        for attr_name, attr_val in attrs.items():
            if isinstance(attr_val, Field):
                attr_val.model_name = target
                # @api.depends vive en el método compute: ligarlo al campo.
                compute = attr_val.compute
                if isinstance(compute, str) and compute in attrs:
                    attr_val.depends = getattr(attrs[compute], "_depends", ())
                elif callable(compute):
                    attr_val.depends = getattr(compute, "_depends", ())
                fields_def[attr_name] = attr_val.to_spec()

        # ---- métodos: TODOS menos dunders (los _privados son la lógica) ----
        methods = []
        constrains = []
        onchanges = []
        for attr_name, attr_val in attrs.items():
            if attr_name.startswith("__") and attr_name.endswith("__"):
                continue
            func = attr_val
            if isinstance(func, (staticmethod, classmethod)):
                func = func.__func__
            if not callable(func):
                continue
            methods.append(attr_name)
            if getattr(func, "_constrains", None):
                constrains.append((list(func._constrains), attr_name))
            if getattr(func, "_onchange", None):
                onchanges.append((list(func._onchange), attr_name))

        fragment_id = _nexus.register_fragment({
            "model": target,
            "name": model_name,                  # None => extensión pura
            "inherit": inherit,
            "inherits": inherits,
            "module": _detect_module(module),
            "class_name": name,
            "order": attrs.get("_order"),
            "description": attrs.get("_description"),
            "rec_name": attrs.get("_rec_name"),
            "auto": attrs.get("_auto", True),
            "abstract": attrs.get("_abstract", False),
            "transient": attrs.get("_transient", False),
            "fields": fields_def,
            "methods": methods,
            "constrains": constrains,
            "onchanges": onchanges,
        })
        FRAGMENTS[fragment_id] = cls
        cls._fragment_id = fragment_id
        return cls


# ---------------------------------------------------------------------------
# BaseModel: el fondo del MRO, cableado a _nexus
# ---------------------------------------------------------------------------

class BaseModel(metaclass=MetaModel):
    _register = False
    _name = None
    _inherit = ()
    _inherits = {}
    _description = None
    _order = "id"
    _rec_name = None
    _auto = True
    _abstract = True
    _transient = False

    # ---- construcción de recordsets ----

    @classmethod
    def _browse(cls, env, ids, prefetch_ids=None):
        rec = object.__new__(cls)
        rec.env = env
        rec._ids = tuple(ids)
        rec._prefetch_ids = tuple(prefetch_ids) if prefetch_ids else rec._ids
        return rec

    def browse(self, ids=()):
        if ids is None:
            ids = ()
        elif isinstance(ids, int):
            ids = (ids,)
        return self._browse(self.env, tuple(ids))

    # ---- protocolo de recordset (lo que los addons usan a diario) ----

    @property
    def ids(self):
        return list(self._ids)

    @property
    def id(self):
        if not self._ids:
            return False
        self.ensure_one()
        return self._ids[0]

    def __len__(self):
        return len(self._ids)

    def __bool__(self):
        return bool(self._ids)

    def __iter__(self):
        for rid in self._ids:
            yield self._browse(self.env, (rid,), prefetch_ids=self._prefetch_ids)

    def __getitem__(self, key):
        if isinstance(key, slice):
            return self._browse(self.env, self._ids[key], prefetch_ids=self._prefetch_ids)
        return self._browse(self.env, (self._ids[key],), prefetch_ids=self._prefetch_ids)

    def __contains__(self, item):
        if isinstance(item, BaseModel):
            return set(item._ids) <= set(self._ids)
        return item in self._ids

    def __add__(self, other):
        return self._browse(self.env, self._ids + other._ids)

    def __sub__(self, other):
        drop = set(other._ids)
        return self._browse(self.env, tuple(i for i in self._ids if i not in drop))

    def __or__(self, other):
        seen = set(self._ids)
        extra = tuple(i for i in other._ids if i not in seen)
        return self._browse(self.env, self._ids + extra)

    def __and__(self, other):
        keep = set(other._ids)
        return self._browse(self.env, tuple(i for i in self._ids if i in keep))

    def __eq__(self, other):
        if not isinstance(other, BaseModel):
            return NotImplemented
        return self._name == other._name and set(self._ids) == set(other._ids)

    def __hash__(self):
        return hash((self._name, frozenset(self._ids)))

    def __repr__(self):
        return "%s%s" % (self._name, tuple(self._ids))

    def ensure_one(self):
        if len(self._ids) != 1:
            raise ValueError("Expected singleton: %s" % self)
        return self

    # ---- lectura de campos: caché del Env + prefetch por lote ----

    _MISSING = object()

    def _read_field(self, field):
        if not self._ids:
            return field.convert_to_python(None, self)
        if len(self._ids) != 1:
            raise ValueError("Expected singleton: %s" % self)

        if field.related:
            return self._read_related(field)

        rid = self._ids[0]
        cache = self.env.cache
        key = (self._name, rid)
        slot = cache.get(key)
        value = slot.get(field.name, self._MISSING) if slot is not None else self._MISSING

        # Un None en un campo compute significa "no materializado": hay que
        # calcular. En un campo normal, None es un NULL legítimo.
        if value is self._MISSING or (value is None and field.compute):
            if field.compute and not field.store:
                self._trigger_compute(field)
            else:
                if value is self._MISSING:
                    self._prefetch(field)
                slot = cache.get(key)
                value = slot.get(field.name) if slot else None
                if value is None and field.compute:
                    self._trigger_compute(field)
            slot = cache.get(key) or {}
            value = slot.get(field.name)

        return field.convert_to_python(value, self)

    def _read_related(self, field):
        current = self
        *path, last = field.related.split(".")
        for step in path:
            current = getattr(current, step)
        return getattr(current, last) if current else False

    def _prefetch(self, field):
        cache = self.env.cache
        missing = [
            rid for rid in self._prefetch_ids
            if field.name not in cache.get((self._name, rid), ())
        ]
        if not missing:
            return
        data = _nexus.read_batch(self.env._handle, self._name, missing, [field.name])
        for rid, values in data.items():
            cache.setdefault((self._name, int(rid)), {}).update(values)
        for rid in missing:   # el kernel pudo omitir nulos: fijarlos
            cache.setdefault((self._name, rid), {}).setdefault(field.name, None)

    def _trigger_compute(self, field):
        cache = self.env.cache
        missing = [
            rid for rid in self._prefetch_ids
            if cache.get((self._name, rid), {}).get(field.name) is None
        ]
        records = self._browse(self.env, tuple(missing) or self._ids)
        method = field.compute if callable(field.compute) else getattr(type(self), field.compute)
        self.env._in_compute += 1
        try:
            method(records)
        finally:
            self.env._in_compute -= 1
        for rid in missing:
            cache.setdefault((self._name, rid), {}).setdefault(field.name, None)

    # ---- escritura de campos (asignación directa: record.state = 'sale') ----

    def _write_field(self, field, value):
        raw = field.convert_to_write(value)
        if self.env._in_compute:
            # Dentro de un compute, la asignación SOLO escribe la caché
            # (semántica exacta de Odoo).
            for rid in self._ids:
                self.env.cache.setdefault((self._name, rid), {})[field.name] = raw
        else:
            self.write({field.name: raw})

    # ---- API ORM principal ----

    def create(self, vals_list):
        if isinstance(vals_list, dict):
            vals_list = [vals_list]
        processed = []
        for vals in vals_list:
            out = {}
            for key, value in vals.items():
                f = self._fields.get(key)
                out[key] = f.convert_to_write(value) if f else value
            # defaults callables: se evalúan en Python (los literales los pone Rust)
            for fname, f in self._fields.items():
                if fname in out or f.compute or not f.store:
                    continue
                if f.default is not NO_VALUE and callable(f.default):
                    out[fname] = f.convert_to_write(f.default(self))
            processed.append(out)
        new_ids = _nexus.create(self.env._handle, self._name, processed)
        records = self._browse(self.env, tuple(new_ids))
        self.env.invalidate(self._name, new_ids)
        _invalidate_dependents(self.env, self._name, set().union(*map(set, processed)) if processed else set())
        try:
            records._check_constrains(None)
        except Exception:
            # Espejo del rollback del kernel: un create rechazado no deja fila.
            _nexus.unlink(self.env._handle, self._name, list(new_ids))
            self.env.invalidate(self._name, new_ids)
            raise
        return records

    def write(self, vals):
        out = {}
        for key, value in vals.items():
            f = self._fields.get(key)
            out[key] = f.convert_to_write(value) if f else value
        # Snapshot de los campos almacenados tocados, para revertir si un
        # constrain rechaza la escritura (en producción lo hace el kernel).
        snap_fields = [
            k for k in out
            if (f := self._fields.get(k)) is not None and f.store
        ]
        before = (
            _nexus.read_batch(self.env._handle, self._name, list(self._ids), snap_fields)
            if snap_fields and type(self)._constraint_methods else None
        )
        _nexus.write(self.env._handle, self._name, list(self._ids), out)
        self.env.invalidate(self._name, self._ids)
        _invalidate_dependents(self.env, self._name, set(out))
        try:
            self._check_constrains(set(out))
        except Exception:
            if before is not None:
                for rid, old_vals in before.items():
                    _nexus.write(self.env._handle, self._name, [int(rid)], old_vals)
            self.env.invalidate(self._name, self._ids)
            _invalidate_dependents(self.env, self._name, set(out))
            raise
        return True

    def unlink(self):
        _nexus.unlink(self.env._handle, self._name, list(self._ids))
        self.env.invalidate(self._name, self._ids)
        _invalidate_dependents(self.env, self._name, None)
        return True

    def search(self, domain, offset=0, limit=None, order=None, count=False, **kwargs):
        if count:
            return self.search_count(domain, **kwargs)
        ids = _nexus.search(self.env._handle, self._name, list(domain), offset, limit, order)
        return self._browse(self.env, tuple(ids))

    def search_count(self, domain, limit=None, **kwargs):
        return _nexus.search_count(self.env._handle, self._name, list(domain))

    def search_read(self, domain=None, fields=None, offset=0, limit=None, order=None, **kwargs):
        records = self.search(domain or [], offset=offset, limit=limit, order=order, **kwargs)
        if not records:
            return []
        return records.read(fields)

    def exists(self):
        alive = _nexus.exists(self.env._handle, self._name, list(self._ids))
        return self._browse(self.env, tuple(alive))

    def read(self, fields=None, load="_classic_read"):
        names = fields or [n for n, f in self._fields.items() if f.store]
        result = []
        for record in self:
            row = {"id": record.id}
            for fname in names:
                if fname == "id":
                    continue
                f = self._fields[fname]
                value = record._read_field(f)
                if f.type == "many2one":
                    value = (value.id, value.display_name) if value else False
                elif f.relational:
                    value = value.ids
                row[fname] = value
            result.append(row)
        return result

    def copy(self, default=None):
        self.ensure_one()
        values = {}
        for fname, f in self._fields.items():
            if not f.store or f.compute or not f.args.get("copy", True):
                continue
            values[fname] = f.convert_to_write(self._read_field(f))
        values.update(default or {})
        return self.create(values)

    def default_get(self, fields_list):
        result = {}
        for fname in fields_list:
            f = self._fields.get(fname)
            if f is None or f.default is NO_VALUE:
                continue
            result[fname] = f.default(self) if callable(f.default) else f.default
        return result

    def fields_get(self, allfields=None, attributes=None):
        return {
            n: dict(f.to_spec(), name=n)
            for n, f in self._fields.items()
            if allfields is None or n in allfields
        }

    # ---- utilidades de recordset usadas masivamente por los addons ----

    def mapped(self, func):
        if callable(func):
            return [func(rec) for rec in self]
        result = None
        values = []
        for rec in self:
            current = rec
            for step in func.split("."):
                current = getattr(current, step)
            if isinstance(current, BaseModel):
                result = current if result is None else (result | current)
            else:
                values.append(current)
        return result if result is not None else values

    def filtered(self, func):
        if isinstance(func, str):
            path = func
            func = lambda rec, _p=path: any(
                [rec.mapped(_p)] if not isinstance(rec.mapped(_p), list) else rec.mapped(_p)
            )
        keep = tuple(rec._ids[0] for rec in self if func(rec))
        return self._browse(self.env, keep, prefetch_ids=self._prefetch_ids)

    def sorted(self, key=None, reverse=False):
        if key is None:
            ordered = sorted(self._ids, reverse=reverse)
            return self._browse(self.env, tuple(ordered))
        if isinstance(key, str):
            field_name = key
            key = lambda rec: rec._read_field(rec._fields[field_name])
        records = sorted(self, key=key, reverse=reverse)
        return self._browse(self.env, tuple(r._ids[0] for r in records))

    # ---- variantes de entorno ----

    def with_env(self, env):
        return self._browse(env, self._ids, prefetch_ids=self._prefetch_ids)

    def with_context(self, *args, **kwargs):
        context = dict(args[0]) if args else dict(self.env.context)
        context.update(kwargs)
        return self.with_env(self.env._derive(context=context))

    def with_user(self, user):
        uid = user.id if isinstance(user, BaseModel) else int(user)
        return self.with_env(self.env._derive(uid=uid))

    def with_company(self, company):
        cid = company.id if isinstance(company, BaseModel) else int(company)
        ctx = dict(self.env.context)
        ctx["allowed_company_ids"] = [cid]
        return self.with_env(self.env._derive(context=ctx))

    def sudo(self, flag=True):
        return self.with_user(1) if flag else self

    # ---- presentación ----

    @property
    def display_name(self):
        if not self._ids:
            return False
        names = dict(self.name_get())
        return names.get(self._ids[0], "%s,%s" % (self._name, self._ids[0]))

    def name_get(self):
        rec_field = self._rec_name or ("name" if "name" in self._fields else None)
        result = []
        for record in self:
            if rec_field:
                value = record._read_field(self._fields[rec_field])
                result.append((record.id, value if value else "%s,%s" % (self._name, record.id)))
            else:
                result.append((record.id, "%s,%s" % (self._name, record.id)))
        return result

    # ---- restricciones (@api.constrains): el shim las dispara tras
    #      create/write; en producción el kernel Rust invoca este mismo hook ----

    def _check_constrains(self, touched):
        if not self._ids:
            return
        for watched, method_name in type(self)._constraint_methods:
            if touched is not None and not (set(watched) & touched):
                continue
            getattr(self, method_name)()

    # ---- despacho hacia implementaciones nativas de Rust -------------------
    # Cuando un addon llama super().metodo() y la cadena Python se agota aquí,
    # si el kernel tiene una implementación nativa/transpilada la ejecuta.

    def __getattr__(self, name):
        if name.startswith("_") or self.__class__._name is None:
            raise AttributeError(name)
        if _nexus.has_native(self._name, name):
            handle = self.env._handle
            model, ids = self._name, list(self._ids)
            return lambda *args, **kwargs: _nexus.call_native(
                handle, model, name, ids, list(args), dict(kwargs)
            )
        raise AttributeError("'%s' object has no attribute '%s'" % (self._name, name))


class Model(BaseModel):
    _register = False
    _abstract = False
    _auto = True


class AbstractModel(BaseModel):
    _register = False
    _abstract = True
    _auto = False


class TransientModel(Model):
    _register = False
    _transient = True


# ---------------------------------------------------------------------------
# Invalidación de dependientes (espejo ligero del propagate del kernel):
# si cambia sale.order.line.price_subtotal, el amount_total cacheado del
# pedido padre debe descartarse de la caché Python.
# ---------------------------------------------------------------------------

def _invalidate_dependents(env, model_name, touched):
    """Cierre transitivo: si cambia line.product_uom_qty, se invalida
    line.price_subtotal (depends directo) y eso a su vez invalida
    order.amount_total (depends con punto). Con guarda anti-ciclos."""
    work = [(model_name, f) for f in touched] if touched is not None else [(model_name, None)]
    already_stale = set()

    while work:
        src_model, src_field = work.pop()
        for parent_name, parent_cls in REGISTRY.items():
            for fname, f in parent_cls._fields.items():
                if not f.compute or not f.depends or (parent_name, fname) in already_stale:
                    continue
                hit = False
                for dep in f.depends:
                    head, _, tail = dep.partition(".")
                    if not tail:
                        if parent_name == src_model and (src_field is None or head == src_field):
                            hit = True
                            break
                    else:
                        rel = parent_cls._fields.get(head)
                        if rel is not None and rel.relational and rel.comodel_name == src_model \
                                and (src_field is None or tail == src_field):
                            hit = True
                            break
                if hit:
                    already_stale.add((parent_name, fname))
                    for key, slot in env.cache.items():
                        if key[0] == parent_name:
                            slot.pop(fname, None)
                    work.append((parent_name, fname))   # propagar en cadena


# ---------------------------------------------------------------------------
# Construcción del registry: MRO dinámico al estilo Odoo
# ---------------------------------------------------------------------------

def build_registry():
    """Pide a Rust el orden lineal de fragmentos por modelo (más derivado
    primero) y construye una clase dinámica por modelo. El super() nativo de
    Python recorre exactamente esa linealización."""
    REGISTRY.clear()
    plan = _nexus.finalize()   # {model_name: [fragment_id, ...]}

    for model_name, fragment_ids in plan.items():
        fragment_classes = [FRAGMENTS[fid] for fid in fragment_ids]
        first = fragment_classes[0] if fragment_classes else Model
        base_tail = TransientModel if getattr(first, "_transient", False) else Model
        bases = tuple(fragment_classes) + (base_tail,)

        # Metadatos plegados: la última declaración no-None gana.
        folded = {"_order": "id", "_description": None, "_rec_name": None}
        for frag in reversed(fragment_classes):
            for key in ("_order", "_description", "_rec_name"):
                value = frag.__dict__.get(key)
                if value:
                    folded[key] = value

        cls = MetaModel(
            "Registry[%s]" % model_name,
            bases,
            {
                "_register": False,        # la clase dinámica no re-registra
                "_name": model_name,
                "_inherit": (),
                "__module__": "odoo.registry",
                **folded,
            },
        )

        # _fields: recorrer el MRO recogiendo descriptores Field.
        all_fields = {}
        for klass in reversed(cls.__mro__):
            for attr_name, attr_val in vars(klass).items():
                if isinstance(attr_val, Field):
                    all_fields[attr_name] = attr_val
        cls._fields = all_fields

        # constrains plegados (en orden MRO, sin duplicar).
        constraint_methods = []
        seen = set()
        for klass in cls.__mro__:
            for attr_name, attr_val in vars(klass).items():
                func = attr_val.__func__ if isinstance(attr_val, (staticmethod, classmethod)) else attr_val
                if callable(func) and getattr(func, "_constrains", None) and attr_name not in seen:
                    constraint_methods.append((tuple(func._constrains), attr_name))
                    seen.add(attr_name)
        cls._constraint_methods = constraint_methods

        REGISTRY[model_name] = cls

    return REGISTRY
