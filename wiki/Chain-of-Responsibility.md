# Chain of Responsibility

Chain of Responsibility pasa una solicitud por una cadena de manejadores hasta que uno la procesa, la rechaza o todos la dejan pasar.

## Idea central

Cada manejador conoce una regla pequeña. El cliente no necesita saber qué paso resolverá la solicitud; solo conoce la cadena completa.

En Rust suele aparecer como:

- Traits para definir manejadores con una interfaz común.
- Enums y `match` cuando las decisiones son cerradas y explícitas.
- Listas de closures o funciones cuando la cadena es ligera.

## Ejemplos del repositorio

- Pipeline de validación de requests: `src/patterns/gof/behavioral/chain_of_responsibility/request_validation.rs`
- Resolución de soporte por niveles.
- Filtros de moderación de contenido.

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/behavioral/chain_of_responsibility/README.md`
