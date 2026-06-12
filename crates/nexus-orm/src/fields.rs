//! Metadatos de campos — el equivalente de `odoo.fields.*`.
//!
//! El transpilador (FASE 2) extrae las declaraciones `fields.Char(...)`,
//! `fields.Many2one(...)`, `@api.depends(...)` del AST de Python y las
//! materializa como `FieldDef`. El kernel las usa para:
//!  - compilar dominios a SQL (tipo de columna),
//!  - decodificar filas de Postgres a `OVal`,
//!  - construir el grafo de dependencias de campos computados.

use crate::value::OVal;

/// Tipo de campo de Odoo.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Boolean,
    Integer,
    Float,
    /// numeric en BD; siempre `Decimal` en el kernel (nunca f64).
    Monetary,
    Char,
    Text,
    Html,
    Selection,
    Date,
    Datetime,
    Binary,
    /// jsonb (i18n como `product_template.name`).
    Json,
    Many2one {
        comodel: String,
    },
    One2many {
        comodel: String,
        /// Campo many2one inverso en el comodelo.
        inverse: String,
    },
    Many2many {
        comodel: String,
        relation: Option<String>,
        column1: Option<String>,
        column2: Option<String>,
    },
}

impl FieldType {
    pub fn is_relational(&self) -> bool {
        matches!(
            self,
            FieldType::Many2one { .. } | FieldType::One2many { .. } | FieldType::Many2many { .. }
        )
    }

    /// Nombre del comodelo si el campo es relacional.
    pub fn comodel(&self) -> Option<&str> {
        match self {
            FieldType::Many2one { comodel }
            | FieldType::One2many { comodel, .. }
            | FieldType::Many2many { comodel, .. } => Some(comodel),
            _ => None,
        }
    }
}

/// Declaración de campo computado (`compute=` + `@api.depends`).
#[derive(Debug, Clone, PartialEq)]
pub struct ComputeDef {
    /// Método del modelo que calcula el campo (p. ej. `_compute_amounts`).
    pub method: String,
    /// Dependencias declaradas. Las rutas con punto
    /// (`order_line.price_subtotal`) cruzan modelos y se resuelven en una
    /// fase posterior; las simples disparan recálculo intra-modelo.
    pub depends: Vec<String>,
}

/// Definición completa de un campo — la fila del «ir.model.fields» en memoria.
#[derive(Debug, Clone)]
pub struct FieldDef {
    pub name: String,
    pub ftype: FieldType,
    /// Etiqueta visible (`string=` de Odoo).
    pub string: String,
    pub required: bool,
    pub readonly: bool,
    /// `store=` — si el campo vive como columna en la tabla.
    pub store: bool,
    /// Opciones de Selection: (valor, etiqueta).
    pub selection: Vec<(String, String)>,
    pub default: Option<OVal>,
    pub compute: Option<ComputeDef>,
    /// `related=` (ruta con puntos), pendiente de resolución.
    pub related: Option<String>,
}

impl FieldDef {
    pub fn new(name: &str, ftype: FieldType) -> Self {
        FieldDef {
            name: name.to_string(),
            string: name.to_string(),
            ftype,
            required: false,
            readonly: false,
            store: true,
            selection: Vec::new(),
            default: None,
            compute: None,
            related: None,
        }
    }

    // Constructores estilo `fields.*` de Odoo ──────────────────────────────

    pub fn boolean(name: &str) -> Self {
        Self::new(name, FieldType::Boolean)
    }
    pub fn integer(name: &str) -> Self {
        Self::new(name, FieldType::Integer)
    }
    pub fn float(name: &str) -> Self {
        Self::new(name, FieldType::Float)
    }
    pub fn monetary(name: &str) -> Self {
        Self::new(name, FieldType::Monetary)
    }
    pub fn char(name: &str) -> Self {
        Self::new(name, FieldType::Char)
    }
    pub fn text(name: &str) -> Self {
        Self::new(name, FieldType::Text)
    }
    pub fn html(name: &str) -> Self {
        Self::new(name, FieldType::Html)
    }
    pub fn date(name: &str) -> Self {
        Self::new(name, FieldType::Date)
    }
    pub fn datetime(name: &str) -> Self {
        Self::new(name, FieldType::Datetime)
    }
    pub fn json(name: &str) -> Self {
        Self::new(name, FieldType::Json)
    }

    pub fn selection(name: &str, options: &[(&str, &str)]) -> Self {
        let mut f = Self::new(name, FieldType::Selection);
        f.selection = options
            .iter()
            .map(|(v, l)| (v.to_string(), l.to_string()))
            .collect();
        f
    }

    pub fn many2one(name: &str, comodel: &str) -> Self {
        Self::new(
            name,
            FieldType::Many2one {
                comodel: comodel.to_string(),
            },
        )
    }

    pub fn one2many(name: &str, comodel: &str, inverse: &str) -> Self {
        let mut f = Self::new(
            name,
            FieldType::One2many {
                comodel: comodel.to_string(),
                inverse: inverse.to_string(),
            },
        );
        f.store = false; // nunca es columna propia
        f
    }

    pub fn many2many(name: &str, comodel: &str) -> Self {
        Self::new(
            name,
            FieldType::Many2many {
                comodel: comodel.to_string(),
                relation: None,
                column1: None,
                column2: None,
            },
        )
    }

    // Builder encadenable ──────────────────────────────────────────────────

    pub fn string(mut self, label: &str) -> Self {
        self.string = label.to_string();
        self
    }
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    pub fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }
    pub fn default_val(mut self, v: impl Into<OVal>) -> Self {
        self.default = Some(v.into());
        self
    }

    /// Marca el campo como computado (`compute=` + `@api.depends`).
    /// Por defecto NO almacenado, como en Odoo; encadenar `.stored()` para
    /// el equivalente de `store=True`.
    pub fn computed(mut self, method: &str, depends: &[&str]) -> Self {
        self.compute = Some(ComputeDef {
            method: method.to_string(),
            depends: depends.iter().map(|d| d.to_string()).collect(),
        });
        self.store = false;
        self
    }

    pub fn stored(mut self) -> Self {
        self.store = true;
        self
    }

    /// ¿El campo existe como columna física en la tabla del modelo?
    /// (one2many/many2many nunca; computados solo con `store=True`).
    pub fn is_column(&self) -> bool {
        !matches!(
            self.ftype,
            FieldType::One2many { .. } | FieldType::Many2many { .. }
        ) && self.store
    }
}
