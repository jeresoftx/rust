# Observer

Observer distribuye eventos o cambios de estado desde un publicador hacia varios suscriptores independientes.

## Idea central

El publicador emite un evento. Los observadores registrados reciben una copia o vista de ese evento y reaccionan sin que el publicador conozca sus detalles internos.

En Rust suele aparecer como:

- Traits de suscriptor cuando hay comportamiento intercambiable.
- Closures o callbacks cuando el flujo es pequeño.
- Canales o buses de eventos cuando los consumidores son asíncronos.

## Ejemplos del repositorio

- Eventos de dominio para orden creada: `src/patterns/gof/behavioral/observer/order_events.rs`
- Suscriptores de métricas y logs: `src/patterns/gof/behavioral/observer/metrics_logs.rs`
- Notificaciones al cambiar estado de inventario: pendiente.

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/behavioral/observer/README.md`
