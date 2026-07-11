# Mediator

Mediator concentra la comunicación entre participantes para que no tengan que depender directamente unos de otros.

## Idea central

Los participantes informan eventos o comandos al mediador. El mediador decide qué validaciones, acciones o notificaciones ejecutar y en qué orden.

En Rust suele aparecer como:

- Una estructura que coordina servicios de un caso de uso.
- Enums de eventos cuando el flujo se expresa como mensajes.
- Traits para participantes cuando el mediador debe trabajar con implementaciones intercambiables.

## Ejemplos del repositorio

- Coordinador de eventos entre formulario, validación y guardado: `src/patterns/gof/behavioral/mediator/form_workflow.rs`
- Sala de chat que media usuarios: `src/patterns/gof/behavioral/mediator/chat_room.rs`
- Orquestador de módulos de checkout: `src/patterns/gof/behavioral/mediator/checkout_orchestrator.rs`

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/behavioral/mediator/README.md`
