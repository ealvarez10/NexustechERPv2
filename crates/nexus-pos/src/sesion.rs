//! Carrito de compra y sesión de caja en memoria
//! No requiere persistencia hasta que se finaliza la venta

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};
use crate::error::PosError;

/// Ítem en el carrito activo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCarrito {
    pub product_id:     i32,
    pub nombre:         String,
    pub cantidad:       Decimal,
    pub precio_unitario: Decimal,
    pub descuento_pct:  Decimal,
}

impl ItemCarrito {
    /// Crea un nuevo ítem sin descuento
    pub fn nuevo(product_id: i32, nombre: impl Into<String>, precio: Decimal) -> Self {
        Self {
            product_id,
            nombre: nombre.into(),
            cantidad: dec!(1),
            precio_unitario: precio,
            descuento_pct: Decimal::ZERO,
        }
    }

    /// Calcula el subtotal de este ítem
    pub fn subtotal(&self) -> Decimal {
        let factor = dec!(1) - self.descuento_pct / dec!(100);
        (self.cantidad * self.precio_unitario * factor).round_dp(2)
    }
}

/// Carrito de compra activo en la sesión POS
#[derive(Debug, Clone, Serialize)]
pub struct Carrito {
    pub items:            Vec<ItemCarrito>,
    pub descuento_global: Decimal,
    pub tasa_iva:         Decimal,
    pub creado_en:        DateTime<Local>,
}

impl Default for Carrito {
    fn default() -> Self { Self::nuevo() }
}

impl Carrito {
    /// Crea un carrito vacío con IVA 16% (México)
    pub fn nuevo() -> Self {
        Self {
            items: Vec::new(),
            descuento_global: Decimal::ZERO,
            tasa_iva: dec!(16),
            creado_en: Local::now(),
        }
    }

    /// Agrega o incrementa cantidad de un producto
    pub fn agregar(&mut self, item: ItemCarrito) {
        if let Some(existing) = self.items.iter_mut().find(|i| i.product_id == item.product_id) {
            existing.cantidad += item.cantidad;
        } else {
            self.items.push(item);
        }
    }

    /// Remueve un producto del carrito
    pub fn remover(&mut self, product_id: i32) {
        self.items.retain(|i| i.product_id != product_id);
    }

    /// Subtotal antes de IVA (con descuentos por línea)
    pub fn subtotal(&self) -> Decimal {
        let sub: Decimal = self.items.iter().map(|i| i.subtotal()).sum();
        if self.descuento_global > Decimal::ZERO {
            let factor = dec!(1) - self.descuento_global / dec!(100);
            (sub * factor).round_dp(2)
        } else {
            sub
        }
    }

    /// IVA calculado sobre el subtotal
    pub fn iva(&self) -> Decimal {
        (self.subtotal() * self.tasa_iva / dec!(100)).round_dp(2)
    }

    /// Total a pagar
    pub fn total(&self) -> Decimal {
        self.subtotal() + self.iva()
    }

    /// Cuenta de artículos
    pub fn num_items(&self) -> usize {
        self.items.len()
    }

    /// Calcula el cambio a devolver
    pub fn cambio(&self, pago_recibido: Decimal) -> Result<Decimal, PosError> {
        let total = self.total();
        if pago_recibido < total {
            Err(PosError::PagoInsuficiente {
                recibido: pago_recibido.to_string(),
                total: total.to_string(),
            })
        } else {
            Ok((pago_recibido - total).round_dp(2))
        }
    }

    /// Limpia el carrito para una nueva venta
    pub fn limpiar(&mut self) {
        self.items.clear();
        self.descuento_global = Decimal::ZERO;
        self.creado_en = Local::now();
    }

    /// True si el carrito tiene al menos un ítem
    pub fn tiene_items(&self) -> bool {
        !self.items.is_empty()
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn item(id: i32, precio: Decimal) -> ItemCarrito {
        ItemCarrito::nuevo(id, format!("Producto {id}"), precio)
    }

    #[test]
    fn test_carrito_vacio() {
        let c = Carrito::nuevo();
        assert_eq!(c.subtotal(), Decimal::ZERO);
        assert_eq!(c.total(), Decimal::ZERO);
        assert!(!c.tiene_items());
    }

    #[test]
    fn test_agregar_item() {
        let mut c = Carrito::nuevo();
        c.agregar(item(1, dec!(100.00)));
        assert_eq!(c.num_items(), 1);
        assert_eq!(c.subtotal(), dec!(100.00));
    }

    #[test]
    fn test_agregar_mismo_producto_acumula_cantidad() {
        let mut c = Carrito::nuevo();
        c.agregar(item(1, dec!(50.00)));
        c.agregar(item(1, dec!(50.00)));
        assert_eq!(c.num_items(), 1);
        assert_eq!(c.items[0].cantidad, dec!(2));
        assert_eq!(c.subtotal(), dec!(100.00));
    }

    #[test]
    fn test_remover_item() {
        let mut c = Carrito::nuevo();
        c.agregar(item(1, dec!(100.00)));
        c.agregar(item(2, dec!(200.00)));
        c.remover(1);
        assert_eq!(c.num_items(), 1);
        assert_eq!(c.subtotal(), dec!(200.00));
    }

    #[test]
    fn test_iva_16_pct() {
        let mut c = Carrito::nuevo();
        c.agregar(item(1, dec!(1000.00)));
        assert_eq!(c.iva(), dec!(160.00));
        assert_eq!(c.total(), dec!(1160.00));
    }

    #[test]
    fn test_cambio_correcto() {
        let mut c = Carrito::nuevo();
        c.agregar(item(1, dec!(100.00)));
        // Total = $100 + $16 IVA = $116
        let cambio = c.cambio(dec!(200.00)).unwrap();
        assert_eq!(cambio, dec!(84.00));
    }

    #[test]
    fn test_cambio_insuficiente() {
        let mut c = Carrito::nuevo();
        c.agregar(item(1, dec!(500.00)));
        // Total = $580, pago = $100 — insuficiente
        assert!(c.cambio(dec!(100.00)).is_err());
    }

    #[test]
    fn test_descuento_global() {
        let mut c = Carrito::nuevo();
        c.agregar(item(1, dec!(1000.00)));
        c.descuento_global = dec!(10); // 10% descuento global
        // subtotal = $900, IVA = $144, total = $1044
        assert_eq!(c.subtotal(), dec!(900.00));
        assert_eq!(c.iva(), dec!(144.00));
        assert_eq!(c.total(), dec!(1044.00));
    }

    #[test]
    fn test_limpiar_carrito() {
        let mut c = Carrito::nuevo();
        c.agregar(item(1, dec!(100.00)));
        c.limpiar();
        assert!(!c.tiene_items());
        assert_eq!(c.total(), Decimal::ZERO);
    }
}
