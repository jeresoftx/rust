# Design Patterns Rust

Catalogo didactico de patrones de diseno implementados en Rust. El proyecto avanza en tres niveles:

1. Patrones GoF traducidos a Rust de forma idiomatica.
2. Patrones propios del dia a dia en Rust.
3. Patrones de arquitectura para sistemas reales.

Cada ejemplo busca resolver un caso cotidiano de sistemas: configuracion, pagos, notificaciones, cache, logs, APIs, colas, reportes, permisos o procesamiento de datos.

## Ruta rapida

- Plan vivo del proyecto: [plan/catalogo-patrones-rust.md](plan/catalogo-patrones-rust.md)
- Wiki local: [wiki/Home.md](wiki/Home.md)
- Como ejecutar los ejemplos: [wiki/Como-ejecutar-los-ejemplos.md](wiki/Como-ejecutar-los-ejemplos.md)
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

La documentacion didactica vive en `wiki/`. La documentacion tecnica cercana al codigo vivira junto a cada patron conforme se agreguen ejemplos.

## Comandos

```bash
cargo fmt --check
cargo check
cargo test
```

## Convenciones

- Cada patron tiene documentacion propia.
- Cada ejemplo tiene una prueba o assert ejecutable.
- Cada ejemplo practico se agrega en un commit individual.
- El README funciona como portada; la wiki funciona como guia de estudio.
- El plan marca el avance real del proyecto.

## Fases

### Fase 1: Patrones GoF

- Creacionales: Abstract Factory, Builder, Factory Method, Prototype, Singleton.
- Estructurales: Adapter, Bridge, Composite, Decorator, Facade, Flyweight, Proxy.
- Comportamiento: Chain of Responsibility, Command, Interpreter, Iterator, Mediator, Memento, Observer, State, Strategy, Template Method, Visitor.

### Fase 2: Patrones Rust idiomaticos

Newtype, Typestate, RAII, Extension Trait, Iterator Adapters, manejo de errores con `Result`, interior mutability, message passing y workers estilo actor.

### Fase 3: Patrones de arquitectura

Layered Architecture, Hexagonal Architecture, Clean Architecture, DDD tactico, CQRS, Event Sourcing, Repository and Unit of Work, Service Layer, Pipeline Architecture y Plugin Architecture.
