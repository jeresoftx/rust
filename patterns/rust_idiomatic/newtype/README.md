# Newtype

## Intención

Newtype envuelve un valor primitivo o genérico en un tipo propio para darle significado de dominio y evitar mezclas accidentales.

## Problema cotidiano

En sistemas reales hay muchos valores que comparten la misma representación, pero no significan lo mismo:

- `UserId`, `OrderId` y `ProductId` podrían ser `String`, pero no deberían mezclarse.
- Dinero en centavos y moneda no deberían confundirse con cantidades, porcentajes o IDs.
- Un email validado y un token no deberían ser simples strings intercambiables.

Si todo se modela como `String`, `u64` o `i64`, el compilador no puede ayudarte a distinguir unidades ni conceptos de negocio.

## Cómo se ve en Rust

En Rust, Newtype suele escribirse como una tupla struct o una struct pequeña:

```rust
pub struct UserId(String);
```

El tipo puede exponer constructores, validaciones, métodos de acceso y traits como `Display`, `Clone`, `Eq` o `Hash`.

## Cuándo usarlo

- Cuando dos valores comparten representación, pero tienen significados distintos.
- Cuando quieres evitar mezclar unidades, IDs o estados validados.
- Cuando una validación debe ejecutarse antes de construir el valor.

## Cuándo evitarlo

- Si el tipo no agrega significado ni reglas.
- Si estás envolviendo todo sin beneficio claro.
- Si un enum expresa mejor el conjunto de valores permitidos.

## Diferencia con un alias

`type UserId = String` solo crea un alias. `struct UserId(String)` crea un tipo distinto que el compilador no mezcla con otros strings.

## Ejemplos

- [x] IDs tipados para usuario, orden y producto.
- [x] Dinero y moneda sin mezclar unidades.
- [x] Tipos seguros para email y token.

### IDs tipados

El módulo `typed_ids` modela `UserId`, `OrderId` y `ProductId` como tipos distintos.

El ejemplo muestra cómo validar prefijos de dominio y evitar que IDs con la misma representación se mezclen accidentalmente.

### Dinero y moneda

El módulo `money_currency` modela `CurrencyCode` y `Money` como newtypes de dominio.

El ejemplo muestra cómo formatear dinero y rechazar operaciones entre monedas distintas antes de producir totales incorrectos.

### Email y token seguros

El módulo `safe_email_token` modela `EmailAddress`, `AuthToken` y `LoginSession`.

El ejemplo muestra cómo normalizar email, validar tokens y construir sesiones solo con datos ya revisados.

## Comandos

```bash
cargo test newtype
```

## Medición y property testing

- Benchmarks: no aplica por ahora. Newtype enseña claridad semántica y validación de dominio; medir wrappers pequeños no aporta una decisión de diseño útil.
- Property testing: aplica. `tests/property_newtype_test.rs` genera monedas, montos y códigos para verificar que las invariantes de validación y suma se mantengan en rangos amplios.
