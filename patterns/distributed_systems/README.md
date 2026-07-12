# Sistemas Distribuidos y Resiliencia

Esta familia reúne patrones para construir servicios que toleran fallas parciales, latencia, duplicados, límites de capacidad y comunicación entre procesos.

Los ejemplos son simulados y deterministas para que puedan ejecutarse con `cargo test` sin depender de red, tiempo real ni servicios externos.

## Patrones incluidos en la fase

- [Retry with Backoff](retry_with_backoff/README.md)
- [ ] Circuit Breaker.
- [ ] Bulkhead.
- [ ] Rate Limiting.
- [ ] Idempotency Key.
- [ ] Outbox Pattern.
- [ ] Saga / Process Manager.
- [ ] Health Checks y readiness.
- [ ] Cache Aside.
- [ ] Leader Election simulado.
