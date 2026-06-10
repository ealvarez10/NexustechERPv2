//! c_FormaPago — Catálogo SAT formas de pago CFDI 4.0
use super::{Catalogo, ClaveNoEncontrada};

pub static FORMAS_PAGO: &[Catalogo] = &[
    Catalogo { clave: "01", descripcion: "Efectivo" },
    Catalogo { clave: "02", descripcion: "Cheque nominativo" },
    Catalogo { clave: "03", descripcion: "Transferencia electrónica de fondos" },
    Catalogo { clave: "04", descripcion: "Tarjeta de crédito" },
    Catalogo { clave: "05", descripcion: "Monedero electrónico" },
    Catalogo { clave: "06", descripcion: "Dinero electrónico" },
    Catalogo { clave: "08", descripcion: "Vales de despensa" },
    Catalogo { clave: "12", descripcion: "Dación en pago" },
    Catalogo { clave: "13", descripcion: "Pago por subrogación" },
    Catalogo { clave: "14", descripcion: "Pago por consignación" },
    Catalogo { clave: "15", descripcion: "Condonación" },
    Catalogo { clave: "17", descripcion: "Compensación" },
    Catalogo { clave: "23", descripcion: "Novación" },
    Catalogo { clave: "24", descripcion: "Confusión" },
    Catalogo { clave: "25", descripcion: "Remisión de deuda" },
    Catalogo { clave: "26", descripcion: "Prescripción o caducidad" },
    Catalogo { clave: "27", descripcion: "A satisfacción del acreedor" },
    Catalogo { clave: "28", descripcion: "Tarjeta de débito" },
    Catalogo { clave: "29", descripcion: "Tarjeta de servicios" },
    Catalogo { clave: "30", descripcion: "Aplicación de anticipos" },
    Catalogo { clave: "31", descripcion: "Intermediario pagos" },
    Catalogo { clave: "99", descripcion: "Por definir" },
];

pub fn buscar(clave: &str) -> Result<&'static Catalogo, ClaveNoEncontrada> {
    FORMAS_PAGO.iter().find(|c| c.clave == clave)
        .ok_or_else(|| ClaveNoEncontrada(format!("c_FormaPago: {}", clave)))
}

pub fn es_valida(clave: &str) -> bool { buscar(clave).is_ok() }
