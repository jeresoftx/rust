# Decorator

Decorator añade comportamiento envolviendo un componente con capas que exponen la misma interfaz.

## Idea central

El cliente trabaja contra una abstracción común. Cada decorador recibe otro componente compatible y agrega una responsabilidad antes o después de delegar.

En Rust suele aparecer como:

- Traits para la interfaz común.
- Structs genéricas que envuelven otro implementador.
- Composición explícita de capas pequeñas.

## Ejemplos del repositorio

- Cliente HTTP con retry, timeout y logging: `src/patterns/gof/structural/decorator/http_client.rs`
- Repositorio con caché encima de almacenamiento base: `src/patterns/gof/structural/decorator/cached_repository.rs`
- Pipeline de validaciones sobre una orden.

## Guía técnica

La guía cercana al código vive en:

`patterns/gof/structural/decorator/README.md`
