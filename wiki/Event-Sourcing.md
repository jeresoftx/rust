# Event Sourcing

Event Sourcing guarda hechos del dominio como eventos y reconstruye el estado aplicándolos en orden.

## Idea central

El estado actual es una consecuencia de la historia. En lugar de persistir solo el último valor, se guardan eventos inmutables que permiten reproducir, auditar y proyectar cambios.

En Rust suele aparecer como:

- Enums para representar eventos del dominio.
- Funciones `apply` para actualizar estado.
- Streams de eventos por agregado.
- Proyecciones o read models derivados.
- Snapshots para streams largos.

## Ejemplos del repositorio

- Cuenta bancaria reconstruida desde eventos: pendiente.
- Auditoría de cambios de orden: pendiente.
- Snapshots para acelerar reconstrucción: pendiente.

## Guía técnica

La guía cercana al código vive en:

`patterns/architecture/event_sourcing/README.md`
