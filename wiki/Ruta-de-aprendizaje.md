# Ruta de aprendizaje

La ruta recomendada va de patrones pequeños a estructuras de sistema completas.

## 1. Base de Rust para patrones

Antes de estudiar patrones conviene tener claros estos conceptos:

- Traits y trait objects.
- Generics.
- Ownership y borrowing.
- `Result` y errores de dominio.
- Módulos y visibilidad.
- Tests con `cargo test`.

## 2. Patrones GoF

Los patrones GoF ayudan a nombrar soluciones recurrentes. En Rust no siempre se implementan igual que en lenguajes OOP clásicos; muchas veces se reemplaza herencia por traits, enums, generics o composición.

## 3. Patrones Rust idiomáticos

Después de GoF, estudiamos soluciones que aparecen mucho en Rust real: Newtype, Typestate, RAII, iteradores, message passing e interior mutability.

## 4. Patrones de arquitectura

Finalmente conectamos los patrones con diseño de aplicaciones completas: capas, puertos y adaptadores, Clean Architecture, DDD táctico, CQRS, Event Sourcing, Repository and Unit of Work, Service Layer, Pipeline Architecture y Plugin Architecture.
