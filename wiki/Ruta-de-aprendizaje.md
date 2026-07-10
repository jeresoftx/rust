# Ruta de aprendizaje

La ruta recomendada va de patrones pequenos a estructuras de sistema completas.

## 1. Base de Rust para patrones

Antes de estudiar patrones conviene tener claros estos conceptos:

- Traits y trait objects.
- Generics.
- Ownership y borrowing.
- `Result` y errores de dominio.
- Modulos y visibilidad.
- Tests con `cargo test`.

## 2. Patrones GoF

Los patrones GoF ayudan a nombrar soluciones recurrentes. En Rust no siempre se implementan igual que en lenguajes OOP clasicos; muchas veces se reemplaza herencia por traits, enums, generics o composicion.

## 3. Patrones Rust idiomaticos

Despues de GoF, estudiamos soluciones que aparecen mucho en Rust real: Newtype, Typestate, RAII, iteradores, message passing e interior mutability.

## 4. Patrones de arquitectura

Finalmente conectamos los patrones con diseno de aplicaciones completas: capas, puertos y adaptadores, CQRS, Event Sourcing y pipelines.
