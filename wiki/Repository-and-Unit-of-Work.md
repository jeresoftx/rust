# Repository and Unit of Work

Repository abstrae colecciones de entidades y Unit of Work coordina cuándo confirmar o descartar cambios.

## Idea central

Los casos de uso trabajan con repositorios en vez de detalles de base de datos. Cuando una operación toca varios cambios, la unidad de trabajo los agrupa para confirmar todo junto o revertirlo.

En Rust suele aparecer como:

- Traits para repositorios.
- Implementaciones en memoria para pruebas.
- Estructuras con cambios pendientes.
- Métodos `commit` y `rollback`.
- Errores explícitos para fallos transaccionales.

## Ejemplos del repositorio

- Repositorio en memoria para pruebas: `in_memory_repository`.
- Unidad de trabajo para confirmar varios cambios: pendiente.
- Transacción simulada con rollback: pendiente.

## Guía técnica

La guía cercana al código vive en:

`patterns/architecture/repository_unit_of_work/README.md`
