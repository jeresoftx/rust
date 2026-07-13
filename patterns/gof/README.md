# Patrones GoF

Esta familia reúne los 23 patrones clásicos del libro Gang of Four, adaptados a Rust sin forzar herencia ni jerarquías innecesarias. Cada patrón tiene documentación técnica, ejemplos cotidianos y pruebas ejecutables.

## Creacionales

- [Abstract Factory](creational/abstract_factory/README.md)
- [Builder](creational/builder/README.md)
- [Factory Method](creational/factory_method/README.md)
- [Prototype](creational/prototype/README.md)
- [Singleton](creational/singleton/README.md)

## Estructurales

- [Adapter](structural/adapter/README.md)
- [Bridge](structural/bridge/README.md)
- [Composite](structural/composite/README.md)
- [Decorator](structural/decorator/README.md)
- [Facade](structural/facade/README.md)
- [Flyweight](structural/flyweight/README.md)
- [Proxy](structural/proxy/README.md)

## Comportamiento

- [Chain of Responsibility](behavioral/chain_of_responsibility/README.md)
- [Command](behavioral/command/README.md)
- [Interpreter](behavioral/interpreter/README.md)
- [Iterator](behavioral/iterator/README.md)
- [Mediator](behavioral/mediator/README.md)
- [Memento](behavioral/memento/README.md)
- [Observer](behavioral/observer/README.md)
- [State](behavioral/state/README.md)
- [Strategy](behavioral/strategy/README.md)
- [Template Method](behavioral/template_method/README.md)
- [Visitor](behavioral/visitor/README.md)

## Medición y property testing

- Benchmarks: no aplica por ahora. Este patrón enseña estructura, límites de responsabilidad o semántica de dominio; no hay una ruta de costo representativa que justifique Criterion todavía.
- Property testing: no aplica por ahora. Las invariantes relevantes están acotadas por los ejemplos unitarios actuales; se agregará generación aleatoria cuando aparezcan reglas algebraicas o combinatorias más amplias.
