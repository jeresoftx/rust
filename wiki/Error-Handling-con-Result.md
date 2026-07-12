# Error Handling con `Result`

`Result<T, E>` modela éxito y falla de forma explícita en la firma de una función.

## Idea central

Una función que puede fallar devuelve `Result`. El llamador debe manejar `Ok` y `Err`, o propagar el error con `?`.

En Rust suele aparecer como:

- Enums de error de dominio.
- Conversión de errores entre capas.
- `map_err` y `From` para traducir fallas.
- `?` para propagar errores compatibles.
- Validaciones fail-fast o acumuladas según el caso de uso.

## Ejemplos del repositorio

- Errores de dominio para checkout: `checkout_domain`.
- Conversión de errores de infraestructura a errores de aplicación: `infrastructure_conversion`.
- Validación acumulada y validación fail-fast: `validation_modes`.

## Guía técnica

La guía cercana al código vive en:

`patterns/rust_idiomatic/error_handling_result/README.md`
