#!/usr/bin/env python3
"""Suite de validación del shim RUSTOO (corre contra el stub _nexus en memoria).

Verifica los comportamientos críticos del runtime híbrido:
  1. Carga de addons SIN MODIFICAR (manifest + topo + import)
  2. auto_install (sale_mini_ext entra solo, como sale_stock en Odoo)
  3. MRO dinámico: super() nativo de Python cruza módulos en orden Odoo
  4. Campos como descriptores + caché + prefetch por lote
  5. @api.depends: computes en cadena (línea -> subtotal -> total del pedido)
  6. Invalidación de dependientes al escribir
  7. Comandos x2many (0,0,vals) en create
  8. @api.constrains con ValidationError
  9. @api.model_create_multi normalizando dict -> [dict]
 10. mapped / filtered / sorted / sudo / with_context
 11. Despacho a implementaciones nativas del kernel (has_native/call_native)
"""

import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "shim"))

import _nexus                                          # noqa: E402
import odoo                                            # noqa: E402
from odoo import api                                   # noqa: E402
from odoo.exceptions import UserError, ValidationError  # noqa: E402
from odoo.modules import load_addons                   # noqa: E402

PASS = []


def check(label, condition):
    status = "OK " if condition else "FAIL"
    print("  [%s] %s" % (status, label))
    PASS.append(bool(condition))


print("== 1-2. Carga de addons sin modificar (con auto_install) ==")
registry = load_addons(
    [os.path.join(HERE, "demo_addons")],
    ["sale_mini"],   # base_mini entra por depends; sale_mini_ext por auto_install
)
check("registry contiene res.partner / sale.order / sale.order.line",
      {"res.partner", "sale.order", "sale.order.line"} <= set(registry))
check("auto_install metió el campo de sale_mini_ext",
      "delivery_note" in registry["sale.order"]._fields)

env = api.Environment(_nexus.env_new(1, {}), odoo.SUPERUSER_ID, {})

print("== 3. MRO dinámico ==")
mro_names = [c.__name__ for c in registry["sale.order"].__mro__]
check("orden: ext -> core -> Model -> BaseModel",
      mro_names.index("SaleOrder") < mro_names.index("Model") < mro_names.index("BaseModel")
      and mro_names.count("SaleOrder") == 2)

print("== 4-7. ORM: create con comandos, computes, prefetch ==")
partner = env["res.partner"].create({"name": "Acme", "email": "hola@acme.mx"})
check("create devuelve recordset con id", partner.id == 1)
check("compute no almacenado (display_info)", partner.display_info == "Acme <hola@acme.mx>")

order = env["sale.order"].create({
    "partner_id": partner.id,
    "order_line": [
        (0, 0, {"name": "Silla", "price_unit": 10.0, "product_uom_qty": 3}),
        (0, 0, {"name": "Mesa", "price_unit": 5.0}),
    ],
})
check("@api.model_create_multi normalizó dict y puso folio", order.name == "SO001")
check("default literal del Selection", order.state == "draft")
check("one2many desde el campo inverso", len(order.order_line) == 2)
check("compute en cadena (subtotales -> total): 10*3 + 5*1 = 35",
      order.amount_total == 35.0)
check("many2one materializado como recordset", order.partner_id.name == "Acme")

print("== 6. Invalidación de dependientes ==")
first_line = order.order_line.sorted()[0]
first_line.write({"product_uom_qty": 5})
check("al escribir la línea, el total del pedido se recalcula (10*5+5=55)",
      order.amount_total == 55.0)

print("== 3b. super() entre módulos (sale_mini_ext -> sale_mini) ==")
order.action_confirm()
check("la cadena corrió en orden Odoo", order.confirmation_log == "sale_mini+sale_mini_ext")
check("estado confirmado por el core", order.state == "sale")
check("campo del módulo de extensión escrito", order.delivery_note == "preparar envio")

print("== 8. @api.constrains ==")
try:
    first_line.write({"price_unit": -1})
    check("constrains rechaza precio negativo", False)
except ValidationError:
    check("constrains rechaza precio negativo", True)

print("== UserError de negocio ==")
empty_order = env["sale.order"].create({"partner_id": partner.id})
try:
    empty_order.action_confirm()
    check("UserError al confirmar sin líneas", False)
except UserError:
    check("UserError al confirmar sin líneas", True)

print("== 10. Utilidades de recordset ==")
lines = order.order_line
check("mapped('name')", sorted(lines.mapped("name")) == ["Mesa", "Silla"])
check("mapped relacional devuelve recordset", lines.mapped("order_id") == order)
check("filtered por lambda", len(lines.filtered(lambda l: l.price_unit > 6)) == 1)
check("sorted por campo", lines.sorted("price_unit")[0].name == "Mesa")
check("sudo / with_context preservan ids",
      order.sudo().with_context(lang="es_MX").id == order.id)
check("search con dominio", env["sale.order"].search([("state", "=", "sale")]) == order)
check("search_count", env["sale.order"].search_count([]) == 2)

print("== 11. Despacho a nativas del kernel ==")
_nexus.register_native(
    "sale.order", "action_fast_total",
    lambda handle, model, ids: sum(
        _nexus.read_batch(handle, model, ids, ["amount_total"])[i]["amount_total"] or 0
        for i in ids
    ),
)
# El cache del shim tiene el total (store=True se materializa vía compute);
# para la nativa, escribimos el valor almacenado simulando al kernel:
_nexus.write(1, "sale.order", [order.id], {"amount_total": order.amount_total})
check("método inexistente en Python cae en el kernel (has_native)",
      order.action_fast_total() == 55.0)

print()
total, good = len(PASS), sum(PASS)
print("RESULTADO: %d/%d pruebas correctas" % (good, total))
sys.exit(0 if good == total else 1)
