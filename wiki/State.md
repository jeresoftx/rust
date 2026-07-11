# State

State modela objetos cuyo comportamiento cambia según su estado actual.

## Idea central

El contexto conserva un estado. Cada operación valida si la transición tiene sentido y mueve el contexto al siguiente estado, o rechaza el intento con un error claro.

En Rust suele aparecer como:

- Enums para representar estados cerrados.
- Métodos que devuelven `Result` para transiciones inválidas.
- Tipos separados cuando se quiere reforzar el flujo en compilación.

## Ejemplos del repositorio

- Flujo de orden pendiente, pagada, enviada y cancelada: `src/patterns/gof/behavioral/state/order_flow.rs`
- Máquina de estados de autenticación: `src/patterns/gof/behavioral/state/auth_machine.rs`
- Lifecycle de ticket de soporte: pendiente.

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/behavioral/state/README.md`
