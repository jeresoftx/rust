# Patrones Rust idiomáticos

Esta familia reúne patrones que aparecen de forma natural al escribir Rust real. Complementan los patrones GoF y aprovechan ownership, borrowing, traits, tipos fuertes, concurrencia segura y manejo explícito de errores.

## Patrones incluidos

- [Newtype](newtype/README.md)
- [Typestate](typestate/README.md)
- [RAII](raii/README.md)
- [Extension Trait](extension_trait/README.md)
- [Iterator Adapters](iterator_adapters/README.md)
- [Error Handling con Result](error_handling_result/README.md)
- [Interior Mutability](interior_mutability/README.md)
- [Message Passing](message_passing/README.md)
- [Actor-like Workers](actor_like_workers/README.md)

## Medición y property testing

- Benchmarks: no aplica por ahora. Este patrón enseña estructura, límites de responsabilidad o semántica de dominio; no hay una ruta de costo representativa que justifique Criterion todavía.
- Property testing: no aplica por ahora. Las invariantes relevantes están acotadas por los ejemplos unitarios actuales; se agregará generación aleatoria cuando aparezcan reglas algebraicas o combinatorias más amplias.
