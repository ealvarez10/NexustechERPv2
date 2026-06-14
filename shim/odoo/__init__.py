# SHIM RUSTOO: paquete `odoo` ficticio.
#
# Los addons reales hacen `from odoo import api, fields, models, _` y
# `from odoo.exceptions import UserError`. Este paquete replica esa superficie
# delegando toda la persistencia y el cómputo al kernel Rust vía `_nexus`.
#
# IMPORTANTE: este paquete NO es el framework de Odoo; es la capa de
# compatibilidad del runtime híbrido. Los addons se cargan sin modificar.

SUPERUSER_ID = 1

from . import exceptions          # noqa: E402,F401
from . import fields              # noqa: E402,F401
from . import api                 # noqa: E402,F401
from . import models              # noqa: E402,F401
from . import tools               # noqa: E402,F401
from .tools.translate import _    # noqa: E402,F401
from .fields import Command       # noqa: E402,F401


class _Release:
    # Algunos addons consultan odoo.release.version_info.
    version = "19.0"
    version_info = (19, 0, 0, "final", 0, "")
    serie = major_version = "19.0"


release = _Release()


def registry(*args, **kwargs):
    from .models import REGISTRY
    return REGISTRY
