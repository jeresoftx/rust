# Message Passing

Message Passing coordina componentes enviando mensajes entre productores y consumidores en lugar de compartir estado mutable directamente.

## Idea central

Un componente envía datos a través de un canal y otro componente los recibe para procesarlos. En Rust, esto combina bien con ownership: el mensaje se mueve al receptor y queda claro quién lo controla.

En Rust suele aparecer como:

- Workers que reciben jobs por canales.
- Fan-out explícito de eventos a varios consumidores.
- Pipelines con etapas conectadas por canales.
- Cierre de canales para indicar fin de trabajo.

## Ejemplos del repositorio

- Worker que procesa jobs desde un canal: `job_worker`.
- Fan-out de eventos a consumidores: pendiente.
- Pipeline de etapas con canales: pendiente.

## Guía técnica

La guía cercana al código vive en:

`patterns/rust_idiomatic/message_passing/README.md`
