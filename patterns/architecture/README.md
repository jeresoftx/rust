# Patrones de arquitectura

Esta familia conecta los ejemplos pequeños con estructuras de aplicación completas. Los patrones muestran separación de responsabilidades, límites de dominio, dependencias reemplazables y formas de organizar código de sistemas reales.

## Patrones incluidos

- [Layered Architecture](layered_architecture/README.md)
- [Hexagonal Architecture](hexagonal_architecture/README.md)
- [Clean Architecture](clean_architecture/README.md)
- [Domain-Driven Design Táctico](domain_driven_design_tactical/README.md)
- [CQRS](cqrs/README.md)
- [Event Sourcing](event_sourcing/README.md)
- [Repository and Unit of Work](repository_unit_of_work/README.md)
- [Service Layer](service_layer/README.md)
- [Pipeline Architecture](pipeline_architecture/README.md)
- [Plugin Architecture](plugin_architecture/README.md)

## Medición y property testing

- Benchmarks: no aplica por ahora. Este patrón enseña estructura, límites de responsabilidad o semántica de dominio; no hay una ruta de costo representativa que justifique Criterion todavía.
- Property testing: no aplica por ahora. Las invariantes relevantes están acotadas por los ejemplos unitarios actuales; se agregará generación aleatoria cuando aparezcan reglas algebraicas o combinatorias más amplias.
