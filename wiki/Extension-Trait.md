# Extension Trait

Extension Trait agrega métodos útiles a tipos existentes mediante traits propios.

## Idea central

Un trait local puede implementarse para tipos de la biblioteca estándar o de otros módulos. Al importar el trait, esos métodos quedan disponibles con sintaxis de método.

En Rust suele aparecer como:

- Helpers para `str`, `String` o slices.
- Métodos de dominio sobre `Result` y `Option`.
- Utilidades pequeñas para colecciones.
- APIs más legibles sin crear wrappers para cada caso.

## Ejemplos del repositorio

- Helpers de strings para normalizar entradas: pendiente.
- Helpers de `Result` para mapear errores de dominio: pendiente.
- Helpers de colecciones para paginar resultados: pendiente.

## Guía técnica

La guía cercana al código vive en:

`patterns/rust_idiomatic/extension_trait/README.md`
