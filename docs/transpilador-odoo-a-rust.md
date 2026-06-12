# Reporte técnico — «odoo2rs»: conversión 100 % automática de módulos Odoo a NexusTech ERP v2

**Fecha:** 2026-06-10
**Pregunta:** ¿Existe alguna manera de lograr la conversión 100 % automática de un módulo Odoo (Python) a Rust/NexusTech, aunque sea reinventando la rueda?
**Respuesta corta:** Sí — pero solo si se redefine qué se traduce. Un transpilador AST→AST puro se estanca en ~85-90 % de cobertura. El 100 % literal se logra con una arquitectura híbrida de tres piezas: **(A)** reimplementar el kernel del ORM de Odoo en Rust (la rueda reinventada), **(B)** un transpilador que traduce el subconjunto declarativo y el Python "disciplinado" a Rust nativo, y **(C)** un intérprete Python embebido (RustPython) como vía de escape para el código residual intraducible, ejecutándose contra el mismo kernel. Lo traducido corre nativo; lo no traducido corre interpretado *dentro del binario Rust* — el módulo completo funciona desde el día uno y se va "nativizando" incrementalmente.

---

## 1. Por qué el análisis anterior decía "no viable" — y cuál es el truco

El bloqueo identificado antes era real: Odoo no es "código Python", es un **runtime metaprogramado** — registro dinámico de modelos, herencia por `_inherit` con MRO calculado en arranque, campos `compute` con grafo de dependencias, `safe_eval` de dominios, recordsets con sobrecarga de operadores. Traducir eso *estáticamente* a Rust idiomático es traducir un lenguaje a otro de paradigma incompatible.

El truco es **no traducir al paradigma de NexusTech, sino traer el paradigma de Odoo a Rust**. Si existe un crate `nexus-orm` que reproduce la semántica del ORM de Odoo (Environment, recordsets, registro de modelos, compute fields, dominios), entonces el transpilador ya no traduce paradigmas: traduce *sintaxis* Python a *llamadas equivalentes* sobre ese kernel. Eso sí es mecanizable.

Tres hechos del código actual hacen esto mucho más barato de lo que sería en frío:

1. **NexusTech ya usa el esquema de BD de Odoo.** `crates/nexus-core/src/db/sale_order.rs` consulta `sale_order`, `sale_order_line`, `res_partner`, `product_template` (con nombres i18n en jsonb: `pt.name->>'es_MX'`), `stock_picking`, `stock_move`, `res_currency`. No hay que mapear esquemas: el módulo Odoo transpilado lee y escribe las mismas tablas que ya existen. El problema más caro de cualquier migración (datos) **ya está resuelto por construcción**.
2. **La lógica de negocio actual ya es una retraducción manual de Odoo.** `sale_order::confirmar()` (sale_order.rs:315) replica a mano `action_confirm` de Odoo: cambia estado, crea el picking y los `stock_move`. Es la prueba de que la semántica es portable — hoy se porta a mano, el transpilador la portaría sola.
3. **Handlers y frontend ya son genéricos/plantillables.** `app/src/handlers/products.rs` y `partners.rs` son mecánicos (listar/obtener/crear contra `db::*`); el frontend tiene `form_view.js` y `kanban_view.js`. Las vistas XML de Odoo se pueden compilar a descriptores JSON que alimenten esos mismos componentes.

---

## 2. Arquitectura del sistema

