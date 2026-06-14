# SHIM RUSTOO: odoo.modules — descubrimiento, ordenación e importación de addons.
#
# Replica las fases 1-3 de la carga de Odoo:
#   Fase 1: escanear addons_path y leer cada __manifest__.py
#           (ast.literal_eval: el manifest es un dict literal; nunca se ejecuta)
#   Fase 2: orden topológico (Kahn) sobre 'depends', con auto_install
#   Fase 3: importlib.import_module('odoo.addons.<m>') — el import dispara la
#           MetaModel, que registra cada fragmento en el kernel Rust
#
# Las fases 4 (esquema SQL) y 5 (XML/CSV) son responsabilidad del kernel
# (F2 de la hoja de ruta): aquí solo se le notifica la ruta de datos.

import ast
import importlib
import os

import _nexus

from .. import addons as addons_namespace
from .. import models


class ModuleGraphError(Exception):
    pass


def discover(addons_paths):
    """Fase 1: {nombre: (ruta, manifest_dict)} para cada addon encontrado."""
    found = {}
    for base in addons_paths:
        base = os.path.abspath(base)
        if not os.path.isdir(base):
            continue
        for entry in sorted(os.listdir(base)):
            manifest_path = os.path.join(base, entry, "__manifest__.py")
            if not os.path.isfile(manifest_path):
                continue
            with open(manifest_path, encoding="utf-8") as fh:
                manifest = ast.literal_eval(fh.read())
            found[entry] = (os.path.join(base, entry), manifest)
    return found


def _topo_order(selected, available):
    """Fase 2: Kahn sobre depends, con expansión de auto_install."""
    # Expandir el cierre de dependencias de los módulos pedidos.
    needed = set()
    stack = list(selected)
    while stack:
        name = stack.pop()
        if name in needed:
            continue
        if name not in available:
            raise ModuleGraphError("addon no encontrado: '%s'" % name)
        needed.add(name)
        stack.extend(available[name][1].get("depends", []))

    # auto_install: si todas sus dependencias están, entra solo (sale_stock).
    changed = True
    while changed:
        changed = False
        for name, (_path, manifest) in available.items():
            if name in needed or not manifest.get("auto_install"):
                continue
            deps = manifest.get("depends", [])
            if deps and all(d in needed for d in deps):
                needed.add(name)
                changed = True

    indegree = {n: 0 for n in needed}
    dependents = {n: [] for n in needed}
    for name in needed:
        for dep in available[name][1].get("depends", []):
            indegree[name] += 1
            dependents[dep].append(name)

    ready = sorted(n for n, d in indegree.items() if d == 0)
    order = []
    while ready:
        current = ready.pop(0)
        order.append(current)
        added = False
        for child in dependents[current]:
            indegree[child] -= 1
            if indegree[child] == 0:
                ready.append(child)
                added = True
        if added:
            ready.sort()
    if len(order) != len(needed):
        cycle = sorted(n for n, d in indegree.items() if d > 0)
        raise ModuleGraphError("ciclo de dependencias entre: %s" % cycle)
    return order


def load_addons(addons_paths, modules=None):
    """Carga addons SIN MODIFICAR y construye el registry.

    1) descubre manifiestos, 2) ordena por depends, 3) importa cada paquete
    como odoo.addons.<m> (la MetaModel registra los fragmentos en Rust),
    4) pide a Rust la linealización y construye las clases dinámicas,
    5) notifica al kernel las rutas de datos (XML/CSV) de cada módulo.
    Devuelve el registry {model_name: clase}.
    """
    available = discover(addons_paths)
    selected = list(modules) if modules else list(available)
    order = _topo_order(selected, available)

    # Hacer visibles las rutas físicas dentro del namespace odoo.addons.
    for base in addons_paths:
        base = os.path.abspath(base)
        if base not in addons_namespace.__path__:
            addons_namespace.__path__.append(base)

    for name in order:
        path, manifest = available[name]
        _nexus.declare_module(name, list(manifest.get("depends", [])))
        models.set_current_module(name)
        importlib.import_module("odoo.addons.%s" % name)   # dispara MetaModel

    registry = models.build_registry()

    for name in order:
        path, manifest = available[name]
        # Fase 4+5 (esquema y datos XML/CSV): el kernel Rust las ejecuta.
        _nexus.load_module_data(name, path, list(manifest.get("data", [])))

    return registry
