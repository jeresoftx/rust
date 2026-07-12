# RAII

RAII liga la vida de un recurso a la vida de un valor.

## Idea central

Cuando un valor adquiere un recurso, también se vuelve responsable de liberarlo. Al salir de scope, Rust llama `Drop` y ejecuta la limpieza asociada.

En Rust suele aparecer como:

- Guards que liberan locks.
- Valores que hacen rollback si no se confirmó una operación.
- Wrappers que limpian archivos, handles o recursos temporales.
- Scopes pequeños que delimitan claramente cuándo vive un recurso.

## Ejemplos del repositorio

- Lock guard para secciones críticas: `src/patterns/rust_idiomatic/raii/lock_guard.rs`
- Transacción que hace rollback si no se confirma: `src/patterns/rust_idiomatic/raii/transaction_rollback.rs`
- Archivo temporal que se limpia al salir de scope: `src/patterns/rust_idiomatic/raii/temporary_file_cleanup.rs`

## Guía técnica

La guía cercana al código vive en:

`patterns/rust_idiomatic/raii/README.md`
