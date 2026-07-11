# Memento

Memento guarda snapshots de estado para poder restaurar un objeto sin revelar cómo está representado por dentro.

## Idea central

El objeto que cambia crea un snapshot. Otro componente puede conservar ese snapshot y pedir una restauración después, sin manipular directamente los campos internos.

En Rust suele aparecer como:

- Structs clonables que representan estados seguros.
- Métodos `save` y `restore` en el objeto editable.
- Historiales que almacenan snapshots y controlan undo o rollback.

## Ejemplos del repositorio

- Snapshots de configuración para rollback: pendiente.
- Historial de cambios de documento: pendiente.
- Restaurar estado de carrito: pendiente.

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/behavioral/memento/README.md`