```
                    ┌─────────────────────────────────────────────┐
                    │              módulo Odoo (addon)             │
                    │  __manifest__.py  models/*.py  views/*.xml   │
                    │  security/*.csv   data/*.xml   wizard/*.py   │
                    └──────────────────────┬──────────────────────┘
                                           │
                           ┌───────────────▼───────────────┐
                           │   FASE 1 · Frontend (parser)   │
                           │ rustpython-parser → AST Python │
                           │ roxmltree → AST XML de vistas  │
                           └───────────────┬───────────────┘
                                           │
                           ┌───────────────▼───────────────┐
                           │   FASE 2 · OdooIR (IR semántica)│
                           │ modelos, campos, _inherit,      │
                           │ @api.depends, dominios, métodos │
                           │ como mini-AST tipado            │
                           └───────┬───────────────┬────────┘
                                   │               │
                  análisis estático│               │ no inferible
                  (tipado abstracto)│              │ (eval, **kwargs,
                                   │               │  reflexión, libs)
                    ┌──────────────▼─────┐   ┌────▼─────────────────┐
                    │ FASE 3a · Codegen   │   │ FASE 3b · Empaquetado │
                    │ Rust (quote +       │   │ del bytecode Python   │
                    │ prettyplease)       │   │ para el intérprete    │
                    └──────────────┬─────┘   └────┬─────────────────┘
                                   │              │
        ┌──────────────────────────▼──────────────▼──────────────────────┐
        │                  RUNTIME (la rueda reinventada)                 │
        │                                                                 │
        │  nexus-orm  ──  kernel ORM compatible-Odoo en Rust              │
        │   · Registry de modelos (arranque, como Odoo)                   │
        │   · Recordset<M> + OVal (valores dinámicos)                     │
        │   · cadena _inherit / MRO como vtable encadenada                │
        │   · compute fields + grafo @api.depends (petgraph)              │
        │   · parser de dominios [('state','=','sale')] → SQL (sqlx)      │
        │   · ir.model.access / record rules                              │
        │                                                                 │
        │  nexus-pyvm  ──  RustPython embebido, puenteado a nexus-orm     │
        │   (ejecuta los métodos que la FASE 3a no pudo traducir)         │
        │                                                                 │
        │  nexus-gateway ── handlers Axum generados:                      │
        │   CRUD genérico + /api/v1/orm/{modelo}/{metodo} (≈ call_kw)     │
        │   montados junto a las rutas existentes de app/src/main.rs      │
        │                                                                 │
        │  vistas XML → JSON → form_view.js / kanban_view.js existentes   │
        └─────────────────────────────────────────────────────────────────┘
```

---

## 3. Componente A — `nexus-orm`: el kernel de Odoo reimplementado en Rust

Es la pieza grande y la que justifica la expresión "reinventar la rueda". Decisiones de diseño:

### 3.1 Tipado dinámico controlado: el enum `OVal`

La fidelidad semántica exige renunciar al tipado estático *dentro* del kernel (igual que Odoo renuncia a él):

```rust
pub enum OVal {
    Null,                      // False de Odoo en campos relacionales/char
    Bool(bool),
    Int(i64),
    Float(f64),
    Decimal(rust_decimal::Decimal),
    Str(SmolStr),
    Date(NaiveDate), DateTime(NaiveDateTime),
    Ref(ModelId, RecordId),    // many2one
    RefSet(ModelId, Vec<RecordId>),  // one2many / many2many
    Json(serde_json::Value),   // jsonb i18n (product_template.name ya lo es)
}
```

Un `Recordset` es `(Arc<Env>, ModelId, Vec<RecordId>)` con caché de campos por transacción — exactamente la estructura de `odoo.models.BaseModel`. Las operaciones de recordset (`mapped`, `filtered`, `sorted`, `browse`, `|`, `&`, indexación) se implementan una vez en el kernel.

Encima del kernel dinámico, el codegen emite **fachadas tipadas** opcionales (structs como el actual `SaleOrder` de sale_order.rs:12 con `Option<T>` por campo) para que el código nuevo escrito a mano siga siendo Rust idiomático. El código transpilado usa el kernel; el código humano usa la fachada.

### 3.2 Registro de modelos y herencia `_inherit`

Odoo construye clases en arranque combinando todas las definiciones de un `_name` según el orden de carga de módulos. Réplica en Rust:

- Cada "fragmento de modelo" transpilado se registra con el crate `inventory` (registro en tiempo de link, sin `main` manual).
- El Registry resuelve en arranque la cadena de fragmentos por modelo → una **vtable encadenada**: `Vec<Arc<dyn ModelFragment>>`. Llamar `action_confirm` recorre la cadena desde el fragmento más derivado; `super()` de Python se traduce a "invocar el siguiente eslabón" (`ctx.call_super(...)`). Esto reproduce el MRO de Odoo sin metaclases.
- `_inherits` (delegación) se modela como JOIN automático, que el esquema ya usa (`product_product` → `product_template`).

### 3.3 Campos computados, onchange y dominios

