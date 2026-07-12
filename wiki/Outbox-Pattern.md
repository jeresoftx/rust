# Outbox Pattern

Outbox Pattern guarda cambios de dominio y mensajes pendientes en la misma unidad de trabajo. Después, un publicador lee la bandeja de salida, envía los mensajes y marca cuáles ya salieron.

## Qué problema resuelve

- Evita perder eventos cuando falla el broker.
- Mantiene entidad y mensaje en un mismo commit lógico.
- Permite publicar de forma asíncrona.
- Hace reintentables los mensajes pendientes.
- Reduce inconsistencias entre base de datos y eventos.

## Ejemplos del repositorio

- [x] Guardar entidad y evento en la misma unidad de trabajo.
- [x] Publicador que marca mensajes como enviados.
- [x] Reintento de mensajes pendientes.

## Código

- Documentación local: `patterns/distributed_systems/outbox_pattern/README.md`
- Módulo Rust: `src/patterns/distributed_systems/outbox_pattern.rs`
- Ejemplo de unidad de trabajo: `src/patterns/distributed_systems/outbox_pattern/unit_of_work.rs`
- Ejemplo de publicador: `src/patterns/distributed_systems/outbox_pattern/publisher_marks_sent.rs`
- Ejemplo de reintento de pendientes: `src/patterns/distributed_systems/outbox_pattern/retry_pending.rs`
