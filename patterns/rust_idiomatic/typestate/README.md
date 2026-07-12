# Typestate

## Intención

Typestate modela los estados válidos de un flujo como tipos distintos. En lugar de revisar todo en tiempo de ejecución, el compilador evita llamar operaciones que no existen para el estado actual.

## Problema cotidiano

En sistemas reales hay operaciones que solo son válidas después de completar pasos previos:

- Un request no debería enviarse si todavía no tiene URL.
- Una orden no debería enviarse si todavía no está pagada.
- Una conexión no debería ejecutar consultas si todavía no está autenticada.

Si todo se modela con flags como `is_ready`, `is_paid` o `is_authenticated`, cada método debe recordar validar el estado correcto. Typestate mueve esas reglas al sistema de tipos.

## Cómo se ve en Rust

En Rust, Typestate suele escribirse con structs genéricas, tipos marcadores y transiciones que consumen `self`:

```rust
pub struct RequestBuilder<State> {
    state: std::marker::PhantomData<State>,
}

pub struct MissingUrl;
pub struct ReadyToSend;
```

Los métodos se implementan solo para los estados donde tienen sentido. Por ejemplo, `send` existe para `RequestBuilder<ReadyToSend>`, pero no para `RequestBuilder<MissingUrl>`.

## Cuándo usarlo

- Cuando un flujo tiene pasos obligatorios y ordenados.
- Cuando quieres que estados inválidos no compilen.
- Cuando una transición de dominio es importante: borrador a pagado, conectado a autenticado, builder incompleto a listo.

## Cuándo evitarlo

- Si el flujo es demasiado dinámico o viene de datos externos que cambian constantemente.
- Si los estados son simples y un enum más una validación clara es suficiente.
- Si el exceso de tipos vuelve más difícil leer un caso trivial.

## Diferencia con State clásico

State clásico suele cambiar comportamiento en tiempo de ejecución usando objetos, enums o delegación. Typestate cambia la API disponible en tiempo de compilación: un valor en estado incorrecto ni siquiera tiene el método que intentas llamar.

## Ejemplos

- [x] Request builder que no permite enviar sin URL.
- [ ] Orden que solo puede enviarse después de pagarse.
- [ ] Conexión que solo ejecuta consultas después de autenticarse.

### Request builder con URL obligatoria

El módulo `request_builder` modela un builder que empieza como `RequestBuilder<MissingUrl>` y solo se convierte en `RequestBuilder<ReadyToSend>` al llamar `url`.

El ejemplo muestra cómo conservar método, headers y body durante la transición, mientras `send` solo existe para el estado listo.

## Comandos

```bash
cargo test typestate
```
