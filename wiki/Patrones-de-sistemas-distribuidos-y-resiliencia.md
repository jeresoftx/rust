# Patrones de sistemas distribuidos y resiliencia

La Fase 4 reúne patrones para construir servicios que se comportan mejor frente a fallas parciales, latencia, duplicados, límites de capacidad y coordinación entre procesos.

Los ejemplos del repositorio son deterministas y se ejecutan con pruebas unitarias, sin depender de red ni infraestructura externa.

## Patrones de la fase

- Retry with Backoff
- Circuit Breaker
- Bulkhead
- Rate Limiting
- Idempotency Key
- Outbox Pattern
- Saga / Process Manager
- Health Checks y readiness
- Cache Aside
- Leader Election simulado

Cada patrón debe explicar qué problema operativo resuelve, cómo simularlo en Rust y qué riesgos aparecen en sistemas reales.
