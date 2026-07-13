# Rust Design Patterns

Catálogo didáctico de patrones de diseño implementados en Rust. El proyecto avanza en cuatro niveles:

1. Patrones GoF traducidos a Rust de forma idiomática.
2. Patrones propios del día a día en Rust.
3. Patrones de arquitectura para sistemas reales.
4. Patrones de sistemas distribuidos y resiliencia.

Cada ejemplo busca resolver un caso cotidiano de sistemas: configuración, pagos, notificaciones, caché, logs, APIs, colas, reportes, permisos o procesamiento de datos.

Estado actual: el catálogo de cuatro fases está completo. La Fase 4 cerró los patrones de resiliencia y sistemas distribuidos con el mismo ritmo de documentación, pruebas y commits individuales.

## Ruta rápida

- Guía de arranque para agentes: [AGENTS.md](AGENTS.md)
- Estado y rumbo del repositorio: [ROADMAP.md](ROADMAP.md)
- Plan vivo del proyecto: [plan/catalogo-patrones-rust.md](plan/catalogo-patrones-rust.md)
- Checklist de alineación RFC-0001: [plan/estandar-rfc-0001.md](plan/estandar-rfc-0001.md)
- Wiki local: [wiki/Home.md](wiki/Home.md)
- Cómo ejecutar los ejemplos: [wiki/Como-ejecutar-los-ejemplos.md](wiki/Como-ejecutar-los-ejemplos.md)
- Ruta de aprendizaje: [wiki/Ruta-de-aprendizaje.md](wiki/Ruta-de-aprendizaje.md)
- Licencias del código y contenido: [LICENSE.md](LICENSE.md)

## Estructura

```text
.
├── Cargo.toml
├── AGENTS.md
├── ROADMAP.md
├── README.md
├── benches/
├── diagrams/
├── plan/
├── patterns/
│   ├── gof/
│   ├── rust_idiomatic/
│   ├── architecture/
│   └── distributed_systems/
├── src/
├── tests/
└── wiki/
```

La documentación didáctica vive en `wiki/`. La documentación técnica cercana al código vive junto a cada patrón dentro de `patterns/`.

## Comandos

```bash
cargo fmt --check
cargo clippy --all-targets
cargo test
cargo bench --no-run
```

## Convenciones

- Cada patrón tiene documentación propia.
- Cada ejemplo tiene una prueba o assert ejecutable.
- Cada patrón declara si benchmarks y property testing aplican.
- Cada ejemplo práctico se agrega en un commit pequeño y descriptivo.
- El README funciona como portada; la wiki funciona como guía de estudio.
- El plan marca el avance real del proyecto.

## Licencias

El código Rust se publica como `MIT OR Apache-2.0`. El contenido educativo de
`wiki/`, `patterns/`, `diagrams/`, `assets/` y Markdown se publica como
CC BY-SA 4.0. El índice canónico está en [LICENSE.md](LICENSE.md).

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

Retry with Backoff, Circuit Breaker, Bulkhead, Rate Limiting, Idempotency Key, Outbox Pattern, Saga / Process Manager, Health Checks y Readiness, Cache Aside y Leader Election simulado.
