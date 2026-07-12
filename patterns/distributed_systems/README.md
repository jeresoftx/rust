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
- [ ] Saga / Process Manager.
- [ ] Health Checks y readiness.
- [ ] Cache Aside.
- [ ] Leader Election simulado.
