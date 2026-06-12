"""Bootstrap de nexus-py: la capa Odoo sobre el modulo nativo `_nexus`.

Define `Env` y `Recordset` con la ergonomia del ORM de Odoo para que los
metodos de negocio intraducibles corran TAL CUAL dentro del binario Rust:

    def action_confirm(self):
        for order in self:
            if not order.partner_id:
                raise UserError("Pedido sin cliente")
            order.state = "sale"
        return True

Cada operacion (`self.campo`, `self.campo = v`, `self.env['modelo']`,
`self.metodo()`) cruza a Rust via `_nexus` y opera sobre el MISMO Env
(misma cache, misma transaccion) que el codigo Rust de nexus-orm.

Diferencia deliberada con Odoo: `super().metodo(...)` se escribe
`self.super_(...)` — el transpilador hace esa reescritura.
"""
import _nexus


class UserError(Exception):
    """odoo.exceptions.UserError — cruza a Rust como OError::User."""


class ValidationError(UserError):
    """odoo.exceptions.ValidationError — OError::Validation."""


def _wrap(v):
    """Valor crudo de _nexus -> valor Python (recordsets etiquetados)."""
    if isinstance(v, tuple) and len(v) == 2 and v[0] == "__rs__":
        return Recordset(v[1])
    return v


def _unwrap(v):
    """Valor Python -> valor que _nexus entiende."""
    if isinstance(v, Recordset):
        return ("__rs__", v._h)
    return v


class Env:
    """`self.env` — proxy del Env activo del lado Rust."""

    def __getitem__(self, model):
        return Recordset(_nexus.env_model(model))

    @property
    def uid(self):
        return _nexus.env_ctx()["uid"]

    @property
    def company_id(self):
        return _nexus.env_ctx()["company_id"]

    @property
    def lang(self):
        return _nexus.env_ctx()["lang"]

    @property
    def su(self):
        return _nexus.env_ctx()["su"]

    @property
    def context(self):
        return _nexus.env_ctx()


class Recordset:
    """Proxy de un `nexus_orm::Recordset` identificado por handle."""

    __slots__ = ("_h",)

    def __init__(self, h):
        object.__setattr__(self, "_h", h)

    # -- identidad ----------------------------------------------------------
    @property
    def env(self):
        return Env()

    @property
    def ids(self):
        return _nexus.rs_ids(self._h)

    @property
    def id(self):
        ids = _nexus.rs_ids(self._h)
        if not ids:
            return False
        if len(ids) > 1:
            raise ValueError(
                "record.id: esperaba singleton, el recordset tiene %d" % len(ids)
            )
        return ids[0]

    def __len__(self):
        return len(_nexus.rs_ids(self._h))

    def __bool__(self):
        return len(self) > 0

    def __iter__(self):
        for i in _nexus.rs_ids(self._h):
            yield Recordset(_nexus.rs_browse(self._h, [i]))

    def __getitem__(self, i):
        return Recordset(_nexus.rs_browse(self._h, [_nexus.rs_ids(self._h)[i]]))

    def __or__(self, other):
        return Recordset(_nexus.rs_union(self._h, other._h))

    def __sub__(self, other):
        return Recordset(_nexus.rs_minus(self._h, other._h))

    def __and__(self, other):
        return Recordset(_nexus.rs_intersect(self._h, other._h))

    def __repr__(self):
        return "%s%s" % (_nexus.rs_model(self._h), _nexus.rs_ids(self._h))

    def __del__(self):
        try:
            _nexus.rs_free(self._h)
        except Exception:
            pass

    # -- API Odoo -----------------------------------------------------------
    def ensure_one(self):
        if len(self) != 1:
            raise ValueError(
                "ensure_one(): el recordset tiene %d registros" % len(self)
            )
        return self

    def exists(self):
        return self  # v0: sin verificacion contra BD

    def browse(self, ids):
        if isinstance(ids, int):
            ids = [ids]
        return Recordset(_nexus.rs_browse(self._h, list(ids)))

    def search(self, domain, limit=-1, offset=-1):
        return Recordset(
            _nexus.env_search(_nexus.rs_model(self._h), domain, limit, offset)
        )

    def create(self, vals):
        return Recordset(
            _nexus.env_create(
                _nexus.rs_model(self._h),
                list(vals.keys()),
                [_unwrap(v) for v in vals.values()],
            )
        )

    def write(self, vals):
        _nexus.rs_write(
            self._h, list(vals.keys()), [_unwrap(v) for v in vals.values()]
        )
        return True

    def read(self, fields):
        _nexus.rs_read(self._h, list(fields))

    def mapped(self, field):
        return _wrap(_nexus.rs_mapped(self._h, field))

    def filtered(self, fn):
        return self.browse([r.id for r in self if fn(r)])

    def sorted(self, key=None, reverse=False):
        recs = list(self)
        if isinstance(key, str):
            field = key
            key = lambda r: getattr(r, field)
        recs.sort(key=key, reverse=reverse)
        return self.browse([r.id for r in recs])

    def super_(self, *args):
        """El `super().metodo(...)` de Odoo: siguiente eslabon _inherit."""
        return _wrap(_nexus.call_super(self._h, [_unwrap(a) for a in args]))

    # -- azucar dinamico: campos y metodos ------------------------------------
    def __getattr__(self, name):
        if name.startswith("_"):
            raise AttributeError(name)
        if _nexus.rs_has_field(self._h, name):
            return _wrap(_nexus.rs_get(self._h, name))
        if _nexus.rs_has_method(self._h, name):
            h = self._h

            def _call(*args):
                return _wrap(_nexus.rs_call(h, name, [_unwrap(a) for a in args]))

            return _call
        raise AttributeError(
            "'%s' no es campo ni metodo de %s" % (name, _nexus.rs_model(self._h))
        )

    def __setattr__(self, name, value):
        _nexus.rs_set(self._h, name, _unwrap(value))


def _wrap_handle(h):
    """Usado por Rust para construir el `self` de un metodo."""
    return Recordset(h)


# Global de conveniencia para exec/eval: siempre apunta al Env activo.
env = Env()
