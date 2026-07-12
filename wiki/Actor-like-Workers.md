# Actor-like Workers

Actor-like Workers encapsulan estado y comportamiento en un worker que recibe comandos por canales.

## Idea central

El estado pertenece al worker. Otros componentes no lo modifican directamente; envían comandos y, cuando necesitan una respuesta, incluyen un canal de retorno.

En Rust suele aparecer como:

- Un enum de comandos.
- Un hilo o tarea que procesa comandos en orden.
- Canales de respuesta `oneshot` usando `mpsc::channel`.
- Un método de apagado explícito.

## Ejemplos del repositorio

- Actor de email que recibe comandos: pendiente.
- Actor de inventario que serializa cambios: pendiente.
- Actor de métricas que agrega eventos: pendiente.

## Guía técnica

La guía cercana al código vive en:

`patterns/rust_idiomatic/actor_like_workers/README.md`
