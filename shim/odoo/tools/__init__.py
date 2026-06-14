# Parte del shim RUSTOO: utilerías mínimas de odoo.tools que los addons
# importan con frecuencia. Diseñado para crecer según lo pidan los addons.
import itertools
from . import translate
from .translate import _

DEFAULT_SERVER_DATE_FORMAT = "%Y-%m-%d"
DEFAULT_SERVER_TIME_FORMAT = "%H:%M:%S"
DEFAULT_SERVER_DATETIME_FORMAT = DEFAULT_SERVER_DATE_FORMAT + " " + DEFAULT_SERVER_TIME_FORMAT


def float_round(value, precision_digits=None, precision_rounding=None, rounding_method="HALF-UP"):
    if value == 0:
        return 0.0
    if precision_digits is not None:
        precision_rounding = 10 ** -precision_digits
    if not precision_rounding:
        return value
    import math
    normalized = value / precision_rounding
    epsilon = 2 ** -52 * abs(normalized) * 10
    if rounding_method == "UP":
        rounded = math.ceil(abs(normalized) - epsilon) * (1 if value > 0 else -1)
    elif rounding_method == "DOWN":
        rounded = math.floor(abs(normalized) + epsilon) * (1 if value > 0 else -1)
    else:  # HALF-UP
        sign = 1 if normalized >= 0 else -1
        rounded = sign * math.floor(abs(normalized) + 0.5 + epsilon)
    return rounded * precision_rounding


def float_compare(value1, value2, precision_digits=None, precision_rounding=None):
    v1 = float_round(value1, precision_digits, precision_rounding)
    v2 = float_round(value2, precision_digits, precision_rounding)
    return 0 if v1 == v2 else (-1 if v1 < v2 else 1)


def float_is_zero(value, precision_digits=None, precision_rounding=None):
    return float_compare(value, 0.0, precision_digits, precision_rounding) == 0


def groupby(iterable, key=None):
    # La versión de Odoo NO exige el iterable preordenado: agrupa todo.
    groups = {}
    order = []
    for item in iterable:
        k = key(item) if key else item
        if k not in groups:
            groups[k] = []
            order.append(k)
        groups[k].append(item)
    return [(k, groups[k]) for k in order]


def ormcache(*args, **kwargs):
    # En el runtime híbrido la caché vive en Rust: passthrough transparente.
    def decorator(fn):
        return fn
    return decorator


ormcache_context = ormcache


class frozendict(dict):
    def __setitem__(self, key, value):
        raise TypeError("frozendict es inmutable")

    def __hash__(self):
        return hash(frozenset(self.items()))


def str2bool(value, default=None):
    value = str(value).lower()
    if value in ("1", "true", "yes", "t", "on"):
        return True
    if value in ("0", "false", "no", "f", "off", ""):
        return False
    if default is None:
        raise ValueError("no es un booleano: %r" % value)
    return default


def mute_logger(*loggers):
    def decorator(fn):
        return fn
    return decorator


def html_escape(text):
    import html
    return html.escape(text) if text else text


class SQL(str):
    """Marcador compatible con odoo.tools.SQL (consultas crudas)."""
    def __new__(cls, code="", *args, **kwargs):
        return super().__new__(cls, code)
