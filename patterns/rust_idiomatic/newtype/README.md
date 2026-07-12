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
- [ ] Dinero y moneda sin mezclar unidades.
- [ ] Tipos seguros para email y token.

### IDs tipados

El módulo `typed_ids` modela `UserId`, `OrderId` y `ProductId` como tipos distintos.

El ejemplo muestra cómo validar prefijos de dominio y evitar que IDs con la misma representación se mezclen accidentalmente.

## Comandos

```bash
cargo test newtype
```
