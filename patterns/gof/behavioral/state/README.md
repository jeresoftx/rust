# State

## Intención

State permite que un objeto cambie su comportamiento cuando cambia su estado interno.

## Problema cotidiano

En sistemas reales muchos flujos dependen del estado actual:

- Una orden pendiente puede pagarse, pero una enviada ya no debería cancelarse.
- Una sesión de autenticación cambia entre anónima, verificada y bloqueada.
- Un ticket de soporte avanza por estados con reglas distintas.

Si estas reglas viven como `if` dispersos, cada cambio de estado se vuelve frágil. State concentra transiciones y comportamiento permitido para que el código exprese el flujo del dominio.

## Cómo se ve en Rust

En Rust, State suele representarse con enums y métodos que devuelven `Result`, porque el compilador ayuda a listar estados y transiciones válidas. También puede modelarse con tipos separados cuando quieres que transiciones inválidas sean imposibles en compilación.

Para ejemplos de negocio cotidianos, un enum de estado más una estructura de contexto suele ser claro y mantenible.

## Cuándo usarlo

- Cuando el comportamiento depende fuertemente del estado actual.
- Cuando las transiciones válidas importan para el negocio.
- Cuando quieres eliminar condicionales repetidos sobre el mismo estado.

## Cuándo evitarlo

- Si solo hay dos ramas simples y estables.
- Si una tabla de reglas expresa mejor el flujo.
- Si la abstracción es más compleja que las transiciones reales.

## Diferencia con Strategy

Strategy cambia un algoritmo elegido por el cliente. State cambia el comportamiento porque el objeto transitó a otro estado interno.

## Ejemplos

- [x] Flujo de orden pendiente, pagada, enviada y cancelada.
- [ ] Máquina de estados de autenticación.
- [ ] Lifecycle de ticket de soporte.

### Flujo de orden

El módulo `order_flow` modela una orden que transita entre `Pending`, `Paid`, `Shipped` y `Cancelled`.

El ejemplo muestra cómo el estado actual decide qué operaciones son válidas y qué errores se devuelven cuando la transición no corresponde.

## Comandos

```bash
cargo test state
```
