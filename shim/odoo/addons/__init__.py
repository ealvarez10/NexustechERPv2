# Paquete namespace para los addons: el loader añade aquí las rutas físicas
# (odoo.addons.__path__.append(ruta)) y los módulos se importan SIN MODIFICAR
# como odoo.addons.<nombre>, igual que en Odoo.
import os

__path__ = [os.path.dirname(__file__)]
