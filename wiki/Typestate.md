# Typestate

Typestate usa el sistema de tipos para representar estados válidos de un flujo.

## Idea central

Un valor cambia de tipo conforme avanza por un proceso. Cada estado expone solo las operaciones válidas para ese punto.

En Rust suele aparecer como:

- Structs genéricas con un parámetro de estado.
- Tipos marcadores como `MissingUrl`, `Paid` o `Authenticated`.
- `PhantomData` cuando el estado vive en el tipo, pero no en los datos.
- Métodos que consumen `self` para producir el siguiente estado.

## Ejemplos del repositorio

- Request builder que no permite enviar sin URL: pendiente.
- Orden que solo puede enviarse después de pagarse: pendiente.
- Conexión que solo ejecuta consultas después de autenticarse: pendiente.

## Guía técnica

La guía cercana al código vive en:

`patterns/rust_idiomatic/typestate/README.md`
