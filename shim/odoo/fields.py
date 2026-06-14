# Parte del shim RUSTOO: réplica de odoo.fields.
#
# Cada campo es un DESCRIPTOR de Python (__get__/__set__): el acceso
# `record.name` no pasa por __getattr__ (vía lenta) sino por el protocolo de
# descriptores, que consulta la caché del Env y, en fallo, dispara UN prefetch
# por lote hacia el kernel (_nexus.read_batch).
#
# Compatibilidad: __init__ acepta **kwargs para tragar parámetros que el
# runtime aún no usa (states, groups, tracking, ...) sin romper la carga de
# addons reales. Todo kwarg conocido viaja a Rust en to_spec().

import datetime

DATE_FORMAT = "%Y-%m-%d"
DATETIME_FORMAT = "%Y-%m-%d %H:%M:%S"


class Default:
    """Centinela para distinguir 'no pasado' de None/False."""


NO_VALUE = Default()


class Field:
    type = None  # cada subclase lo define
    relational = False

    # kwargs que viajan al kernel tal cual (si están presentes)
    _spec_attrs = (
        "string", "required", "readonly", "index", "store", "copy",
        "translate", "company_dependent", "tracking", "groups", "help",
        "default_export_compatible", "group_operator", "aggregator",
    )

    def __init__(self, string=NO_VALUE, **kwargs):
        if string is not NO_VALUE:
            kwargs["string"] = string
        self.args = kwargs
        self.name = None           # lo fija __set_name__
        self.model_name = None
        self.compute = kwargs.get("compute")          # str (nombre de método) o callable
        self.inverse = kwargs.get("inverse")
        self.search = kwargs.get("search")
        self.related = kwargs.get("related")
        self.default = kwargs.get("default", NO_VALUE)
        self.required = bool(kwargs.get("required", False))
        self.readonly = bool(kwargs.get("readonly", False))
        store = kwargs.get("store")
        # Convención de Odoo: un compute no almacena salvo store=True;
        # un campo normal almacena salvo store=False.
        if store is None:
            self.store = not (self.compute or self.related)
        else:
            self.store = bool(store)
        self.depends = ()   # lo rellenan los decoradores @api.depends vía MetaModel

    def __set_name__(self, owner, name):
        self.name = name

    # ---- protocolo de descriptor: el camino caliente ----

    def __get__(self, record, owner=None):
        if record is None:
            return self
        return record._read_field(self)

    def __set__(self, record, value):
        record._write_field(self, value)

    # ---- serialización hacia el kernel Rust ----

    def to_spec(self):
        spec = {"type": self.type, "store": self.store}
        for key in self._spec_attrs:
            if key in self.args:
                spec[key] = self.args[key]
        if self.required:
            spec["required"] = True
        if self.readonly:
            spec["readonly"] = True
        if self.compute:
            spec["compute"] = self.compute if isinstance(self.compute, str) else getattr(self.compute, "__name__", "<lambda>")
        if self.related:
            spec["related"] = self.related
        # Los default literales viajan a Rust; los callables se quedan en
        # Python y el kernel pide su evaluación al crear (default_python).
        if self.default is not NO_VALUE:
            if callable(self.default):
                spec["default_python"] = True
            else:
                spec["default"] = self.default
        if self.depends:
            spec["depends"] = list(self.depends)
        return spec

    # ---- conversiones valor crudo <-> valor Python (convención Odoo) ----

    def convert_to_python(self, value, record):
        # Convención de Odoo: los campos vacíos devuelven False, no None.
        return False if value is None else value

    def convert_to_write(self, value):
        return value


class Boolean(Field):
    type = "boolean"

    def convert_to_python(self, value, record):
        return bool(value)


class Integer(Field):
    type = "integer"

    def convert_to_python(self, value, record):
        return int(value) if value is not None else 0


class Float(Field):
    type = "float"

    def __init__(self, string=NO_VALUE, digits=None, **kwargs):
        if digits is not None:
            kwargs["digits"] = digits
        super().__init__(string, **kwargs)

    def convert_to_python(self, value, record):
        return float(value) if value is not None else 0.0


class Monetary(Float):
    type = "monetary"

    def __init__(self, string=NO_VALUE, currency_field="currency_id", **kwargs):
        kwargs["currency_field"] = currency_field
        super().__init__(string, **kwargs)


class Char(Field):
    type = "char"

    def __init__(self, string=NO_VALUE, size=None, **kwargs):
        if size is not None:
            kwargs["size"] = size
        super().__init__(string, **kwargs)


class Text(Field):
    type = "text"


class Html(Text):
    type = "html"


class Selection(Field):
    type = "selection"

    def __init__(self, selection=NO_VALUE, string=NO_VALUE, **kwargs):
        super().__init__(string, **kwargs)
        self.selection = None if selection is NO_VALUE else selection

    def to_spec(self):
        spec = super().to_spec()
        if isinstance(self.selection, (list, tuple)):
            spec["selection"] = [list(pair) for pair in self.selection]
        elif self.selection is not None:
            spec["selection_python"] = True  # callable o nombre de método
        return spec


