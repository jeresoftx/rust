# Error Handling con `Result`

## Intención

Error Handling con `Result` modela fallas esperadas como parte explícita de la firma de una función. En lugar de ocultar errores o usar valores mágicos, la API obliga a decidir qué hacer con el caso exitoso y con el caso fallido.

## Problema cotidiano

En sistemas reales muchas operaciones pueden fallar de forma normal:

- Un checkout puede fallar por inventario, pago rechazado o dirección inválida.
- Una llamada a infraestructura puede devolver errores técnicos que no conviene exponer a la capa de aplicación.
- Una validación puede necesitar acumular varios errores antes de responder al usuario.

Si todos los errores se tratan igual, el dominio pierde precisión y las capas se acoplan a detalles técnicos.

## Cómo se ve en Rust

Rust usa `Result<T, E>` para representar éxito o falla:

```rust
pub fn reserve_stock(quantity: u32) -> Result<(), CheckoutError> {
    if quantity == 0 {
        return Err(CheckoutError::EmptyCart);
    }

    Ok(())
}
```

El operador `?` propaga errores compatibles y permite escribir flujos lineales:

```rust
let payment = authorize_payment(order)?;
reserve_stock(order)?;
Ok(payment)
```

## Cuándo usarlo

- Cuando una falla es esperada y recuperable.
- Cuando quieres errores de dominio claros.
- Cuando una capa debe traducir errores técnicos a errores de aplicación.
- Cuando necesitas elegir entre validación fail-fast o validación acumulada.

## Cuándo evitarlo

- Si el problema representa un bug interno irrecuperable.
- Si una ausencia simple se expresa mejor con `Option`.
- Si estás creando un enum de error enorme sin fronteras de dominio claras.

## Diferencia con `panic!`

`panic!` señala estados inesperados o bugs. `Result` comunica errores esperados que el llamador debe manejar.

## Ejemplos

- [ ] Errores de dominio para checkout.
- [ ] Conversión de errores de infraestructura a errores de aplicación.
- [ ] Validación acumulada y validación fail-fast.

## Comandos

```bash
cargo test error_handling_result
```
