# Sistemas Distribuidos y Resiliencia

Esta familia reúne patrones para construir servicios que toleran fallas parciales, latencia, duplicados, límites de capacidad y comunicación entre procesos.

Los ejemplos son simulados y deterministas para que puedan ejecutarse con `cargo test` sin depender de red, tiempo real ni servicios externos.

## Patrones incluidos en la fase

- [Retry with Backoff](retry_with_backoff/README.md)
- [Circuit Breaker](circuit_breaker/README.md)
- [Bulkhead](bulkhead/README.md)
- [Rate Limiting](rate_limiting/README.md)
- [Idempotency Key](idempotency_key/README.md)
- [Outbox Pattern](outbox_pattern/README.md)
- [Saga / Process Manager](saga_process_manager/README.md)
- [Health Checks y Readiness](health_checks_readiness/README.md)
- [Cache Aside](cache_aside/README.md)
- [Leader Election simulado](leader_election/README.md)

## Medición y property testing

- Benchmarks: no aplica por ahora. Este patrón enseña estructura, límites de responsabilidad o semántica de dominio; no hay una ruta de costo representativa que justifique Criterion todavía.
- Property testing: no aplica por ahora. Las invariantes relevantes están acotadas por los ejemplos unitarios actuales; se agregará generación aleatoria cuando aparezcan reglas algebraicas o combinatorias más amplias.
