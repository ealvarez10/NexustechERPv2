# Parte del shim RUSTOO: excepciones con la misma jerarquía que odoo.exceptions.
# Los addons hacen `from odoo.exceptions import UserError, ValidationError`.


class UserError(Exception):
    """Error de negocio mostrable al usuario final."""

    def __init__(self, message):
        super().__init__(message)


class RedirectWarning(Exception):
    def __init__(self, message, action, button_text, additional_context=None):
        super().__init__(message, action, button_text, additional_context)


class AccessDenied(UserError):
    def __init__(self, message="Access Denied"):
        super().__init__(message)


class AccessError(UserError):
    pass


class CacheMiss(KeyError):
    pass


class MissingError(UserError):
    pass


class ValidationError(UserError):
    pass
