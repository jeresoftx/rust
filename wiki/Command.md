# Command

Command representa una acción como dato para poder ejecutarla, guardarla, auditarla, deshacerla o encolarla.

## Idea central

El cliente construye un comando y un invocador lo ejecuta sin conocer todos los detalles internos de la acción.

En Rust suele aparecer como:

- Enums cuando el conjunto de comandos es cerrado.
- Traits cuando quieres comandos extensibles.
- Structs pequeñas cuando cada comando tiene datos propios.

## Ejemplos del repositorio

- Comandos de CLI para crear, actualizar y borrar usuarios: `src/patterns/gof/behavioral/command/user_cli.rs`
- Acciones reversibles para editar una orden: `src/patterns/gof/behavioral/command/reversible_order.rs`
- Cola de trabajos con comandos serializables.

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/behavioral/command/README.md`
