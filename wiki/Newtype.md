# Newtype

Newtype crea tipos pequeños alrededor de valores existentes para darles significado de dominio.

## Idea central

Un valor como `String`, `u64` o `i64` puede representar muchas cosas. Newtype crea un tipo distinto para que el compilador ayude a evitar mezclas accidentales.

En Rust suele aparecer como:

- Tupla structs como `UserId(String)`.
- Constructores que validan antes de crear el valor.
- Métodos de acceso controlados.
- Traits como `Display`, `Eq`, `Hash`, `Clone` y `Copy` cuando hacen sentido.

## Ejemplos del repositorio

- IDs tipados para usuario, orden y producto: `src/patterns/rust_idiomatic/newtype/typed_ids.rs`
- Dinero y moneda sin mezclar unidades: `src/patterns/rust_idiomatic/newtype/money_currency.rs`
- Tipos seguros para email y token: pendiente.

## Guía técnica

La guía cercana al código vive en:

`patterns/rust_idiomatic/newtype/README.md`
