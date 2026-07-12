# Design Patterns Rust

Catálogo didáctico de patrones de diseño implementados en Rust. El proyecto avanza en cuatro niveles:

1. Patrones GoF traducidos a Rust de forma idiomática.
2. Patrones propios del día a día en Rust.
3. Patrones de arquitectura para sistemas reales.
4. Patrones de sistemas distribuidos y resiliencia.

Cada ejemplo busca resolver un caso cotidiano de sistemas: configuración, pagos, notificaciones, caché, logs, APIs, colas, reportes, permisos o procesamiento de datos.

Estado actual: el catálogo inicial de tres fases está completo. La Fase 4 agrega patrones de resiliencia y sistemas distribuidos con el mismo ritmo de documentación, pruebas y commits individuales.

## Ruta rápida

- Plan vivo del proyecto: [plan/catalogo-patrones-rust.md](plan/catalogo-patrones-rust.md)
- Wiki local: [wiki/Home.md](wiki/Home.md)
- Cómo ejecutar los ejemplos: [wiki/Como-ejecutar-los-ejemplos.md](wiki/Como-ejecutar-los-ejemplos.md)
- Ruta de aprendizaje: [wiki/Ruta-de-aprendizaje.md](wiki/Ruta-de-aprendizaje.md)

## Estructura

```text
.
├── Cargo.toml
├── README.md
├── plan/
├── src/
├── tests/
└── wiki/
```

La documentación didáctica vive en `wiki/`. La documentación técnica cercana al código vive junto a cada patrón dentro de `patterns/`.

## Comandos

```bash
cargo fmt --check
cargo check
cargo test --quiet
```

## Convenciones

- Cada patrón tiene documentación propia.
- Cada ejemplo tiene una prueba o assert ejecutable.
- Cada ejemplo práctico se agrega en un commit individual.
- El README funciona como portada; la wiki funciona como guía de estudio.
- El plan marca el avance real del proyecto.

## Fases

### Fase 1: Patrones GoF

- Creacionales: Abstract Factory, Builder, Factory Method, Prototype, Singleton.
- Estructurales: Adapter, Bridge, Composite, Decorator, Facade, Flyweight, Proxy.
- Comportamiento: Chain of Responsibility, Command, Interpreter, Iterator, Mediator, Memento, Observer, State, Strategy, Template Method, Visitor.

### Fase 2: Patrones Rust idiomáticos

Newtype, Typestate, RAII, Extension Trait, Iterator Adapters, manejo de errores con `Result`, interior mutability, message passing y workers estilo actor.

### Fase 3: Patrones de arquitectura

Layered Architecture, Hexagonal Architecture, Clean Architecture, DDD táctico, CQRS, Event Sourcing, Repository and Unit of Work, Service Layer, Pipeline Architecture y Plugin Architecture.

### Fase 4: Sistemas distribuidos y resiliencia

Retry with Backoff, Circuit Breaker, Bulkhead, Rate Limiting, Idempotency Key, Outbox Pattern, Saga / Process Manager, Health Checks, Cache Aside y Leader Election simulado.
