# Patrones de sistemas distribuidos y resiliencia

La Fase 4 reúne patrones para construir servicios que se comportan mejor frente a fallas parciales, latencia, duplicados, límites de capacidad y coordinación entre procesos.

Los ejemplos del repositorio son deterministas y se ejecutan con pruebas unitarias, sin depender de red ni infraestructura externa.

## Patrones de la fase

- [Retry with Backoff](Retry-with-Backoff.md)
- [Circuit Breaker](Circuit-Breaker.md)
- [Bulkhead](Bulkhead.md)
- [Rate Limiting](Rate-Limiting.md)
- [Idempotency Key](Idempotency-Key.md)
- [Outbox Pattern](Outbox-Pattern.md)
- [Saga / Process Manager](Saga-Process-Manager.md)
- [Health Checks y Readiness](Health-Checks-y-Readiness.md)
- [Cache Aside](Cache-Aside.md)
- [Leader Election simulado](Leader-Election-simulado.md)

Cada patrón debe explicar qué problema operativo resuelve, cómo simularlo en Rust y qué riesgos aparecen en sistemas reales.
