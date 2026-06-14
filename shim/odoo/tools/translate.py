# odoo.tools.translate: en el runtime híbrido la traducción real (i18n .po)
# vivirá en el kernel; por ahora _() interpola y devuelve la cadena fuente.
def _(source, *args, **kwargs):
    if args:
        return source % args
    if kwargs:
        return source % kwargs
    return source


class LazyTranslate:
    def __init__(self, module=None):
        self._module = module

    def __call__(self, source, *args, **kwargs):
        return _(source, *args, **kwargs)
