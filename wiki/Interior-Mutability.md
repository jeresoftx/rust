# Interior Mutability

Interior Mutability permite modificar estado interno desde una referencia compartida cuando esa mutación está encapsulada y tiene una razón clara.

## Idea central

Rust normalmente requiere `&mut` para modificar estado. Con Interior Mutability, tipos como `RefCell`, `Mutex` y `RwLock` permiten mover esa regla a tiempo de ejecución o a una primitiva de sincronización.

En Rust suele aparecer como:

- `RefCell` para mutación interna en un solo hilo.
- `Mutex` para exclusión mutua entre hilos.
- `RwLock` para muchas lecturas concurrentes y escrituras exclusivas.
- Cachés, métricas y contadores internos.

## Ejemplos del repositorio

- Caché en memoria con `RefCell`: `refcell_cache`.
- Contador compartido con `Mutex`: `mutex_counter`.
- Lectura concurrente con `RwLock`: `rwlock_config`.

## Guía técnica

La guía cercana al código vive en:

`patterns/rust_idiomatic/interior_mutability/README.md`