- `@api.depends('order_line.price_subtotal')` es **declarativo** → se extrae al IR y se materializa como grafo de dependencias (petgraph) que el kernel usa para invalidar/recalcular, igual que hace hoy a mano `_recalcular_totales()` (sale_order.rs:608). De hecho, `_recalcular_totales` es literalmente lo que el kernel haría solo si `amount_total` estuviera declarado como compute.
- Los dominios `[('state','!=','cancel'), ('company_id','=',cid)]` son un mini-lenguaje cerrado → parser propio que compila a SQL parametrizado sqlx. El `listar()` actual con su `Vec<String>` de condiciones y `$idx` incrementales (sale_order.rs:179-214) es exactamente el output de ese compilador de dominios, escrito a mano.

### 3.4 Seguridad y datos

`ir.model.access.csv` y las record rules son datos declarativos → se cargan al Registry y el kernel los aplica en `search/read/write`, integrándose con el `JwtClaims.company_id` que ya fluye por el middleware actual.

---

## 4. Componente B — el transpilador `odoo2rs`

### 4.1 Lo que es ~100 % mecánico (la mayoría del módulo)

| Artefacto Odoo | Técnica | Destino |
|---|---|---|
| `fields.Char/Many2one/...` | extracción declarativa del AST | metadatos del Registry + fachada struct |
| `_name`, `_inherit`, `_order`, `_sql_constraints` | declarativo | Registry |
| `@api.depends/constrains/onchange` | declarativo | grafo de dependencias |
| vistas XML (form/tree/kanban) | roxmltree → JSON | descriptores para `form_view.js`/`kanban_view.js` |
| `security/*.csv`, `data/*.xml` | declarativo | seeds SQL / Registry |
| `__manifest__.py` (depends) | declarativo | orden de carga del Registry |

En un addon típico esto es el 60-70 % de los archivos y no requiere "entender" Python: son estructuras de datos disfrazadas de código.

### 4.2 Cuerpos de método: traducción de un subconjunto de Python

Parser: **`rustpython-parser`** (crate maduro, AST completo de Python 3). Sobre el AST se hace **interpretación abstracta** para inferir, por variable, si es recordset (y de qué modelo), escalar, o desconocido. La clave: en código Odoo el tipo de `self` es conocido (el modelo), `self.partner_id` se tipa por los metadatos del campo, y `mapped/filtered/browse` preservan tipos. La inferencia se propaga lejos.

Reglas de traducción (muestra):

```python
# Odoo
def action_confirm(self):
    for order in self:
        if order.state not in ('draft', 'sent'):
            raise UserError(_("Solo borradores"))
        order.state = 'sale'
        order._create_delivery()
```
```rust
// generado — corre contra nexus-orm
fn action_confirm(env: &Env, self_: Recordset) -> OResult<OVal> {
    for order in self_.iter() {
        if !matches!(order.get_str("state")?, "draft" | "sent") {
            return Err(OError::user("Solo borradores"));
        }
        order.set("state", OVal::from("sale"))?;
        env.call(&order, "_create_delivery", &[])?;
    }
    Ok(OVal::Null)
}
```

No es Rust "bonito" — es Rust *correcto y compilable* que reproduce la semántica. La belleza llega después, refactorizando hacia la fachada tipada (y eso ya es opcional).

Construcciones cubiertas por el subconjunto: asignación/lectura de campos, recordset ops, aritmética (con `Decimal` donde el campo es Monetary — el codegen lo sabe por los metadatos), f-strings, `raise UserError/ValidationError`, comprehensions → iteradores, `with` de savepoints → transacciones sqlx, llamadas a otros métodos del registro. Esto cubre empíricamente el grueso del código de negocio de addons normales.

### 4.3 Biblioteca de shims: `nexus-pyshim`

Odoo usa un vocabulario sorprendentemente pequeño de stdlib/terceros: `datetime/dateutil.relativedelta`, `json`, `re`, `base64`, `lxml` (poco en modelos), `float_round/float_compare` de `odoo.tools`. Un crate de ~40-60 funciones con semántica idéntica (¡incluido el redondeo half-up de `float_round`!) elimina la cola larga de imports. Reinventar esa rueda es trabajo finito y testeable función por función contra CPython.

---

## 5. Componente C — el seguro de 100 %: RustPython embebido

Por mucho subconjunto que se cubra, siempre habrá un método con `getattr` dinámico, `exec`, una lib exótica o un decorador raro. Para que la promesa sea **100 % automática y no 90 %**, el residuo no se rechaza: se **empaqueta**.