class Date(Field):
    type = "date"

    @staticmethod
    def today(*args):
        return datetime.date.today()

    @staticmethod
    def to_date(value):
        if not value:
            return False
        if isinstance(value, datetime.datetime):
            return value.date()
        if isinstance(value, datetime.date):
            return value
        return datetime.datetime.strptime(value[:10], DATE_FORMAT).date()

    to_string = staticmethod(lambda v: v.strftime(DATE_FORMAT) if v else False)

    def convert_to_python(self, value, record):
        return self.to_date(value)

    def convert_to_write(self, value):
        if isinstance(value, (datetime.date, datetime.datetime)):
            return value.strftime(DATE_FORMAT)
        return value


class Datetime(Field):
    type = "datetime"

    @staticmethod
    def now(*args):
        return datetime.datetime.now().replace(microsecond=0)

    @staticmethod
    def to_datetime(value):
        if not value:
            return False
        if isinstance(value, datetime.datetime):
            return value
        if isinstance(value, datetime.date):
            return datetime.datetime.combine(value, datetime.time.min)
        return datetime.datetime.strptime(value[:19], DATETIME_FORMAT)

    to_string = staticmethod(lambda v: v.strftime(DATETIME_FORMAT) if v else False)

    def convert_to_python(self, value, record):
        return self.to_datetime(value)

    def convert_to_write(self, value):
        if isinstance(value, datetime.datetime):
            return value.strftime(DATETIME_FORMAT)
        return value


class Binary(Field):
    type = "binary"


class Image(Binary):
    type = "image"

    def __init__(self, string=NO_VALUE, max_width=0, max_height=0, **kwargs):
        super().__init__(string, **kwargs)


class Json(Field):
    type = "json"


class Properties(Field):
    type = "properties"


class PropertiesDefinition(Field):
    type = "properties_definition"


class Reference(Selection):
    type = "reference"


class _Relational(Field):
    relational = True

    def __init__(self, comodel_name=NO_VALUE, string=NO_VALUE, **kwargs):
        super().__init__(string, **kwargs)
        self.comodel_name = None if comodel_name is NO_VALUE else comodel_name
        self.domain = kwargs.get("domain")
        self.context = kwargs.get("context")

    def to_spec(self):
        spec = super().to_spec()
        spec["comodel"] = self.comodel_name
        return spec


class Many2one(_Relational):
    type = "many2one"

    def __init__(self, comodel_name=NO_VALUE, string=NO_VALUE, ondelete=None, **kwargs):
        super().__init__(comodel_name, string, **kwargs)
        self.ondelete = ondelete or "set null"

    def to_spec(self):
        spec = super().to_spec()
        spec["ondelete"] = self.ondelete
        return spec

    def convert_to_python(self, value, record):
        # Un m2one se materializa como recordset del comodel (vacío si null).
        comodel = record.env[self.comodel_name]
        if not value:
            return comodel.browse(())
        if isinstance(value, (list, tuple)):   # formato (id, display_name)
            value = value[0]
        return comodel.browse((value,))

    def convert_to_write(self, value):
        if hasattr(value, "_ids"):
            return value._ids[0] if value._ids else False
        return value


class _RelationalMulti(_Relational):
    def convert_to_python(self, value, record):
        comodel = record.env[self.comodel_name]
        return comodel.browse(tuple(value or ()))

    def convert_to_write(self, value):
        if hasattr(value, "_ids"):
            return [(6, 0, list(value._ids))]
        return value


class One2many(_RelationalMulti):
    type = "one2many"

    def __init__(self, comodel_name=NO_VALUE, inverse_name=NO_VALUE, string=NO_VALUE, **kwargs):
        super().__init__(comodel_name, string, **kwargs)
        self.inverse_name = None if inverse_name is NO_VALUE else inverse_name
        self.store = False  # un o2m nunca tiene columna propia

    def to_spec(self):
        spec = super().to_spec()
        spec["inverse_name"] = self.inverse_name
        return spec


class Many2many(_RelationalMulti):
    type = "many2many"

    def __init__(self, comodel_name=NO_VALUE, relation=NO_VALUE, column1=NO_VALUE,
                 column2=NO_VALUE, string=NO_VALUE, **kwargs):
        super().__init__(comodel_name, string, **kwargs)
        self.relation = None if relation is NO_VALUE else relation
        self.column1 = None if column1 is NO_VALUE else column1
        self.column2 = None if column2 is NO_VALUE else column2

    def to_spec(self):
        spec = super().to_spec()
        if self.relation:
            spec["relation"] = self.relation
            spec["column1"] = self.column1
            spec["column2"] = self.column2
        return spec


# Comandos x2many con la API moderna de Odoo (Command.create, etc.)
class Command:
    CREATE = 0
    UPDATE = 1
    DELETE = 2
    UNLINK = 3
    LINK = 4
    CLEAR = 5
    SET = 6

    @classmethod
    def create(cls, values):
        return (cls.CREATE, 0, values)

    @classmethod
    def update(cls, id, values):
        return (cls.UPDATE, id, values)

    @classmethod
    def delete(cls, id):
        return (cls.DELETE, id, 0)

    @classmethod
    def unlink(cls, id):
        return (cls.UNLINK, id, 0)

    @classmethod
    def link(cls, id):
        return (cls.LINK, id, 0)

    @classmethod
    def clear(cls):
        return (cls.CLEAR, 0, 0)

    @classmethod
    def set(cls, ids):
        return (cls.SET, 0, ids)