- El transpilador marca cada método como `NATIVE` (traducido) o `INTERP` (no traducible, con el motivo).
- Los `INTERP` se embeben como fuente/bytecode y se ejecutan con **RustPython** (VM de Python en Rust puro, embebible, sin CPython) dentro del mismo proceso.
- El puente expone `self`/`env` de `nexus-orm` como objetos Python: `order.partner_id.name` dentro de la VM llama al mismo kernel, la misma caché, la misma transacción sqlx. Un método nativo puede llamar a uno interpretado y viceversa (la vtable encadenada no distingue).

Consecuencias:

- **Día uno:** cualquier addon corre completo (parte nativa rápida, parte interpretada lenta pero correcta).
- **Mejora monótona:** cada versión del transpilador convierte más métodos `INTERP` → `NATIVE`. El reporte de cobertura (`odoo2rs report`) dice exactamente qué falta y por qué.
- Es la misma jugada histórica de los transpiladores serios (RPython, GWT, Js_of_ocaml, py2many): subconjunto compilado + runtime que absorbe el resto.

---

## 6. Verificación: cómo se *demuestra* el "100 %"

Sin esto, el proyecto es fe. Con esto, es ingeniería:

1. **Testing diferencial**: el mismo escenario (crear orden → agregar líneas → confirmar → facturar) se ejecuta contra un Odoo real y contra el binario NexusTech transpilado, **sobre la misma BD inicial** (posible porque comparten esquema). Se diffean los estados finales de las tablas. Cualquier divergencia es un bug del kernel o del codegen.
2. **Transpilar la suite de tests del addon**: los `tests/test_*.py` de Odoo pasan por el mismo pipeline (los asserts son Python simple → casi siempre `NATIVE`).
3. **Oráculo de redondeo**: suite específica para `Decimal` vs float de Python en montos — la fuente nº 1 de divergencias silenciosas.

---

## 7. Plan de construcción y esfuerzo

| Fase | Entregable | Esfuerzo estimado |
|---|---|---|
| 1 | `nexus-orm` mínimo: OVal, Recordset, Registry, CRUD, dominios→SQL | 6-8 semanas |
| 2 | Extractor declarativo (campos, herencia, vistas→JSON, security) | 3-4 semanas |
| 3 | Traductor de métodos (subconjunto) + inferencia abstracta + shims | 8-12 semanas |
| 4 | Puente RustPython + empaquetado `INTERP` | 4-6 semanas |
| 5 | Gateway Axum generado + integración con `main.rs` y frontend actual | 2-3 semanas |
| 6 | Harness diferencial + transpilación de `sale` como módulo piloto | 4 semanas |

**Total: ~7-9 meses-persona** para el primer módulo real corriendo 100 % automático, con el kernel reutilizable para todos los siguientes (el módulo n+1 cuesta horas, no meses).

### Riesgos principales

- **Fidelidad del kernel** (caché/invalidación de computes, orden de flush): es donde Odoo tiene 15 años de bugs resueltos. Mitigación: testing diferencial desde la fase 1.
- **Rendimiento del modo `INTERP`**: RustPython es 5-20× más lento que CPython en algunos cargas. Aceptable porque es transitorio y solo afecta métodos residuales.
- **Tentación de fidelidad infinita**: el objetivo es compatibilidad con *addons de negocio*, no con `base`/`web` enteros. Hay que fijar la frontera explícitamente (p. ej., sin soporte de `ir.actions.server` con código arbitrario en v1... que igualmente caería en `INTERP`).

---

## 8. Conclusión

La conversión 100 % automática **es alcanzable**, pero no como "traductor de Python a Rust": esa formulación es la que correctamente se descartó. Es alcanzable como **plataforma de ejecución compatible con Odoo escrita en Rust** (`nexus-orm` + RustPython embebido) más un transpilador que nativiza todo lo nativizable. La rueda que se reinventa no es el lenguaje — es el runtime. Y NexusTech v2 parte con dos ventajas que nadie más tiene: ya vive sobre el esquema de datos de Odoo y ya reimplementó a mano (en `db/sale_order.rs` y compañía) exactamente la semántica que el kernel automatizaría — ese código manual es, a la vez, la especificación y el test de referencia del kernel.
